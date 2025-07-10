use tempfile::NamedTempFile;
use std::process::Command;
use uuid::Uuid;

#[derive(Clone)]
pub struct VideoSnapshotService;

#[derive(Debug, Clone)]
pub enum StreamProtocol {
    RTSP,
    RTMP,
    HLS,
    HTTP,
    File,
    Unknown,
}

impl VideoSnapshotService {
    pub fn new() -> Self {
        VideoSnapshotService
    }

    /// 检查ffmpeg版本是否支持reconnect选项
    fn check_ffmpeg_supports_reconnect() -> bool {
        let output = Command::new("ffmpeg")
            .args(["-h", "full"])
            .output();
        
        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                stdout.contains("-reconnect")
            }
            Err(_) => false
        }
    }

    /// 检测流协议类型
    fn detect_protocol(url: &str) -> StreamProtocol {
        if url.starts_with("rtsp://") {
            StreamProtocol::RTSP
        } else if url.starts_with("rtmp://") {
            StreamProtocol::RTMP
        } else if url.contains(".m3u8") || url.starts_with("hls://") {
            StreamProtocol::HLS
        } else if url.starts_with("http://") || url.starts_with("https://") {
            StreamProtocol::HTTP
        } else if url.starts_with("file://") || (!url.contains("://") && std::path::Path::new(url).exists()) {
            StreamProtocol::File
        } else {
            StreamProtocol::Unknown
        }
    }

    /// 根据协议类型生成特定的ffmpeg参数
    fn get_protocol_args(protocol: &StreamProtocol, url: &str) -> Vec<String> {
        match protocol {
            StreamProtocol::RTSP => {
                // 对于某些摄像头，使用最基本的参数
                if url.contains("realmonitor") {
                    vec![
                        "-rtsp_transport".to_string(), "tcp".to_string(),
                        "-fflags".to_string(), "nobuffer".to_string(),
                        "-analyzeduration".to_string(), "1000000".to_string(), // 1秒分析时间
                        "-probesize".to_string(), "1000000".to_string(), // 1MB探测大小
                    ]
                } else {
                    vec![
                        "-rtsp_transport".to_string(), "tcp".to_string(),
                        "-timeout".to_string(), "10000000".to_string(),
                        "-analyzeduration".to_string(), "5000000".to_string(),
                        "-probesize".to_string(), "5000000".to_string(),
                        "-max_delay".to_string(), "500000".to_string(),
                    ]
                }
            },
            StreamProtocol::RTMP => vec![
                "-timeout".to_string(), "10000000".to_string(),
                "-analyzeduration".to_string(), "2000000".to_string(),
                "-probesize".to_string(), "2000000".to_string(),
            ],
            StreamProtocol::HLS => vec![
                "-timeout".to_string(), "10000000".to_string(),
                "-user_agent".to_string(), "Mozilla/5.0 (compatible; VideoServer/1.0)".to_string(),
                "-analyzeduration".to_string(), "3000000".to_string(),
                "-probesize".to_string(), "3000000".to_string(),
            ],
            StreamProtocol::HTTP => vec![
                "-user_agent".to_string(), "Mozilla/5.0 (compatible; VideoServer/1.0)".to_string(),
                "-timeout".to_string(), "10000000".to_string(),
            ],
            StreamProtocol::File => vec![],
            StreamProtocol::Unknown => vec![
                "-timeout".to_string(), "10000000".to_string(),
                "-analyzeduration".to_string(), "3000000".to_string(),
                "-probesize".to_string(), "3000000".to_string(),
            ],
        }
    }

    /// 截取视频流指定时间的图片，返回 PNG 二进制
    pub async fn capture_frame(&self, url: &str, timestamp: f64) -> Result<Vec<u8>, String> {
        let temp_file = NamedTempFile::new()
            .map_err(|e| format!("Failed to create temp file: {}", e))?;
        let output_path = temp_file.path().to_str()
            .ok_or("Invalid temp file path")?;
        
        let protocol = Self::detect_protocol(url);
        
        // 为特定摄像头使用简化的命令
        let mut args = if url.contains("realmonitor") {
            vec![
                "-rtsp_transport".to_string(), "tcp".to_string(),
                "-i".to_string(), url.to_string(),
                "-vframes".to_string(), "1".to_string(),
                "-f".to_string(), "image2".to_string(),
                "-y".to_string(),
                output_path.to_string(),
            ]
        } else {
            let protocol_args = Self::get_protocol_args(&protocol, url);
            let timestamp_str = timestamp.to_string();
            
            let mut args = Vec::new();
            
            // 添加协议特定参数
            args.extend(protocol_args);
            
            // 添加基本参数
            args.extend(vec![
                "-i".to_string(), url.to_string(),
                "-ss".to_string(), timestamp_str,
                "-vframes".to_string(), "1".to_string(),
                "-q:v".to_string(), "2".to_string(), // 高质量截图
                "-f".to_string(), "image2".to_string(),
                "-y".to_string(),
                output_path.to_string(),
            ]);
            
            args
        };

        let mut cmd = Command::new("ffmpeg");
        cmd.args(&args);
        
        tracing::info!("Detected protocol: {:?}", protocol);
        tracing::info!("Executing ffmpeg command: {:?}", cmd);
        
        let output = cmd.output()
            .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("FFmpeg failed: {}", stderr));
        }
        
        let image_data = std::fs::read(output_path)
            .map_err(|e| format!("Failed to read output file: {}", e))?;
            
        Ok(image_data)
    }

    /// 截取视频流一段，保存为本地文件，返回文件名
    pub async fn clip_video(&self, url: &str, start: f64, duration: f64) -> Result<String, String> {
        let filename = format!("{}.mp4", Uuid::new_v4());
        let output_path = format!("clips/{}", filename);
        
        let protocol = Self::detect_protocol(url);
        
        // 为特定摄像头使用简化的命令
        let mut args = if url.contains("realmonitor") {
            vec![
                "-rtsp_transport".to_string(), "tcp".to_string(),
                "-ss".to_string(), start.to_string(),
                "-i".to_string(), url.to_string(),
                "-t".to_string(), duration.to_string(),
                "-c:v".to_string(), "copy".to_string(), // 视频流复制
                "-c:a".to_string(), "aac".to_string(),  // 音频重新编码为AAC
                "-b:a".to_string(), "128k".to_string(), // 音频比特率
                "-avoid_negative_ts".to_string(), "make_zero".to_string(), // 避免负时间戳
                "-fflags".to_string(), "+genpts".to_string(), // 生成PTS
                "-y".to_string(),
                output_path.clone(),
            ]
        } else {
            let protocol_args = Self::get_protocol_args(&protocol, url);
            let start_str = start.to_string();
            let duration_str = duration.to_string();
            
            let mut args = Vec::new();
            
            // 添加协议特定参数
            args.extend(protocol_args);
            
            // 添加基本参数
            args.extend(vec![
                "-ss".to_string(), start_str,
                "-i".to_string(), url.to_string(),
                "-t".to_string(), duration_str,
            ]);
            
            // 根据协议选择编码策略
            match protocol {
                StreamProtocol::RTSP | StreamProtocol::RTMP => {
                    // 实时流视频复制，音频重新编码以确保兼容性
                    args.extend(vec![
                        "-c:v".to_string(), "copy".to_string(),
                        "-c:a".to_string(), "aac".to_string(),
                        "-b:a".to_string(), "128k".to_string(),
                    ]);
                },
                StreamProtocol::HLS | StreamProtocol::HTTP => {
                    // HTTP流可能需要重新编码以确保兼容性
                    args.extend(vec![
                        "-c:v".to_string(), "libx264".to_string(),
                        "-preset".to_string(), "fast".to_string(),
                        "-crf".to_string(), "23".to_string(),
                    ]);
                },
                StreamProtocol::File => {
                    // 本地文件视频复制，音频重新编码以确保兼容性
                    args.extend(vec![
                        "-c:v".to_string(), "copy".to_string(),
                        "-c:a".to_string(), "aac".to_string(),
                        "-b:a".to_string(), "128k".to_string(),
                    ]);
                },
                StreamProtocol::Unknown => {
                    // 未知协议使用保守的重编码
                    args.extend(vec![
                        "-c:v".to_string(), "libx264".to_string(),
                        "-preset".to_string(), "fast".to_string(),
                    ]);
                },
            }
            
            args.extend(vec![
                "-y".to_string(),
                output_path.clone(),
            ]);
            
            args
        };

        let mut cmd = Command::new("ffmpeg");
        cmd.args(&args);
        
        tracing::info!("Detected protocol: {:?}", protocol);
        tracing::info!("Executing ffmpeg command for clip: {:?}", cmd);
        
        let status = cmd.status()
            .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;
            
        if status.success() {
            // 清理clips目录，最多只保留100个文件
            if let Err(e) = Self::cleanup_clips_dir(100) {
                tracing::warn!("Failed to cleanup clips dir: {}", e);
            }
            Ok(filename)
        } else {
            Err("FFmpeg clip failed".to_string())
        }
    }

    /// 保证clips目录下最多只保留max_files个文件，删除最旧的
    fn cleanup_clips_dir(max_files: usize) -> Result<(), String> {
        use std::fs;
        use std::path::Path;
        
        let dir = Path::new("clips");
        let mut entries: Vec<_> = fs::read_dir(dir)
            .map_err(|e| format!("Failed to read clips dir: {}", e))?
            .filter_map(|e| e.ok())
            .filter(|e| {
                if let Some(ext) = e.path().extension() {
                    ext == "mp4"
                } else {
                    false
                }
            })
            .collect();
            
        if entries.len() > max_files {
            // 按修改时间排序，最旧的在前
            entries.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());
            let num_to_remove = entries.len() - max_files;
            
            for entry in entries.iter().take(num_to_remove) {
                if let Err(e) = fs::remove_file(entry.path()) {
                    tracing::warn!("Failed to remove old clip: {}", e);
                }
            }
        }
        Ok(())
    }

    /// 检测流是否为实时流（保持向后兼容）
    fn is_realtime_stream(url: &str) -> bool {
        matches!(Self::detect_protocol(url), 
            StreamProtocol::RTSP | StreamProtocol::RTMP | StreamProtocol::HLS)
    }
} 