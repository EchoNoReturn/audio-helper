use audio_helper::{trans_pcm_file_to_wav, auto_trans_pcm_to_wav, infer_pcm_config_from_filename, PcmToWavConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎵 音频转码工具演示");
    
    // 1. 手动配置转换
    println!("\n1️⃣ 手动配置转换示例:");
    let manual_config = PcmToWavConfig::new(48000, 1, 16);
    println!("   配置: {:?}", manual_config);
    
    if std::path::Path::new("pcmFile/浪花一朵朵片段48k16bit单声道.pcm").exists() {
        match trans_pcm_file_to_wav(
            "pcmFile/浪花一朵朵片段48k16bit单声道.pcm", 
            "demo_manual.wav", 
            Some(manual_config)
        ) {
            Ok(_) => println!("   ✅ 手动配置转换成功"),
            Err(e) => println!("   ❌ 转换失败: {}", e),
        }
    }
    
    // 2. 智能配置推断
    println!("\n2️⃣ 智能配置推断示例:");
    let test_files = vec![
        "浪花一朵朵片段8k16bit单声道.pcm",
        "浪花一朵朵片段32k16bit单声道.pcm", 
        "浪花一朵朵片段48k16bit单声道.pcm",
        "冰雨片段8k16bit单声道.pcm",
    ];
    
    for filename in test_files {
        let config = infer_pcm_config_from_filename(filename);
        println!("   📁 {}", filename);
        println!("      推断配置: {:?}", config);
    }
    
    // 3. 自动转换
    println!("\n3️⃣ 自动转换示例:");
    if std::path::Path::new("pcmFile/浪花一朵朵片段32k16bit单声道.pcm").exists() {
        match auto_trans_pcm_to_wav(
            "pcmFile/浪花一朵朵片段32k16bit单声道.pcm", 
            "demo_auto.wav"
        ) {
            Ok(used_config) => {
                println!("   ✅ 自动转换成功");
                println!("   📋 使用配置: {:?}", used_config);
            }
            Err(e) => println!("   ❌ 转换失败: {}", e),
        }
    }
    
    // 4. 批量处理示例
    println!("\n4️⃣ 批量处理示例:");
    let pcm_files = std::fs::read_dir("pcmFile")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path()
                .extension()
                .map_or(false, |ext| ext == "pcm")
        })
        .take(3) // 只处理前3个文件作为演示
        .collect::<Vec<_>>();
    
    for entry in pcm_files {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_string_lossy();
        let input_path = path.to_string_lossy();
        let output_path = format!("batch_{}.wav", filename.replace(".pcm", ""));
        
        println!("   🔄 处理: {}", filename);
        
        match auto_trans_pcm_to_wav(&input_path, &output_path) {
            Ok(config) => {
                let file_size = std::fs::metadata(&output_path)
                    .map(|m| m.len())
                    .unwrap_or(0);
                println!("      ✅ 成功 | 配置: {}Hz, {}ch, {}bit | 大小: {} 字节", 
                         config.sample_rate.unwrap_or(0),
                         config.channels.unwrap_or(0), 
                         config.bits_per_sample.unwrap_or(0),
                         file_size);
                
                // 清理演示文件
                let _ = std::fs::remove_file(&output_path);
            }
            Err(e) => println!("      ❌ 失败: {}", e),
        }
    }
    
    // 清理演示文件
    let _ = std::fs::remove_file("demo_manual.wav");
    let _ = std::fs::remove_file("demo_auto.wav");
    
    println!("\n🎉 演示完成！");
    println!("\n📚 功能总结:");
    println!("   • 手动配置 PCM 转 WAV: trans_pcm_file_to_wav()");
    println!("   • 智能配置推断: infer_pcm_config_from_filename()");
    println!("   • 自动转换: auto_trans_pcm_to_wav()");
    println!("   • 支持多种采样率: 8k, 16k, 22k, 32k, 44.1k, 48k, 96k");
    println!("   • 支持多种声道: 单声道、立体声");
    println!("   • 支持多种位深度: 8bit, 16bit, 24bit, 32bit");
    
    Ok(())
}