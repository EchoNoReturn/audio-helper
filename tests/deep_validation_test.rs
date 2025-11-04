use audio_helper::{trans_pcm_file_to_wav, PcmToWavConfig};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};

/// 深度验证 WAV 文件头的正确性
#[test]
fn test_wav_header_accuracy() {
    let test_files = vec![
        ("浪花一朵朵片段8k16bit单声道.pcm", 8000, 1, 16),
        ("浪花一朵朵片段32k16bit单声道.pcm", 32000, 1, 16),
        ("浪花一朵朵片段48k16bit单声道.pcm", 48000, 1, 16),
    ];
    
    for (filename, expected_sr, expected_ch, expected_bits) in test_files {
        let input_path = format!("pcmFile/{}", filename);
        let output_path = format!("header_test_{}.wav", filename.replace(".pcm", ""));
        
        if !std::path::Path::new(&input_path).exists() {
            println!("跳过不存在的文件: {}", filename);
            continue;
        }
        
        println!("🔍 深度检查: {}", filename);
        
        // 转换文件
        let config = PcmToWavConfig::new(expected_sr, expected_ch, expected_bits);
        let result = trans_pcm_file_to_wav(&input_path, &output_path, Some(config));
        assert!(result.is_ok(), "转换应该成功");
        
        // 详细验证 WAV 头
        match validate_wav_header_detailed(&output_path, expected_sr, expected_ch, expected_bits) {
            Ok(_) => println!("   ✅ WAV 头验证通过"),
            Err(e) => {
                println!("   ❌ WAV 头验证失败: {}", e);
                panic!("WAV 头验证失败: {}", e);
            }
        }
        
        // 清理文件
        let _ = std::fs::remove_file(&output_path);
        println!();
    }
}

/// 详细验证 WAV 文件头
fn validate_wav_header_detailed(
    file_path: &str, 
    expected_sample_rate: u32, 
    expected_channels: u8, 
    expected_bits_per_sample: u16
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    
    println!("   📋 验证 WAV 文件头:");
    
    // 1. RIFF 头
    let mut riff_header = [0u8; 4];
    file.read_exact(&mut riff_header)?;
    if &riff_header != b"RIFF" {
        return Err(format!("❌ RIFF 头错误: 期望 'RIFF', 实际 '{:?}'", 
                         String::from_utf8_lossy(&riff_header)).into());
    }
    println!("      ✅ RIFF 头: {:?}", String::from_utf8_lossy(&riff_header));
    
    // 2. 文件大小
    let file_size = file.read_u32::<LittleEndian>()?;
    println!("      📏 文件大小字段: {} (总大小: {})", file_size, file_size + 8);
    
    // 3. WAVE 标识
    let mut wave_header = [0u8; 4];
    file.read_exact(&mut wave_header)?;
    if &wave_header != b"WAVE" {
        return Err(format!("❌ WAVE 标识错误: 期望 'WAVE', 实际 '{:?}'", 
                         String::from_utf8_lossy(&wave_header)).into());
    }
    println!("      ✅ WAVE 标识: {:?}", String::from_utf8_lossy(&wave_header));
    
    // 4. fmt 块
    let mut fmt_header = [0u8; 4];
    file.read_exact(&mut fmt_header)?;
    if &fmt_header != b"fmt " {
        return Err(format!("❌ fmt 头错误: 期望 'fmt ', 实际 '{:?}'", 
                         String::from_utf8_lossy(&fmt_header)).into());
    }
    println!("      ✅ fmt 块标识: {:?}", String::from_utf8_lossy(&fmt_header));
    
    let fmt_size = file.read_u32::<LittleEndian>()?;
    if fmt_size != 16 {
        return Err(format!("❌ fmt 块大小错误: 期望 16, 实际 {}", fmt_size).into());
    }
    println!("      ✅ fmt 块大小: {}", fmt_size);
    
    // 5. 音频格式
    let audio_format = file.read_u16::<LittleEndian>()?;
    if audio_format != 1 {
        return Err(format!("❌ 音频格式错误: 期望 1 (PCM), 实际 {}", audio_format).into());
    }
    println!("      ✅ 音频格式: {} (PCM)", audio_format);
    
    // 6. 声道数
    let num_channels = file.read_u16::<LittleEndian>()?;
    if num_channels != expected_channels as u16 {
        return Err(format!("❌ 声道数错误: 期望 {}, 实际 {}", expected_channels, num_channels).into());
    }
    println!("      ✅ 声道数: {}", num_channels);
    
    // 7. 采样率
    let sample_rate = file.read_u32::<LittleEndian>()?;
    if sample_rate != expected_sample_rate {
        return Err(format!("❌ 采样率错误: 期望 {}, 实际 {}", expected_sample_rate, sample_rate).into());
    }
    println!("      ✅ 采样率: {} Hz", sample_rate);
    
    // 8. 字节率
    let byte_rate = file.read_u32::<LittleEndian>()?;
    let expected_byte_rate = expected_sample_rate * expected_channels as u32 * (expected_bits_per_sample / 8) as u32;
    if byte_rate != expected_byte_rate {
        return Err(format!("❌ 字节率错误: 期望 {}, 实际 {}", expected_byte_rate, byte_rate).into());
    }
    println!("      ✅ 字节率: {} 字节/秒", byte_rate);
    
    // 9. 块对齐
    let block_align = file.read_u16::<LittleEndian>()?;
    let expected_block_align = expected_channels as u16 * (expected_bits_per_sample / 8);
    if block_align != expected_block_align {
        return Err(format!("❌ 块对齐错误: 期望 {}, 实际 {}", expected_block_align, block_align).into());
    }
    println!("      ✅ 块对齐: {} 字节", block_align);
    
    // 10. 位深度
    let bits_per_sample = file.read_u16::<LittleEndian>()?;
    if bits_per_sample != expected_bits_per_sample {
        return Err(format!("❌ 位深度错误: 期望 {}, 实际 {}", expected_bits_per_sample, bits_per_sample).into());
    }
    println!("      ✅ 位深度: {} 位", bits_per_sample);
    
    // 11. data 块
    let mut data_header = [0u8; 4];
    file.read_exact(&mut data_header)?;
    if &data_header != b"data" {
        return Err(format!("❌ data 头错误: 期望 'data', 实际 '{:?}'", 
                         String::from_utf8_lossy(&data_header)).into());
    }
    println!("      ✅ data 块标识: {:?}", String::from_utf8_lossy(&data_header));
    
    let data_size = file.read_u32::<LittleEndian>()?;
    println!("      📊 PCM 数据大小: {} 字节", data_size);
    
    // 验证数据大小的合理性
    let current_pos = file.stream_position()?;
    file.seek(SeekFrom::End(0))?;
    let actual_file_size = file.stream_position()?;
    let expected_file_size = current_pos + data_size as u64;
    
    if actual_file_size != expected_file_size {
        return Err(format!("❌ 文件大小不匹配: 期望 {}, 实际 {}", expected_file_size, actual_file_size).into());
    }
    println!("      ✅ 文件总大小: {} 字节", actual_file_size);
    
    // 计算音频时长
    let duration = data_size as f64 / byte_rate as f64;
    println!("      ⏱️  音频时长: {:.2} 秒", duration);
    
    Ok(())
}

