#ifndef AUDIO_HELPER_H
#define AUDIO_HELPER_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// ==================== 类型定义 ====================

/**
 * C 兼容的 PCM 配置结构体
 */
typedef struct {
    uint32_t sample_rate;      // 采样率 (Hz)
    uint16_t channels;         // 声道数 (1=单声道, 2=双声道)
    uint16_t bits_per_sample;  // 位深度 (通常是 16)
} CPcmConfig;

/**
 * C 兼容的 MP3 配置结构体
 */
typedef struct {
    uint32_t sample_rate;      // 采样率 (Hz)
    uint8_t channels;          // 声道数 (1=单声道, 2=双声道)
    uint32_t bitrate;          // 比特率 (64, 128, 192, 256, 320)
    uint8_t quality;           // 质量 (0=低, 1=中, 2=高, 3=最佳)
} CMp3Config;

/**
 * C 兼容的音频格式枚举
 */
typedef enum {
    AUDIO_FORMAT_WAV = 0,      // WAV 格式
    AUDIO_FORMAT_MP3 = 1       // MP3 格式
} CAudioFormat;

// ==================== 核心转换函数 ====================

/**
 * PCM 转 WAV
 * @param input_path 输入 PCM 文件路径
 * @param output_path 输出 WAV 文件路径
 * @param config PCM 配置，可以为 NULL 使用默认配置
 * @return 0 成功，-1 失败
 */
int pcm_to_wav(const char* input_path, const char* output_path, const CPcmConfig* config);

/**
 * PCM 转 MP3
 * @param input_path 输入 PCM 文件路径
 * @param output_path 输出 MP3 文件路径
 * @param config MP3 配置，可以为 NULL 使用默认配置
 * @return 0 成功，-1 失败
 */
int pcm_to_mp3(const char* input_path, const char* output_path, const CMp3Config* config);

/**
 * 智能自动转换 PCM 到指定格式（从文件名推断配置）
 * @param input_path 输入 PCM 文件路径
 * @param output_path 输出文件路径
 * @param format 输出格式 (AUDIO_FORMAT_WAV 或 AUDIO_FORMAT_MP3)
 * @return 0 成功，-1 失败
 */
int auto_convert_audio(const char* input_path, const char* output_path, CAudioFormat format);

// ==================== 辅助功能 ====================

/**
 * 从文件名推断音频配置
 * @param filename 文件名（支持中文和各种格式）
 * @param config 输出配置结构体指针
 * @return 0 成功，-1 失败
 */
int infer_config_from_filename(const char* filename, CPcmConfig* config);

/**
 * 获取最后一次错误信息
 * @return 错误信息字符串，需要调用 free_string 释放内存
 */
char* get_last_error(void);

/**
 * 释放由库分配的字符串内存
 * @param str_ptr 要释放的字符串指针
 */
void free_string(char* str_ptr);

/**
 * 获取库版本信息
 * @return 版本字符串，需要调用 free_string 释放内存
 */
char* get_version(void);

// ==================== 常用配置预设 ====================

/**
 * 创建默认 PCM 配置 (44.1kHz, 双声道, 16位)
 */
static inline CPcmConfig create_default_pcm_config() {
    CPcmConfig config = { 44100, 2, 16 };
    return config;
}

/**
 * 创建电话质量 PCM 配置 (8kHz, 单声道, 16位)
 */
static inline CPcmConfig create_phone_quality_config() {
    CPcmConfig config = { 8000, 1, 16 };
    return config;
}

/**
 * 创建 CD 质量 PCM 配置 (44.1kHz, 双声道, 16位)
 */
static inline CPcmConfig create_cd_quality_config() {
    CPcmConfig config = { 44100, 2, 16 };
    return config;
}

/**
 * 创建高质量 MP3 配置 (44.1kHz, 双声道, 320kbps, 最佳质量)
 */
static inline CMp3Config create_high_quality_mp3_config() {
    CMp3Config config = { 44100, 2, 320, 3 };
    return config;
}

/**
 * 创建标准 MP3 配置 (44.1kHz, 双声道, 192kbps, 高质量)
 */
static inline CMp3Config create_standard_mp3_config() {
    CMp3Config config = { 44100, 2, 192, 2 };
    return config;
}

/**
 * 创建压缩 MP3 配置 (22kHz, 单声道, 128kbps, 中等质量)
 */
static inline CMp3Config create_compressed_mp3_config() {
    CMp3Config config = { 22050, 1, 128, 1 };
    return config;
}

// ==================== 使用示例 (注释) ====================

/*
// 简单的 PCM 到 WAV 转换
if (pcm_to_wav("input.pcm", "output.wav", NULL) == 0) {
    printf("转换成功!\n");
}

// 使用自定义配置的 PCM 到 MP3 转换
CMp3Config mp3_config = create_standard_mp3_config();
if (pcm_to_mp3("input.pcm", "output.mp3", &mp3_config) == 0) {
    printf("MP3 转换成功!\n");
}

// 智能自动转换（从文件名推断配置）
if (auto_convert_audio("audio_8k16bit单声道.pcm", "output.wav", AUDIO_FORMAT_WAV) == 0) {
    printf("自动转换成功!\n");
}

// 推断配置信息
CPcmConfig inferred_config;
if (infer_config_from_filename("test_48k16bit双声道.pcm", &inferred_config) == 0) {
    printf("检测到: %dHz, %d声道, %d位\n", 
           inferred_config.sample_rate, 
           inferred_config.channels, 
           inferred_config.bits_per_sample);
}
*/

#ifdef __cplusplus
}
#endif

#endif // AUDIO_HELPER_H