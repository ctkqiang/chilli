import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { processService } from '@/domain/services';
import type { SystemOverview } from '@/domain/models';

export const useProcessStore = defineStore('process', () => {
  const systemOverview = ref<SystemOverview | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const searchQuery = ref('');

  const processes = computed(() => systemOverview.value?.processes || []);
  const totalMemory = computed(() => systemOverview.value?.total_memory || 0);
  const usedMemory = computed(() => systemOverview.value?.used_memory || 0);
  const uptime = computed(() => systemOverview.value?.uptime_seconds || 0);

  const filteredProcesses = computed(() => {
    return processService.searchProcesses(processes.value, searchQuery.value);
  });

  const memoryUsagePercent = computed(() => {
    if (totalMemory.value === 0) return 0;
    return Math.round((usedMemory.value / totalMemory.value) * 100);
  });

  async function fetchProcesses() {
    loading.value = true;
    error.value = null;
    try {
      systemOverview.value = await processService.getSystemOverview();
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to fetch processes';
    } finally {
      loading.value = false;
    }
  }

  async function killProcess(pid: number) {
    try {
      await processService.terminateProcess(pid);
      await fetchProcesses();
      return true;
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to kill process';
      return false;
    }
  }

  function setSearchQuery(query: string) {
    searchQuery.value = query;
  }

  function formatMemory(bytes: number): string {
    return processService.formatMemory(bytes);
  }

  function formatUptime(seconds: number): string {
    return processService.formatUptime(seconds);
  }

  return {
    systemOverview,
    loading,
    error,
    searchQuery,
    processes,
    totalMemory,
    usedMemory,
    uptime,
    filteredProcesses,
    memoryUsagePercent,
    fetchProcesses,
    killProcess,
    setSearchQuery,
    formatMemory,
    formatUptime
  };
});
