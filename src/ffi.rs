// FFI (Foreign Function Interface) 绑定，用于移动端调用

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use crate::{
    trans_pcm_file_to_wav, trans_pcm_file_to_mp3, auto_convert_pcm,
    PcmToWavConfig, Mp3Config, AudioFormat,
    Mp3Bitrate, AudioQuality
};

// ==================== C 结构体定义 ====================

/// C 兼容的 PCM 配置结构体
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPcmConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
}

/// C 兼容的 MP3 配置结构体
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CMp3Config {
    pub sample_rate: u32,
    pub channels: u8,
    pub bitrate: u32,      // 实际比特率值 (64, 128, 192, 256, 320)
    pub quality: u8,       // 0=Low, 1=Medium, 2=High, 3=Best
}

/// C 兼容的音频格式枚举
#[repr(C)]
pub enum CAudioFormat {
    Wav = 0,
    Mp3 = 1,
}

// ==================== 辅助函数 ====================

/// 将 C 字符串转换为 Rust 字符串
unsafe fn c_str_to_string(c_str: *const c_char) -> Result<String, Box<dyn std::error::Error>> {
    if c_str.is_null() {
        return Err("Null pointer provided".into());
    }
    
    let c_str = unsafe { CStr::from_ptr(c_str) };
    Ok(c_str.to_str()?.to_owned())
}

/// 将 CMp3Config 转换为 Mp3Config
fn c_mp3_config_to_rust(c_config: CMp3Config) -> Result<Mp3Config, Box<dyn std::error::Error>> {
    let bitrate = match c_config.bitrate {
        64 => Mp3Bitrate::Kbps64,
        128 => Mp3Bitrate::Kbps128,
        192 => Mp3Bitrate::Kbps192,
        256 => Mp3Bitrate::Kbps256,
        320 => Mp3Bitrate::Kbps320,
        _ => return Err(format!("Unsupported bitrate: {}", c_config.bitrate).into()),
    };
    
    let quality = match c_config.quality {
        0 => AudioQuality::Low,
        1 => AudioQuality::Medium,
        2 => AudioQuality::High,
        3 => AudioQuality::Best,
        _ => return Err(format!("Unsupported quality: {}", c_config.quality).into()),
    };
    
    Ok(Mp3Config::new(c_config.sample_rate, c_config.channels, bitrate, quality))
}

// ==================== PCM 到 WAV 转换 ====================

