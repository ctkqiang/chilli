import api from '@/api/axios';
import type { IpAccessLog } from '@/domain/models';

export class IpAccessRepository {
  async getIpAccessLogs(limit: number = 50): Promise<IpAccessLog[]> {
    const { data } = await api.get<IpAccessLog[]>('/ip-access-logs', {
      params: { limit },
    });
    return data;
  }
}

export const ipAccessRepository = new IpAccessRepository();
