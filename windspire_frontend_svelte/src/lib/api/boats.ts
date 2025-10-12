import { apiClient } from './client';
import type {
	Boat,
	BoatCreate,
	BoatUpdate,
	PaginatedResult,
	PaginationParams,
	BoatWithOwners
} from './types';

export class BoatsApi {
	async addOwnerToBoat(boatId: string, ownerId: string): Promise<void> {
		console.log(`Adding owner ${ownerId} to boat ${boatId}`);
		return apiClient.post<void>(`/boats/${boatId}/owners/${ownerId}`, {});
	}

	async removeOwnerFromBoat(boatId: string, ownerId: string): Promise<void> {
		console.log(`Removing owner ${ownerId} from boat ${boatId}`);
		return apiClient.delete<void>(`/boats/${boatId}/owners/${ownerId}`);
	}
	async getBoats(params: PaginationParams = {}): Promise<PaginatedResult<Boat>> {
		const searchParams = new URLSearchParams();

		if (params.page) {
			searchParams.append('page', params.page.toString());
		}
		if (params.limit) {
			searchParams.append('limit', params.limit.toString());
		}

		// Include owners in the response
		searchParams.append('include', 'owners');

		const query = searchParams.toString();
		const endpoint = query ? `/boats?${query}` : '/boats?include=owners';

		const result = await apiClient.get<PaginatedResult<BoatWithOwners>>(endpoint);

		// Transform BoatWithOwners[] to Boat[] with owners property
		const transformedData: Boat[] = result.data.map((boatWithOwners) => ({
			...boatWithOwners.boat,
			owners: boatWithOwners.owners
		}));

		return {
			...result,
			data: transformedData
		};
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
