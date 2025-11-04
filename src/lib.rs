use std::fs::File;
use std::io::{Read, Write, BufWriter};
use byteorder::{LittleEndian, WriteBytesExt};

// ==================== 公共结构体和枚举 ====================

/// 音频格式枚举
#[derive(Debug, Clone, PartialEq)]
pub enum AudioFormat {
    Wav,
    Mp3,
}

/// 音频质量设置
#[derive(Debug, Clone, PartialEq)]
pub enum AudioQuality {
    Low,     // 低质量
    Medium,  // 中等质量
    High,    // 高质量
    Best,    // 最佳质量
}

/// MP3 比特率枚举
#[derive(Debug, Clone, PartialEq)]
pub enum Mp3Bitrate {
    Kbps64,
    Kbps128,
    Kbps192,
    Kbps256,
    Kbps320,
}

// ==================== 工具函数 ====================

/// 检查文件是否为 PCM 文件
fn is_pcm_file(file_path: &str) -> bool {
    file_path.ends_with(".pcm")
}

// FFI 模块（用于移动端集成）
pub mod ffi;

// 检查文件是否存在
fn file_exists(file_path: &str) -> bool {
    std::path::Path::new(file_path).exists()
}

// ==================== 配置结构体 ====================

/// MP3 转换配置
#[derive(Debug, Clone, PartialEq)]
pub struct Mp3Config {
    pub sample_rate: u32,
    pub channels: u8,
    pub bitrate: Mp3Bitrate,
    pub quality: AudioQuality,
}

impl Mp3Config {
    /// 创建新的 MP3 配置
    pub fn new(sample_rate: u32, channels: u8, bitrate: Mp3Bitrate, quality: AudioQuality) -> Self {
        Mp3Config {
            sample_rate,
            channels,
            bitrate,
            quality,
        }
    }

    /// 创建默认 MP3 配置
    pub fn default() -> Self {
        Mp3Config {
            sample_rate: 44100,
            channels: 2,
            bitrate: Mp3Bitrate::Kbps192,
            quality: AudioQuality::High,
        }
    }
}

impl Default for Mp3Config {
    fn default() -> Self {
        Mp3Config::default()
    }
}

/// 通用音频转换配置
#[derive(Debug, Clone, PartialEq)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub channels: u8,
    pub bits_per_sample: u16,
}

impl AudioConfig {
    pub fn new(sample_rate: u32, channels: u8, bits_per_sample: u16) -> Self {
        AudioConfig {
            sample_rate,
            channels,
            bits_per_sample,
        }
    }

    pub fn default() -> Self {
        AudioConfig {
            sample_rate: 44100,
            channels: 2,
            bits_per_sample: 16,
        }
    }
}

// ==================== 配置推断函数 ====================

/// 从文件名智能推断音频配置
pub fn infer_audio_config_from_filename(filename: &str) -> AudioConfig {
    let filename_lower = filename.to_lowercase();
    
    // 推断采样率 - 按照更精确的顺序匹配，避免子串匹配问题
    let sample_rate = if filename_lower.contains("96k") {
        96000
    } else if filename_lower.contains("48k") {
        48000
    } else if filename_lower.contains("44.1k") || filename_lower.contains("44k") {
        44100
    } else if filename_lower.contains("32k") {
        32000
    } else if filename_lower.contains("22k") {
        22050
    } else if filename_lower.contains("16k") {
        16000
    } else if filename_lower.contains("8k") {
        8000
    } else {
        // 默认采样率
        44100
    };
    
    // 推断声道数
    let channels = if filename_lower.contains("单声道") || filename_lower.contains("mono") {
        1
    } else if filename_lower.contains("立体声") || filename_lower.contains("stereo") || filename_lower.contains("双声道") {
        2
    } else {
        // 根据其他线索推断
        if filename_lower.contains("1ch") {
            1
        } else if filename_lower.contains("2ch") {
            2
        } else {
            // 默认立体声
            2
        }
    };
    
    // 推断位深度
    let bits_per_sample = if filename_lower.contains("8bit") {
        8
    } else if filename_lower.contains("16bit") {
        16
    } else if filename_lower.contains("24bit") {
        24
    } else if filename_lower.contains("32bit") {
        32
    } else {
        // 默认 16 位
        16
    };
    
    AudioConfig::new(sample_rate, channels, bits_per_sample)
}

/// 从音频配置创建 WAV 配置（保持兼容性）
pub fn audio_config_to_wav_config(audio_config: &AudioConfig) -> PcmToWavConfig {
    PcmToWavConfig::new(
        audio_config.sample_rate,
        audio_config.channels,
        audio_config.bits_per_sample,
    )
}

