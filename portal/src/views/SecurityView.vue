<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useSecurityStore } from '@/stores/securityStore';

const securityStore = useSecurityStore();
const severityFilter = ref('all');
const expandedServices = ref<Set<number>>(new Set());

onMounted(() => {
  securityStore.scanVulnerabilities();
});

const onFilterChange = () => {
  securityStore.setSeverityFilter(severityFilter.value);
};

const toggleService = (port: number) => {
  if (expandedServices.value.has(port)) {
    expandedServices.value.delete(port);
  } else {
    expandedServices.value.add(port);
  }
};

const getSeverityIcon = (severity: string) => {
  switch (severity.toLowerCase()) {
    case 'critical': return '🔴';
    case 'high': return '🟠';
    case 'medium': return '🟡';
    case 'low': return '🟢';
    default: return '⚪';
  }
};

const getSeverityColor = (severity: string) => {
  switch (severity.toLowerCase()) {
    case 'critical': return '#ef4444';
    case 'high': return '#f97316';
    case 'medium': return '#eab308';
    case 'low': return '#22c55e';
    default: return '#6b7280';
  }
};

const getSeverityBg = (severity: string) => {
  switch (severity.toLowerCase()) {
    case 'critical': return 'rgba(239, 68, 68, 0.1)';
    case 'high': return 'rgba(249, 115, 22, 0.1)';
    case 'medium': return 'rgba(234, 179, 8, 0.1)';
    case 'low': return 'rgba(34, 197, 94, 0.1)';
    default: return 'rgba(107, 114, 128, 0.1)';
  }
};
</script>

