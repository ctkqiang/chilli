import api from '@/api/axios';
import type { VulnerabilityScanResult, DockerScanResult } from '@/domain/models';

export class SecurityRepository {
  async scanVulnerabilities(): Promise<VulnerabilityScanResult> {
    const { data } = await api.get<VulnerabilityScanResult>('/security/scan');
    return data;
  }

  async scanDockerSecurity(): Promise<DockerScanResult> {
    const { data } = await api.get<DockerScanResult>('/security/docker');
    return data;
  }
}

export const securityRepository = new SecurityRepository();
