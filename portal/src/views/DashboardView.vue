<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useProcessStore } from '@/stores/processStore';
import { useSecurityStore } from '@/stores/securityStore';

const processStore = useProcessStore();
const securityStore = useSecurityStore();

const currentTime = ref(new Date());
let timeInterval: number;

onMounted(() => {
  processStore.fetchProcesses();
  securityStore.scanVulnerabilities();
  timeInterval = window.setInterval(() => {
    currentTime.value = new Date();
  }, 1000);
});

onUnmounted(() => {
  clearInterval(timeInterval);
});

const formattedTime = computed(() => {
  return currentTime.value.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  });
});

const formattedDate = computed(() => {
  return currentTime.value.toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    weekday: 'long'
  });
});

const memoryUsagePercent = computed(() => {
  if (!processStore.totalMemory) return 0;
  return Math.round((processStore.usedMemory / processStore.totalMemory) * 100);
});

const memoryColor = computed(() => {
  const percent = memoryUsagePercent.value;
  if (percent > 80) return 'var(--color-danger)';
  if (percent > 60) return 'var(--color-warning)';
  return 'var(--color-success)';
});

const quickActions = [
  { name: '查看进程', path: '/processes', icon: '⚡', color: 'var(--color-primary)' },
  { name: '安全扫描', path: '/security', icon: '🛡️', color: 'var(--color-secondary)' },
  { name: 'Docker', path: '/docker', icon: '🐳', color: 'var(--color-accent)' },
];

const stats = computed(() => [
  {
    label: '运行进程',
    value: processStore.processes.length,
    icon: '⚡',
    color: 'var(--color-primary)',
    change: '+2',
    positive: true
  },
  {
    label: '内存使用',
    value: `${memoryUsagePercent.value}%`,
    icon: '💾',
    color: memoryColor.value,
    subtext: processStore.formatMemory(processStore.usedMemory),
    change: null,
    positive: true
  },
  {
    label: '运行时间',
    value: processStore.formatUptime(processStore.uptime),
    icon: '⏱️',
    color: 'var(--color-accent)',
    change: null,
    positive: true
  },
  {
    label: '安全威胁',
    value: securityStore.totalVulnerabilities,
    icon: '🛡️',
    color: securityStore.totalVulnerabilities > 0 ? 'var(--color-danger)' : 'var(--color-success)',
    change: securityStore.totalVulnerabilities > 0 ? '需关注' : '安全',
    positive: securityStore.totalVulnerabilities === 0
  }
]);
</script>

<template>
  <div class="dashboard">
    <!-- Header Section -->
    <header class="dashboard-header">
      <div class="header-content">
        <div class="header-text">
          <h1 class="page-title">
            <span class="greeting">你好</span>
            <span class="title-highlight">，欢迎回来</span>
          </h1>
          <p class="page-subtitle">这里是您的系统监控仪表盘</p>
        </div>
        <div class="clock-card">
          <div class="clock-time">{{ formattedTime }}</div>
          <div class="clock-date">{{ formattedDate }}</div>
        </div>
      </div>
    </header>

    <!-- Stats Grid -->
    <section class="stats-section">
      <div class="stats-grid">
        <div
          v-for="(stat, index) in stats"
          :key="stat.label"
          class="stat-card"
          :class="{ 'danger': stat.color === 'var(--color-danger)' }"
          :style="{ animationDelay: `${index * 0.1}s` }"
        >
          <div class="stat-icon" :style="{ background: stat.color }">
            {{ stat.icon }}
          </div>
          <div class="stat-info">
            <span class="stat-label">{{ stat.label }}</span>
            <span class="stat-value">{{ stat.value }}</span>
            <span v-if="stat.subtext" class="stat-subtext">{{ stat.subtext }}</span>
          </div>
          <div v-if="stat.change" class="stat-change" :class="{ positive: stat.positive, negative: !stat.positive }">
            {{ stat.change }}
          </div>
        </div>
      </div>
    </section>

    <!-- Memory Usage Progress -->
    <section class="memory-section">
      <div class="memory-card">
        <div class="memory-header">
          <h3>内存使用情况</h3>
          <span class="memory-percent" :style="{ color: memoryColor }">
            {{ memoryUsagePercent }}%
          </span>
        </div>
        <div class="progress-bar">
          <div
            class="progress-bar-fill"
            :style="{ width: `${memoryUsagePercent}%`, background: memoryColor }"
          ></div>
        </div>
        <div class="memory-details">
          <span>已用: {{ processStore.formatMemory(processStore.usedMemory) }}</span>
          <span>总计: {{ processStore.formatMemory(processStore.totalMemory) }}</span>
        </div>
      </div>
    </section>

    <!-- Quick Actions -->
    <section class="actions-section">
      <h2 class="section-title">快捷操作</h2>
      <div class="actions-grid">
        <router-link
          v-for="(action, index) in quickActions"
          :key="action.path"
          :to="action.path"
          class="action-card"
          :style="{ animationDelay: `${index * 0.1}s` }"
        >
          <div class="action-icon" :style="{ background: action.color }">
            {{ action.icon }}
          </div>
          <span class="action-name">{{ action.name }}</span>
          <div class="action-arrow">→</div>
        </router-link>
      </div>
    </section>

    <!-- System Status -->
    <section class="status-section">
      <h2 class="section-title">系统状态</h2>
      <div class="status-grid">
        <div class="status-card">
          <div class="status-indicator active"></div>
          <div class="status-info">
            <span class="status-label">API 服务</span>
            <span class="status-value">运行正常</span>
          </div>
        </div>
        <div class="status-card">
          <div class="status-indicator active"></div>
          <div class="status-info">
            <span class="status-label">监控服务</span>
            <span class="status-value">运行正常</span>
          </div>
        </div>
        <div class="status-card">
          <div class="status-indicator" :class="{ warning: securityStore.totalVulnerabilities > 0 }"></div>
          <div class="status-info">
            <span class="status-label">安全状态</span>
            <span class="status-value" :class="{ warning: securityStore.totalVulnerabilities > 0 }">
              {{ securityStore.totalVulnerabilities > 0 ? '存在风险' : '安全' }}
            </span>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<style scoped>
