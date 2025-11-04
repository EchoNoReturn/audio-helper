# Audio Helper

一个高性能的 Rust 音频处理库，专为移动端应用设计，支持 PCM 到 WAV/MP3 的转换，具有智能配置推断和跨平台 FFI 接口。

## ✨ 特性

### 🎵 核心功能
- **PCM 到 WAV 转换** - 支持多种采样率和声道配置
- **PCM 到 MP3 转换** - 基于 mp3lame-encoder，支持可变比特率和质量设置
- **智能配置推断** - 从文件名自动识别音频参数（采样率、声道数、位深度）
- **自动格式转换** - 一键转换到目标格式，无需手动配置

### 📱 移动端支持
- **C FFI 接口** - 完整的 C 兼容接口，适用于 iOS/Android 集成
- **跨平台编译** - 支持 `cdylib`、`staticlib` 和 `rlib` 多种库类型
- **内存安全** - Rust 的内存安全保证，避免移动端崩溃
- **性能优化** - 发布模式下启用 LTO 和最高优化级别

### 🔧 技术特性
- **智能文件名解析** - 支持中英文混合的文件名格式
- **多格式支持** - 8k/16k/22k/32k/44.1k/48k/96k 等多种采样率
- **高性能处理** - 测试显示处理速度超过 300MB/s
- **压缩效率** - MP3 压缩比可达 4-8:1

## 🚀 快速开始

### 添加依赖

```toml
[dependencies]
audio-helper = "0.1.0"
```

### 基本使用

```rust
use audio_helper::{trans_pcm_file_to_wav, auto_convert_pcm, AudioFormat};

// 基本 PCM 到 WAV 转换
trans_pcm_file_to_wav("input.pcm", "output.wav", None)?;

// 智能自动转换（从文件名推断配置）
auto_convert_pcm("audio_48k16bit单声道.pcm", "output.mp3", AudioFormat::Mp3)?;
```

## 📚 API 参考

### Rust API

#### 核心转换函数

```rust
// PCM 转 WAV
pub fn trans_pcm_file_to_wav(
    input_path: &str, 
    output_path: &str, 
    config: Option<PcmToWavConfig>
) -> Result<(), Box<dyn std::error::Error>>

// PCM 转 MP3
pub fn trans_pcm_file_to_mp3(
    input_path: &str, 
    output_path: &str, 
    config: Option<Mp3Config>
) -> Result<(), Box<dyn std::error::Error>>

// 智能自动转换
pub fn auto_convert_pcm(
    input_path: &str, 
    output_path: &str, 
    format: AudioFormat
) -> Result<AudioConfig, Box<dyn std::error::Error>>
```

#### 配置结构体

```rust
// PCM 到 WAV 配置
pub struct PcmToWavConfig {
    pub sample_rate: u32,    // 采样率 (Hz)
    pub channels: u8,        // 声道数
    pub bits_per_sample: u16, // 位深度
}

// MP3 配置
pub struct Mp3Config {
    pub sample_rate: u32,     // 采样率 (Hz)
    pub channels: u8,         // 声道数
    pub bitrate: Mp3Bitrate,  // 比特率
    pub quality: AudioQuality, // 编码质量
}
```

#### 智能推断

```rust
// 从文件名推断音频配置
pub fn infer_audio_config_from_filename(filename: &str) -> AudioConfig

// 支持的文件名格式：
// "audio_8k16bit单声道.pcm" -> 8000Hz, 1ch, 16bit
// "music_44.1k16bit双声道.pcm" -> 44100Hz, 2ch, 16bit
// "voice_48k16bits单声道.pcm" -> 48000Hz, 1ch, 16bit
```

### C FFI API

#### 基本转换

```c
// PCM 转 WAV
int pcm_to_wav(const char* input_path, const char* output_path, const CPcmConfig* config);

// PCM 转 MP3  
int pcm_to_mp3(const char* input_path, const char* output_path, const CMp3Config* config);

// 智能自动转换
int auto_convert_audio(const char* input_path, const char* output_path, CAudioFormat format);
```

#### 辅助功能

