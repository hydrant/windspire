import { apiClient } from './client';
import type { Boat, BoatCreate, BoatUpdate, PaginatedResult, PaginationParams } from './types';

export class BoatsApi {
    async getBoats(params: PaginationParams = {}): Promise<PaginatedResult<Boat>> {
        const searchParams = new URLSearchParams();

        if (params.page) {
            searchParams.append('page', params.page.toString());
        }
        if (params.limit) {
            searchParams.append('limit', params.limit.toString());
        }

        const query = searchParams.toString();
        const endpoint = query ? `/boats?${query}` : '/boats';

        return apiClient.get<PaginatedResult<Boat>>(endpoint);
    }

    async getBoat(id: string): Promise<Boat> {
        return apiClient.get<Boat>(`/boats/${id}`);
    }

    async createBoat(boat: BoatCreate): Promise<Boat> {
        return apiClient.post<Boat>('/boats', boat);
    }

    async updateBoat(id: string, boat: BoatUpdate): Promise<Boat> {
        return apiClient.put<Boat>(`/boats/${id}`, boat);
    }

    async deleteBoat(id: string): Promise<void> {
        return apiClient.delete<void>(`/boats/${id}`);
    }
}

export const boatsApi = new BoatsApi();