# Migration to GitHub Container Registry

## Summary

Successfully migrated from **Azure Container Registry (ACR)** to **GitHub Container Registry (GHCR)** for storing Docker images.

## Why GitHub Container Registry?

- ✅ **Free for public repositories** (unlimited storage and bandwidth)
- ✅ **Integrated with GitHub** (automatic authentication with `GITHUB_TOKEN`)
- ✅ **No additional Azure costs** (saves ~$5/month for Basic ACR)
- ✅ **Better CI/CD integration** with GitHub Actions
- ✅ **Package management** alongside code repository

## What Changed

### Infrastructure (`infrastructure/main.bicep`)

**Removed:**
- Azure Container Registry module
- ACR admin credentials configuration
- ACR resource reference

**Added:**
- GitHub Container Registry parameters:
  - `ghcrUsername` - GitHub organization/username
  - `ghcrToken` - GitHub Personal Access Token (uses `GITHUB_TOKEN` in workflows)
  - `containerImage` - Full GHCR image path

**Updated:**
- Container App registry configuration now points to `ghcr.io`
- Registry secret uses GitHub token instead of ACR password
- Container image references GHCR image

### Parameters (`infrastructure/main.bicepparam`)

**Added:**
```bicep
param ghcrUsername = readEnvironmentVariable('GHCR_USERNAME', 'hydrant')
param ghcrToken = readEnvironmentVariable('GHCR_TOKEN')
param containerImage = 'ghcr.io/hydrant/windspire-backend:latest'
```

### Workflows

#### `build-backend.yml`

**Removed:**
- `registry-name` and `registry-login-server` inputs
- Azure CLI `az acr build` command
- ACR authentication steps

**Added:**
- GitHub Container Registry authentication using `docker/login-action@v3`
- Docker Buildx setup for efficient builds
- `docker/build-push-action@v6` for building and pushing
- GitHub Actions cache for faster builds

**Key Changes:**
```yaml
# Old: Azure Container Registry
az acr build \
  --registry ${{ inputs.registry-name }} \
  --image windspire-backend:latest

# New: GitHub Container Registry  
uses: docker/build-push-action@v6
with:
  push: true
  tags: |
    ghcr.io/${{ github.repository_owner }}/windspire-backend:${{ env.IMAGE_TAG }}
    ghcr.io/${{ github.repository_owner }}/windspire-backend:latest
```

#### `deploy.yml`

**Removed:**
- `registry-name` and `registry-login-server` parameters passed to backend workflow

**Updated:**
- Backend workflow call simplified (only needs `container-app-name`)
- Summary output shows GHCR image location

#### `build-infrastructure.yml`

**Removed:**
- ACR output variables

**Added:**
- GHCR parameters to Bicep deployment:
  - `ghcrUsername`: `${{ github.repository_owner }}`
  - `ghcrToken`: `${{ secrets.GITHUB_TOKEN }}`
  - `containerImage`: Full GHCR image path

## Container Image Location

**Old:** `crXXXXXXdev.azurecr.io/windspire-backend:latest`  
**New:** `ghcr.io/hydrant/windspire-backend:latest`

Images are automatically pushed to:
- `ghcr.io/hydrant/windspire-backend:latest` (always latest)
- `ghcr.io/hydrant/windspire-backend:<commit-sha>` (specific version)

## Authentication

### GitHub Actions (Automatic)
The workflows use `secrets.GITHUB_TOKEN` which is automatically provided by GitHub Actions. No additional configuration needed!

### Local Development
To pull images locally or deploy manually:

```bash
# Create a GitHub Personal Access Token with 'read:packages' scope
# https://github.com/settings/tokens/new

# Login to GHCR
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin

# Pull image
docker pull ghcr.io/hydrant/windspire-backend:latest
```

### Azure Container Apps
The Bicep configuration automatically configures the Container App to authenticate with GHCR using the provided token.

## Required GitHub Secrets

