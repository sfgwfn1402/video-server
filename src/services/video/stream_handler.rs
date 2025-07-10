use std::process::Command;
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamProtocol {
    RTSP {
        transport: RtspTransport,
        timeout: Duration,
        reconnect: bool,
    },
    RTMP {
        chunk_size: usize,
        live: bool,
    },
    HLS {
        segment_duration: u32,
        playlist_size: u32,
    },
    HTTP {
        timeout: Duration,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RtspTransport {
    TCP,
    UDP,
}

impl RtspTransport {
    pub fn as_str(&self) -> &'static str {
        match self {
            RtspTransport::TCP => "tcp",
            RtspTransport::UDP => "udp",
        }
    }
}

pub trait StreamHandler {
    fn build_snapshot_command(&self, url: &str, timestamp: f64, output: &str) -> Command;
    fn build_clip_command(&self, url: &str, start: f64, duration: f64, output: &str) -> Command;
    fn validate_url(&self, url: &str) -> bool;
}

pub struct RTSPHandler {
    transport: RtspTransport,
    timeout: Duration,
    reconnect: bool,
}

impl RTSPHandler {
    pub fn new(transport: RtspTransport, timeout: Duration, reconnect: bool) -> Self {
        Self {
            transport,
            timeout,
            reconnect,
        }
    }
}

impl StreamHandler for RTSPHandler {
    fn build_snapshot_command(&self, url: &str, timestamp: f64, output: &str) -> Command {
        let mut cmd = Command::new("ffmpeg");
        cmd.args([
            "-rtsp_transport", self.transport.as_str(),
            "-timeout", &self.timeout.as_micros().to_string(),
            "-analyzeduration", "5000000",
            "-probesize", "5000000",
            "-i", url,
            "-ss", &timestamp.to_string(),
            "-vframes", "1",
            "-f", "image2",
            "-y",
            output,
        ]);

        cmd
    }

    fn build_clip_command(&self, url: &str, start: f64, duration: f64, output: &str) -> Command {
        let mut cmd = Command::new("ffmpeg");
        cmd.args([
            "-rtsp_transport", self.transport.as_str(),
            "-timeout", &self.timeout.as_micros().to_string(),
            "-analyzeduration", "5000000",
            "-probesize", "5000000",
            "-ss", &start.to_string(),
            "-i", url,
            "-t", &duration.to_string(),
            "-c:v", "copy", // 视频流复制
            "-c:a", "aac",  // 音频重新编码为AAC
            "-b:a", "128k", // 音频比特率
            "-y",
            output,
        ]);

        cmd
    }

    fn validate_url(&self, url: &str) -> bool {
        url.starts_with("rtsp://")
    }
}

pub struct RTMPHandler {
    chunk_size: usize,
    live: bool,
}

impl RTMPHandler {
    pub fn new(chunk_size: usize, live: bool) -> Self {
        Self {
            chunk_size,
            live,
        }
    }
}

impl StreamHandler for RTMPHandler {
    fn build_snapshot_command(&self, url: &str, timestamp: f64, output: &str) -> Command {
        let mut cmd = Command::new("ffmpeg");
        cmd.args([
            "-i", url,
            "-ss", &timestamp.to_string(),
            "-vframes", "1",
            "-f", "image2",
            "-y",
        ]);

        if self.live {
            cmd.args([
                "-live_start_index", "0",
                "-analyzeduration", "1000000", // 1秒
                "-probesize", "1000000",
            ]);
        }

        cmd.arg(output);
        cmd
    }

    fn build_clip_command(&self, url: &str, start: f64, duration: f64, output: &str) -> Command {
        let mut cmd = Command::new("ffmpeg");
        
        if self.live {
            cmd.args([
                "-live_start_index", "0",
                "-analyzeduration", "1000000",
                "-probesize", "1000000",
            ]);
        }

        cmd.args([
            "-i", url,
            "-ss", &start.to_string(),
            "-t", &duration.to_string(),
            "-c:v", "libx264", // RTMP流通常需要重新编码
            "-preset", "veryfast",
            "-maxrate", "3000k",
            "-bufsize", "6000k",
            "-y",
            output,
        ]);

        cmd
    }

    fn validate_url(&self, url: &str) -> bool {
        url.starts_with("rtmp://")
    }
}

pub struct HLSHandler {
    segment_duration: u32,
    playlist_size: u32,
}

impl HLSHandler {
    pub fn new(segment_duration: u32, playlist_size: u32) -> Self {
        Self {
            segment_duration,
            playlist_size,
        }
    }
}

impl StreamHandler for HLSHandler {
    fn build_snapshot_command(&self, url: &str, timestamp: f64, output: &str) -> Command {
        let mut cmd = Command::new("ffmpeg");
        cmd.args([
            "-i", url,
            "-ss", &timestamp.to_string(),
            "-vframes", "1",
            "-f", "image2",
            "-y",
            "-timeout", "10000000",
            output,
        ]);

        cmd
    }

    fn build_clip_command(&self, url: &str, start: f64, duration: f64, output: &str) -> Command {
        let mut cmd = Command::new("ffmpeg");
        cmd.args([
            "-i", url,
            "-ss", &start.to_string(),
            "-t", &duration.to_string(),
            "-c:v", "libx264",
            "-hls_time", &self.segment_duration.to_string(),
            "-hls_list_size", &self.playlist_size.to_string(),
            "-hls_flags", "delete_segments",
            "-f", "hls",
            "-y",
            output,
        ]);

        cmd
    }

    fn validate_url(&self, url: &str) -> bool {
        url.ends_with(".m3u8") || url.starts_with("http") && url.contains("m3u8")
    }
}

pub struct HTTPHandler {
    timeout: Duration,
}

impl HTTPHandler {
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }
}

impl StreamHandler for HTTPHandler {
    fn build_snapshot_command(&self, url: &str, timestamp: f64, output: &str) -> Command {
        let mut cmd = Command::new("ffmpeg");
        cmd.args([
            "-i", url,
            "-ss", &timestamp.to_string(),
            "-vframes", "1",
            "-f", "image2",
            "-y",
            "-timeout", &self.timeout.as_micros().to_string(),
            output,
        ]);

        cmd
    }

    fn build_clip_command(&self, url: &str, start: f64, duration: f64, output: &str) -> Command {
        let mut cmd = Command::new("ffmpeg");
        cmd.args([
            "-i", url,
            "-ss", &start.to_string(),
            "-t", &duration.to_string(),
            "-c:v", "copy",
            "-y",
            "-timeout", &self.timeout.as_micros().to_string(),
            output,
        ]);

        cmd
    }

    fn validate_url(&self, url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }
}

pub fn create_handler(url: &str) -> Box<dyn StreamHandler> {
    if url.starts_with("rtsp://") {
        Box::new(RTSPHandler::new(
            RtspTransport::TCP,
            Duration::from_secs(10),
            true,
        ))
    } else if url.starts_with("rtmp://") {
        Box::new(RTMPHandler::new(4096, true))
    } else if url.ends_with(".m3u8") || (url.starts_with("http") && url.contains("m3u8")) {
        Box::new(HLSHandler::new(6, 10))
    } else {
        Box::new(HTTPHandler::new(Duration::from_secs(30)))
    }
}