import api from '@/api/axios';
import type { SystemOverview, SystemStatus } from '@/domain/models';

export class ProcessRepository {
  async getSystemStatus(): Promise<SystemStatus> {
    const { data } = await api.get<SystemStatus>('/health');
    return data;
  }

  async getRunningProcesses(): Promise<SystemOverview> {
    const { data } = await api.get<SystemOverview>('/running');
    return data;
  }

  async killProcess(pid: number): Promise<void> {
    await api.post(`/kill/${pid}`);
  }
}

export const processRepository = new ProcessRepository();
