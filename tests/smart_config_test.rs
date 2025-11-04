use audio_helper::{infer_pcm_config_from_filename, auto_trans_pcm_to_wav, PcmToWavConfig};

/// 测试智能配置推断功能
#[test]
fn test_smart_config_inference() {
    let test_cases = vec![
        ("浪花一朵朵片段8k16bit单声道.pcm", 8000, 1, 16),
        ("浪花一朵朵片段32k16bit单声道.pcm", 32000, 1, 16),
        ("浪花一朵朵片段48k16bit单声道.pcm", 48000, 1, 16),
        ("冰雨片段8k16bit单声道.pcm", 8000, 1, 16),
        ("冰雨片段32k16bit单声道.pcm", 32000, 1, 16),
        ("冰雨片段48k16bit单声道.pcm", 48000, 1, 16),
        ("北京北京8k16bits单声道.pcm", 8000, 1, 16),
        ("test_44k_stereo_16bit.pcm", 44100, 2, 16),
        ("music_22k_mono_8bit.pcm", 22050, 1, 8),
        ("voice_16k_1ch_16bit.pcm", 16000, 1, 16),
        ("audio_96k_2ch_24bit.pcm", 96000, 2, 24),
        ("sample.pcm", 44100, 2, 16), // 默认配置
    ];
    
    println!("🧠 测试智能配置推断:");
    
    for (filename, expected_sr, expected_ch, expected_bits) in test_cases {
        let inferred_config = infer_pcm_config_from_filename(filename);
        
        let actual_sr = inferred_config.sample_rate.unwrap_or(0);
        let actual_ch = inferred_config.channels.unwrap_or(0);
        let actual_bits = inferred_config.bits_per_sample.unwrap_or(0);
        
        let correct = actual_sr == expected_sr && actual_ch == expected_ch && actual_bits == expected_bits;
        
        println!("   📁 {}", filename);
        println!("      期望: {}Hz, {} 声道, {} 位", expected_sr, expected_ch, expected_bits);
        println!("      推断: {}Hz, {} 声道, {} 位", actual_sr, actual_ch, actual_bits);
        println!("      结果: {}", if correct { "✅ 正确" } else { "❌ 错误" });
        
        assert_eq!(actual_sr, expected_sr, "采样率推断错误: {}", filename);
        assert_eq!(actual_ch, expected_ch, "声道数推断错误: {}", filename);
        assert_eq!(actual_bits, expected_bits, "位深度推断错误: {}", filename);
        println!();
    }
}

/// 测试自动转换功能
#[test]
fn test_auto_conversion() {
    let test_files = vec![
        "浪花一朵朵片段8k16bit单声道.pcm",
        "浪花一朵朵片段32k16bit单声道.pcm",
        "浪花一朵朵片段48k16bit单声道.pcm",
    ];
    
    println!("🤖 测试自动转换功能:");
    
    for filename in test_files {
        let input_path = format!("pcmFile/{}", filename);
        let output_path = format!("auto_{}.wav", filename.replace(".pcm", ""));
        
        if !std::path::Path::new(&input_path).exists() {
            println!("   ⚠️  跳过不存在的文件: {}", filename);
            continue;
        }
        
        println!("   🔄 自动转换: {}", filename);
        
        match auto_trans_pcm_to_wav(&input_path, &output_path) {
            Ok(used_config) => {
                println!("      ✅ 转换成功");
                println!("      📋 使用配置: {:?}", used_config);
                
                // 验证输出文件存在
                if std::path::Path::new(&output_path).exists() {
                    let file_size = std::fs::metadata(&output_path)
                        .map(|m| m.len())
                        .unwrap_or(0);
                    println!("      📊 输出大小: {} 字节", file_size);
                } else {
                    panic!("输出文件不存在: {}", output_path);
                }
                
                // 清理文件
                let _ = std::fs::remove_file(&output_path);
            }
            Err(e) => {
                println!("      ❌ 转换失败: {}", e);
                panic!("自动转换失败: {}", e);
            }
        }
        println!();
    }
}

/// 对比手动配置与自动推断配置的结果
#[test]
fn test_manual_vs_auto_config() {
    let filename = "浪花一朵朵片段32k16bit单声道.pcm";
    let input_path = format!("pcmFile/{}", filename);
    
    if !std::path::Path::new(&input_path).exists() {
        println!("跳过对比测试：文件不存在");
        return;
    }
    
    println!("⚖️  对比手动配置与自动推断:");
    
    // 手动配置转换
    let manual_output = "manual_config_test.wav";
    let manual_config = PcmToWavConfig::new(32000, 1, 16);
    let manual_result = audio_helper::trans_pcm_file_to_wav(&input_path, manual_output, Some(manual_config.clone()));
    
    // 自动推断转换
    let auto_output = "auto_config_test.wav";
    let auto_result = auto_trans_pcm_to_wav(&input_path, auto_output);
    
    println!("   📋 手动配置: {:?}", manual_config);
    if let Ok(auto_config) = &auto_result {
        println!("   🧠 自动推断: {:?}", auto_config);
        
        // 比较配置
        if manual_config == *auto_config {
            println!("   ✅ 配置完全一致");
        } else {
            println!("   ⚠️  配置有差异");
        }
    }
    
    // 比较结果
    match (manual_result, auto_result) {
        (Ok(_), Ok(_)) => {
            println!("   ✅ 两种方式都成功");
            
            // 比较文件大小
            let manual_size = std::fs::metadata(manual_output).map(|m| m.len()).unwrap_or(0);
            let auto_size = std::fs::metadata(auto_output).map(|m| m.len()).unwrap_or(0);
            
            println!("   📊 手动配置输出: {} 字节", manual_size);
            println!("   📊 自动推断输出: {} 字节", auto_size);
            
            if manual_size == auto_size {
                println!("   ✅ 输出文件大小一致");
            } else {
                println!("   ⚠️  输出文件大小不同，差异: {} 字节", 
                        (manual_size as i64 - auto_size as i64).abs());
            }
        }
        (Ok(_), Err(e)) => {
            println!("   ❌ 自动推断失败: {}", e);
        }
        (Err(e), Ok(_)) => {
            println!("   ❌ 手动配置失败: {}", e);
        }
        (Err(e1), Err(e2)) => {
            println!("   ❌ 两种方式都失败: {} / {}", e1, e2);
        }
    }
    
    // 清理文件
    let _ = std::fs::remove_file(manual_output);
    let _ = std::fs::remove_file(auto_output);
}