/// 从音频配置创建 MP3 配置
pub fn audio_config_to_mp3_config(audio_config: &AudioConfig, bitrate: Mp3Bitrate, quality: AudioQuality) -> Mp3Config {
    Mp3Config::new(
        audio_config.sample_rate,
        audio_config.channels,
        bitrate,
        quality,
    )
}

// ==================== 兼容性函数 ====================

/// 从文件名智能推断 PCM 配置 (保持兼容性)
pub fn infer_pcm_config_from_filename(filename: &str) -> PcmToWavConfig {
    let audio_config = infer_audio_config_from_filename(filename);
    audio_config_to_wav_config(&audio_config)
}

/// 自动转换 PCM 文件到 WAV，从文件名推断配置
/// # Arguments
/// * `input_path` - 输入 PCM 文件路径
/// * `output_path` - 输出 WAV 文件路径
/// # Returns
/// * `Result<PcmToWavConfig, Box<dyn std::error::Error>>` - 转换结果和使用的配置
pub fn auto_trans_pcm_to_wav(input_path: &str, output_path: &str) -> Result<PcmToWavConfig, Box<dyn std::error::Error>> {
    let filename = std::path::Path::new(input_path)
        .file_name()
        .ok_or("无效的文件路径")?
        .to_string_lossy();
        
    let config = infer_pcm_config_from_filename(&filename);
    
    trans_pcm_file_to_wav(input_path, output_path, Some(config.clone()))?;
    
    Ok(config)
}

// ==================== 新的通用转换函数 ====================

/// 自动转换 PCM 到指定格式
pub fn auto_convert_pcm(input_path: &str, output_path: &str, format: AudioFormat) -> Result<AudioConfig, Box<dyn std::error::Error>> {
    let filename = std::path::Path::new(input_path)
        .file_name()
        .ok_or("无效的文件路径")?
        .to_string_lossy();
        
    let audio_config = infer_audio_config_from_filename(&filename);
    
    match format {
        AudioFormat::Wav => {
            let wav_config = audio_config_to_wav_config(&audio_config);
            trans_pcm_file_to_wav(input_path, output_path, Some(wav_config))?;
        }
        AudioFormat::Mp3 => {
            let mp3_config = audio_config_to_mp3_config(&audio_config, Mp3Bitrate::Kbps192, AudioQuality::High);
            trans_pcm_file_to_mp3(input_path, output_path, Some(mp3_config))?;
        }
    }
    
    Ok(audio_config)
}

// ==================== MP3 转换函数 ====================

