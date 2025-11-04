use audio_helper::{trans_pcm_file_to_wav, PcmToWavConfig};
use std::fs;
use std::path::Path;

/// 测试使用真实 PCM 文件进行转换
#[test]
fn test_real_pcm_files_conversion() {
    let pcm_dir = "pcmFile";
    
    // 测试几个不同的 PCM 文件
    let test_files = vec![
        ("01_amylee.pcm", PcmToWavConfig::new(44100, 2, 16)),
        ("冰雨片段8k16bit单声道.pcm", PcmToWavConfig::new(8000, 1, 16)),
        ("冰雨片段32k16bit单声道.pcm", PcmToWavConfig::new(32000, 1, 16)),
        ("冰雨片段48k16bit单声道.pcm", PcmToWavConfig::new(48000, 1, 16)),
        ("浪花一朵朵片段8k16bit单声道.pcm", PcmToWavConfig::new(8000, 1, 16)),
    ];
    
    for (filename, config) in test_files {
        let input_path = format!("{}/{}", pcm_dir, filename);
        let output_path = format!("output_{}.wav", filename.replace(".pcm", ""));
        
        // 检查输入文件是否存在
        if !Path::new(&input_path).exists() {
            println!("跳过不存在的文件: {}", input_path);
            continue;
        }
        
        // 获取输入文件信息
        let input_size = fs::metadata(&input_path).expect("Failed to get input file metadata").len();
        println!("测试文件: {} (大小: {} 字节)", filename, input_size);
        
        // 执行转换
        let result = trans_pcm_file_to_wav(&input_path, &output_path, Some(config));
        
        // 验证转换结果
        assert!(result.is_ok(), "转换文件 {} 应该成功", filename);
        
        // 验证输出文件存在
        assert!(Path::new(&output_path).exists(), "输出文件 {} 应该存在", output_path);
        
        // 检查输出文件大小
        let output_size = fs::metadata(&output_path).expect("Failed to get output file metadata").len();
        let expected_min_size = 44 + input_size; // WAV 头 + PCM 数据
        assert!(output_size >= expected_min_size, "输出文件 {} 大小应该至少为 {} 字节，实际为 {} 字节", 
                output_path, expected_min_size, output_size);
        
        println!("✓ 成功转换 {} -> {} (输出大小: {} 字节)", filename, output_path, output_size);
        
        // 清理输出文件（可选，保留用于验证）
        // let _ = fs::remove_file(&output_path);
    }
}

/// 测试批量转换所有 PCM 文件
#[test]
fn test_batch_convert_all_pcm_files() {
    let pcm_dir = "pcmFile";
    let output_dir = "output_wav";
    
    // 创建输出目录
    if !Path::new(output_dir).exists() {
        fs::create_dir(output_dir).expect("Failed to create output directory");
    }
    
    // 读取 pcmFile 目录中的所有 .pcm 文件
    let entries = fs::read_dir(pcm_dir).expect("Failed to read pcmFile directory");
    let mut converted_count = 0;
    let mut failed_count = 0;
    
    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        
        if let Some(filename) = path.file_name() {
            let filename_str = filename.to_string_lossy();
            
            // 只处理 .pcm 文件
            if filename_str.ends_with(".pcm") {
                let input_path = path.to_string_lossy().to_string();
                let output_filename = filename_str.replace(".pcm", ".wav");
                let output_path = format!("{}/{}", output_dir, output_filename);
                
                // 使用智能配置推断
                let config = audio_helper::infer_pcm_config_from_filename(&filename_str);
                
                println!("转换: {} -> {}", filename_str, output_filename);
                
                match trans_pcm_file_to_wav(&input_path, &output_path, Some(config)) {
                    Ok(_) => {
                        converted_count += 1;
                        
                        // 验证输出文件
                        if Path::new(&output_path).exists() {
                            let output_size = fs::metadata(&output_path)
                                .map(|m| m.len())
                                .unwrap_or(0);
                            println!("  ✓ 成功 (输出大小: {} 字节)", output_size);
                        }
                    }
                    Err(e) => {
                        failed_count += 1;
                        println!("  ✗ 失败: {}", e);
                    }
                }
            }
        }
    }
    
    println!("\n批量转换完成:");
    println!("  成功转换: {} 个文件", converted_count);
    println!("  转换失败: {} 个文件", failed_count);
    
    // 至少应该成功转换一些文件
    assert!(converted_count > 0, "至少应该成功转换一个文件");
}

/// 测试对比已有的 WAV 文件
#[test]
fn test_compare_with_existing_wav() {
    let pcm_file = "pcmFile/26_starsky.pcm";
    let existing_wav = "pcmFile/26_starsky.wav";
    let generated_wav = "test_26_starsky_generated.wav";
    
    // 检查文件是否存在
    if !Path::new(pcm_file).exists() || !Path::new(existing_wav).exists() {
        println!("跳过对比测试：文件不存在");
        return;
    }
    
    // 转换 PCM 到 WAV
    let config = PcmToWavConfig::new(44100, 2, 16); // 假设是标准配置
    let result = trans_pcm_file_to_wav(pcm_file, generated_wav, Some(config));
    
    assert!(result.is_ok(), "PCM 到 WAV 转换应该成功");
    assert!(Path::new(generated_wav).exists(), "生成的 WAV 文件应该存在");
    
    // 比较文件大小
    let existing_size = fs::metadata(existing_wav).expect("Failed to get existing WAV metadata").len();
    let generated_size = fs::metadata(generated_wav).expect("Failed to get generated WAV metadata").len();
    
    println!("现有 WAV 文件大小: {} 字节", existing_size);
    println!("生成 WAV 文件大小: {} 字节", generated_size);
    
    // WAV 文件大小应该相近（允许一些差异，因为可能有不同的头信息）
    let size_diff = if existing_size > generated_size {
        existing_size - generated_size
    } else {
        generated_size - existing_size
    };
    
    // 允许最多 1KB 的差异（头信息可能不同）
    assert!(size_diff < 1024, "生成的 WAV 文件大小与现有文件差异过大: {} 字节", size_diff);
    
    println!("✓ WAV 文件大小对比通过，差异: {} 字节", size_diff);
    
    // 清理生成的文件
    let _ = fs::remove_file(generated_wav);
}