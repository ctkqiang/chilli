<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { useIpAccessStore } from '@/stores/ipAccessStore';

const { t } = useI18n();
const store = useIpAccessStore();
const limitInput = ref(50);

onMounted(() => {
  store.fetchLogs();
});

const handleRefresh = () => {
  store.fetchLogs(limitInput.value);
};

const portIcons: Record<number, string> = {
  3306: '🐬',
  5432: '🐘',
  6379: '🔴',
};

const getPortIcon = (port: number) => portIcons[port] || '🔌';

const copyToClipboard = (text: string) => {
  navigator.clipboard.writeText(text);
};
</script>

<template>
  <div class="ip-access">
    <header class="page-header">
      <div class="header-content">
        <div>
          <h1 class="page-title">{{ t('ipAccess.title') }}</h1>
          <p class="page-subtitle">{{ t('ipAccess.subtitle') }}</p>
        </div>
        <div class="header-actions">
          <button class="btn btn-secondary" @click="handleRefresh">
            <span class="btn-icon">🔄</span>
            {{ t('common.refresh') }}
          </button>
        </div>
      </div>
    </header>

    <div class="stats-grid">
      <div class="stat-card">
        <span class="stat-icon">📋</span>
        <div class="stat-info">
          <span class="stat-value">{{ store.totalCount }}</span>
          <span class="stat-label">{{ t('ipAccess.totalRecords') }}</span>
        </div>
      </div>
      <div class="stat-card">
        <span class="stat-icon">💻</span>
        <div class="stat-info">
          <span class="stat-value">{{ store.uniqueProcesses }}</span>
          <span class="stat-label">{{ t('ipAccess.uniqueProcesses') }}</span>
        </div>
      </div>
      <div class="stat-card">
        <span class="stat-icon">🌐</span>
        <div class="stat-info">
          <span class="stat-value">{{ store.uniqueIps }}</span>
          <span class="stat-label">{{ t('ipAccess.uniqueIps') }}</span>
        </div>
      </div>
      <div
        v-for="(count, port) in store.portDistribution"
        :key="port"
        class="stat-card"
      >
        <span class="stat-icon">{{ getPortIcon(Number(port)) }}</span>
        <div class="stat-info">
          <span class="stat-value">{{ count }}</span>
          <span class="stat-label">{{ store.getPortLabel(Number(port)) }}</span>
        </div>
      </div>
    </div>

    <div class="toolbar">
      <div class="limit-control">
        <label class="limit-label">{{ t('ipAccess.limit') }}:</label>
        <select v-model.number="limitInput" class="limit-select" @change="handleRefresh">
          <option :value="20">20</option>
          <option :value="50">50</option>
          <option :value="100">100</option>
          <option :value="200">200</option>
        </select>
      </div>
      <div class="toolbar-info">
        <span class="info-badge">
          <span class="info-icon">📊</span>
          {{ t('ipAccess.showing', { n: store.logs.length, total: store.totalCount }) }}
        </span>
      </div>
    </div>

    <div v-if="store.loading" class="loading-state">
      <div class="loading-spinner"></div>
      <span>{{ t('ipAccess.loading') }}</span>
    </div>

    <div v-else-if="store.error" class="error-state">
      <span class="error-icon">⚠️</span>
      <span>{{ store.error }}</span>
      <button class="btn" @click="handleRefresh">{{ t('common.retry') }}</button>
    </div>

    <div v-else class="table-container">
      <table class="ip-table">
        <thead>
          <tr>
            <th>#</th>
            <th>{{ t('ipAccess.srcIp') }}</th>
            <th>{{ t('ipAccess.dstPort') }}</th>
            <th>{{ t('ipAccess.processName') }}</th>
            <th>{{ t('ipAccess.pid') }}</th>
            <th>{{ t('ipAccess.timestamp') }}</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="(log, index) in store.logs"
            :key="log.id"
            class="ip-row"
            :style="{ animationDelay: `${index * 0.02}s` }"
          >
            <td class="col-id">
              <span class="id-badge">{{ log.id }}</span>
            </td>
            <td class="col-ip">
              <span class="ip-text" @click="copyToClipboard(log.src_ip)" :title="t('ipAccess.clickToCopy')">
                {{ log.src_ip }}
              </span>
            </td>
            <td class="col-port">
              <span class="port-badge" :class="`port-${log.dst_port}`">
                <span class="port-icon">{{ getPortIcon(log.dst_port) }}</span>
                <span>{{ store.getPortLabel(log.dst_port) }}</span>
                <span class="port-num">:{{ log.dst_port }}</span>
              </span>
            </td>
            <td class="col-process">
              <span class="process-name">{{ log.process_name }}</span>
            </td>
            <td class="col-pid">
              <span class="pid-badge">{{ log.pid }}</span>
            </td>
            <td class="col-time">
              {{ store.formatTimestamp(log.timestamp) }}
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-if="!store.loading && !store.error && store.logs.length === 0" class="empty-state">
      <span class="empty-icon">📭</span>
      <h3>{{ t('ipAccess.noData') }}</h3>
      <p>{{ t('ipAccess.noDataTip') }}</p>
    </div>
  </div>
