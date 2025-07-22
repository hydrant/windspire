// API Response types
export interface ApiResponse<T> {
    success: boolean;
    data: T;
    message?: string;
}

// Pagination types
export interface PaginationParams {
    page?: number;
    limit?: number;
}

export interface PaginatedResult<T> {
    data: T[];
    total: number;
    page: number;
    limit: number;
    totalPages: number;
}

// Boat types
export interface Boat {
    id: string;
    name: string;
    brand?: string;
    model?: string;
    sailNumber?: string;
    countryId: string;
}

export interface BoatCreate {
    name: string;
    brand?: string;
    model?: string;
    sailNumber?: string;
    countryId: string;
}

export interface BoatUpdate {
    name: string;
    brand?: string;
    model?: string;
    sailNumber?: string;
    countryId: string;
}

// Country types (for boat form)
export interface Country {
    id: string;
    name: string;
    isoAlpha2: string;
    isoAlpha3: string;
}