```c
// 配置推断
int infer_config_from_filename(const char* filename, CPcmConfig* config);

// 版本信息
char* get_version(void);

// 内存管理
void free_string(char* str_ptr);
```

## 🏗️ 项目结构

```
audio-helper/
├── src/
│   ├── lib.rs          # 主库文件
│   └── ffi.rs          # C FFI 绑定
├── examples/
│   ├── demo.rs                    # 基本使用示例
│   ├── complete_demo.rs           # 完整功能演示
│   ├── comprehensive_mp3_test.rs  # MP3 转换测试
│   └── mp3_conversion_test.rs     # MP3 功能测试
├── tests/
│   ├── integration_test.rs        # 集成测试
│   ├── real_pcm_test.rs          # 真实文件测试
│   └── smart_config_test.rs      # 智能配置测试
├── audio_helper.h      # C 头文件
└── Cargo.toml         # 项目配置
```

## 🧪 测试

### 运行所有测试

```bash
cargo test
```

### 运行示例

```bash
# 基本演示
cargo run --example demo

# 完整功能演示
cargo run --example complete_demo

# MP3 转换测试
cargo run --example comprehensive_mp3_test
```

### 性能测试

```bash
# 运行性能基准测试
cargo test --release performance_test
```

## 📱 移动端集成

### iOS 集成

1. **编译静态库**
   ```bash
   cargo build --release --target aarch64-apple-ios
   ```

2. **集成头文件**
   ```c
   #include "audio_helper.h"
   ```

3. **使用示例**
   ```c
   // 创建配置
   CPcmConfig config = create_phone_quality_config(); // 8kHz, 单声道
   
   // 转换音频
   if (pcm_to_wav("input.pcm", "output.wav", &config) == 0) {
       NSLog(@"转换成功");
   }
   ```

### Android 集成

1. **编译动态库**
   ```bash
   cargo build --release --target aarch64-linux-android
   ```

2. **JNI 绑定**
   ```java
   public class AudioHelper {
       static {
           System.loadLibrary("audio_helper");
       }
       
       public static native int pcmToWav(String inputPath, String outputPath);
       public static native String getVersion();
   }
   ```

## 🎯 支持的格式

### 输入格式
- **PCM** - 原始 PCM 音频数据
- **采样率**: 8kHz, 16kHz, 22.05kHz, 32kHz, 44.1kHz, 48kHz, 96kHz
- **声道**: 单声道, 双声道  
- **位深度**: 16bit, 24bit

### 输出格式
- **WAV** - 无损音频格式，完整保留音质
- **MP3** - 压缩音频格式，支持多种比特率
  - 比特率: 64kbps, 128kbps, 192kbps, 256kbps, 320kbps
  - 质量: Low, Medium, High, Best

## 📈 性能指标

基于真实测试数据：

| 操作类型 | 处理速度 | 压缩比 | 内存使用 |
|---------|---------|-------|---------|
| PCM→WAV | 300+ MB/s | 1:1 (无损) | 低 |
| PCM→MP3 | 50+ MB/s | 4-8:1 | 中等 |
| 配置推断 | 即时 | N/A | 极低 |

测试环境: Apple M1, 8GB RAM, Rust 1.70+

## 🛠️ 开发

### 构建要求

- Rust 1.70+ 
- mp3lame 库 (自动通过 mp3lame-encoder crate 处理)

### 编译

```bash
# 开发模式
cargo build

# 发布模式 (启用所有优化)
cargo build --release

# 生成文档
cargo doc --open
```

### 跨平台编译

```bash
# iOS
rustup target add aarch64-apple-ios
cargo build --release --target aarch64-apple-ios

# Android  
rustup target add aarch64-linux-android
cargo build --release --target aarch64-linux-android
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

### 开发指南

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)  
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [mp3lame-encoder](https://crates.io/crates/mp3lame-encoder) - MP3 编码支持
- [byteorder](https://crates.io/crates/byteorder) - 二进制数据处理

## 📞 联系

如有问题或建议，请通过以下方式联系：

- 提交 [Issue](https://github.com/EchoNoReturn/audio-helper/issues)
- 发送邮件到 yoyojcoder@qq.com

---

⭐ 如果这个项目对你有帮助，请给它一个 star！