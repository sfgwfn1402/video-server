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
  <div style="max-width: 800px; margin: 40px auto; padding: 24px; border-radius: 12px; box-shadow: 0 2px 8px #eee; background: #fff;">
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
  color: #222;
  font-size: 2rem;
  font-weight: bold;
  margin-bottom: 8px;
}
.desc {
  color: #444;
  font-size: 1.1rem;
  margin-bottom: 18px;
}
.label {
  color: #222;
  font-size: 1rem;
  font-weight: bold;
  margin-bottom: 6px;
  display: block;
}
.input {
  width: 100%;
  padding: 10px 12px;
  font-size: 1.1rem;
  border-radius: 5px;
  border: 1.5px solid #bbb;
  background: #fff;
  color: #222;
  margin-bottom: 2px;
  outline: none;
  transition: border 0.2s;
}
.input:focus {
  border: 1.5px solid #42b983;
}
.example-btn {
  padding: 7px 16px;
  background: #f7f7f7;
  border: 1.5px solid #bbb;
  border-radius: 4px;
  cursor: pointer;
  font-size: 1rem;
  color: #222;
  font-weight: 500;
  transition: background 0.2s, border 0.2s;
}
.example-btn:hover {
  background: #e0f7ef;
  border: 1.5px solid #42b983;
}
.main-btn {
  width: 100%;
  padding: 12px;
  background: #42b983;
  color: #fff;
  border: none;
  border-radius: 5px;
  font-size: 1.2rem;
  font-weight: bold;
  cursor: pointer;
  margin-bottom: 16px;
  transition: background 0.2s;
}
.main-btn:disabled {
  background: #b2dfcf;
  cursor: not-allowed;
}
.error-box {
  color: #b71c1c;
  background: #ffeaea;
  border: 1.5px solid #ffbdbd;
  border-radius: 4px;
  padding: 10px;
  margin-top: 10px;
  font-size: 1.05rem;
}
.result-title {
  color: #222;
  font-size: 1.15rem;
  font-weight: bold;
  margin-bottom: 10px;
}
</style> 