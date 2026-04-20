export interface SecurityIssue {
  service: string;
  port: number;
  severity: string;
  summary: string;
  ghsa_id?: string;
  cve_id?: string;
  vulnerability?: string;
}

export interface DetectedService {
  port: number;
  service_type: string;
  version: string | null;
  vulnerabilities: string[];
}

export interface VulnerabilityScanResult {
  total_processes: number;
  vulnerabilities_found: number;
  detected_services: DetectedService[];
  issues: SecurityIssue[];
}

export interface DockerSecurityIssue {
  container_id: string;
  container_name: string;
  issue_type: string;
  severity: string;
  description: string;
  remediation: string;
}

export interface DockerScanResult {
  total_containers_scanned: number;
  critical: number;
  high: number;
  medium: number;
  low: number;
  issues: DockerSecurityIssue[];
}
