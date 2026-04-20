<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useDockerStore } from '@/stores/dockerStore';
import type { Container } from '@/domain/models';

const dockerStore = useDockerStore();
const searchQuery = ref('');
const statusFilter = ref('all');
const showCreateModal = ref(false);
const newContainerName = ref('');
const newContainerImage = ref('');
const newContainerPorts = ref('');

onMounted(() => {
  dockerStore.fetchContainers();
});

const filteredContainers = computed(() => {
  let containers = dockerStore.containers;
  
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    containers = containers.filter(c => 
      c.name.toLowerCase().includes(query) ||
      c.image.toLowerCase().includes(query) ||
      c.id.toLowerCase().includes(query)
    );
  }
  
  if (statusFilter.value !== 'all') {
    containers = containers.filter(c => c.status === statusFilter.value);
  }
  
  return containers;
});

const getStatusClass = (status: string) => {
  switch (status) {
    case 'running': return 'running';
    case 'stopped': return 'stopped';
    case 'paused': return 'paused';
    default: return 'unknown';
  }
};

const getStatusText = (status: string) => {
  switch (status) {
    case 'running': return '运行中';
    case 'stopped': return '已停止';
    case 'paused': return '已暂停';
    default: return '未知';
  }
};

const formatUptime = (seconds: number) => {
  if (seconds < 60) return `${seconds}秒`;
  if (seconds < 3600) return `${Math.floor(seconds / 60)}分钟`;
  if (seconds < 86400) return `${Math.floor(seconds / 3600)}小时`;
  return `${Math.floor(seconds / 86400)}天`;
};

const startContainer = async (container: Container) => {
  await dockerStore.startContainer(container.id);
};

const stopContainer = async (container: Container) => {
  await dockerStore.stopContainer(container.id);
};

const restartContainer = async (container: Container) => {
  await dockerStore.restartContainer(container.id);
};

const removeContainer = async (container: Container) => {
  if (confirm(`确定要删除容器 "${container.name}" 吗？`)) {
    await dockerStore.removeContainer(container.id);
  }
};

const createContainer = async () => {
  const ports: Record<string, string> = {};
  if (newContainerPorts.value) {
    newContainerPorts.value.split(',').forEach(pair => {
      const [host, container] = pair.split(':');
      if (host && container) {
        ports[host.trim()] = container.trim();
      }
    });
  }
  
  await dockerStore.createContainer({
    name: newContainerName.value,
    image: newContainerImage.value,
    ports
  });
  
  showCreateModal.value = false;
  newContainerName.value = '';
  newContainerImage.value = '';
  newContainerPorts.value = '';
};

const getContainerHealth = (container: Container) => {
  const cpuPercent = container.cpu_percent || 0;
  const memoryPercent = container.memory_percent || 0;
  
  if (cpuPercent > 80 || memoryPercent > 90) return { text: '高负载', class: 'warning' };
  if (cpuPercent > 50 || memoryPercent > 70) return { text: '正常', class: 'normal' };
  return { text: '健康', class: 'healthy' };
};
</script>

