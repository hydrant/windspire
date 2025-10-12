# Windspire Frontend (CSR)

This is the Client-Side Rendered (CSR) version of the Windspire frontend, specifically optimized for deployment to Azure Static Web Apps.

## Architecture

- **Framework**: Svelte 5 with TypeScript
- **Bundler**: Vite
- **Routing**: svelte-spa-router for client-side navigation
- **Styling**: Tailwind CSS
- **Deployment**: Azure Static Web Apps

## Key Differences from SvelteKit Version

### CSR vs SSR

- **No Server-Side Rendering**: All rendering happens in the browser
- **Static File Deployment**: Builds to static HTML, CSS, and JS files
- **Client-Side Routing**: Uses hash-based routing for SPA behavior
- **API Integration**: Direct fetch calls to Azure Functions backend

### File Structure

```
src/
├── lib/
│   ├── api/           # API clients and types
│   ├── components/    # Reusable Svelte components
│   ├── pages/         # Page components
│   ├── stores/        # Svelte stores for state management
│   └── config.ts      # Environment configuration
├── main.ts            # Application entry point
├── App.svelte         # Root component with routing
└── app.css           # Global styles
```

## Environment Configuration

### Development

```bash
VITE_API_BASE_URL=http://localhost:8080/api
VITE_FIREBASE_CONFIG={"apiKey":"...","authDomain":"..."}
```

### Production (Azure Static Web Apps)

```bash
VITE_API_BASE_URL=/api
VITE_FIREBASE_CONFIG={"apiKey":"...","authDomain":"..."}
```

## Scripts

```bash
# Development
pnpm dev          # Start development server on http://localhost:3000

# Production
pnpm build        # Build for production
pnpm preview      # Preview production build

# Quality
pnpm check        # Type checking
pnpm format       # Format code
pnpm lint         # Lint code
```

## Routing

Uses `svelte-spa-router` for client-side routing:

- `/` - Home page
- `/boats` - Boats listing (requires authentication)
- `/auth` - Authentication page
- `*` - 404 not found page

### Route Protection

Authentication-required routes use route conditions to redirect unauthenticated users to the login modal.

## API Integration

### API Client

The `apiClient` automatically:

- Adds authentication headers from localStorage
- Handles 401 responses by clearing invalid tokens
- Parses JSON responses with error handling
- Supports typed responses via TypeScript generics

### Environment-Based URLs

API URLs are configured via environment variables:

- **Development**: `http://localhost:8080/api`
- **Production**: `/api` (relative to Azure Static Web Apps domain)

## State Management

### User Store

Manages authentication state using Svelte stores:

```typescript
import { userStore } from '$lib/stores/user';

// Check if user is authenticated
$userStore; // null or User object

// Update user state
userStore.set(userData);
```

### Authentication Flow

1. User signs in via Firebase
2. Firebase token sent to backend `/api/auth/firebase`
3. Backend returns JWT token
4. JWT stored in localStorage
5. API calls include JWT in Authorization header

## Azure Static Web Apps Configuration

### staticwebapp.config.json

- **API Routes**: `/api/*` forwarded to Azure Functions
- **SPA Fallback**: All other routes serve `index.html`
- **Asset Handling**: Static assets served directly

### Build Process

1. Vite builds static files to `dist/` directory
2. Azure Static Web Apps serves files from `dist/`
3. API calls routed to Azure Functions backend

## Deployment

### Automatic Deployment

- Push to `main` branch triggers deployment
- GitHub Actions builds both frontend and backend
- Frontend deployed to Azure Static Web Apps
- Backend deployed as Azure Functions

### Manual Deployment

```bash
# Build locally
pnpm build

# Deploy using Azure CLI
az staticwebapp deploy \
  --name windspire-app \
  --source ./dist \
  --api-location ../windspire_backend/api
```

## Development Workflow

### Local Development

```bash
# Terminal 1: Start backend
cd windspire_backend
cargo run

# Terminal 2: Start frontend
cd windspire_frontend_svelte_csr
pnpm dev
```

### Testing

- Frontend: http://localhost:3000
- Backend: http://localhost:8080/api
- Integration: Both connected via API calls

## Production Considerations

### Performance

- Code splitting via Vite's manual chunks
- Static asset optimization
- Firebase SDK lazy loading

### SEO

- SPA architecture means limited SEO
- Consider pre-rendering for marketing pages
- Use meta tags appropriately

### Security

- No server secrets in frontend code
- Environment variables for configuration
- Firebase handles authentication securely

## Migration from SvelteKit

The CSR version maintains API compatibility with the SvelteKit version:

- Same API endpoints and data structures
- Same authentication flow
- Same UI components (adapted for pure Svelte)
- Same styling and design system

This allows for easy comparison and gradual migration if needed.
