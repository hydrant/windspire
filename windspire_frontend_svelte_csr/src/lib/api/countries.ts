import { apiClient } from './client';
import type { Country } from './types';

export class CountriesApi {
	async getCountries(): Promise<Country[]> {
		return apiClient.get<Country[]>('/countries');
	}

	async getCountry(id: string): Promise<Country> {
		return apiClient.get<Country>(`/countries/${id}`);
	}

	async getCountryByCode(code: string): Promise<Country> {
		return apiClient.get<Country>(`/countries/code/${code}`);
	}
}

export const countriesApi = new CountriesApi();