</template>

<style scoped>
.ip-access {
  padding: 0;
}

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

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.stat-card {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  padding: 1rem 1.25rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  transition: transform 0.2s, box-shadow 0.2s;
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px var(--color-shadow);
}

.stat-icon {
  font-size: 1.5rem;
}

.stat-info {
  display: flex;
  flex-direction: column;
}

.stat-value {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--color-heading);
}

.stat-label {
  font-size: 0.8rem;
  color: var(--color-text-soft);
}

.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
  flex-wrap: wrap;
  gap: 1rem;
}

.limit-control {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.limit-label {
  font-size: 0.875rem;
  color: var(--color-text-soft);
}

.limit-select {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  padding: 0.4rem 0.75rem;
  color: var(--color-text);
  font-size: 0.875rem;
  cursor: pointer;
}

.info-badge {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  padding: 0.4rem 0.9rem;
  font-size: 0.85rem;
  color: var(--color-text-soft);
}

.info-icon {
  font-size: 0.9rem;
}

.loading-state,
.error-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.75rem;
  padding: 4rem 2rem;
  color: var(--color-text-soft);
}

.loading-spinner {
  width: 36px;
  height: 36px;
  border: 3px solid var(--color-border);
  border-top-color: var(--color-primary);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.error-icon {
  font-size: 2rem;
}

.empty-icon {
  font-size: 3rem;
}

.table-container {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  overflow: hidden;
}

.ip-table {
  width: 100%;
  border-collapse: collapse;
}

.ip-table th {
  text-align: left;
  padding: 0.9rem 1rem;
  font-size: 0.8rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--color-text-soft);
  background: var(--color-surface-alt);
  border-bottom: 1px solid var(--color-border);
}

.ip-table td {
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--color-border-light);
}

.ip-row {
  animation: fadeInRow 0.4s ease-out both;
  transition: background 0.2s;
}

.ip-row:hover {
  background: var(--color-surface-alt);
}

@keyframes fadeInRow {
  from {
    opacity: 0;
    transform: translateY(6px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.id-badge {
  font-family: monospace;
  font-size: 0.8rem;
  color: var(--color-text-soft);
  background: var(--color-surface-alt);
  border-radius: 4px;
  padding: 0.15rem 0.4rem;
}

.ip-text {
  font-family: monospace;
  font-size: 0.875rem;
  color: var(--color-primary);
  cursor: pointer;
  transition: opacity 0.2s;
}

.ip-text:hover {
  opacity: 0.7;
}

.port-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.35rem;
  padding: 0.2rem 0.6rem;
  border-radius: 6px;
  font-size: 0.8rem;
  font-weight: 600;
}

.port-3306 {
  background: rgba(66, 133, 244, 0.12);
  color: #4285F4;
}

.port-5432 {
  background: rgba(51, 103, 145, 0.12);
  color: #336791;
}

.port-6379 {
  background: rgba(220, 56, 45, 0.12);
  color: #DC382D;
}

.port-icon {
  font-size: 0.9rem;
}

.port-num {
  font-family: monospace;
  font-weight: 400;
  opacity: 0.7;
}

.process-name {
  font-weight: 500;
  color: var(--color-heading);
}

.pid-badge {
  font-family: monospace;
  font-size: 0.85rem;
  background: var(--color-surface-alt);
  border: 1px solid var(--color-border);
  border-radius: 4px;
  padding: 0.15rem 0.45rem;
}

.col-time {
  font-size: 0.8rem;
  color: var(--color-text-soft);
  white-space: nowrap;
}

.btn {
  padding: 0.5rem 1rem;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  background: var(--color-surface);
  color: var(--color-text);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  display: inline-flex;
  align-items: center;
  gap: 0.4rem;
}

.btn:hover {
  border-color: var(--color-primary);
  color: var(--color-primary);
}

.btn-secondary {
  background: var(--color-surface-alt);
}

.btn-icon {
  font-size: 0.9rem;
}
</style>
