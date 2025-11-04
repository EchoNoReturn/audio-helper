use audio_helper::{trans_pcm_file_to_wav, PcmToWavConfig};
use std::fs;

/// 分析不同采样率 PCM 文件的测试
#[test]
fn analyze_sample_rate_files() {
    let test_files = vec![
        ("浪花一朵朵片段8k16bit单声道.pcm", 8000, 1, 16),
        ("浪花一朵朵片段32k16bit单声道.pcm", 32000, 1, 16),
        ("浪花一朵朵片段48k16bit单声道.pcm", 48000, 1, 16),
        ("冰雨片段8k16bit单声道.pcm", 8000, 1, 16),
        ("冰雨片段32k16bit单声道.pcm", 32000, 1, 16),
        ("冰雨片段48k16bit单声道.pcm", 48000, 1, 16),
    ];
    
    for (filename, sample_rate, channels, bits_per_sample) in test_files {
        let input_path = format!("pcmFile/{}", filename);
        let output_path = format!("debug_{}.wav", filename.replace(".pcm", ""));
        
        if !std::path::Path::new(&input_path).exists() {
            println!("⚠️  跳过不存在的文件: {}", filename);
            continue;
        }
        
        // 获取文件大小
        let file_size = fs::metadata(&input_path).unwrap().len();
        
        // 计算理论音频时长（秒）
        let bytes_per_sample = (bits_per_sample / 8) as u64;
        let bytes_per_second = sample_rate as u64 * channels as u64 * bytes_per_sample;
        let duration_seconds = file_size as f64 / bytes_per_second as f64;
        
        println!("📊 分析文件: {}", filename);
        println!("   文件大小: {} 字节", file_size);
        println!("   配置: {}Hz, {} 声道, {} bit", sample_rate, channels, bits_per_sample);
        println!("   理论时长: {:.2} 秒", duration_seconds);
        println!("   期望字节率: {} 字节/秒", bytes_per_second);
        
        // 测试转换
        let config = PcmToWavConfig::new(sample_rate, channels, bits_per_sample);
        let result = trans_pcm_file_to_wav(&input_path, &output_path, Some(config));
        
        match result {
            Ok(_) => {
                println!("   ✅ 转换成功");
                
                // 验证输出文件
                if let Ok(output_metadata) = fs::metadata(&output_path) {
                    let output_size = output_metadata.len();
                    let expected_size = 44 + file_size; // WAV 头 + PCM 数据
                    println!("   输出大小: {} 字节 (期望: {} 字节)", output_size, expected_size);
                    
                    if output_size == expected_size {
                        println!("   ✅ 文件大小正确");
                    } else {
                        println!("   ❌ 文件大小不匹配，差异: {} 字节", 
                                (output_size as i64 - expected_size as i64).abs());
                    }
                }
                
                // 清理输出文件
                let _ = fs::remove_file(&output_path);
            }
            Err(e) => {
                println!("   ❌ 转换失败: {}", e);
            }
        }
        println!();
    }
}

/// 测试使用错误配置的情况
#[test]
fn test_wrong_configurations() {
    let test_cases = vec![
        // 文件名，实际配置，错误配置
        ("浪花一朵朵片段8k16bit单声道.pcm", (8000, 1, 16), (44100, 2, 16)),
        ("浪花一朵朵片段32k16bit单声道.pcm", (32000, 1, 16), (44100, 2, 16)),
        ("浪花一朵朵片段48k16bit单声道.pcm", (48000, 1, 16), (44100, 2, 16)),
    ];
    
    for (filename, (correct_sr, correct_ch, correct_bits), (wrong_sr, wrong_ch, wrong_bits)) in test_cases {
        let input_path = format!("pcmFile/{}", filename);
        
        if !std::path::Path::new(&input_path).exists() {
            continue;
        }
        
        println!("🔍 测试配置对比: {}", filename);
        
        // 使用正确配置
        let correct_output = format!("correct_{}.wav", filename.replace(".pcm", ""));
        let correct_config = PcmToWavConfig::new(correct_sr, correct_ch, correct_bits);
        let correct_result = trans_pcm_file_to_wav(&input_path, &correct_output, Some(correct_config));
        
        // 使用错误配置
        let wrong_output = format!("wrong_{}.wav", filename.replace(".pcm", ""));
        let wrong_config = PcmToWavConfig::new(wrong_sr, wrong_ch, wrong_bits);
        let wrong_result = trans_pcm_file_to_wav(&input_path, &wrong_output, Some(wrong_config));
        
        println!("   正确配置 ({}Hz, {}声道): {:?}", correct_sr, correct_ch, correct_result.is_ok());
        println!("   错误配置 ({}Hz, {}声道): {:?}", wrong_sr, wrong_ch, wrong_result.is_ok());
        
        // 比较文件大小
        if correct_result.is_ok() && wrong_result.is_ok() {
            let correct_size = fs::metadata(&correct_output).map(|m| m.len()).unwrap_or(0);
            let wrong_size = fs::metadata(&wrong_output).map(|m| m.len()).unwrap_or(0);
            
            println!("   正确配置输出大小: {} 字节", correct_size);
            println!("   错误配置输出大小: {} 字节", wrong_size);
            
            if correct_size != wrong_size {
                println!("   ⚠️  配置不同导致输出大小不同");
            }
        }
        
        // 清理文件
        let _ = fs::remove_file(&correct_output);
        let _ = fs::remove_file(&wrong_output);
        println!();
    }
}