/// PCM 转 MP3
/// # Arguments
/// * `input_path` - 输入 PCM 文件路径  
/// * `output_path` - 输出 MP3 文件路径
/// * `config` - MP3 配置，如果为 None 则使用默认配置
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - 转换结果
pub fn trans_pcm_file_to_mp3(input_path: &str, output_path: &str, config: Option<Mp3Config>) -> Result<(), Box<dyn std::error::Error>> {
    use mp3lame_encoder::{Builder, InterleavedPcm, DualPcm, FlushNoGap};
    use std::mem::MaybeUninit;
    
    let mp3_config = config.unwrap_or_default();
    
    // 读取 PCM 数据
    let pcm_data = std::fs::read(input_path)?;
    
    // 创建 MP3 编码器
    let mut builder = Builder::new()
        .ok_or("Failed to create MP3 encoder builder (mp3lame library not available)")?;
    
    builder.set_num_channels(mp3_config.channels)
        .map_err(|e| format!("Failed to set channels: {:?}", e))?;
    
    builder.set_sample_rate(mp3_config.sample_rate)
        .map_err(|e| format!("Failed to set sample rate: {:?}", e))?;
    
    // 转换 bitrate 枚举到实际值
    let bitrate_value = match mp3_config.bitrate {
        Mp3Bitrate::Kbps64 => mp3lame_encoder::Bitrate::Kbps64,
        Mp3Bitrate::Kbps128 => mp3lame_encoder::Bitrate::Kbps128,
        Mp3Bitrate::Kbps192 => mp3lame_encoder::Bitrate::Kbps192,
        Mp3Bitrate::Kbps256 => mp3lame_encoder::Bitrate::Kbps256,
        Mp3Bitrate::Kbps320 => mp3lame_encoder::Bitrate::Kbps320,
    };
    
    builder.set_brate(bitrate_value)
        .map_err(|e| format!("Failed to set bitrate: {:?}", e))?;
    
    // 转换质量枚举
    let quality_value = match mp3_config.quality {
        AudioQuality::Low => mp3lame_encoder::Quality::Worst,
        AudioQuality::Medium => mp3lame_encoder::Quality::Good,
        AudioQuality::High => mp3lame_encoder::Quality::Best,
        AudioQuality::Best => mp3lame_encoder::Quality::Best, // 最高质量
    };
    
    builder.set_quality(quality_value)
        .map_err(|e| format!("Failed to set quality: {:?}", e))?;
    
    let mut encoder = builder.build()
        .map_err(|e| format!("Failed to build encoder: {:?}", e))?;
    
    // 转换 PCM 数据为 i16 样本
    let mut samples: Vec<i16> = Vec::new();
    for chunk in pcm_data.chunks_exact(2) {
        let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
        samples.push(sample);
    }
    
    // 创建输出缓冲区
    let mut mp3_output = vec![MaybeUninit::uninit(); pcm_data.len()]; // 预留足够空间
    let mut total_mp3_data = Vec::new();
    
    // 编码为 MP3
    if mp3_config.channels == 1 {
        // 单声道
        let interleaved = InterleavedPcm(&samples);
        let bytes_written = encoder.encode(interleaved, &mut mp3_output)
            .map_err(|e| format!("Failed to encode mono audio: {:?}", e))?;
        
        // 将编码的数据复制到最终输出
        for i in 0..bytes_written {
            unsafe {
                total_mp3_data.push(mp3_output[i].assume_init());
            }
        }
    } else {
        // 双声道 - 需要分离左右声道
        let mut left_samples = Vec::new();
        let mut right_samples = Vec::new();
        
        for chunk in samples.chunks_exact(2) {
            left_samples.push(chunk[0]);
            right_samples.push(chunk[1]);
        }
        
        let dual = DualPcm { left: &left_samples, right: &right_samples };
        let bytes_written = encoder.encode(dual, &mut mp3_output)
            .map_err(|e| format!("Failed to encode stereo audio: {:?}", e))?;
        
        // 将编码的数据复制到最终输出
        for i in 0..bytes_written {
            unsafe {
                total_mp3_data.push(mp3_output[i].assume_init());
            }
        }
    }
    
    // 完成编码 - flush 剩余数据
    let flush_bytes = encoder.flush::<FlushNoGap>(&mut mp3_output)
        .map_err(|e| format!("Failed to flush encoder: {:?}", e))?;
    
    // 将 flush 的数据添加到最终输出
    for i in 0..flush_bytes {
        unsafe {
            total_mp3_data.push(mp3_output[i].assume_init());
        }
    }
    
    // 写入文件
    std::fs::write(output_path, total_mp3_data)?;
    
    println!("Successfully converted {} to {} (MP3, {}kbps, {} channels)", 
             input_path, output_path, 
             match mp3_config.bitrate {
                 Mp3Bitrate::Kbps64 => 64,
                 Mp3Bitrate::Kbps128 => 128,
                 Mp3Bitrate::Kbps192 => 192,
                 Mp3Bitrate::Kbps256 => 256,
                 Mp3Bitrate::Kbps320 => 320,
             },
             mp3_config.channels);
    
    Ok(())
}

/// PCM 转 WAV 的配置参数
#[derive(Debug, Clone, PartialEq)]
pub struct PcmToWavConfig {
    /// 采样率，单位为 Hz
    pub sample_rate: Option<u32>,
    /// 声道数
    pub channels: Option<u8>,
    /// 每个样本的位数
    pub bits_per_sample: Option<u16>,
}

impl PcmToWavConfig {
    /// 创建默认配置
    pub fn default() -> Self {
        PcmToWavConfig {
            sample_rate: Some(44100),
            channels: Some(2),
            bits_per_sample: Some(16),
        }
    }
    
    /// 创建自定义配置
    pub fn new(sample_rate: u32, channels: u8, bits_per_sample: u16) -> Self {
        PcmToWavConfig {
            sample_rate: Some(sample_rate),
            channels: Some(channels),
            bits_per_sample: Some(bits_per_sample),
        }
    }
}

