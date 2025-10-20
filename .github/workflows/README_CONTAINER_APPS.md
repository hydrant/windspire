# GitHub Actions Workflows for Container Apps

This document explains the updated GitHub Actions workflows for deploying the Windspire backend to Azure Container Apps.

## Overview

The CI/CD pipeline has been updated from Azure Functions to Azure Container Apps. The key changes are:

1. **Backend now runs in Docker containers** instead of Azure Functions Custom Handlers
2. **New workflow**: `build-backend-container.yml` replaces `build-backend.yml`
3. **Container Registry integration**: Docker images are built and pushed to Azure Container Registry
4. **Scale-to-zero capability**: Container Apps natively supports scaling down to 0 replicas

## Workflow Files

### 1. `build-infrastructure.yml` (Updated)

Deploys all Azure infrastructure using Bicep templates with Azure Verified Modules (AVM).

**Key Changes:**
- Added outputs for Container Apps:
  - `container-app-name`: Name of the Container App
  - `container-app-url`: HTTPS URL of the Container App (e.g., `https://app.region.azurecontainerapps.io`)
  - `registry-name`: Name of Azure Container Registry
  - `registry-login-server`: ACR login server URL
- Removed Function App-specific CORS configuration
- Updated outputs to use correct Bicep output names

**Outputs:**
```yaml
outputs:
  static-web-app-url: string      # Frontend URL
  container-app-name: string      # Backend Container App name
  container-app-url: string       # Backend URL
  registry-name: string           # ACR name
  registry-login-server: string   # ACR server (e.g., myacr.azurecr.io)
  postgres-server-name: string    # PostgreSQL server name
  key-vault-name: string          # Key Vault name
  deployment-token: string        # Static Web App deployment token
```

### 2. `build-backend-container.yml` (New)

Builds the Rust backend as a Docker image and deploys it to Azure Container Apps.

**Workflow Steps:**

#### Job 1: `build_and_push`
1. **Checkout code** from repository
2. **Set up Rust** with musl target for static linking
3. **Install dependencies** (SQLx CLI, musl-tools)
4. **Start PostgreSQL** container for testing
5. **Verify SQLx offline mode** - ensures query cache is current
6. **Azure Login** using service principal credentials
7. **Build & Push Docker image** using `az acr build`:
   - Builds image in Azure (avoids local Docker complexity)
   - Tags with both `$GITHUB_SHA` and `latest`
   - Uses multi-stage Dockerfile (rust:alpine → scratch)
   - Pushes to Azure Container Registry

#### Job 2: `deploy`
1. **Azure Login**
2. **Update Container App** with new image using `az containerapp update`
3. **Verify deployment** by testing health endpoint

**Inputs:**
```yaml
inputs:
  environment: string               # dev, staging, prod
  container-app-name: string        # Container App to update
  registry-name: string             # ACR name (e.g., windspiredevacr)
  registry-login-server: string     # ACR server (e.g., windspiredevacr.azurecr.io)
  resource-group: string            # Azure resource group

secrets:
  AZURE_CLIENT_ID: string           # Service principal client ID
  AZURE_CLIENT_SECRET: string       # Service principal secret
  AZURE_TENANT_ID: string           # Azure AD tenant ID
  AZURE_SUBSCRIPTION_ID: string     # Azure subscription ID
```

**Key Features:**
- Uses `az acr build` to build images in Azure (faster, no local Docker daemon needed)
- Verifies SQLx offline mode before building (catches migration issues early)
- Tags images with both commit SHA and `latest` for easy rollback
- Tests health endpoint after deployment
- Provides detailed logging and error messages

### 3. `deploy.yml` (Updated)

Master orchestration workflow that coordinates infrastructure and application deployments.

**Key Changes:**
- Backend job now calls `build-backend-container.yml` instead of `build-backend.yml`
- Passes Container Apps-specific parameters:
  ```yaml
  container-app-name: ${{ needs.infrastructure.outputs.container-app-name }}
  registry-name: ${{ needs.infrastructure.outputs.registry-name }}
  registry-login-server: ${{ needs.infrastructure.outputs.registry-login-server }}
  resource-group: ${{ secrets.AZURE_RESOURCE_GROUP }}
  ```
