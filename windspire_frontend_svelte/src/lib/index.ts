// place files you want to import through the `$lib` alias in this folder.

// Export API types
export type {
	Boat,
	BoatCreate,
	BoatUpdate,
	Country,
	PaginatedResult,
	PaginationParams,
	ApiResponse
} from './api/types';

// Export API clients
export { boatsApi } from './api/boats';
export { countriesApi } from './api/countries';
export { apiClient } from './api/client';