<template>
  <div class="docker">
    <!-- Header -->
    <header class="page-header">
      <div class="header-content">
        <div>
          <h1 class="page-title">Docker容器</h1>
          <p class="page-subtitle">管理容器化应用</p>
        </div>
        <div class="header-actions">
          <button class="btn btn-secondary" @click="dockerStore.fetchContainers()">
            <span class="btn-icon">🔄</span>
            刷新
          </button>
          <button class="btn" @click="showCreateModal = true">
            <span class="btn-icon">➕</span>
            创建容器
          </button>
        </div>
      </div>
    </header>

    <!-- Stats Cards -->
    <div class="stats-cards">
      <div class="stat-card">
        <div class="stat-icon running">🐳</div>
        <div class="stat-info">
          <span class="stat-value">{{ dockerStore.runningCount }}</span>
          <span class="stat-label">运行中</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon stopped">⏹️</div>
        <div class="stat-info">
          <span class="stat-value">{{ dockerStore.stoppedCount }}</span>
          <span class="stat-label">已停止</span>
        </div>
      </div>
      <div class="stat-card">
        <div class="stat-icon total">📦</div>
        <div class="stat-info">
          <span class="stat-value">{{ dockerStore.totalCount }}</span>
          <span class="stat-label">总容器</span>
        </div>
      </div>
    </div>

    <!-- Toolbar -->
    <div class="toolbar">
      <div class="search-box">
        <span class="search-icon">🔍</span>
        <input
          v-model="searchQuery"
          type="text"
          placeholder="搜索容器名称、镜像或ID..."
          class="search-input"
        />
        <button v-if="searchQuery" class="clear-btn" @click="searchQuery = ''">✕</button>
      </div>
      <div class="filter-group">
        <select v-model="statusFilter" class="filter-select">
          <option value="all">全部状态</option>
          <option value="running">运行中</option>
          <option value="stopped">已停止</option>
          <option value="paused">已暂停</option>
        </select>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="dockerStore.loading" class="loading-state">
      <div class="loading-spinner"></div>
      <span>正在加载容器数据...</span>
    </div>

    <!-- Error State -->
    <div v-else-if="dockerStore.error" class="error-state">
      <span class="error-icon">⚠️</span>
      <span>{{ dockerStore.error }}</span>
      <button class="btn" @click="dockerStore.fetchContainers()">重试</button>
    </div>

    <!-- Container Grid -->
    <div v-else class="container-grid">
      <div
        v-for="(container, index) in filteredContainers"
        :key="container.id"
        class="container-card"
        :class="{ 'running': container.status === 'running' }"
        :style="{ animationDelay: `${index * 0.05}s` }"
      >
        <!-- Card Header -->
        <div class="card-header">
          <div class="container-status" :class="getStatusClass(container.status)">
            <span class="status-dot"></span>
            <span class="status-text">{{ getStatusText(container.status) }}</span>
          </div>
          <div class="container-health" :class="getContainerHealth(container).class">
            {{ getContainerHealth(container).text }}
          </div>
        </div>

        <!-- Card Body -->
        <div class="card-body">
          <h3 class="container-name">{{ container.name }}</h3>
          <p class="container-image">{{ container.image }}</p>
          
          <!-- Metrics -->
          <div class="container-metrics" v-if="container.status === 'running'">
            <div class="metric">
              <span class="metric-label">CPU</span>
              <div class="metric-bar">
                <div 
                  class="metric-fill"
                  :style="{ width: `${Math.min(container.cpu_percent || 0, 100)}%` }"
                ></div>
              </div>
              <span class="metric-value">{{ (container.cpu_percent || 0).toFixed(1) }}%</span>
            </div>
            <div class="metric">
              <span class="metric-label">内存</span>
              <div class="metric-bar">
                <div 
                  class="metric-fill memory"
                  :style="{ width: `${Math.min(container.memory_percent || 0, 100)}%` }"
                ></div>
              </div>
              <span class="metric-value">{{ (container.memory_percent || 0).toFixed(1) }}%</span>
            </div>
          </div>

          <!-- Info -->
          <div class="container-info">
            <div class="info-item">
              <span class="info-label">ID:</span>
              <span class="info-value">{{ container.id.substring(0, 12) }}</span>
            </div>
            <div class="info-item">
              <span class="info-label">运行时间:</span>
              <span class="info-value">{{ formatUptime(container.uptime_seconds || 0) }}</span>
            </div>
            <div class="info-item" v-if="container.ports && Object.keys(container.ports).length">
              <span class="info-label">端口映射:</span>
              <div class="port-mappings">
                <span
                  v-for="(containerPort, hostPort) in container.ports"
                  :key="hostPort"
                  class="port-badge"
                >
                  {{ hostPort }}:{{ containerPort }}
                </span>
              </div>
            </div>
          </div>
        </div>

        <!-- Card Actions -->
        <div class="card-actions">
          <button
            v-if="container.status !== 'running'"
            class="action-btn start"
            @click="startContainer(container)"
            title="启动"
          >
            ▶️
          </button>
          <button
            v-if="container.status === 'running'"
            class="action-btn stop"
            @click="stopContainer(container)"
            title="停止"
          >
            ⏹️
          </button>
          <button
            class="action-btn restart"
            @click="restartContainer(container)"
            title="重启"
            :disabled="container.status !== 'running'"
          >
            🔄
          </button>
          <button
            class="action-btn remove"
            @click="removeContainer(container)"
            title="删除"
          >
            🗑️
          </button>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-if="!dockerStore.loading && !dockerStore.error && filteredContainers.length === 0" class="empty-state">
      <span class="empty-icon">📦</span>
      <h3>暂无容器</h3>
      <p>点击"创建容器"按钮添加新容器</p>
    </div>

    <!-- Create Modal -->
    <transition name="modal">
      <div v-if="showCreateModal" class="modal-overlay" @click.self="showCreateModal = false">
        <div class="modal-content">
          <div class="modal-header">
            <h3>创建新容器</h3>
            <button class="close-btn" @click="showCreateModal = false">✕</button>
          </div>
          <div class="modal-body">
            <div class="form-group">
              <label>容器名称</label>
              <input
                v-model="newContainerName"
                type="text"
                placeholder="my-container"
                class="form-input"
              />
            </div>
            <div class="form-group">
              <label>镜像</label>
              <input
                v-model="newContainerImage"
                type="text"
                placeholder="nginx:latest"
                class="form-input"
              />
            </div>
            <div class="form-group">
              <label>端口映射 (可选)</label>
              <input
                v-model="newContainerPorts"
                type="text"
                placeholder="8080:80, 443:443"
                class="form-input"
              />
              <span class="form-hint">格式: 主机端口:容器端口, 多个用逗号分隔</span>
            </div>
          </div>
          <div class="modal-footer">
            <button class="btn btn-secondary" @click="showCreateModal = false">取消</button>
            <button 
              class="btn" 
              @click="createContainer"
              :disabled="!newContainerName || !newContainerImage"
            >
              创建
            </button>
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>