.dashboard {
  padding: 0;
}

/* Header */
.dashboard-header {
  margin-bottom: 2rem;
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 1.5rem;
}

.page-title {
  font-size: 2.5rem;
  font-weight: 800;
  margin: 0;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.greeting {
  background: var(--gradient-primary);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.title-highlight {
  color: var(--color-gray-900);
}

@media (prefers-color-scheme: dark) {
  .title-highlight {
    color: var(--color-gray-100);
  }
}

.page-subtitle {
  color: var(--color-text-soft);
  margin: 0.5rem 0 0;
  font-size: 1.1rem;
}

.clock-card {
  background: var(--color-background-soft);
  padding: 1.5rem 2rem;
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow);
  border: 1px solid var(--color-border);
  text-align: center;
  animation: fadeInScale 0.5s ease-out;
}

.clock-time {
  font-size: 2rem;
  font-weight: 700;
  font-family: var(--font-mono);
  color: var(--color-primary);
  letter-spacing: 0.05em;
}

.clock-date {
  font-size: 0.875rem;
  color: var(--color-text-soft);
  margin-top: 0.25rem;
}

/* Stats Section */
.stats-section {
  margin-bottom: 2rem;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 1.5rem;
}

@media (max-width: 1024px) {
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 640px) {
  .stats-grid {
    grid-template-columns: 1fr;
  }
}

.stat-card {
  background: var(--color-background-soft);
  padding: 1.5rem;
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow);
  border: 1px solid var(--color-border);
  display: flex;
  align-items: center;
  gap: 1rem;
  position: relative;
  overflow: hidden;
  animation: fadeIn 0.5s ease-out backwards;
  transition: all var(--transition-base);
}

.stat-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: var(--gradient-primary);
  transform: scaleX(0);
  transition: transform var(--transition-slow);
}

.stat-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-lg);
}

.stat-card:hover::before {
  transform: scaleX(1);
}

.stat-card.danger::before {
  background: var(--gradient-danger);
}

.stat-icon {
  width: 56px;
  height: 56px;
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.75rem;
  flex-shrink: 0;
}

.stat-info {
  display: flex;
  flex-direction: column;
  flex: 1;
}

.stat-label {
  font-size: 0.875rem;
  color: var(--color-text-soft);
  font-weight: 500;
}

.stat-value {
  font-size: 1.75rem;
  font-weight: 800;
  color: var(--color-gray-900);
  line-height: 1.2;
}

@media (prefers-color-scheme: dark) {
  .stat-value {
    color: var(--color-gray-100);
  }
}

