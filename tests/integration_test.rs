use audio_helper::{trans_pcm_file_to_wav, PcmToWavConfig};
use std::fs;
use std::path::Path;

#[test]
fn integration_test_pcm_to_wav_conversion() {
    let input_path = "integration_test_input.pcm";
    let output_path = "integration_test_output.wav";
    
    // 创建测试用的 PCM 数据 (16-bit stereo, 44.1kHz)
    let sample_count = 2000;
    let pcm_data: Vec<u8> = (0..sample_count).flat_map(|i| {
        // 生成正弦波测试数据
        let angle = 2.0 * std::f32::consts::PI * 440.0 * i as f32 / 44100.0; // 440Hz 音调
        let sample = (angle.sin() * 16000.0) as i16;
        sample.to_le_bytes()
    }).collect();
    
    // 记录 PCM 数据大小
    let pcm_data_len = pcm_data.len();
    
    // 写入测试 PCM 文件
    fs::write(input_path, pcm_data).expect("Failed to write test PCM file");
    
    // 使用自定义配置进行转换
    let config = PcmToWavConfig::new(44100, 2, 16);
    let result = trans_pcm_file_to_wav(input_path, output_path, Some(config));
    
    // 验证转换结果
    assert!(result.is_ok(), "PCM to WAV conversion should succeed");
    
    // 验证输出文件存在
    assert!(Path::new(output_path).exists(), "Output WAV file should exist");
    
    // 检查输出文件大小是否合理（应该包含 WAV 头 + PCM 数据）
    let output_size = fs::metadata(output_path).expect("Failed to get output file metadata").len();
    let expected_min_size = 44 + pcm_data_len as u64; // WAV 头 + PCM 数据
    assert!(output_size >= expected_min_size, "Output file size should be at least the sum of header and PCM data");
    
    // 清理测试文件
    let _ = fs::remove_file(input_path);
    let _ = fs::remove_file(output_path);
    
    println!("Integration test passed: PCM to WAV conversion works correctly");
}

#[test]
fn integration_test_error_handling() {
    // 测试不存在的文件
    let result = trans_pcm_file_to_wav("nonexistent.pcm", "output.wav", None);
    assert!(result.is_err());
    
    // 测试非 PCM 文件
    let result = trans_pcm_file_to_wav("test.mp3", "output.wav", None);
    assert!(result.is_err());
    
    println!("Integration test passed: Error handling works correctly");
}

#[test]
fn integration_test_different_configurations() {
    let input_path = "config_test_input.pcm";
    
    // 创建单声道 8kHz 16-bit PCM 数据
    let pcm_data: Vec<u8> = (0..800).flat_map(|i| {
        let sample = (i * 80) as i16;
        sample.to_le_bytes()
    }).collect();
    
    fs::write(input_path, pcm_data).expect("Failed to write test PCM file");
    
    // 测试不同的配置
    let configs = vec![
        PcmToWavConfig::new(8000, 1, 16),   // 8kHz 单声道
        PcmToWavConfig::new(22050, 1, 16),  // 22.05kHz 单声道
        PcmToWavConfig::new(48000, 2, 16),  // 48kHz 立体声
    ];
    
    for (i, config) in configs.into_iter().enumerate() {
        let output = format!("config_test_output_{}.wav", i);
        let result = trans_pcm_file_to_wav(input_path, &output, Some(config));
        assert!(result.is_ok(), "Configuration test {} should succeed", i);
        assert!(Path::new(&output).exists(), "Output file {} should exist", output);
        
        // 清理输出文件
        let _ = fs::remove_file(&output);
    }
    
    // 清理输入文件
    let _ = fs::remove_file(input_path);
    
    println!("Integration test passed: Different configurations work correctly");
}