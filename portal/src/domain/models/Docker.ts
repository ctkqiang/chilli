export interface Container {
  id: string;
  name: string;
  image: string;
  status: 'running' | 'stopped' | 'paused';
  uptime_seconds: number;
  cpu_percent?: number;
  memory_percent?: number;
  ports: Record<string, string>;
}

export interface ContainerCreateRequest {
  name: string;
  image: string;
  ports?: Record<string, string>;
  environment?: Record<string, string>;
}

export interface ContainerStats {
  container_id: string;
  cpu_percent: number;
  memory_percent: number;
  memory_usage: number;
  network_rx: number;
  network_tx: number;
}