### Existing Secrets (Unchanged)
- `AZURE_CLIENT_ID`
- `AZURE_CLIENT_SECRET`
- `AZURE_TENANT_ID`
- `AZURE_SUBSCRIPTION_ID`
- `AZURE_RESOURCE_GROUP`
- `POSTGRES_ADMIN_LOGIN`
- `POSTGRES_ADMIN_PASSWORD`
- `FIREBASE_*` (all Firebase secrets)
- `JWT_SECRET`

### New/Modified Secrets
**None required!** 

The `GITHUB_TOKEN` is automatically provided by GitHub Actions. For manual deployments, you can set:
- `GHCR_USERNAME` (defaults to 'hydrant')
- `GHCR_TOKEN` (for local/manual Bicep deployments only)

## Deployment Flow

```
1. Code pushed to GitHub
   ↓
2. GitHub Actions workflow triggered
   ↓
3. Infrastructure deployed (Container App configured for GHCR)
   ↓
4. Docker image built
   ↓
5. Image pushed to ghcr.io/hydrant/windspire-backend
   ↓
6. Container App updated with new GHCR image
   ↓
7. Container App pulls image from GHCR and starts
```

## Image Visibility

By default, GHCR packages are **private**. To make them public:

1. Go to https://github.com/orgs/hydrant/packages
2. Find `windspire-backend` package
3. Click **Package settings**
4. Scroll to **Danger Zone**
5. Click **Change visibility** → **Public**

**Note:** For private organizations, keeping images private is recommended. Azure Container Apps can authenticate with private GHCR images using the provided token.

## Cost Savings

**Before (with ACR):**
- Azure Container Registry (Basic): ~$5/month
- Storage: Included
- **Total: ~$5/month**

**After (with GHCR):**
- GitHub Container Registry: $0 (free for public repos, included with GitHub subscription for private)
- **Total: $0/month**

**Annual Savings: ~$60**

## Troubleshooting

### Image Pull Errors

If Container App fails to pull image:

```bash
# Check if image exists
docker pull ghcr.io/hydrant/windspire-backend:latest

# Verify GHCR token is valid
curl -H "Authorization: Bearer $GITHUB_TOKEN" \
  https://ghcr.io/v2/hydrant/windspire-backend/tags/list
```

### Build Failures

Check GitHub Actions logs:
1. Go to repository **Actions** tab
2. Find failed workflow
3. Check "Build & Push Container Image" step

Common issues:
- Docker build context errors → Check `wind spire_backend/Dockerfile`
- Permission errors → Verify `GITHUB_TOKEN` has packages:write scope
- Rate limits → Wait and retry

### Container App Not Starting

```bash
# Check Container App logs
az containerapp logs show \
  --name windspire-api-dev \
  --resource-group rg-windspire-dev-eastus2 \
  --follow

# Verify image is configured
az containerapp show \
  --name windspire-api-dev \
  --resource-group rg-windspire-dev-eastus2 \
  --query "properties.template.containers[0].image"
```

## Next Steps

1. **Deploy infrastructure** with new GHCR configuration:
   ```bash
   # Set environment variables
   export GHCR_USERNAME="hydrant"
   export GHCR_TOKEN="$GITHUB_TOKEN"  # Or create PAT
   
   # Deploy via GitHub Actions
   git tag dev-$(date +%Y%m%d-%H%M%S)
   git push --tags
   ```

2. **Verify image is pushed** to GHCR:
   - Visit https://github.com/orgs/hydrant/packages
   - Find `windspire-backend` package
   - Verify latest tag exists

3. **Test Container App** pulls from GHCR:
   ```bash
   # Check Container App status
   az containerapp show \
     --name windspire-api-dev \
     --resource-group rg-windspire-dev-eastus2 \
     --query "properties.runningStatus"
   ```

## Files Modified

- ✅ `infrastructure/main.bicep` - Removed ACR, added GHCR config
- ✅ `infrastructure/main.bicepparam` - Added GHCR parameters
- ✅ `.github/workflows/build-backend.yml` - Changed to push to GHCR
- ✅ `.github/workflows/deploy.yml` - Removed ACR parameters
- ✅ `.github/workflows/build-infrastructure.yml` - Added GHCR parameters to deployment

All changes are backward compatible with existing workflows (except they now use GHCR instead of ACR).