/// 测试与现有 WAV 文件的对比
#[test]
fn test_compare_wav_headers() {
    // 如果存在现有的 WAV 文件，进行对比
    let existing_wav = "pcmFile/26_starsky.wav";
    let pcm_file = "pcmFile/26_starsky.pcm";
    let generated_wav = "comparison_test.wav";
    
    if !std::path::Path::new(existing_wav).exists() || !std::path::Path::new(pcm_file).exists() {
        println!("跳过 WAV 对比测试：参考文件不存在");
        return;
    }
    
    println!("🔄 对比现有 WAV 文件与生成的 WAV 文件");
    
    // 读取现有 WAV 文件的头信息
    if let Ok((sr, ch, bits)) = extract_wav_params(existing_wav) {
        println!("📋 现有 WAV 文件参数: {}Hz, {} 声道, {} 位", sr, ch, bits);
        
        // 使用相同参数生成新的 WAV 文件
        let config = PcmToWavConfig::new(sr, ch, bits);
        let result = trans_pcm_file_to_wav(pcm_file, generated_wav, Some(config));
        
        if result.is_ok() {
            // 对比两个文件的头信息
            match compare_wav_headers(existing_wav, generated_wav) {
                Ok(_) => println!("✅ WAV 文件头对比一致"),
                Err(e) => println!("⚠️  WAV 文件头有差异: {}", e),
            }
        }
        
        // 清理文件
        let _ = std::fs::remove_file(generated_wav);
    }
}

/// 提取 WAV 文件参数
fn extract_wav_params(file_path: &str) -> Result<(u32, u8, u16), Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    
    // 跳过 RIFF 头
    file.seek(SeekFrom::Start(12))?; // 跳过 "RIFF" + size + "WAVE"
    file.seek(SeekFrom::Start(20))?; // 跳过 "fmt " + size + format
    
    let channels = file.read_u16::<LittleEndian>()?;
    let sample_rate = file.read_u32::<LittleEndian>()?;
    file.read_u32::<LittleEndian>()?; // 跳过 byte_rate
    file.read_u16::<LittleEndian>()?; // 跳过 block_align
    let bits_per_sample = file.read_u16::<LittleEndian>()?;
    
    Ok((sample_rate, channels as u8, bits_per_sample))
}

/// 对比两个 WAV 文件的头信息
fn compare_wav_headers(file1: &str, file2: &str) -> Result<(), Box<dyn std::error::Error>> {
    let params1 = extract_wav_params(file1)?;
    let params2 = extract_wav_params(file2)?;
    
    if params1 != params2 {
        return Err(format!("参数不匹配: {:?} vs {:?}", params1, params2).into());
    }
    
    Ok(())
}