<style scoped>
.docker {
  padding: 0;
}

/* Header */
.page-header {
  margin-bottom: 2rem;
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  flex-wrap: wrap;
  gap: 1rem;
}

.page-title {
  font-size: 2rem;
  font-weight: 800;
  margin: 0;
  background: var(--gradient-primary);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.page-subtitle {
  color: var(--color-text-soft);
  margin: 0.5rem 0 0;
}

.header-actions {
  display: flex;
  gap: 0.75rem;
}

/* Stats Cards */
.stats-cards {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
  margin-bottom: 2rem;
}

@media (max-width: 768px) {
  .stats-cards {
    grid-template-columns: repeat(3, 1fr);
  }
}

.stat-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem;
  background: var(--color-background-soft);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow);
  border: 1px solid var(--color-border);
  transition: all var(--transition-base);
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
}

.stat-icon.running {
  background: rgba(16, 185, 129, 0.1);
}

.stat-icon.stopped {
  background: rgba(239, 68, 68, 0.1);
}

.stat-icon.total {
  background: rgba(99, 102, 241, 0.1);
}

.stat-info {
  display: flex;
  flex-direction: column;
}

.stat-value {
  font-size: 1.75rem;
  font-weight: 800;
  color: var(--color-gray-900);
  line-height: 1;
}

.stat-label {
  font-size: 0.875rem;
  color: var(--color-text-soft);
}

/* Toolbar */
.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
  flex-wrap: wrap;
  gap: 1rem;
}

.search-box {
  position: relative;
  flex: 1;
  max-width: 400px;
}

.search-icon {
  position: absolute;
  left: 1rem;
  top: 50%;
  transform: translateY(-50%);
  font-size: 1rem;
  opacity: 0.5;
}

.search-input {
  width: 100%;
  padding: 0.875rem 2.5rem;
  background: var(--color-background-soft);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-xl);
  font-size: 0.9375rem;
  transition: all var(--transition-base);
}

.search-input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 4px rgba(99, 102, 241, 0.1);
}

.clear-btn {
  position: absolute;
  right: 0.75rem;
  top: 50%;
  transform: translateY(-50%);
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: none;
  background: var(--color-gray-200);
  color: var(--color-gray-600);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.75rem;
  transition: all var(--transition-fast);
}

.clear-btn:hover {
  background: var(--color-gray-300);
}

.filter-select {
  padding: 0.625rem 1rem;
  background: var(--color-background-soft);
  border: 2px solid var(--color-border);
  border-radius: var(--radius-lg);
  font-size: 0.9375rem;
  cursor: pointer;
  transition: all var(--transition-base);
}

.filter-select:focus {
  outline: none;
  border-color: var(--color-primary);
}

/* Loading & Error States */
.loading-state,
.error-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  text-align: center;
  gap: 1rem;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--color-gray-200);
  border-top-color: var(--color-primary);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

.error-icon,
.empty-icon {
  font-size: 3rem;
}

.error-state {
  color: var(--color-danger);
}

.empty-state h3 {
  margin: 0;
  color: var(--color-gray-900);
}

.empty-state p {
  margin: 0;
  color: var(--color-text-soft);
}

/* Container Grid */
.container-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 1rem;
}

@media (max-width: 640px) {
  .container-grid {
    grid-template-columns: 1fr;
  }
}

.container-card {
  background: var(--color-background-soft);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow);
  border: 1px solid var(--color-border);
  overflow: hidden;
  animation: slideIn 0.3s ease-out backwards;
  transition: all var(--transition-base);
}

.container-card:hover {
  box-shadow: var(--shadow-lg);
  transform: translateY(-2px);
}

.container-card.running {
  border-color: var(--color-success);
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Card Header */
.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.25rem;
  background: var(--color-gray-50);
  border-bottom: 1px solid var(--color-border);
}

.container-status {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  font-weight: 500;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  animation: pulse 2s ease-in-out infinite;
}

.container-status.running .status-dot {
  background: var(--color-success);
}

.container-status.stopped .status-dot {
  background: var(--color-danger);
}

