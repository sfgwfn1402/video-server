<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import VideoClipTool from './components/VideoClipTool.vue'

const videoUrl = ref('')
const timestamp = ref(0)
const imageUrl = ref('')
const loading = ref(false)
const error = ref('')

const tab = ref('snapshot')

// 系统监控统计相关
const systemStats = ref({
  cpu_usage: 0,
  memory_usage: 0,
  memory_total: 0,
  memory_used: 0,
  current_requests: 0,
  uptime: 0
})
const lastUpdateTime = ref('')
const statsLoading = ref(false)
let statsInterval = null

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

// 获取系统监控统计
async function fetchSystemStats() {
  try {
    statsLoading.value = true
    const res = await fetch('/api/system-stats')
    if (res.ok) {
      const data = await res.json()
      systemStats.value = data
      lastUpdateTime.value = new Date().toLocaleTimeString()
    }
  } catch (e) {
    console.warn('获取系统统计失败:', e.message)
  } finally {
    statsLoading.value = false
  }
}

// 格式化内存大小
function formatMemorySize(bytes) {
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  let size = bytes
  let unitIndex = 0
  
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024
    unitIndex++
  }
  
  return `${size.toFixed(1)} ${units[unitIndex]}`
}

// 启动定时获取统计数据
function startStatsPolling() {
  fetchSystemStats() // 立即获取一次
  statsInterval = setInterval(fetchSystemStats, 2000) // 每2秒更新一次
}

// 停止定时获取
function stopStatsPolling() {
  if (statsInterval) {
    clearInterval(statsInterval)
    statsInterval = null
  }
}

async function takeSnapshot() {
  error.value = ''
  imageUrl.value = ''
  if (!videoUrl.value) {
    error.value = '请输入视频流地址'
    return
  }
  loading.value = true
  
  // 开始处理时立即更新一次统计
  fetchSystemStats()
  
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
    // 处理完成后再次更新统计
    setTimeout(fetchSystemStats, 500)
  }
}

// 生命周期钩子
onMounted(() => {
  startStatsPolling()
})

onUnmounted(() => {
  stopStatsPolling()
})
</script>

<template>
  <div style="max-width: 800px; margin: 40px auto; padding: 24px; border-radius: 12px; box-shadow: 0 2px 8px #eee; background: #fff;">
    <!-- 系统监控统计显示区域 -->
    <div class="stats-container">
      <div class="stats-grid">
        <!-- 并发请求统计 -->
        <div class="stats-card">
          <div class="stats-icon">📊</div>
          <div class="stats-content">
            <div class="stats-title">并发请求</div>
            <div class="stats-number" :class="{ 'stats-loading': statsLoading }">
              {{ systemStats.current_requests }}
            </div>
          </div>
        </div>

        <!-- CPU使用率统计 -->
        <div class="stats-card">
          <div class="stats-icon">🧠</div>
          <div class="stats-content">
            <div class="stats-title">CPU 使用率</div>
            <div class="stats-number" :class="{ 'stats-loading': statsLoading }">
              {{ systemStats.cpu_usage.toFixed(1) }}%
            </div>
          </div>
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: systemStats.cpu_usage + '%' }"></div>
          </div>
        </div>

        <!-- 内存使用率统计 -->
        <div class="stats-card">
          <div class="stats-icon">💾</div>
          <div class="stats-content">
            <div class="stats-title">内存使用率</div>
            <div class="stats-number" :class="{ 'stats-loading': statsLoading }">
              {{ systemStats.memory_usage.toFixed(1) }}%
            </div>
            <div class="stats-subtitle">
              {{ formatMemorySize(systemStats.memory_used) }} / {{ formatMemorySize(systemStats.memory_total) }}
            </div>
          </div>
          <div class="progress-bar">
            <div class="progress-fill memory" :style="{ width: systemStats.memory_usage + '%' }"></div>
          </div>
        </div>
      </div>

      <!-- 最后更新时间 -->
      <div class="stats-time" v-if="lastUpdateTime">
        最后更新: {{ lastUpdateTime }}
      </div>
    </div>

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
@import url('https://fonts.googleapis.com/css2?family=SF+Pro+Display:wght@300;400;500;600;700&display=swap');

