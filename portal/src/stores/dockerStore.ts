import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Container, ContainerCreateRequest } from '@/domain/models';

export const useDockerStore = defineStore('docker', () => {
  const containers = ref<Container[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  const runningCount = computed(() => containers.value.filter(c => c.status === 'running').length);
  const stoppedCount = computed(() => containers.value.filter(c => c.status === 'stopped').length);
  const totalCount = computed(() => containers.value.length);

  const fetchContainers = async () => {
    loading.value = true;
    error.value = null;
    try {
      // 模拟API调用 - 实际项目中应该调用真实的API
      await new Promise(resolve => setTimeout(resolve, 500));
      containers.value = [
        {
          id: 'abc123def456',
          name: 'nginx-proxy',
          image: 'nginx:alpine',
          status: 'running',
          uptime_seconds: 86400,
          cpu_percent: 2.5,
          memory_percent: 15.3,
          ports: { '80': '80', '443': '443' }
        },
        {
          id: 'def789ghi012',
          name: 'redis-cache',
          image: 'redis:7-alpine',
          status: 'running',
          uptime_seconds: 43200,
          cpu_percent: 1.2,
          memory_percent: 8.7,
          ports: { '6379': '6379' }
        },
        {
          id: 'ghi345jkl678',
          name: 'postgres-db',
          image: 'postgres:15',
          status: 'stopped',
          uptime_seconds: 0,
          ports: { '5432': '5432' }
        }
      ];
    } catch (err) {
      error.value = err instanceof Error ? err.message : '获取容器列表失败';
    } finally {
      loading.value = false;
    }
  };

  const startContainer = async (id: string) => {
    try {
      await new Promise(resolve => setTimeout(resolve, 300));
      const container = containers.value.find(c => c.id === id);
      if (container) {
        container.status = 'running';
        container.uptime_seconds = 1;
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : '启动容器失败';
    }
  };

  const stopContainer = async (id: string) => {
    try {
      await new Promise(resolve => setTimeout(resolve, 300));
      const container = containers.value.find(c => c.id === id);
      if (container) {
        container.status = 'stopped';
        container.uptime_seconds = 0;
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : '停止容器失败';
    }
  };

  const restartContainer = async (id: string) => {
    try {
      await new Promise(resolve => setTimeout(resolve, 500));
      const container = containers.value.find(c => c.id === id);
      if (container) {
        container.status = 'running';
        container.uptime_seconds = 1;
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : '重启容器失败';
    }
  };

  const removeContainer = async (id: string) => {
    try {
      await new Promise(resolve => setTimeout(resolve, 300));
      containers.value = containers.value.filter(c => c.id !== id);
    } catch (err) {
      error.value = err instanceof Error ? err.message : '删除容器失败';
    }
  };

  const createContainer = async (request: ContainerCreateRequest) => {
    try {
      await new Promise(resolve => setTimeout(resolve, 500));
      const newContainer: Container = {
        id: Math.random().toString(36).substring(2, 14),
        name: request.name,
        image: request.image,
        status: 'running',
        uptime_seconds: 1,
        cpu_percent: 0,
        memory_percent: 0,
        ports: request.ports || {}
      };
      containers.value.unshift(newContainer);
    } catch (err) {
      error.value = err instanceof Error ? err.message : '创建容器失败';
    }
  };

  return {
    containers,
    loading,
    error,
    runningCount,
    stoppedCount,
    totalCount,
    fetchContainers,
    startContainer,
    stopContainer,
    restartContainer,
    removeContainer,
    createContainer
  };
});
