import { apiClient } from './client';
import type { Owner, PaginatedResult, PaginationParams } from './types';

export class UsersApi {
	async getUsers(params: PaginationParams = {}): Promise<Owner[]> {
		const searchParams = new URLSearchParams();

		if (params.page) {
			searchParams.append('page', params.page.toString());
		}
		if (params.limit) {
			searchParams.append('limit', params.limit.toString());
		}

		const query = searchParams.toString();
		const endpoint = query ? `/users?${query}` : '/users';

		return apiClient.get<Owner[]>(endpoint);
	}

	async searchUsers(query: string): Promise<Owner[]> {
		try {
			// For now, get all users and filter client-side
			// In a real app, you'd want server-side search
			const users = await this.getUsers({ limit: 100 });

			if (!users || !Array.isArray(users)) {
				console.warn('No user data received from API');
				return [];
			}

			if (!query.trim()) {
				return users;
			}

			const searchTerm = query.toLowerCase();
			return users.filter(
				(user: Owner) =>
					user.firstName?.toLowerCase().includes(searchTerm) ||
					user.lastName?.toLowerCase().includes(searchTerm) ||
					user.email?.toLowerCase().includes(searchTerm)
			);
		} catch (error) {
			console.error('Error searching users:', error);
			return [];
		}
	}

	async getUserById(id: string): Promise<Owner> {
		return apiClient.get<Owner>(`/users/${id}`);
	}
}

export const usersApi = new UsersApi();
