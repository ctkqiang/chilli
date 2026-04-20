<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { useProcessStore } from '@/stores/processStore';
import type { Process } from '@/domain/models';

const { t, locale } = useI18n();
const processStore = useProcessStore();
const searchQuery = ref('');
const sortBy = ref<'name' | 'pid' | 'memory'>('memory');
const sortOrder = ref<'asc' | 'desc'>('desc');
const showKillConfirm = ref(false);
const processToKill = ref<Process | null>(null);

onMounted(() => {
  processStore.fetchProcesses();
});

const filteredProcesses = computed(() => {
  let processes = processStore.processes;
  
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    processes = processes.filter(p => 
      p.name.toLowerCase().includes(query) ||
      p.pid.toString().includes(query) ||
      p.listening_ports.some(port => port.toString().includes(query))
    );
  }
  
  return processes.sort((a, b) => {
    let comparison = 0;
    switch (sortBy.value) {
      case 'name':
        comparison = a.name.localeCompare(b.name);
        break;
      case 'pid':
        comparison = a.pid - b.pid;
        break;
      case 'memory':
        comparison = a.memory_bytes - b.memory_bytes;
        break;
    }
    return sortOrder.value === 'asc' ? comparison : -comparison;
  });
});

const toggleSort = (field: 'name' | 'pid' | 'memory') => {
  if (sortBy.value === field) {
    sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc';
  } else {
    sortBy.value = field;
    sortOrder.value = 'desc';
  }
};



const confirmKill = (process: Process) => {
  processToKill.value = process;
  showKillConfirm.value = true;
};

const executeKill = async () => {
  if (processToKill.value) {
    await processStore.killProcess(processToKill.value.pid);
    showKillConfirm.value = false;
    processToKill.value = null;
  }
};

const getProcessStatus = (process: Process) => {
  if (process.listening_ports.length > 0) {
    return { text: t('processes.running'), class: 'active' };
  }
  return { text: t('processes.normal'), class: 'normal' };
};
</script>

