import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { ipAccessRepository } from '@/data/repositories';
import type { IpAccessLog } from '@/domain/models';

export const useIpAccessStore = defineStore('ipAccess', () => {
  const logs = ref<IpAccessLog[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const logLimit = ref(50);

  const totalCount = computed(() => logs.value.length);

  const portDistribution = computed(() => {
    const dist: Record<number, number> = {};
    for (const log of logs.value) {
      dist[log.dst_port] = (dist[log.dst_port] || 0) + 1;
    }
    return dist;
  });

  const uniqueProcesses = computed(() => {
    const set = new Set(logs.value.map((l) => l.process_name));
    return set.size;
  });

  const uniqueIps = computed(() => {
    const set = new Set(logs.value.map((l) => l.src_ip));
    return set.size;
  });

  async function fetchLogs(limit?: number) {
    loading.value = true;
    error.value = null;
    try {
      logs.value = await ipAccessRepository.getIpAccessLogs(limit ?? logLimit.value);
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch IP access logs';
    } finally {
      loading.value = false;
    }
  }

  function setLimit(limit: number) {
    logLimit.value = limit;
  }

  function getPortLabel(port: number): string {
    const labels: Record<number, string> = {
      3306: 'MySQL',
      5432: 'PostgreSQL',
      6379: 'Redis',
    };
    return labels[port] || port.toString();
  }

  function formatTimestamp(iso: string): string {
    const date = new Date(iso);
    return date.toLocaleString();
  }

  return {
    logs,
    loading,
    error,
    logLimit,
    totalCount,
    portDistribution,
    uniqueProcesses,
    uniqueIps,
    fetchLogs,
    setLimit,
    getPortLabel,
    formatTimestamp,
  };
});