<template>
  <div class="security">
    <!-- Header -->
    <header class="page-header">
      <div class="header-content">
        <div>
          <h1 class="page-title">安全扫描</h1>
          <p class="page-subtitle">检测系统安全漏洞和风险</p>
        </div>
        <button
          class="btn"
          :class="{ 'scanning': securityStore.loading }"
          :disabled="securityStore.loading"
          @click="securityStore.scanVulnerabilities()"
        >
          <span class="btn-icon" :class="{ 'spin': securityStore.loading }">🔄</span>
          {{ securityStore.loading ? '扫描中...' : '重新扫描' }}
        </button>
      </div>
    </header>

    <!-- Loading State -->
    <div v-if="securityStore.loading && !securityStore.vulnerabilityResult" class="loading-state">
      <div class="scan-animation">
        <div class="scan-ring"></div>
        <div class="scan-ring"></div>
        <div class="scan-ring"></div>
        <span class="scan-icon">🛡️</span>
      </div>
      <h3>正在扫描系统安全...</h3>
      <p>请稍候，正在分析进程和端口</p>
    </div>

    <!-- Error State -->
    <div v-else-if="securityStore.error" class="error-state">
      <span class="error-icon">⚠️</span>
      <span>{{ securityStore.error }}</span>
      <button class="btn" @click="securityStore.scanVulnerabilities()">重试</button>
    </div>

    <!-- Results -->
    <div v-else-if="securityStore.vulnerabilityResult" class="results">
      <!-- Severity Stats -->
      <div class="severity-cards">
        <div class="severity-card critical" :class="{ 'active': securityStore.criticalCount > 0 }">
          <div class="severity-icon">🔴</div>
          <div class="severity-info">
            <span class="severity-count">{{ securityStore.criticalCount }}</span>
            <span class="severity-label">严重</span>
          </div>
        </div>
        <div class="severity-card high" :class="{ 'active': securityStore.highCount > 0 }">
          <div class="severity-icon">🟠</div>
          <div class="severity-info">
            <span class="severity-count">{{ securityStore.highCount }}</span>
            <span class="severity-label">高危</span>
          </div>
        </div>
        <div class="severity-card medium" :class="{ 'active': securityStore.mediumCount > 0 }">
          <div class="severity-icon">🟡</div>
          <div class="severity-info">
            <span class="severity-count">{{ securityStore.mediumCount }}</span>
            <span class="severity-label">中危</span>
          </div>
        </div>
        <div class="severity-card low" :class="{ 'active': securityStore.lowCount > 0 }">
          <div class="severity-icon">🟢</div>
          <div class="severity-info">
            <span class="severity-count">{{ securityStore.lowCount }}</span>
            <span class="severity-label">低危</span>
          </div>
        </div>
      </div>

      <!-- Filter Bar -->
      <div class="filter-bar">
        <div class="filter-group">
          <label>严重程度筛选:</label>
          <select v-model="severityFilter" @change="onFilterChange" class="filter-select">
            <option value="all">全部</option>
            <option value="critical">严重</option>
            <option value="high">高危</option>
            <option value="medium">中危</option>
            <option value="low">低危</option>
          </select>
        </div>
        <div class="scan-info">
          <span class="info-item">
            <span class="info-icon">📊</span>
            扫描进程: {{ securityStore.vulnerabilityResult.total_processes }}
          </span>
          <span class="info-item">
            <span class="info-icon">🔍</span>
            发现问题: {{ securityStore.totalVulnerabilities }}
          </span>
        </div>
      </div>

      <!-- Services List -->
      <div v-if="securityStore.detectedServices.length" class="services-section">
        <h2 class="section-title">检测到的服务</h2>
        <div class="services-list">
          <div
            v-for="service in securityStore.detectedServices"
            :key="service.port"
            class="service-item"
            :class="{ 'has-vulns': service.vulnerabilities.length > 0, 'expanded': expandedServices.has(service.port) }"
            @click="toggleService(service.port)"
          >
            <div class="service-header">
              <div class="service-info">
                <span class="service-type">{{ service.service_type }}</span>
                <span class="service-port">:{{ service.port }}</span>
              </div>
              <div class="service-badges">
                <span v-if="service.vulnerabilities.length" class="vuln-badge">
                  {{ service.vulnerabilities.length }} 个漏洞
                </span>
                <span class="expand-icon">{{ expandedServices.has(service.port) ? '▼' : '▶' }}</span>
              </div>
            </div>
            <transition name="expand">
              <div v-if="expandedServices.has(service.port) && service.vulnerabilities.length" class="service-details">
                <div class="vuln-list">
                  <div
                    v-for="vuln in service.vulnerabilities"
                    :key="vuln"
                    class="vuln-item"
                  >
                    <span class="vuln-icon">⚠️</span>
                    <span class="vuln-text">{{ vuln }}</span>
                  </div>
                </div>
              </div>
            </transition>
          </div>
        </div>
      </div>

      <!-- Vulnerability Issues -->
      <div v-if="securityStore.vulnerabilityIssues.length" class="issues-section">
        <h2 class="section-title">漏洞详情</h2>
        <div class="issues-list">
          <div
            v-for="(issue, index) in securityStore.sortedVulnerabilityIssues"
            :key="index"
            class="issue-card"
            :style="{ animationDelay: `${index * 0.05}s` }"
          >
            <div class="issue-header">
              <div class="issue-service">
                <span class="service-name">{{ issue.service }}</span>
                <span class="service-port">:{{ issue.port }}</span>
              </div>
              <div
                class="severity-badge"
                :style="{ background: getSeverityBg(issue.severity), color: getSeverityColor(issue.severity) }"
              >
                {{ getSeverityIcon(issue.severity) }}
                {{ issue.severity }}
              </div>
            </div>
            <div class="issue-body">
              <p class="issue-summary">{{ issue.summary || issue.vulnerability }}</p>
              <div v-if="issue.ghsa_id || issue.cve_id" class="issue-ids">
                <span v-if="issue.ghsa_id" class="id-badge">{{ issue.ghsa_id }}</span>
                <span v-if="issue.cve_id" class="id-badge cve">{{ issue.cve_id }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- No Issues State -->
      <div v-else class="no-issues">
        <div class="success-animation">
          <span class="success-icon">✅</span>
          <div class="success-ring"></div>
        </div>
        <h3>未发现安全漏洞</h3>
        <p>您的系统目前看起来是安全的</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.security {
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

.btn.scanning {
  opacity: 0.8;
}

.btn-icon.spin {
  animation: spin 1s linear infinite;
}

/* Loading State */
.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  text-align: center;
}

.scan-animation {
  position: relative;
  width: 120px;
  height: 120px;
  margin-bottom: 1.5rem;
}

.scan-ring {
  position: absolute;
  inset: 0;
  border: 3px solid var(--color-primary);
  border-radius: 50%;
  opacity: 0;
  animation: scanPulse 2s ease-out infinite;
}

.scan-ring:nth-child(2) {
  animation-delay: 0.5s;
}

.scan-ring:nth-child(3) {
  animation-delay: 1s;
}

@keyframes scanPulse {
  0% {
    transform: scale(0.5);
    opacity: 1;
  }
  100% {
    transform: scale(1.5);
    opacity: 0;
  }
}

.scan-icon {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 3rem;
}

.loading-state h3 {
  margin: 0 0 0.5rem;
  color: var(--color-gray-900);
}

.loading-state p {
  margin: 0;
  color: var(--color-text-soft);
}

/* Error State */
.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  text-align: center;
  gap: 1rem;
  color: var(--color-danger);
}

.error-icon {
  font-size: 3rem;
}

/* Severity Cards */
.severity-cards {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 1rem;
  margin-bottom: 2rem;
}

@media (max-width: 768px) {
  .severity-cards {
    grid-template-columns: repeat(2, 1fr);
  }
}

.severity-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem;
  background: var(--color-background-soft);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow);
  border: 1px solid var(--color-border);
  opacity: 0.5;
  transition: all var(--transition-base);
}

.severity-card.active {
  opacity: 1;
}

.severity-card.critical.active {
  border-color: var(--color-danger);
  box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
}

.severity-card.high.active {
  border-color: #f97316;
  box-shadow: 0 0 0 3px rgba(249, 115, 22, 0.1);
}

.severity-card.medium.active {
  border-color: #eab308;
  box-shadow: 0 0 0 3px rgba(234, 179, 8, 0.1);
}

.severity-card.low.active {
  border-color: var(--color-success);
  box-shadow: 0 0 0 3px rgba(34, 197, 94, 0.1);
}

