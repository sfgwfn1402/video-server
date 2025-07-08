<script setup>
import { ref } from 'vue'
import VideoClipTool from './components/VideoClipTool.vue'

const videoUrl = ref('')
const timestamp = ref(0)
const imageUrl = ref('')
const loading = ref(false)
const error = ref('')

const tab = ref('snapshot')

// 预设的示例流
const examples = [
  { name: 'RTMP 流', url: 'rtmp://live.example.com/live/stream' },
  { name: 'RTSP 流', url: 'rtsp://camera.example.com:554/stream' },
  { name: 'HLS 流', url: 'https://example.com/live/stream.m3u8' },
  { name: 'HTTP 视频', url: 'https://sample-videos.com/zip/10/mp4/SampleVideo_1280x720_1mb.mp4' },
]

function setExample(example) {
  videoUrl.value = example.url
}

async function takeSnapshot() {
  error.value = ''
  imageUrl.value = ''
  if (!videoUrl.value) {
    error.value = '请输入视频流地址'
    return
  }
  loading.value = true
  try {
    const res = await fetch('/api/snapshot', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ 
        url: videoUrl.value,
        timestamp: parseFloat(timestamp.value) || 0
      })
    })
    if (!res.ok) throw new Error('请求失败')
    // 假设后端返回图片二进制流
    const blob = await res.blob()
    imageUrl.value = URL.createObjectURL(blob)
  } catch (e) {
    error.value = e.message || '请求出错'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div style="max-width: 800px; margin: 40px auto; padding: 24px; border-radius: 12px; box-shadow: 0 2px 8px #eee; background: #fff;">
    <div style="display: flex; gap: 16px; margin-bottom: 24px;">
      <button :class="['tab-btn', tab === 'snapshot' ? 'active' : '']" @click="tab = 'snapshot'">截图工具</button>
      <button :class="['tab-btn', tab === 'clip' ? 'active' : '']" @click="tab = 'clip'">剪辑工具</button>
    </div>
    <div v-if="tab === 'snapshot'">
      <h2 class="title">视频流截图工具</h2>
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
      <div style="margin-bottom: 16px;">
        <label class="label">截图时间 (秒):</label>
        <input 
          v-model="timestamp" 
          type="number" 
          min="0" 
          step="0.1"
          placeholder="0" 
          class="input"
        />
        <small class="tip">对于实时流，建议设置为 0 或较小的值</small>
      </div>
      <button 
        @click="takeSnapshot" 
        :disabled="loading" 
        class="main-btn"
      >
        {{ loading ? '正在截图...' : '截图' }}
      </button>
      <div v-if="error" class="error-box">
        {{ error }}
      </div>
      <div v-if="imageUrl" style="margin-top: 20px; text-align: center;">
        <h3 class="result-title">截图结果:</h3>
        <img :src="imageUrl" alt="截图" style="max-width: 100%; border-radius: 8px; border: 1px solid #eee;" />
      </div>
    </div>
    <div v-else>
      <VideoClipTool />
    </div>
  </div>
</template>

<style scoped>
body {
  background: #181818;
}
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
.tip {
  color: #888;
  font-size: 0.95rem;
  margin-left: 2px;
}
.result-title {
  color: #222;
  font-size: 1.15rem;
  font-weight: bold;
  margin-bottom: 10px;
}
.tab-btn {
  padding: 10px 24px;
  border: none;
  border-bottom: 3px solid transparent;
  background: #f7f7f7;
  color: #222;
  font-size: 1.1rem;
  font-weight: bold;
  cursor: pointer;
  border-radius: 6px 6px 0 0;
  transition: border 0.2s, background 0.2s;
}
.tab-btn.active {
  background: #fff;
  border-bottom: 3px solid #42b983;
  color: #42b983;
}
</style>
