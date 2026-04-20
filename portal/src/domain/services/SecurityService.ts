import { securityRepository } from '@/data/repositories';
import type { VulnerabilityScanResult, DockerScanResult } from '@/domain/models';

export class SecurityService {
  async scanVulnerabilities(): Promise<VulnerabilityScanResult> {
    return securityRepository.scanVulnerabilities();
  }

  async scanDockerSecurity(): Promise<DockerScanResult> {
    return securityRepository.scanDockerSecurity();
  }

  getSeverityColor(severity: string): string {
    switch (severity.toLowerCase()) {
      case 'critical':
        return '#dc3545';
      case 'high':
        return '#fd7e14';
      case 'medium':
        return '#ffc107';
      case 'low':
        return '#28a745';
      default:
        return '#6c757d';
    }
  }

  getSeverityOrder(severity: string): number {
    switch (severity.toLowerCase()) {
      case 'critical':
        return 4;
      case 'high':
        return 3;
      case 'medium':
        return 2;
      case 'low':
        return 1;
      default:
        return 0;
    }
  }

  sortIssuesBySeverity<T extends { severity: string }>(issues: T[]): T[] {
    return [...issues].sort((a, b) => 
      this.getSeverityOrder(b.severity) - this.getSeverityOrder(a.severity)
    );
  }

  filterIssuesBySeverity<T extends { severity: string }>(
    issues: T[], 
    severity: string
  ): T[] {
    if (severity === 'all') return issues;
    return issues.filter(issue => 
      issue.severity.toLowerCase() === severity.toLowerCase()
    );
  }

  getIssueTypeLabel(type: string): string {
    const labels: Record<string, string> = {
      'PrivilegedMode': '特权模式',
      'SensitiveMount': '敏感挂载',
      'NoResourceLimits': '无资源限制',
      'RootUser': 'Root用户',
      'InsecureCapability': '不安全权限',
      'ExposedDockerSocket': '暴露Docker Socket',
      'HostNetwork': 'Host网络',
      'HostPid': 'Host PID',
      'WritableRootfs': '可写Rootfs'
    };
    return labels[type] || type;
  }
}

export const securityService = new SecurityService();