.severity-icon {
  font-size: 2rem;
}

.severity-info {
  display: flex;
  flex-direction: column;
}

.severity-count {
  font-size: 1.75rem;
  font-weight: 800;
  color: var(--color-gray-900);
  line-height: 1;
}

.severity-label {
  font-size: 0.875rem;
  color: var(--color-text-soft);
}

/* Filter Bar */
.filter-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  flex-wrap: wrap;
  gap: 1rem;
}

.filter-group {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.filter-group label {
  font-weight: 500;
  color: var(--color-text);
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

.scan-info {
  display: flex;
  gap: 1.5rem;
}

.info-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: var(--color-text-soft);
  font-size: 0.9375rem;
}

/* Services Section */
.services-section {
  margin-bottom: 2rem;
}

.section-title {
  font-size: 1.25rem;
  font-weight: 700;
  margin-bottom: 1rem;
  color: var(--color-gray-900);
}

.services-list {
  display: grid;
  gap: 0.75rem;
}

.service-item {
  background: var(--color-background-soft);
  border-radius: var(--radius-lg);
  border: 1px solid var(--color-border);
  overflow: hidden;
  cursor: pointer;
  transition: all var(--transition-base);
}

.service-item:hover {
  border-color: var(--color-primary);
  box-shadow: var(--shadow-md);
}

.service-item.has-vulns {
  border-left: 4px solid var(--color-danger);
}

.service-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.25rem;
}

.service-info {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.service-type {
  font-weight: 600;
  color: var(--color-gray-900);
}

.service-port {
  font-family: var(--font-mono);
  color: var(--color-text-soft);
  font-size: 0.875rem;
}

.service-badges {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.vuln-badge {
  padding: 0.375rem 0.75rem;
  background: rgba(239, 68, 68, 0.1);
  color: var(--color-danger);
  border-radius: var(--radius-full);
  font-size: 0.75rem;
  font-weight: 600;
}

.expand-icon {
  color: var(--color-text-muted);
  font-size: 0.75rem;
  transition: transform var(--transition-fast);
}

.service-item.expanded .expand-icon {
  transform: rotate(90deg);
}

.service-details {
  padding: 0 1.25rem 1rem;
  border-top: 1px solid var(--color-border);
}

.vuln-list {
  padding-top: 1rem;
}

.vuln-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0;
  color: var(--color-text);
}

.vuln-icon {
  color: var(--color-warning);
}

/* Expand Transition */
.expand-enter-active,
.expand-leave-active {
  transition: all var(--transition-base);
  max-height: 200px;
  overflow: hidden;
}

.expand-enter-from,
.expand-leave-to {
  max-height: 0;
  opacity: 0;
}

/* Issues Section */
.issues-section {
  margin-bottom: 2rem;
}

.issues-list {
  display: grid;
  gap: 0.75rem;
}

.issue-card {
  background: var(--color-background-soft);
  border-radius: var(--radius-lg);
  border: 1px solid var(--color-border);
  padding: 1.25rem;
  animation: slideIn 0.3s ease-out backwards;
  transition: all var(--transition-base);
}

.issue-card:hover {
  box-shadow: var(--shadow-md);
  transform: translateX(4px);
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

.issue-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.75rem;
}

.issue-service {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.service-name {
  font-weight: 600;
  color: var(--color-gray-900);
}

.service-port {
  font-family: var(--font-mono);
  color: var(--color-text-soft);
  font-size: 0.875rem;
}

.severity-badge {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  padding: 0.375rem 0.75rem;
  border-radius: var(--radius-full);
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
}

.issue-body {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.issue-summary {
  margin: 0;
  color: var(--color-text);
  line-height: 1.5;
}

.issue-ids {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.id-badge {
  padding: 0.25rem 0.5rem;
  background: var(--color-gray-100);
  border-radius: var(--radius);
  font-size: 0.75rem;
  font-family: var(--font-mono);
  color: var(--color-text-soft);
}

.id-badge.cve {
  background: rgba(239, 68, 68, 0.1);
  color: var(--color-danger);
}

/* No Issues State */
.no-issues {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  text-align: center;
}

.success-animation {
  position: relative;
  width: 100px;
  height: 100px;
  margin-bottom: 1.5rem;
}

.success-icon {
  font-size: 4rem;
  position: relative;
  z-index: 1;
}

.success-ring {
  position: absolute;
  inset: 0;
  border: 3px solid var(--color-success);
  border-radius: 50%;
  animation: successPulse 2s ease-out infinite;
}

@keyframes successPulse {
  0%, 100% {
    transform: scale(1);
    opacity: 0.5;
  }
  50% {
    transform: scale(1.1);
    opacity: 0;
  }
}

.no-issues h3 {
  margin: 0 0 0.5rem;
  color: var(--color-success);
}

.no-issues p {
  margin: 0;
  color: var(--color-text-soft);
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
  .filter-bar {
    flex-direction: column;
    align-items: stretch;
  }
  
  .scan-info {
    justify-content: space-between;
  }
  
  .issue-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.75rem;
  }
}
</style>
