export interface IpAccessLog {
  id: number;
  src_ip: string;
  dst_port: number;
  process_name: string;
  pid: number;
  timestamp: string;
}