/// 将 PCM 文件转换为 WAV 文件
/// # Arguments
/// * `input_path` - 输入 PCM 文件路径
/// * `output_path` - 输出 WAV 文件路径
/// * `config` - PCM 转 WAV 的配置参数
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - 转换结果
pub fn trans_pcm_file_to_wav(input_path: &str, output_path: &str, config: Option<PcmToWavConfig>) -> Result<(), Box<dyn std::error::Error>> {
    // 1. 读取 pcm 文件
    // 检查输入文件是否为 pcm 文件
    if !is_pcm_file(input_path) {
        return Err("Input file is not a PCM file".into());
    } 
    // 判断文件是否存在
    if !std::path::Path::new(input_path).exists() {
        return Err("Input file does not exist".into());
    }
    // 读取 pcm 文件内容
    let mut input_file = File::open(input_path)?;
    let mut pcm_data = Vec::new();
    input_file.read_to_end(&mut pcm_data)?;

    // 2. 获取配置参数
    let config = config.unwrap_or_else(PcmToWavConfig::default);
    let sample_rate = config.sample_rate.unwrap_or(44100);
    let channels = config.channels.unwrap_or(2);
    let bits_per_sample = config.bits_per_sample.unwrap_or(16);

    // 计算音频参数
    let byte_rate = sample_rate * channels as u32 * (bits_per_sample / 8) as u32;
    let block_align = channels as u16 * (bits_per_sample / 8);
    let data_size = pcm_data.len() as u32;

    // 3. 创建输出文件并写入 WAV 头
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    // 写入 WAV 文件头
    write_wav_header(&mut writer, sample_rate, channels, bits_per_sample, byte_rate, block_align, data_size)?;

    // 4. 写入 PCM 数据
    writer.write_all(&pcm_data)?;

    println!("Successfully converted PCM file from {} to WAV file at {}", input_path, output_path);
    Ok(())
}

/// 写入 WAV 文件头
fn write_wav_header<W: Write>(
    writer: &mut W,
    sample_rate: u32,
    channels: u8,
    bits_per_sample: u16,
    byte_rate: u32,
    block_align: u16,
    data_size: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // RIFF 头
    writer.write_all(b"RIFF")?;
    writer.write_u32::<LittleEndian>(36 + data_size)?; // 文件大小 - 8
    writer.write_all(b"WAVE")?;

    // fmt 块
    writer.write_all(b"fmt ")?;
    writer.write_u32::<LittleEndian>(16)?; // fmt 块大小
    writer.write_u16::<LittleEndian>(1)?;  // PCM 格式
    writer.write_u16::<LittleEndian>(channels as u16)?;
    writer.write_u32::<LittleEndian>(sample_rate)?;
    writer.write_u32::<LittleEndian>(byte_rate)?;
    writer.write_u16::<LittleEndian>(block_align)?;
    writer.write_u16::<LittleEndian>(bits_per_sample)?;

    // data 块
    writer.write_all(b"data")?;
    writer.write_u32::<LittleEndian>(data_size)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_is_pcm_file() {
        assert_eq!(is_pcm_file("test.pcm"), true);
        assert_eq!(is_pcm_file("test.wav"), false);
        assert_eq!(is_pcm_file("test.mp3"), false);
    }

    #[test]
    fn test_pcm_to_wav_config_default() {
        let config = PcmToWavConfig::default();
        assert_eq!(config.sample_rate, Some(44100));
        assert_eq!(config.channels, Some(2));
        assert_eq!(config.bits_per_sample, Some(16));
    }

    #[test]
    fn test_pcm_to_wav_with_non_pcm_file() {
        let input_path = "test.wav";
        let output_path = "output.wav";
        let result = trans_pcm_file_to_wav(input_path, output_path, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not a PCM file"));
    }

    #[test]
    fn test_pcm_to_wav_with_nonexistent_file() {
        let input_path = "nonexistent.pcm";
        let output_path = "output.wav";
        let result = trans_pcm_file_to_wav(input_path, output_path, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_pcm_to_wav_success() {
        // 创建一个临时的 PCM 文件进行测试
        let input_path = "test_input.pcm";
        let output_path = "test_output.wav";
        
        // 创建一些假的 PCM 数据 (16-bit stereo, 44.1kHz)
        let pcm_data: Vec<u8> = (0..1000).flat_map(|i| {
            let sample = (i * 100) as i16;
            sample.to_le_bytes()
        }).collect();
        
        // 写入测试 PCM 文件
        fs::write(input_path, pcm_data).unwrap();
        
        // 执行转换
        let result = trans_pcm_file_to_wav(input_path, output_path, None);
        
        // 清理测试文件
        let _ = fs::remove_file(input_path);
        let _ = fs::remove_file(output_path);
        
        // 验证结果
        assert!(result.is_ok());
    }
}