* {
  font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Display', 'Segoe UI', Roboto, sans-serif;
}

body {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  min-height: 100vh;
}

/* 系统监控统计样式 - 苹果风格 */
.stats-container {
  margin-bottom: 20px;
  padding: 16px;
  background: rgba(255, 255, 255, 0.15);
  backdrop-filter: blur(20px);
  border-radius: 16px;
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 12px;
  margin-bottom: 12px;
}

.stats-card {
  background: rgba(255, 255, 255, 0.9);
  border-radius: 12px;
  padding: 12px;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.3);
  transition: all 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.stats-card:hover {
  transform: translateY(-2px);
  background: rgba(255, 255, 255, 0.95);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.1);
}

.stats-icon {
  font-size: 1.2rem;
  margin-bottom: 6px;
  display: block;
  font-weight: 600;
}

.stats-card:nth-child(1) .stats-icon {
  color: #FF6B6B;
}

.stats-card:nth-child(2) .stats-icon {
  color: #4ECDC4;
}

.stats-card:nth-child(3) .stats-icon {
  color: #667eea;
}

.stats-content {
  margin-bottom: 8px;
}

.stats-title {
  font-size: 0.75rem;
  font-weight: 600;
  margin-bottom: 3px;
  letter-spacing: 0.5px;
  text-transform: uppercase;
}

.stats-card:nth-child(1) .stats-title {
  color: #FF6B6B;
}

.stats-card:nth-child(2) .stats-title {
  color: #4ECDC4;
}

.stats-card:nth-child(3) .stats-title {
  color: #667eea;
}

.stats-number {
  font-size: 1.8rem;
  font-weight: 700;
  line-height: 1;
  color: #2c3e50;
  transition: all 0.3s ease;
}

.stats-number.stats-loading {
  opacity: 0.6;
}

.stats-subtitle {
  font-size: 0.7rem;
  color: #7f8c8d;
  margin-top: 3px;
  font-weight: 500;
}

.progress-bar {
  width: 100%;
  height: 4px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 6px;
  overflow: hidden;
  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.1);
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #FF6B6B, #FF8E8E);
  border-radius: 6px;
  transition: width 0.6s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  box-shadow: 0 1px 4px rgba(255, 107, 107, 0.3);
}

.progress-fill.memory {
  background: linear-gradient(90deg, #667eea, #764ba2);
  box-shadow: 0 1px 4px rgba(102, 126, 234, 0.3);
}

.stats-time {
  text-align: center;
  font-size: 0.7rem;
  color: rgba(255, 255, 255, 0.9);
  font-style: italic;
  font-weight: 500;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

/* 标签页样式 - 苹果风格 */
.tab-btn {
  padding: 12px 24px;
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.4);
  border-radius: 12px;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 600;
  color: #2c3e50;
  transition: all 0.3s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.tab-btn.active {
  background: rgba(255, 255, 255, 1);
  border-color: #3498db;
  color: #3498db;
  transform: scale(1.05);
  box-shadow: 0 4px 20px rgba(52, 152, 219, 0.2);
}

.tab-btn:hover:not(.active) {
  background: rgba(255, 255, 255, 0.95);
  border-color: rgba(255, 255, 255, 0.6);
  color: #2c3e50;
  transform: translateY(-1px);
}

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

.tip {
  color: #7f8c8d;
  font-size: 0.85rem;
  margin-left: 4px;
  font-style: italic;
  font-weight: 400;
}

.result-title {
  color: #27ae60;
  font-size: 1.8rem;
  margin-bottom: 16px;
  font-weight: 600;
}

/* 主容器样式 */
div[style*="max-width: 800px"] {
  background: rgba(255, 255, 255, 0.95) !important;
  backdrop-filter: blur(20px) !important;
  border: 1px solid rgba(255, 255, 255, 0.2) !important;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.1) !important;
}

/* 复选框标签样式修复 */
label[style*="font-size: 1rem"] {
  color: #2c3e50 !important;
  font-weight: 500 !important;
}

/* 内联样式标签的通用修复 */
div[style*="margin-bottom: 16px"] label {
  color: #2c3e50 !important;
}
</style>
