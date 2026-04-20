import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { securityService } from '@/domain/services';
import type { VulnerabilityScanResult, DockerScanResult } from '@/domain/models';

export const useSecurityStore = defineStore('security', () => {
  const vulnerabilityResult = ref<VulnerabilityScanResult | null>(null);
  const dockerResult = ref<DockerScanResult | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const severityFilter = ref('all');

  const vulnerabilityIssues = computed(() => vulnerabilityResult.value?.issues || []);
  const dockerIssues = computed(() => dockerResult.value?.issues || []);
  const detectedServices = computed(() => vulnerabilityResult.value?.detected_services || []);

  const totalVulnerabilities = computed(() => vulnerabilityResult.value?.vulnerabilities_found || 0);
  const criticalCount = computed(() => dockerResult.value?.critical || 0);
  const highCount = computed(() => dockerResult.value?.high || 0);
  const mediumCount = computed(() => dockerResult.value?.medium || 0);
  const lowCount = computed(() => dockerResult.value?.low || 0);

  const filteredVulnerabilityIssues = computed(() => {
    return securityService.filterIssuesBySeverity(vulnerabilityIssues.value, severityFilter.value);
  });

  const filteredDockerIssues = computed(() => {
    return securityService.filterIssuesBySeverity(dockerIssues.value, severityFilter.value);
  });

  const sortedVulnerabilityIssues = computed(() => {
    return securityService.sortIssuesBySeverity(filteredVulnerabilityIssues.value);
  });

  const sortedDockerIssues = computed(() => {
    return securityService.sortIssuesBySeverity(filteredDockerIssues.value);
  });

  async function scanVulnerabilities() {
    loading.value = true;
    error.value = null;
    try {
      vulnerabilityResult.value = await securityService.scanVulnerabilities();
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to scan vulnerabilities';
    } finally {
      loading.value = false;
    }
  }

  async function scanDockerSecurity() {
    loading.value = true;
    error.value = null;
    try {
      dockerResult.value = await securityService.scanDockerSecurity();
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to scan Docker security';
    } finally {
      loading.value = false;
    }
  }

  function setSeverityFilter(filter: string) {
    severityFilter.value = filter;
  }

  function getSeverityColor(severity: string): string {
    return securityService.getSeverityColor(severity);
  }

  function getIssueTypeLabel(type: string): string {
    return securityService.getIssueTypeLabel(type);
  }

  return {
    vulnerabilityResult,
    dockerResult,
    loading,
    error,
    severityFilter,
    vulnerabilityIssues,
    dockerIssues,
    detectedServices,
    totalVulnerabilities,
    criticalCount,
    highCount,
    mediumCount,
    lowCount,
    filteredVulnerabilityIssues,
    filteredDockerIssues,
    sortedVulnerabilityIssues,
    sortedDockerIssues,
    scanVulnerabilities,
    scanDockerSecurity,
    setSeverityFilter,
    getSeverityColor,
    getIssueTypeLabel
  };
});
