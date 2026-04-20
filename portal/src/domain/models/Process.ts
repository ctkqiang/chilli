export interface Process {
  pid: number;
  name: string;
  cmdline: string[];
  memory_bytes: number;
  start_time: string;
  uptime_seconds: number;
  listening_ports: number[];
}

export interface SystemOverview {
  processes: Process[];
  total_memory: number;
  used_memory: number;
  uptime_seconds: number;
}

export interface SystemStatus {
  status: string;
  version: string;
  timestamp: string;
}
