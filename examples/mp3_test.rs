use mp3lame_encoder::{Builder, InterleavedPcm, DualPcm, FlushNoGap};
use std::mem::MaybeUninit;

fn main() {
    println!("Testing mp3lame-encoder API...");
    
    // 创建编码器
    match Builder::new() {
        Some(mut builder) => {
            println!("Builder created successfully");
            
            // 设置参数
            builder.set_num_channels(2).unwrap();
            builder.set_sample_rate(44100).unwrap();
            builder.set_brate(mp3lame_encoder::Bitrate::Kbps192).unwrap();
            builder.set_quality(mp3lame_encoder::Quality::Best).unwrap();
            
            // 构建编码器
            match builder.build() {
                Ok(mut encoder) => {
                    println!("Encoder built successfully!");
                    
                    // 测试编码方法
                    let test_samples = vec![0i16; 1024]; // 简单的测试数据
                    let mut mp3_output = vec![MaybeUninit::uninit(); 8192]; // 输出缓冲区
                    
                    // 测试交错 PCM（左右声道交错）
                    let interleaved = InterleavedPcm(&test_samples);
                    match encoder.encode(interleaved, &mut mp3_output) {
                        Ok(bytes_written) => println!("Interleaved encoding successful, {} bytes written", bytes_written),
                        Err(e) => println!("Interleaved encoding error: {:?}", e),
                    }
                    
                    // 测试分离 PCM（左右声道分开）
                    let left = vec![0i16; 512];
                    let right = vec![0i16; 512];
                    let dual = DualPcm{ left: &left, right: &right };
                    match encoder.encode(dual, &mut mp3_output) {
                        Ok(bytes_written) => println!("Dual channel encoding successful, {} bytes written", bytes_written),
                        Err(e) => println!("Dual channel encoding error: {:?}", e),
                    }
                    
                    // 测试 flush - 使用 FlushNoGap
                    match encoder.flush::<mp3lame_encoder::FlushNoGap>(&mut mp3_output) {
                        Ok(flush_result) => {
                            println!("Flush successful, result: {:?}", flush_result);
                        }
                        Err(e) => println!("Flush error: {:?}", e),
                    }
                }
                Err(e) => println!("Build error: {:?}", e),
            }
        }
        None => {
            println!("Builder creation failed - mp3lame library not available");
        }
    }
}