.stat-subtext {
  font-size: 0.75rem;
  color: var(--color-text-muted);
  margin-top: 0.25rem;
}

.stat-change {
  position: absolute;
  top: 1rem;
  right: 1rem;
  font-size: 0.75rem;
  font-weight: 600;
  padding: 0.25rem 0.75rem;
  border-radius: var(--radius-full);
  background: var(--color-gray-100);
}

.stat-change.positive {
  background: rgba(16, 185, 129, 0.1);
  color: var(--color-success);
}

.stat-change.negative {
  background: rgba(239, 68, 68, 0.1);
  color: var(--color-danger);
}

/* Memory Section */
.memory-section {
  margin-bottom: 2rem;
}

.memory-card {
  background: var(--color-background-soft);
  padding: 1.5rem 2rem;
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow);
  border: 1px solid var(--color-border);
}

.memory-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.memory-header h3 {
  margin: 0;
  font-size: 1.1rem;
  color: var(--color-text);
}

.memory-percent {
  font-size: 1.5rem;
  font-weight: 800;
  font-family: var(--font-mono);
}

.progress-bar {
  width: 100%;
  height: 12px;
  background: var(--color-gray-200);
  border-radius: var(--radius-full);
  overflow: hidden;
  margin-bottom: 1rem;
}

.progress-bar-fill {
  height: 100%;
  border-radius: var(--radius-full);
  transition: width 1s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
}

.progress-bar-fill::after {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(90deg, transparent, rgba(255,255,255,0.3), transparent);
  animation: shimmer 2s infinite;
}

.memory-details {
  display: flex;
  justify-content: space-between;
  font-size: 0.875rem;
  color: var(--color-text-soft);
}

/* Actions Section */
.actions-section {
  margin-bottom: 2rem;
}

.section-title {
  font-size: 1.25rem;
  font-weight: 700;
  margin-bottom: 1rem;
  color: var(--color-gray-900);
}

@media (prefers-color-scheme: dark) {
  .section-title {
    color: var(--color-gray-100);
  }
}

.actions-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
}

@media (max-width: 768px) {
  .actions-grid {
    grid-template-columns: 1fr;
  }
}

.action-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.25rem 1.5rem;
  background: var(--color-background-soft);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow);
  border: 1px solid var(--color-border);
  text-decoration: none;
  color: inherit;
  transition: all var(--transition-base);
  animation: fadeIn 0.5s ease-out backwards;
}

.action-card:hover {
  transform: translateY(-4px) scale(1.02);
  box-shadow: var(--shadow-lg);
  border-color: var(--color-primary);
}

.action-icon {
  width: 48px;
  height: 48px;
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  flex-shrink: 0;
}

.action-name {
  flex: 1;
  font-weight: 600;
  font-size: 1.1rem;
}

.action-arrow {
  font-size: 1.25rem;
  color: var(--color-text-muted);
  transition: transform var(--transition-base);
}

.action-card:hover .action-arrow {
  transform: translateX(4px);
  color: var(--color-primary);
}

/* Status Section */
.status-section {
  margin-bottom: 2rem;
}

.status-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
}

@media (max-width: 768px) {
  .status-grid {
    grid-template-columns: 1fr;
  }
}

.status-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.25rem;
  background: var(--color-background-soft);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow);
  border: 1px solid var(--color-border);
  transition: all var(--transition-base);
}

.status-card:hover {
  box-shadow: var(--shadow-md);
}

.status-indicator {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: var(--color-gray-300);
  position: relative;
}

.status-indicator.active {
  background: var(--color-success);
  box-shadow: 0 0 0 4px rgba(16, 185, 129, 0.2);
  animation: pulse 2s ease-in-out infinite;
}

.status-indicator.warning {
  background: var(--color-warning);
  box-shadow: 0 0 0 4px rgba(245, 158, 11, 0.2);
  animation: pulse 2s ease-in-out infinite;
}

.status-info {
  display: flex;
  flex-direction: column;
}

.status-label {
  font-size: 0.875rem;
  color: var(--color-text-soft);
}

.status-value {
  font-weight: 600;
  color: var(--color-gray-900);
}

.status-value.warning {
  color: var(--color-warning);
}

@media (prefers-color-scheme: dark) {
  .status-value {
    color: var(--color-gray-100);
  }
}

/* Animations */
@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes fadeInScale {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

@keyframes shimmer {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}
</style>
