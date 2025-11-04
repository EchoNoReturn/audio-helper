use audio_helper::{trans_pcm_file_to_mp3, Mp3Config, Mp3Bitrate, AudioQuality};

fn main() {
    println!("Testing PCM to MP3 conversion...");
    
    // 测试文件路径
    let pcm_file = "pcmFile/冰雨片段8k16bit单声道.pcm";
    let mp3_file = "output_wav/test_8k_mono.mp3";
    
    // 检查输入文件是否存在
    if !std::path::Path::new(pcm_file).exists() {
        println!("PCM file not found: {}", pcm_file);
        return;
    }
    
    // 创建输出目录
    if let Some(parent) = std::path::Path::new(mp3_file).parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    
    // 配置 MP3 参数
    let mp3_config = Mp3Config::new(
        8000,  // 8kHz 采样率
        1,     // 单声道
        Mp3Bitrate::Kbps128,
        AudioQuality::High
    );
    
    // 转换 PCM 到 MP3
    match trans_pcm_file_to_mp3(pcm_file, mp3_file, Some(mp3_config)) {
        Ok(()) => {
            println!("✅ MP3 conversion successful!");
            
            // 检查输出文件大小
            if let Ok(metadata) = std::fs::metadata(mp3_file) {
                println!("📁 Output file size: {} bytes", metadata.len());
            }
        }
        Err(e) => {
            println!("❌ MP3 conversion failed: {}", e);
        }
    }
}