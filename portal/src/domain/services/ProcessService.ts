import { processRepository } from '@/data/repositories';
import type { SystemOverview, Process } from '@/domain/models';

export class ProcessService {
  async getSystemOverview(): Promise<SystemOverview> {
    return processRepository.getRunningProcesses();
  }

  async getSystemStatus() {
    return processRepository.getSystemStatus();
  }

  async terminateProcess(pid: number): Promise<void> {
    return processRepository.killProcess(pid);
  }

  formatMemory(bytes: number): string {
    const mb = bytes / 1024 / 1024;
    if (mb < 1024) {
      return `${mb.toFixed(2)} MB`;
    }
    const gb = mb / 1024;
    return `${gb.toFixed(2)} GB`;
  }

  formatUptime(seconds: number): string {
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);

    if (days > 0) {
      return `${days}天 ${hours}小时`;
    }
    if (hours > 0) {
      return `${hours}小时 ${minutes}分钟`;
    }
    return `${minutes}分钟`;
  }

  searchProcesses(processes: Process[], query: string): Process[] {
    if (!query.trim()) return processes;
    const lowerQuery = query.toLowerCase();
    return processes.filter(p =>
      p.name.toLowerCase().includes(lowerQuery) ||
      p.pid.toString().includes(lowerQuery) ||
      p.listening_ports.some(port => port.toString().includes(lowerQuery))
    );
  }
}

export const processService = new ProcessService();
