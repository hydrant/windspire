# Fixed Deployment Order

## Problem

The deployment was failing with:
```
MANIFEST_UNKNOWN: manifest unknown
GET https://ghcr.io/hydrant/windspire-backend:latest
```

**Root Cause:** The workflow was trying to deploy infrastructure (including Container App) **before** the Docker image existed in GHCR.

## Solution

Reordered the deployment workflow to build the Docker image **first**:

### New Deployment Order

```
1. build_image      → Build and push Docker image to GHCR
2. infrastructure   → Deploy Azure resources (Container App can now pull image)
3. backend          → Update Container App with new revision (if needed)
4. frontend         → Deploy frontend (parallel with backend)
5. summary          → Show deployment results
```

### Before (Incorrect)
```yaml
infrastructure → backend (builds image) + frontend
     ❌ Container App tries to pull image that doesn't exist yet!
```

### After (Correct)
```yaml
build_image → infrastructure → backend + frontend
     ✅ Image exists before Container App is created!
```

## Files Changed

### 1. Created: `.github/workflows/build-backend-image.yml`

New workflow that **only** builds and pushes the Docker image to GHCR.
- Does NOT deploy to Container Apps
- Just builds and pushes `ghcr.io/hydrant/windspire-backend:latest`
- Can be used independently or called by deploy workflow

### 2. Updated: `.github/workflows/deploy.yml`

Changed the job order:

```yaml
# Step 1: Build backend Docker image FIRST (before infrastructure)
build_image:
  needs: determine_environment
  uses: ./.github/workflows/build-backend-image.yml

# Step 2: Deploy infrastructure (after image exists)
infrastructure:
  needs: [determine_environment, build_image]  # ← Now waits for image!
  uses: ./.github/workflows/build-infrastructure.yml

# Step 3: Deploy backend to Container App (after infrastructure exists)
backend:
  needs: [determine_environment, infrastructure]
  uses: ./.github/workflows/build-backend.yml
```

## Why This Fixes the Issue

1. **Image is built first**: Docker image gets pushed to GHCR before any Azure resources are created
2. **Infrastructure can find image**: When Bicep deploys the Container App, the image already exists at `ghcr.io/hydrant/windspire-backend:latest`
3. **Container App starts successfully**: It can pull the image and start the container

## Next Steps

1. **Commit these changes**:
   ```bash
   git add .github/workflows/build-backend-image.yml
   git add .github/workflows/deploy.yml
   git commit -m "fix: build Docker image before infrastructure deployment"
   ```

2. **Push and trigger deployment**:
   ```bash
   git push origin azure_csr
   
   # Or trigger manually via GitHub Actions UI
   ```

3. **Watch the deployment**:
   - Go to: https://github.com/hydrant/windspire/actions
   - The order will now be:
     1. ✅ Build image (3-5 minutes)
     2. ✅ Deploy infrastructure (2-3 minutes)
     3. ✅ Deploy backend & frontend (2-3 minutes)

## Verification

After deployment, verify the image exists:

```bash
# Check GHCR package
# Visit: https://github.com/orgs/hydrant/packages

# Or pull locally
docker pull ghcr.io/hydrant/windspire-backend:latest

# Check Container App
az containerapp show \
  --name windspire-api-dev \
  --resource-group rg-windspire-dev-eastus2 \
  --query "properties.template.containers[0].image"
```

Should output: `"ghcr.io/hydrant/windspire-backend:latest"`

## Future Deployments

After the first successful deployment:
- Image updates will be built first
- Infrastructure will only update if Bicep changes
- Container App will use the newly built image

The deployment order is now correct! 🎉
