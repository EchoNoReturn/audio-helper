use audio_helper::{auto_convert_pcm, AudioFormat, infer_audio_config_from_filename};

fn main() {
    println!("=== 全面 MP3 转换测试 ===\n");
    
    let test_files = vec![
        "pcmFile/冰雨片段8k16bit单声道.pcm",
        "pcmFile/冰雨片段32k16bit单声道.pcm", 
        "pcmFile/冰雨片段48k16bit单声道.pcm",
        "pcmFile/浪花一朵朵片段8k16bit单声道.pcm",
        "pcmFile/浪花一朵朵片段32k16bit单声道.pcm",
        "pcmFile/浪花一朵朵片段48k16bit单声道.pcm",
    ];
    
    // 确保输出目录存在
    std::fs::create_dir_all("output_wav").unwrap();
    
    for pcm_file in test_files {
        println!("🎵 转换文件: {}", pcm_file);
        
        if !std::path::Path::new(pcm_file).exists() {
            println!("   ❌ 文件不存在，跳过\n");
            continue;
        }
        
        // 推断音频配置
        let filename = std::path::Path::new(pcm_file).file_name().unwrap().to_string_lossy();
        let audio_config = infer_audio_config_from_filename(&filename);
        println!("   📊 检测到配置: {}Hz, {} channels, {}bits", 
                 audio_config.sample_rate, audio_config.channels, audio_config.bits_per_sample);
        
        // 生成输出文件名
        let mp3_file = format!("output_wav/{}.mp3", 
                              std::path::Path::new(pcm_file).file_stem().unwrap().to_string_lossy());
        
        // 转换为 MP3
        match auto_convert_pcm(pcm_file, &mp3_file, AudioFormat::Mp3) {
            Ok(config) => {
                println!("   ✅ 转换成功!");
                
                // 检查文件大小
                if let Ok(input_meta) = std::fs::metadata(pcm_file) {
                    if let Ok(output_meta) = std::fs::metadata(&mp3_file) {
                        let compression_ratio = input_meta.len() as f32 / output_meta.len() as f32;
                        println!("   📁 输入: {} bytes, 输出: {} bytes", 
                                input_meta.len(), output_meta.len());
                        println!("   📈 压缩比: {:.1}:1", compression_ratio);
                    }
                }
            }
            Err(e) => {
                println!("   ❌ 转换失败: {}", e);
            }
        }
        println!(); // 空行分隔
    }
    
    println!("=== 测试完成 ===");
}