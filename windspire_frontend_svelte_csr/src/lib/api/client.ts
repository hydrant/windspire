import type { ApiResponse } from './types';
import { config } from '../config';

class ApiClient {
	private getAuthHeaders(): HeadersInit {
		const token = localStorage.getItem('windspire_token');
		return {
			'Content-Type': 'application/json',
			...(token && { Authorization: `Bearer ${token}` })
		};
	}

	async request<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
		const url = `${config.API_BASE_URL}${endpoint}`;
		const config_opts: RequestInit = {
			headers: this.getAuthHeaders(),
			...options
		};

		try {
			const response = await fetch(url, config_opts);

			if (!response.ok) {
				// Try to get error details from response body
				let errorMessage = `HTTP error! status: ${response.status}`;
				try {
					const errorText = await response.text();
					if (errorText) {
						try {
							const errorData = JSON.parse(errorText);
							errorMessage = errorData.message || errorData.error || errorMessage;
						} catch {
							// If not JSON, use the text directly
							errorMessage = errorText || errorMessage;
						}
					}
				} catch {
					// If we can't read the response, stick with the status error
				}

				if (response.status === 401) {
					// Token expired or invalid, remove it
					localStorage.removeItem('windspire_token');
					window.dispatchEvent(new CustomEvent('auth-changed'));
					throw new Error('Authentication required');
				}

				console.error(`API Error (${response.status}):`, errorMessage);
				throw new Error(errorMessage);
			}

			// Check if response has content before parsing JSON
			const contentLength = response.headers.get('content-length');
			const contentType = response.headers.get('content-type');

			// If no content or content-length is 0, return undefined for void responses
			if (contentLength === '0' || (!contentType?.includes('application/json') && !response.body)) {
				return undefined as T;
			}

			// Try to get response text first to check if it's empty
			const responseText = await response.text();
			if (!responseText.trim()) {
				return undefined as T;
			}

			// Parse the JSON response
			const data: ApiResponse<T> = JSON.parse(responseText);

			if (!data.success) {
				throw new Error(data.message || 'API request failed');
			}

			return data.data;
		} catch (error) {
			// If it's a JSON parsing error and we expect void, return undefined
			if (error instanceof SyntaxError && error.message.includes('Unexpected end of JSON input')) {
				return undefined as T;
			}
			console.error('API request failed:', error);
			throw error;
		}
	}

	async get<T>(endpoint: string): Promise<T> {
		return this.request<T>(endpoint, { method: 'GET' });
	}

	async post<T>(endpoint: string, data: unknown): Promise<T> {
		return this.request<T>(endpoint, {
			method: 'POST',
			body: JSON.stringify(data)
		});
	}

	async put<T>(endpoint: string, data: unknown): Promise<T> {
		return this.request<T>(endpoint, {
			method: 'PUT',
			body: JSON.stringify(data)
		});
	}

	async delete<T>(endpoint: string): Promise<T> {
		return this.request<T>(endpoint, { method: 'DELETE' });
	}
}

export const apiClient = new ApiClient();