- Updated summary to show Container Apps information

**Deployment Flow:**
```
1. determine_environment
   ↓
2. infrastructure (deploys Bicep)
   ↓
3. backend (builds Docker, pushes to ACR, updates Container App)
   ↓ + ↓
4. frontend (builds Svelte, deploys to Static Web App)
   ↓
5. summary (displays deployment info)
```

## Environment Variables & Secrets

### Required GitHub Secrets

**Azure Credentials:**
- `AZURE_CLIENT_ID` - Service principal client ID (from Entra ID app registration)
- `AZURE_CLIENT_SECRET` - Service principal secret
- `AZURE_TENANT_ID` - Azure AD tenant ID
- `AZURE_SUBSCRIPTION_ID` - Azure subscription ID
- `AZURE_RESOURCE_GROUP` - Resource group name

**Database:**
- `POSTGRES_ADMIN_LOGIN` - PostgreSQL admin username
- `POSTGRES_ADMIN_PASSWORD` - PostgreSQL admin password

**Firebase Authentication:**
- `FIREBASE_PROJECT_ID`
- `FIREBASE_PRIVATE_KEY`
- `FIREBASE_CLIENT_EMAIL`
- `FIREBASE_AUTH_DOMAIN`
- `FIREBASE_API_KEY`
- `FIREBASE_STORAGE_BUCKET`
- `FIREBASE_MESSAGING_SENDER_ID`
- `FIREBASE_APP_ID`

**Application:**
- `JWT_SECRET` - Secret for signing JWT tokens
- `API_BASE_URL` - Base URL for backend API (set to Container App URL)

## Docker Build Strategy

The backend uses a **multi-stage Docker build** for optimal image size:

### Stage 1: Builder (rust:1.83-alpine)
- Installs Rust and musl-dev for static compilation
- Copies source code and dependencies
- Runs `cargo build --release --target x86_64-unknown-linux-musl`
- Produces statically-linked binary (~20-30 MB)

### Stage 2: Runtime (scratch)
- Minimal base image (only Linux kernel interfaces)
- Copies CA certificates for HTTPS
- Copies static binary from builder stage
- **Final image size: ~12-15 MB** (compared to ~1GB+ with full Rust toolchain)

### Benefits:
- ✅ **Minimal attack surface** - no package manager, shell, or utilities
- ✅ **Fast startup** - small image pulls quickly
- ✅ **Low cost** - reduced storage and bandwidth
- ✅ **Static binary** - no runtime dependencies

## SQLx Offline Mode

The workflow verifies SQLx offline mode before building Docker images:

```yaml
- name: Verify SQLx offline mode
  run: |
    export DATABASE_URL="postgresql://test_user:test_password@localhost:5432/windspire"
    sqlx migrate run
    cargo sqlx prepare --check
```

**Why this matters:**
- SQLx queries are compiled at build time using cached metadata
- The Docker build doesn't have access to a live database
- This verification ensures the query cache (`sqlx-data.json`) is current
- **Catches migration issues before building Docker image**

## Scaling & Cost Optimization

### Container Apps Configuration

```bicep
scale: {
  minReplicas: 0    // Scale to zero when idle
  maxReplicas: 10   // Scale up under load
  rules: [
    {
      name: 'http-scaling-rule'
      http: {
        metadata: {
          concurrentRequests: '10'  // Scale up after 10 concurrent requests
        }
      }
    }
  ]
}
```

### Cost Implications

**Idle State (0 replicas):**
- Container App: ~$0.00/month (pay-per-request)
- Container Registry: ~$5/month (Basic tier storage)
- **Total: ~$5/month** when idle

**Active State (1 replica):**
- Container App: ~$15-20/month (depends on CPU/memory usage)
- Container Registry: ~$5/month
- **Total: ~$20-25/month** under typical load

