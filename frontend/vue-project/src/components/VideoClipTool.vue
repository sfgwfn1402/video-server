<script setup>
import { ref } from 'vue'

const videoUrl = ref('')
const start = ref(0)
const duration = ref(10)
const videoClipUrl = ref('')
const loading = ref(false)
const error = ref('')
const returnUrl = ref(true) // 新增：是否返回地址

const examples = [
  { name: 'RTMP 流', url: 'rtmp://live.example.com/live/stream' },
  { name: 'RTSP 流', url: 'rtsp://camera.example.com:554/stream' },
  { name: 'HLS 流', url: 'https://example.com/live/stream.m3u8' },
  { name: 'HTTP 视频', url: 'https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4' },
]

function setExample(example) {
  videoUrl.value = example.url
}

async function extractClip() {
  error.value = ''
  videoClipUrl.value = ''
  if (!videoUrl.value) {
    error.value = '请输入视频流地址'
    return
  }
  loading.value = true
  
  try {
    const res = await fetch('/api/clip', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        url: videoUrl.value,
        start: parseFloat(start.value) || 0,
        duration: parseFloat(duration.value) || 10,
        return_url: returnUrl.value // 新增参数
      })
    })
    if (!res.ok) throw new Error('请求失败')
    if (returnUrl.value) {
      const data = await res.json();
      if (data.video_url) {
        videoClipUrl.value = data.video_url;
      } else {
        error.value = data.error || '剪辑失败';
      }
    } else {
      // 直接播放二进制流
      const blob = await res.blob();
      videoClipUrl.value = URL.createObjectURL(blob);
    }
  } catch (e) {
    error.value = e.message || '请求出错'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div>
    <h2 class="title">视频流剪辑工具</h2>
    <p class="desc">
      支持多种视频格式：MP4, AVI, MOV, MKV, FLV, WebM, RTMP, RTSP, HLS, HTTP 流等
    </p>
    <div style="margin-bottom: 20px;">
      <label class="label">快速示例:</label>
      <div style="display: flex; gap: 10px; flex-wrap: wrap;">
        <button 
          v-for="example in examples" 
          :key="example.name"
          @click="setExample(example)"
          class="example-btn"
        >
          {{ example.name }}
        </button>
      </div>
    </div>
    <div style="margin-bottom: 16px;">
      <label class="label">视频流地址:</label>
      <input 
        v-model="videoUrl" 
        placeholder="请输入视频流地址 (如: rtmp://live.example.com/live/stream)" 
        class="input"
      />
    </div>
    <div style="margin-bottom: 16px; display: flex; gap: 12px;">
      <div style="flex:1">
        <label class="label">起始时间 (秒):</label>
        <input 
          v-model="start" 
          type="number" 
          min="0" 
          step="0.1"
          placeholder="0" 
          class="input"
        />
      </div>
      <div style="flex:1">
        <label class="label">持续时长 (秒):</label>
        <input 
          v-model="duration" 
          type="number" 
          min="1" 
          step="0.1"
          placeholder="10" 
          class="input"
        />
      </div>
    </div>
    <div style="margin-bottom: 16px;">
      <label style="font-size: 1rem;">
        <input type="checkbox" v-model="returnUrl" style="margin-right: 6px;" />
        返回地址（勾选：返回地址播放；不勾选：直接播放视频流）
      </label>
    </div>
    <button 
      @click="extractClip" 
      :disabled="loading" 
      class="main-btn"
    >
      {{ loading ? '正在剪辑...' : '截取片段' }}
    </button>
    <div v-if="error" class="error-box">
      {{ error }}
    </div>
    <div v-if="videoClipUrl" style="margin-top: 20px; text-align: center;">
      <h3 class="result-title">剪辑结果:</h3>
      <video :src="videoClipUrl" controls style="max-width: 100%; border-radius: 8px; border: 1px solid #eee;" />
    </div>
  </div>
</template>

<style scoped>
.title {
  color: #2c3e50;
  font-size: 2.2rem;
  font-weight: 700;
  margin-bottom: 12px;
}

.desc {
  color: #7f8c8d;
  font-size: 1.1rem;
  margin-bottom: 20px;
  font-weight: 400;
  line-height: 1.6;
}

.label {
  color: #34495e;
  font-size: 1rem;
  font-weight: 600;
  margin-bottom: 8px;
  display: block;
}

.input {
  width: 100%;
  padding: 14px 16px;
  font-size: 1rem;
  border-radius: 12px;
  border: 2px solid rgba(52, 152, 219, 0.2);
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(10px);
  color: #2c3e50;
  margin-bottom: 4px;
  outline: none;
  transition: all 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  font-weight: 500;
}

.input:focus {
  border: 2px solid #3498db;
  background: rgba(255, 255, 255, 1);
  transform: translateY(-1px);
  box-shadow: 0 8px 25px rgba(52, 152, 219, 0.15);
}

.example-btn {
  padding: 8px 16px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(52, 152, 219, 0.3);
  border-radius: 8px;
  cursor: pointer;
  font-size: 0.9rem;
  color: #2c3e50;
  font-weight: 500;
  transition: all 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.example-btn:hover {
  background: #3498db;
  color: white;
  transform: translateY(-1px);
  box-shadow: 0 6px 20px rgba(52, 152, 219, 0.3);
}

.main-btn {
  width: 100%;
  padding: 16px;
  background: linear-gradient(135deg, #3498db, #2980b9);
  color: #fff;
  border: none;
  border-radius: 12px;
  font-size: 1.1rem;
  font-weight: 600;
  cursor: pointer;
  margin-bottom: 16px;
  transition: all 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  box-shadow: 0 6px 20px rgba(52, 152, 219, 0.3);
}

.main-btn:disabled {
  background: linear-gradient(135deg, #bdc3c7, #95a5a6);
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.main-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 30px rgba(52, 152, 219, 0.4);
  background: linear-gradient(135deg, #2980b9, #3498db);
}

.error-box {
  background: rgba(231, 76, 60, 0.1);
  backdrop-filter: blur(10px);
  color: #e74c3c;
  padding: 16px;
  border-radius: 12px;
  border: 1px solid rgba(231, 76, 60, 0.3);
  margin-bottom: 16px;
  font-weight: 500;
}

.result-title {
  color: #27ae60;
  font-size: 1.8rem;
  margin-bottom: 16px;
  font-weight: 600;
}

/* 复选框标签样式 */
label[style*="font-size: 1rem"] {
  color: #2c3e50 !important;
  font-weight: 500 !important;
}
</style> 