use tempfile::NamedTempFile;
use std::process::Command;
use uuid::Uuid;

pub struct VideoSnapshotService;

impl VideoSnapshotService {
    pub fn new() -> Self {
        VideoSnapshotService
    }

    /// 截取视频流指定时间的图片，返回 PNG 二进制
    pub async fn capture_frame(&self, url: &str, timestamp: f64) -> Result<Vec<u8>, String> {
        let temp_file = NamedTempFile::new()
            .map_err(|e| format!("Failed to create temp file: {}", e))?;
        let output_path = temp_file.path().to_str()
            .ok_or("Invalid temp file path")?;
        
        let timestamp_str = timestamp.to_string();
        let mut args = vec![
            "-i", url,
            "-ss", &timestamp_str,
            "-vframes", "1",
            "-f", "image2",
            "-y",
        ];
        if Self::is_realtime_stream(url) {
            let realtime_args = vec![
                "-reconnect", "1",
                "-reconnect_streamed", "1",
                "-reconnect_delay_max", "5",
                "-timeout", "10000000",
            ];
            args.extend(realtime_args);
        }
        args.push(output_path);
        let mut cmd = Command::new("ffmpeg");
        cmd.args(&args);
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
        let start_str = start.to_string();
        let duration_str = duration.to_string();
        let mut args = vec![
            "-ss", &start_str,
            "-i", url,
            "-t", &duration_str,
            "-c", "copy",
            "-y",
            &output_path,
        ];
        if Self::is_realtime_stream(url) {
            let realtime_args = vec![
                "-reconnect", "1",
                "-reconnect_streamed", "1",
                "-reconnect_delay_max", "5",
                "-timeout", "10000000",
            ];
            args.splice(0..0, realtime_args); // 插到最前面
        }
        let mut cmd = Command::new("ffmpeg");
        cmd.args(&args);
        tracing::info!("Executing ffmpeg command for clip: {:?}", cmd);
        let status = cmd.status().map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;
        if status.success() {
            // 新增：清理clips目录，最多只保留10个文件
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

    fn is_realtime_stream(url: &str) -> bool {
        let realtime_protocols = [
            "rtmp://", "rtsp://", "rtp://", "udp://", "hls://", 
            "http://", "https://", "srt://", "webrtc://"
        ];
        realtime_protocols.iter().any(|protocol| url.starts_with(protocol))
    }
} 