.container-status.paused .status-dot {
  background: var(--color-warning);
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.container-health {
  padding: 0.25rem 0.75rem;
  border-radius: var(--radius-full);
  font-size: 0.75rem;
  font-weight: 600;
}

.container-health.healthy {
  background: rgba(16, 185, 129, 0.1);
  color: var(--color-success);
}

.container-health.normal {
  background: rgba(99, 102, 241, 0.1);
  color: var(--color-primary);
}

.container-health.warning {
  background: rgba(245, 158, 11, 0.1);
  color: var(--color-warning);
}

/* Card Body */
.card-body {
  padding: 1.25rem;
}

.container-name {
  margin: 0 0 0.25rem;
  font-size: 1.125rem;
  font-weight: 700;
  color: var(--color-gray-900);
}

.container-image {
  margin: 0 0 1rem;
  font-size: 0.875rem;
  color: var(--color-text-soft);
  font-family: var(--font-mono);
}

/* Metrics */
.container-metrics {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  margin-bottom: 1rem;
  padding: 1rem;
  background: var(--color-gray-50);
  border-radius: var(--radius);
}

.metric {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.metric-label {
  width: 40px;
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--color-text-soft);
  text-transform: uppercase;
}

.metric-bar {
  flex: 1;
  height: 6px;
  background: var(--color-gray-200);
  border-radius: var(--radius-full);
  overflow: hidden;
}

.metric-fill {
  height: 100%;
  background: var(--gradient-primary);
  border-radius: var(--radius-full);
  transition: width 0.5s ease-out;
}

.metric-fill.memory {
  background: var(--gradient-secondary);
}

.metric-value {
  width: 50px;
  text-align: right;
  font-size: 0.875rem;
  font-family: var(--font-mono);
  color: var(--color-text-soft);
}

/* Info */
.container-info {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.info-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
}

.info-label {
  color: var(--color-text-soft);
  min-width: 60px;
}

.info-value {
  color: var(--color-text);
  font-family: var(--font-mono);
}

.port-mappings {
  display: flex;
  flex-wrap: wrap;
  gap: 0.375rem;
}

.port-badge {
  padding: 0.25rem 0.5rem;
  background: rgba(99, 102, 241, 0.1);
  color: var(--color-primary);
  border-radius: var(--radius);
  font-size: 0.75rem;
  font-family: var(--font-mono);
}

/* Card Actions */
.card-actions {
  display: flex;
  gap: 0.5rem;
  padding: 1rem 1.25rem;
  background: var(--color-gray-50);
  border-top: 1px solid var(--color-border);
}

.action-btn {
  flex: 1;
  padding: 0.625rem;
  border: none;
  border-radius: var(--radius);
  background: var(--color-background-soft);
  cursor: pointer;
  font-size: 1rem;
  transition: all var(--transition-fast);
}

.action-btn:hover:not(:disabled) {
  transform: scale(1.05);
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.action-btn.start:hover {
  background: rgba(16, 185, 129, 0.1);
}

.action-btn.stop:hover {
  background: rgba(239, 68, 68, 0.1);
}

.action-btn.restart:hover {
  background: rgba(99, 102, 241, 0.1);
}

.action-btn.remove:hover {
  background: rgba(239, 68, 68, 0.1);
}

/* Modal */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: var(--z-modal);
  padding: 1rem;
}

.modal-content {
  background: var(--color-background-soft);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-xl);
  max-width: 480px;
  width: 100%;
  animation: modalIn 0.3s ease-out;
}

@keyframes modalIn {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.25rem;
  border-bottom: 1px solid var(--color-border);
}

.modal-header h3 {
  margin: 0;
  font-size: 1.125rem;
}

.close-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: var(--radius);
  background: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-soft);
  transition: all var(--transition-fast);
}

.close-btn:hover {
  background: var(--color-gray-100);
  color: var(--color-text);
}

.modal-body {
  padding: 1.25rem;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  font-size: 0.875rem;
}

.form-input {
  width: 100%;
  padding: 0.75rem 1rem;
  background: var(--color-background);
  border: 2px solid var(--color-border);
  border-radius: var(--radius);
  font-size: 0.9375rem;
  transition: all var(--transition-base);
}

.form-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.form-hint {
  display: block;
  margin-top: 0.375rem;
  font-size: 0.75rem;
  color: var(--color-text-soft);
}

.modal-footer {
  display: flex;
  gap: 0.75rem;
  padding: 0 1.25rem 1.25rem;
}

.modal-footer .btn {
  flex: 1;
}

/* Modal Transition */
.modal-enter-active,
.modal-leave-active {
  transition: all var(--transition-base);
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .modal-content,
.modal-leave-to .modal-content {
  transform: scale(0.95);
}

/* Animations */
@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

/* Responsive */
@media (max-width: 768px) {
  .toolbar {
    flex-direction: column;
    align-items: stretch;
  }
  
  .search-box {
    max-width: none;
  }
}
</style>