**Compare to Azure Functions B1:**
- B1 Plan: $13/month (runs 24/7, no scale-to-zero)
- Container Apps: $5-25/month (true serverless)

## Deployment Process

### Manual Deployment

1. **Deploy Infrastructure:**
   ```bash
   cd infrastructure
   ./deploy.sh dev
   ```

2. **Trigger Backend Build:**
   - Push code or create a tag
   - Workflow automatically builds and deploys

3. **View Container App Logs:**
   ```bash
   az containerapp logs show \
     --name <container-app-name> \
     --resource-group <resource-group> \
     --follow
   ```

### Automatic Deployment

Workflows trigger on:
- **Tags**: `dev*`, `staging*`, `prod*`
- **Manual**: Use "Run workflow" in GitHub Actions UI

Example tag deployment:
```bash
git tag dev-1.0.0
git push origin dev-1.0.0
```

## Troubleshooting

### Image Build Failures

**Problem:** `az acr build` fails with "unauthorized"
**Solution:** Ensure service principal has `AcrPush` role:
```bash
az role assignment create \
  --assignee <client-id> \
  --role AcrPush \
  --scope /subscriptions/<sub-id>/resourceGroups/<rg>/providers/Microsoft.ContainerRegistry/registries/<acr-name>
```

### Container App Not Starting

**Problem:** Container App stuck in "Provisioning" or fails health checks
**Solution:** Check logs for startup errors:
```bash
az containerapp logs show \
  --name <container-app-name> \
  --resource-group <resource-group> \
  --follow
```

Common issues:
- Missing environment variables
- Database connection failures
- Port mismatch (Container App expects port 8080)

### SQLx Offline Mode Verification Fails

**Problem:** `cargo sqlx prepare --check` fails in CI
**Solution:** Regenerate query cache locally:
```bash
# Start PostgreSQL locally or in Docker
export DATABASE_URL="postgresql://..."
sqlx migrate run
cargo sqlx prepare
git add .sqlx/
git commit -m "Update SQLx query cache"
```

### Health Check Failures

**Problem:** Health endpoint returns 500 or times out
**Solution:**
1. Check Container App logs
2. Verify DATABASE_URL environment variable
3. Ensure PostgreSQL allows connections from Container App
4. Check firewall rules on PostgreSQL Flexible Server

## Comparison: Functions vs Container Apps

| Feature | Azure Functions (Custom Handler) | Azure Container Apps |
|---------|----------------------------------|---------------------|
| **Deployment** | Zip file with binary + host.json | Docker image |
| **Scale to Zero** | ❌ Not on Flex Consumption for custom | ✅ Native support |
| **Cold Start** | ~2-5 seconds | ~3-8 seconds |
| **Image Size** | N/A | ~12 MB (scratch-based) |
| **Cost (idle)** | ~$0-5/month | ~$5/month |
| **Cost (active)** | ~$15-30/month | ~$20-25/month |
| **Custom Runtime** | Limited (Linux only) | Full Docker support |
| **Complexity** | Medium (custom handler config) | Low (standard Docker) |
| **Monitoring** | Application Insights | Container Apps logs + App Insights |
| **Networking** | VNET integration available | VNET integration available |

## Next Steps

- [ ] Test end-to-end deployment in dev environment
- [ ] Configure Application Insights for Container Apps
- [ ] Set up staging and prod environments
- [ ] Implement blue-green deployments using Container App revisions
- [ ] Add performance testing in CI/CD pipeline
- [ ] Configure custom domains and TLS certificates

## References

- [Azure Container Apps Documentation](https://learn.microsoft.com/en-us/azure/container-apps/)
- [Azure Container Registry Build Documentation](https://learn.microsoft.com/en-us/azure/container-registry/container-registry-tasks-overview)
- [SQLx Offline Mode](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#enable-building-in-offline-mode-with-query)
- [Multi-stage Docker Builds](https://docs.docker.com/build/building/multi-stage/)
- [Infrastructure SETUP.md](../infrastructure/SETUP.md)
