use audio_helper::{trans_pcm_file_to_wav, PcmToWavConfig};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};

/// 验证生成的 WAV 文件格式是否正确
#[test]
fn test_wav_file_format_validation() {
    let test_pcm = "pcmFile/冰雨片段8k16bit单声道.pcm";
    let test_wav = "validation_test.wav";
    
    // 如果测试文件不存在就跳过
    if !std::path::Path::new(test_pcm).exists() {
        println!("跳过 WAV 格式验证测试：测试文件不存在");
        return;
    }
    
    // 转换 PCM 到 WAV
    let config = PcmToWavConfig::new(8000, 1, 16);
    let result = trans_pcm_file_to_wav(test_pcm, test_wav, Some(config));
    assert!(result.is_ok(), "PCM 到 WAV 转换应该成功");
    
    // 验证 WAV 文件格式
    validate_wav_format(test_wav, 8000, 1, 16).expect("WAV 文件格式验证失败");
    
    println!("✓ WAV 文件格式验证通过");
    
    // 清理文件
    let _ = std::fs::remove_file(test_wav);
}

/// 验证 WAV 文件格式的辅助函数
fn validate_wav_format(file_path: &str, expected_sample_rate: u32, expected_channels: u8, expected_bits_per_sample: u16) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    
    // 读取 RIFF 头
    let mut riff_header = [0u8; 4];
    file.read_exact(&mut riff_header)?;
    assert_eq!(&riff_header, b"RIFF", "RIFF 头不正确");
    
    let file_size = file.read_u32::<LittleEndian>()?;
    println!("RIFF 文件大小: {}", file_size + 8);
    
    let mut wave_header = [0u8; 4];
    file.read_exact(&mut wave_header)?;
    assert_eq!(&wave_header, b"WAVE", "WAVE 头不正确");
    
    // 读取 fmt 块
    let mut fmt_header = [0u8; 4];
    file.read_exact(&mut fmt_header)?;
    assert_eq!(&fmt_header, b"fmt ", "fmt 头不正确");
    
    let fmt_size = file.read_u32::<LittleEndian>()?;
    assert_eq!(fmt_size, 16, "fmt 块大小应该是 16");
    
    let audio_format = file.read_u16::<LittleEndian>()?;
    assert_eq!(audio_format, 1, "音频格式应该是 PCM (1)");
    
    let num_channels = file.read_u16::<LittleEndian>()?;
    assert_eq!(num_channels, expected_channels as u16, "声道数不匹配");
    
    let sample_rate = file.read_u32::<LittleEndian>()?;
    assert_eq!(sample_rate, expected_sample_rate, "采样率不匹配");
    
    let byte_rate = file.read_u32::<LittleEndian>()?;
    let expected_byte_rate = expected_sample_rate * expected_channels as u32 * (expected_bits_per_sample / 8) as u32;
    assert_eq!(byte_rate, expected_byte_rate, "字节率不匹配");
    
    let block_align = file.read_u16::<LittleEndian>()?;
    let expected_block_align = expected_channels as u16 * (expected_bits_per_sample / 8);
    assert_eq!(block_align, expected_block_align, "块对齐不匹配");
    
    let bits_per_sample = file.read_u16::<LittleEndian>()?;
    assert_eq!(bits_per_sample, expected_bits_per_sample, "位深度不匹配");
    
    // 读取 data 块
    let mut data_header = [0u8; 4];
    file.read_exact(&mut data_header)?;
    assert_eq!(&data_header, b"data", "data 头不正确");
    
    let data_size = file.read_u32::<LittleEndian>()?;
    println!("PCM 数据大小: {} 字节", data_size);
    
    // 验证文件总大小
    let current_pos = file.stream_position()?;
    file.seek(SeekFrom::End(0))?;
    let actual_file_size = file.stream_position()?;
    let expected_file_size = current_pos + data_size as u64;
    
    assert_eq!(actual_file_size, expected_file_size, "文件大小不匹配");
    
    println!("WAV 格式验证通过:");
    println!("  采样率: {} Hz", sample_rate);
    println!("  声道数: {}", num_channels);
    println!("  位深度: {} 位", bits_per_sample);
    println!("  字节率: {} 字节/秒", byte_rate);
    println!("  数据大小: {} 字节", data_size);
    println!("  文件大小: {} 字节", actual_file_size);
    
    Ok(())
}

/// 性能测试：测量转换大文件的时间
#[test]
fn test_performance_large_files() {
    use std::time::Instant;
    
    // 找到最大的 PCM 文件进行性能测试
    let large_files = vec![
        "pcmFile/12_heaven.pcm",          // 通常是较大的文件
        "pcmFile/04_darkforest.pcm",
        "pcmFile/03_coolcool.pcm",
    ];
    
    for file_path in large_files {
        if !std::path::Path::new(file_path).exists() {
            continue;
        }
        
        // 获取文件大小
        let file_size = std::fs::metadata(file_path)
            .map(|m| m.len())
            .unwrap_or(0);
        
        if file_size < 1_000_000 { // 只测试大于 1MB 的文件
            continue;
        }
        
        let output_path = format!("perf_test_{}.wav", 
            std::path::Path::new(file_path).file_stem()
                .unwrap_or_default().to_string_lossy());
        
        println!("性能测试: {} (大小: {:.2} MB)", file_path, file_size as f64 / 1_000_000.0);
        
        let start_time = Instant::now();
        
        let config = PcmToWavConfig::new(44100, 2, 16);
        let result = trans_pcm_file_to_wav(file_path, &output_path, Some(config));
        
        let duration = start_time.elapsed();
        
        assert!(result.is_ok(), "大文件转换应该成功");
        
        let throughput = file_size as f64 / duration.as_secs_f64() / 1_000_000.0;
        println!("  转换时间: {:.2} 秒", duration.as_secs_f64());
        println!("  处理速度: {:.2} MB/秒", throughput);
        
        // 清理文件
        let _ = std::fs::remove_file(&output_path);
        
        // 断言处理速度应该足够快（至少 10 MB/秒）
        assert!(throughput > 10.0, "处理速度太慢: {:.2} MB/秒", throughput);
    }
}