/// PCM 转 WAV (C FFI)
/// # 参数
/// * `input_path` - 输入 PCM 文件路径 (C 字符串)
/// * `output_path` - 输出 WAV 文件路径 (C 字符串)
/// * `config` - PCM 配置，可以为 NULL 使用默认配置
/// # 返回值
/// * 0 - 成功
/// * -1 - 失败
#[unsafe(no_mangle)]
pub extern "C" fn pcm_to_wav(
    input_path: *const c_char,
    output_path: *const c_char,
    config: *const CPcmConfig,
) -> c_int {
    let result = || -> Result<(), Box<dyn std::error::Error>> {
        let input_str = unsafe { c_str_to_string(input_path)? };
        let output_str = unsafe { c_str_to_string(output_path)? };
        
        let wav_config = if config.is_null() {
            None
        } else {
            let c_cfg = unsafe { *config };
            Some(PcmToWavConfig::new(
                c_cfg.sample_rate,
                c_cfg.channels as u8,
                c_cfg.bits_per_sample,
            ))
        };
        
        trans_pcm_file_to_wav(&input_str, &output_str, wav_config)?;
        Ok(())
    };
    
    match result() {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

// ==================== PCM 到 MP3 转换 ====================

/// PCM 转 MP3 (C FFI)
/// # 参数
/// * `input_path` - 输入 PCM 文件路径 (C 字符串)
/// * `output_path` - 输出 MP3 文件路径 (C 字符串)
/// * `config` - MP3 配置，可以为 NULL 使用默认配置
/// # 返回值
/// * 0 - 成功
/// * -1 - 失败
#[unsafe(no_mangle)]
pub extern "C" fn pcm_to_mp3(
    input_path: *const c_char,
    output_path: *const c_char,
    config: *const CMp3Config,
) -> c_int {
    let result = || -> Result<(), Box<dyn std::error::Error>> {
        let input_str = unsafe { c_str_to_string(input_path)? };
        let output_str = unsafe { c_str_to_string(output_path)? };
        
        let mp3_config = if config.is_null() {
            None
        } else {
            let c_cfg = unsafe { *config };
            Some(c_mp3_config_to_rust(c_cfg)?)
        };
        
        trans_pcm_file_to_mp3(&input_str, &output_str, mp3_config)?;
        Ok(())
    };
    
    match result() {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

// ==================== 智能自动转换 ====================

/// 智能自动转换 PCM 到指定格式 (C FFI)
/// # 参数
/// * `input_path` - 输入 PCM 文件路径 (C 字符串)
/// * `output_path` - 输出文件路径 (C 字符串)
/// * `format` - 输出格式 (0=WAV, 1=MP3)
/// # 返回值
/// * 0 - 成功
/// * -1 - 失败
#[unsafe(no_mangle)]
pub extern "C" fn auto_convert_audio(
    input_path: *const c_char,
    output_path: *const c_char,
    format: CAudioFormat,
) -> c_int {
    let result = || -> Result<(), Box<dyn std::error::Error>> {
        let input_str = unsafe { c_str_to_string(input_path)? };
        let output_str = unsafe { c_str_to_string(output_path)? };
        
        let audio_format = match format {
            CAudioFormat::Wav => AudioFormat::Wav,
            CAudioFormat::Mp3 => AudioFormat::Mp3,
        };
        
        auto_convert_pcm(&input_str, &output_str, audio_format)?;
        Ok(())
    };
    
    match result() {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

// ==================== 配置推断 ====================

/// 从文件名推断音频配置 (C FFI)
/// # 参数
/// * `filename` - 文件名 (C 字符串)
/// * `config` - 输出配置结构体指针
/// # 返回值
/// * 0 - 成功
/// * -1 - 失败
#[unsafe(no_mangle)]
pub extern "C" fn infer_config_from_filename(
    filename: *const c_char,
    config: *mut CPcmConfig,
) -> c_int {
    let result = || -> Result<(), Box<dyn std::error::Error>> {
        let filename_str = unsafe { c_str_to_string(filename)? };
        let audio_config = crate::infer_audio_config_from_filename(&filename_str);
        
        unsafe {
            (*config).sample_rate = audio_config.sample_rate;
            (*config).channels = audio_config.channels as u16;
            (*config).bits_per_sample = audio_config.bits_per_sample;
        }
        
        Ok(())
    };
    
    match result() {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

// ==================== 错误处理 ====================

/// 获取最后一次错误信息 (C FFI)
/// # 返回值
/// * 错误信息的 C 字符串指针，调用者需要释放内存
#[unsafe(no_mangle)]
pub extern "C" fn get_last_error() -> *mut c_char {
    // TODO: 实现全局错误状态管理
    let error_msg = CString::new("Error details not implemented yet").unwrap();
    error_msg.into_raw()
}

/// 释放 C 字符串内存
/// # 参数
/// * `str_ptr` - 要释放的 C 字符串指针
#[unsafe(no_mangle)]
pub extern "C" fn free_string(str_ptr: *mut c_char) {
    if !str_ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(str_ptr);
        }
    }
}

// ==================== 版本信息 ====================

/// 获取库版本信息 (C FFI)
/// # 返回值
/// * 版本字符串的 C 字符串指针，调用者需要释放内存
#[unsafe(no_mangle)]
pub extern "C" fn get_version() -> *mut c_char {
    let version = CString::new(env!("CARGO_PKG_VERSION")).unwrap();
    version.into_raw()
}