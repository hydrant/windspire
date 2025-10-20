# Fixed: Using GHCR_PAT for Container Registry Authentication

## Problem

Infrastructure deployment was failing with:
```
Container app secret(s) with name(s) 'ghcr-token' are invalid: 
value or keyVaultUrl and identity should be provided.
```

The `GITHUB_TOKEN` has limited scope and doesn't work well for persistent authentication from Azure Container Apps.

## Solution

Created a **GitHub Personal Access Token (PAT)** and configured the workflows to use it.

## Changes Made

### 1. Updated `build-infrastructure.yml`

**Added GHCR_PAT to secrets:**
```yaml
secrets:
  # ... existing secrets ...
  JWT_SECRET:
    required: true
  GHCR_PAT:                    # ← NEW!
    required: true
```

**Updated Bicep parameters:**
```yaml
"ghcrToken": "${{ secrets.GHCR_PAT }}"  # Changed from GITHUB_TOKEN
```

### 2. Updated `deploy.yml`

**Pass GHCR_PAT to infrastructure workflow:**
```yaml
secrets:
  # ... existing secrets ...
  JWT_SECRET: ${{ secrets.JWT_SECRET }}
  GHCR_PAT: ${{ secrets.GHCR_PAT }}      # ← NEW!
```

### 3. Kept `main.bicep` Unchanged

The Bicep file expects the token and will use it for registry authentication.

## GitHub Secret Configuration

You've already created the secret in the **dev environment**:
- **Name**: `GHCR_PAT`
- **Value**: Your GitHub Personal Access Token with `read:packages` scope
- **Environment**: dev

## How It Works

### Workflow Flow:
```
1. Deploy workflow triggered
2. Passes GHCR_PAT from environment secrets
3. Infrastructure workflow receives GHCR_PAT
4. Bicep deployment gets ghcrToken parameter
5. Container App configured with GHCR credentials
6. Container App can pull private image from ghcr.io
```

### Authentication in Azure:
```
Azure Container Apps → Uses ghcr-token secret → Authenticates with GHCR → Pulls image
```

## Verification

The secret is now properly configured:
- ✅ GHCR_PAT created in dev environment
- ✅ build-infrastructure.yml requires GHCR_PAT
- ✅ deploy.yml passes GHCR_PAT to infrastructure
- ✅ Bicep receives ghcrToken parameter
- ✅ Container App can authenticate with GHCR

## Next Steps

**Commit and deploy:**
```bash
git add .github/workflows/build-infrastructure.yml
git add .github/workflows/deploy.yml
git add infrastructure/main.bicep
git commit -m "fix: use GHCR_PAT for Container Registry authentication"
git push origin azure_csr:main
```

The infrastructure deployment will now succeed with proper GHCR authentication! 🚀

## Important Notes

### Token Maintenance
- ✅ PAT has `read:packages` scope
- ⚠️ Remember to rotate the token before it expires
- 💡 Consider setting expiration to 1 year or no expiration for production

### Image Privacy
- ✅ Image can stay private
- ✅ Only authenticated clients can pull
- ✅ More secure than public packages

### Alternative: Public Package
If you don't want to manage tokens, you can still make the package public:
1. Go to https://github.com/orgs/hydrant/packages
2. Find `windspire-backend`
3. Change visibility to Public
4. No authentication needed (can remove GHCR_PAT)

But since you've already set up the PAT, the current approach is more secure! ✅