<template>
  <div class="processes">
    <!-- Header -->
    <header class="page-header">
      <div class="header-content">
        <div>
          <h1 class="page-title">{{ t('processes.title') }}</h1>
          <p class="page-subtitle">{{ t('processes.subtitle') }}</p>
        </div>
        <div class="header-actions">
          <button class="btn btn-secondary" @click="processStore.fetchProcesses()">
            <span class="btn-icon">🔄</span>
            {{ t('common.refresh') }}
          </button>
        </div>
      </div>
    </header>

    <!-- Toolbar -->
    <div class="toolbar">
      <div class="search-box">
        <span class="search-icon">🔍</span>
        <input
          v-model="searchQuery"
          type="text"
          :placeholder="t('processes.searchPlaceholder')"
          class="search-input"
        />
        <button v-if="searchQuery" class="clear-btn" @click="searchQuery = ''">✕</button>
      </div>
      <div class="toolbar-info">
        <span class="info-badge">
            <span class="info-icon">📊</span>
            {{ t('processes.totalProcesses', { count: filteredProcesses.length }) }}
          </span>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="processStore.loading" class="loading-state">
      <div class="loading-spinner"></div>
      <span>{{ t('processes.loading') }}</span>
    </div>

    <!-- Error State -->
    <div v-else-if="processStore.error" class="error-state">
      <span class="error-icon">⚠️</span>
      <span>{{ processStore.error }}</span>
      <button class="btn" @click="processStore.fetchProcesses()">{{ t('common.retry') }}</button>
    </div>

    <!-- Process Table -->
    <div v-else class="table-container">
      <table class="process-table">
        <thead>
          <tr>
            <th class="col-sortable" @click="toggleSort('pid')">
              {{ t('processes.pid') }}
              <span class="sort-icon" :class="{ active: sortBy === 'pid', asc: sortOrder === 'asc' }">↓</span>
            </th>
            <th class="col-sortable" @click="toggleSort('name')">
              {{ t('processes.name') }}
              <span class="sort-icon" :class="{ active: sortBy === 'name', asc: sortOrder === 'asc' }">↓</span>
            </th>
            <th class="col-sortable" @click="toggleSort('memory')">
              {{ t('processes.memory') }}
              <span class="sort-icon" :class="{ active: sortBy === 'memory', asc: sortOrder === 'asc' }">↓</span>
            </th>
            <th>{{ t('processes.startTime') }}</th>
            <th>{{ t('processes.ports') }}</th>
            <th>{{ t('processes.status') }}</th>
            <th>{{ t('processes.actions') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="(process, index) in filteredProcesses"
            :key="process.pid"
            class="process-row"
            :style="{ animationDelay: `${index * 0.03}s` }"
          >
            <td class="col-pid">
              <span class="pid-badge">{{ process.pid }}</span>
            </td>
            <td class="col-name">
              <div class="process-name">
                <span class="name-text">{{ process.name }}</span>
              </div>
            </td>
            <td class="col-memory">
              <div class="memory-bar">
                <div 
                  class="memory-fill"
                  :style="{ 
                    width: `${Math.min((process.memory_bytes / processStore.totalMemory) * 100 * 10, 100)}%`,
                    background: process.memory_bytes > 100 * 1024 * 1024 ? 'var(--gradient-danger)' : 'var(--gradient-primary)'
                  }"
                ></div>
              </div>
              <span class="memory-text">{{ processStore.formatMemory(process.memory_bytes) }}</span>
            </td>
            <td class="col-time">
              {{ new Date(process.start_time).toLocaleString(locale === 'zh' ? 'zh-CN' : 'en-US') }}
            </td>
            <td class="col-ports">
              <div v-if="process.listening_ports.length" class="port-list">
                <span
                  v-for="port in process.listening_ports.slice(0, 3)"
                  :key="port"
                  class="port-badge"
                >
                  :{{ port }}
                </span>
                <span v-if="process.listening_ports.length > 3" class="port-more">
                  +{{ process.listening_ports.length - 3 }}
                </span>
              </div>
              <span v-else class="no-ports">-</span>
            </td>
            <td class="col-status">
              <span class="status-badge" :class="getProcessStatus(process).class">
                {{ getProcessStatus(process).text }}
              </span>
            </td>
            <td class="col-actions">
              <button class="action-btn kill" @click="confirmKill(process)" :title="t('processes.kill')">
                <span>✕</span>
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Empty State -->
    <div v-if="!processStore.loading && !processStore.error && filteredProcesses.length === 0" class="empty-state">
      <span class="empty-icon">🔍</span>
      <h3>{{ t('processes.noProcesses') }}</h3>
      <p>{{ t('processes.noProcessesTip') }}</p>
    </div>

    <!-- Kill Confirmation Modal -->
    <transition name="modal">
      <div v-if="showKillConfirm" class="modal-overlay" @click.self="showKillConfirm = false">
        <div class="modal-content">
          <div class="modal-header">
            <span class="modal-icon">⚠️</span>
            <h3>{{ t('processes.killConfirmTitle') }}</h3>
          </div>
          <div class="modal-body">
            <p>{{ t('processes.killConfirmMessage') }}</p>
            <div class="process-info" v-if="processToKill">
              <span class="info-label">{{ t('processes.processName') }}:</span>
              <span class="info-value">{{ processToKill.name }}</span>
            </div>
            <div class="process-info" v-if="processToKill">
              <span class="info-label">{{ t('processes.pid') }}:</span>
              <span class="info-value">{{ processToKill.pid }}</span>
            </div>
            <p class="warning-text">{{ t('processes.killWarning') }}</p>
          </div>
          <div class="modal-footer">
            <button class="btn btn-secondary" @click="showKillConfirm = false">{{ t('common.cancel') }}</button>
            <button class="btn btn-danger" @click="executeKill">{{ t('processes.confirmKill') }}</button>
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>

<style scoped>
.processes {
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

.toolbar-info {
  display: flex;
  gap: 0.75rem;
}

.info-badge {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: var(--color-background-soft);
  border-radius: var(--radius-full);
  font-size: 0.875rem;
  color: var(--color-text-soft);
  border: 1px solid var(--color-border);
}

.info-icon {
  font-size: 1rem;
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

/* Table */
.table-container {
  background: var(--color-background-soft);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow);
  border: 1px solid var(--color-border);
  overflow: hidden;
}

.process-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.9375rem;
}

.process-table th {
  padding: 1rem 1.25rem;
  text-align: left;
  font-weight: 600;
  color: var(--color-text-soft);
  background: var(--color-gray-50);
  border-bottom: 1px solid var(--color-border);
  white-space: nowrap;
}

.process-table td {
  padding: 1rem 1.25rem;
  border-bottom: 1px solid var(--color-border);
  vertical-align: middle;
}

.process-table tbody tr:last-child td {
  border-bottom: none;
}

.col-sortable {
  cursor: pointer;
  user-select: none;
  transition: color var(--transition-fast);
}

.col-sortable:hover {
  color: var(--color-primary);
}

.sort-icon {
  display: inline-block;
  margin-left: 0.25rem;
  opacity: 0.3;
  transition: all var(--transition-fast);
}

.sort-icon.active {
  opacity: 1;
  color: var(--color-primary);
}

.sort-icon.asc {
  transform: rotate(180deg);
}

/* Process Row */
.process-row {
  animation: slideIn 0.3s ease-out backwards;
  transition: background var(--transition-fast);
}

.process-row:hover {
  background: var(--color-gray-50);
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateX(-10px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

/* PID */
.col-pid {
  width: 80px;
}

.pid-badge {
  display: inline-flex;
  padding: 0.375rem 0.75rem;
  background: var(--color-gray-100);
  border-radius: var(--radius);
  font-family: var(--font-mono);
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--color-gray-700);
}

/* Name */
.col-name {
  min-width: 200px;
}

.process-name {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.name-text {
  font-weight: 600;
  color: var(--color-gray-900);
}

/* Memory */
.col-memory {
  width: 180px;
}

.memory-bar {
  width: 100%;
  height: 6px;
  background: var(--color-gray-200);
  border-radius: var(--radius-full);
  overflow: hidden;
  margin-bottom: 0.375rem;
}

.memory-fill {
  height: 100%;
  border-radius: var(--radius-full);
  transition: width 0.5s ease-out;
}

.memory-text {
  font-size: 0.875rem;
  color: var(--color-text-soft);
  font-family: var(--font-mono);
}

/* Ports */
.col-ports {
  width: 150px;
}

.port-list {
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
  font-weight: 600;
}

.port-more {
  padding: 0.25rem 0.5rem;
  background: var(--color-gray-100);
  color: var(--color-text-soft);
  border-radius: var(--radius);
  font-size: 0.75rem;
}

.no-ports {
  color: var(--color-text-muted);
}

/* Status */
.col-status {
  width: 100px;
}

.status-badge {
  display: inline-flex;
  padding: 0.375rem 0.875rem;
  border-radius: var(--radius-full);
  font-size: 0.75rem;
  font-weight: 600;
}

.status-badge.active {
  background: rgba(16, 185, 129, 0.1);
  color: var(--color-success);
}

.status-badge.normal {
  background: var(--color-gray-100);
  color: var(--color-text-soft);
}

/* Actions */
.col-actions {
  width: 80px;
  text-align: center;
}

.action-btn {
  width: 32px;
  height: 32px;
  border-radius: var(--radius);
  border: none;
  background: transparent;
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-fast);
}

.action-btn.kill {
  color: var(--color-danger);
}

.action-btn.kill:hover {
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
  max-width: 400px;
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
  padding: 1.5rem 1.5rem 0;
  text-align: center;
}

.modal-icon {
  font-size: 3rem;
  display: block;
  margin-bottom: 0.5rem;
}

.modal-header h3 {
  margin: 0;
  font-size: 1.25rem;
}

.modal-body {
  padding: 1.5rem;
}

.modal-body p {
  margin: 0 0 1rem;
  color: var(--color-text-soft);
  text-align: center;
}

.process-info {
  display: flex;
  justify-content: space-between;
  padding: 0.75rem;
  background: var(--color-gray-50);
  border-radius: var(--radius);
  margin-bottom: 0.5rem;
}

.info-label {
  color: var(--color-text-soft);
  font-size: 0.875rem;
}

.info-value {
  font-weight: 600;
  color: var(--color-gray-900);
  font-family: var(--font-mono);
}

.warning-text {
  color: var(--color-danger) !important;
  font-size: 0.875rem;
  margin-top: 1rem;
}

.modal-footer {
  padding: 0 1.5rem 1.5rem;
  display: flex;
  gap: 0.75rem;
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

/* Responsive */
@media (max-width: 1024px) {
  .process-table {
    font-size: 0.875rem;
  }
  
  .process-table th,
  .process-table td {
    padding: 0.75rem;
  }
  
  .col-time,
  .col-ports {
    display: none;
  }
}

@media (max-width: 640px) {
  .toolbar {
    flex-direction: column;
    align-items: stretch;
  }
  
  .search-box {
    max-width: none;
  }
  
  .col-memory {
    display: none;
  }
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
