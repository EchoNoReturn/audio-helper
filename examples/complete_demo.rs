// 完整功能演示，包括 MP3 转换和 FFI 接口

use audio_helper::{
    // 核心转换功能
    trans_pcm_file_to_wav, trans_pcm_file_to_mp3, auto_convert_pcm,
    
    // 配置结构体
    PcmToWavConfig, Mp3Config,
    
    // 枚举类型
    AudioFormat, Mp3Bitrate, AudioQuality,
    
    // 智能推断功能
    infer_audio_config_from_filename,
    
    // FFI 模块（移动端集成）
    ffi::{
        CPcmConfig, CMp3Config,
        pcm_to_wav, pcm_to_mp3, auto_convert_audio,
        infer_config_from_filename, get_version
    }
};

use std::ffi::CString;
use std::ptr;

fn main() {
    println!("=== Audio Helper 库完整功能演示 ===\n");
    
    // 确保输出目录存在
    std::fs::create_dir_all("output_wav").unwrap();
    
    // 1. 基本 WAV 转换演示
    demo_wav_conversion();
    
    // 2. MP3 转换演示
    demo_mp3_conversion();
    
    // 3. 智能自动转换演示
    demo_auto_conversion();
    
    // 4. 智能配置推断演示
    demo_config_inference();
    
    // 5. FFI 接口演示（移动端集成）
    demo_ffi_interface();
    
    println!("=== 演示完成 ===");
}

fn demo_wav_conversion() {
    println!("🎵 1. WAV 转换演示");
    
    let pcm_file = "pcmFile/冰雨片段8k16bit单声道.pcm";
    let wav_file = "output_wav/demo_wav_output.wav";
    
    if std::path::Path::new(pcm_file).exists() {
        let config = PcmToWavConfig::new(8000, 1, 16);
        
        match trans_pcm_file_to_wav(pcm_file, wav_file, Some(config)) {
            Ok(()) => {
                println!("   ✅ WAV 转换成功: {}", wav_file);
                
                if let Ok(metadata) = std::fs::metadata(wav_file) {
                    println!("   📁 输出文件大小: {} bytes", metadata.len());
                }
            }
            Err(e) => println!("   ❌ WAV 转换失败: {}", e),
        }
    } else {
        println!("   ⚠️  测试文件不存在: {}", pcm_file);
    }
    println!();
}

fn demo_mp3_conversion() {
    println!("🎵 2. MP3 转换演示");
    
    let pcm_file = "pcmFile/浪花一朵朵片段32k16bit单声道.pcm";
    let mp3_file = "output_wav/demo_mp3_output.mp3";
    
    if std::path::Path::new(pcm_file).exists() {
        let config = Mp3Config::new(32000, 1, Mp3Bitrate::Kbps256, AudioQuality::Best);
        
        match trans_pcm_file_to_mp3(pcm_file, mp3_file, Some(config)) {
            Ok(()) => {
                println!("   ✅ MP3 转换成功: {}", mp3_file);
                
                if let Ok(input_meta) = std::fs::metadata(pcm_file) {
                    if let Ok(output_meta) = std::fs::metadata(mp3_file) {
                        let compression_ratio = input_meta.len() as f32 / output_meta.len() as f32;
                        println!("   📊 压缩比: {:.1}:1 ({} -> {} bytes)", 
                                compression_ratio, input_meta.len(), output_meta.len());
                    }
                }
            }
            Err(e) => println!("   ❌ MP3 转换失败: {}", e),
        }
    } else {
        println!("   ⚠️  测试文件不存在: {}", pcm_file);
    }
    println!();
}

fn demo_auto_conversion() {
    println!("🎵 3. 智能自动转换演示");
    
    let test_files = vec![
        ("pcmFile/冰雨片段48k16bit单声道.pcm", AudioFormat::Wav),
        ("pcmFile/浪花一朵朵片段48k16bit单声道.pcm", AudioFormat::Mp3),
    ];
    
    for (pcm_file, format) in test_files {
        if std::path::Path::new(pcm_file).exists() {
            let output_file = match format {
                AudioFormat::Wav => format!("output_wav/auto_{}.wav", 
                    std::path::Path::new(pcm_file).file_stem().unwrap().to_string_lossy()),
                AudioFormat::Mp3 => format!("output_wav/auto_{}.mp3", 
                    std::path::Path::new(pcm_file).file_stem().unwrap().to_string_lossy()),
            };
            
            match auto_convert_pcm(pcm_file, &output_file, format) {
                Ok(config) => {
                    println!("   ✅ 自动转换成功: {} -> {}", pcm_file, output_file);
                    println!("   📊 检测配置: {}Hz, {}ch, {}bit", 
                             config.sample_rate, config.channels, config.bits_per_sample);
                }
                Err(e) => println!("   ❌ 自动转换失败: {}", e),
            }
        }
    }
    println!();
}

fn demo_config_inference() {
    println!("🎵 4. 智能配置推断演示");
    
    let test_filenames = vec![
        "音频_8k16bit单声道.pcm",
        "test_44.1k16bit双声道.pcm",
        "recording_48k16bit单声道.pcm",
        "music_96k24bit双声道.pcm",
        "voice_22k16bits单声道.pcm",
    ];
    
    for filename in test_filenames {
        let config = infer_audio_config_from_filename(filename);
        println!("   📁 {} -> {}Hz, {}ch, {}bit", 
                 filename, config.sample_rate, config.channels, config.bits_per_sample);
    }
    println!();
}

fn demo_ffi_interface() {
    println!("🎵 5. FFI 接口演示（移动端集成）");
    
    // 获取库版本
    let version_ptr = get_version();
    if !version_ptr.is_null() {
        let version_cstr = unsafe { std::ffi::CStr::from_ptr(version_ptr) };
        if let Ok(version_str) = version_cstr.to_str() {
            println!("   📦 库版本: {}", version_str);
        }
        // 释放内存
        unsafe { audio_helper::ffi::free_string(version_ptr) };
    }
    
    // 测试 FFI 配置推断
    let filename = CString::new("test_32k16bit单声道.pcm").unwrap();
    let mut ffi_config = CPcmConfig {
        sample_rate: 0,
        channels: 0,
        bits_per_sample: 0,
    };
    
    let result = infer_config_from_filename(filename.as_ptr(), &mut ffi_config);
    if result == 0 {
        println!("   📊 FFI 推断结果: {}Hz, {}ch, {}bit", 
                 ffi_config.sample_rate, ffi_config.channels, ffi_config.bits_per_sample);
    }
    
    // 测试 FFI 转换（如果有测试文件）
    let test_file = "pcmFile/冰雨片段8k16bit单声道.pcm";
    if std::path::Path::new(test_file).exists() {
        let input_path = CString::new(test_file).unwrap();
        let output_path = CString::new("output_wav/ffi_test_output.wav").unwrap();
        
        let ffi_result = pcm_to_wav(input_path.as_ptr(), output_path.as_ptr(), ptr::null());
        if ffi_result == 0 {
            println!("   ✅ FFI WAV 转换成功");
        } else {
            println!("   ❌ FFI WAV 转换失败");
        }
        
        // 测试 FFI MP3 转换
        let mp3_output = CString::new("output_wav/ffi_test_output.mp3").unwrap();
        let ffi_mp3_result = pcm_to_mp3(input_path.as_ptr(), mp3_output.as_ptr(), ptr::null());
        if ffi_mp3_result == 0 {
            println!("   ✅ FFI MP3 转换成功");
        } else {
            println!("   ❌ FFI MP3 转换失败");
        }
    }
    
    println!();
}