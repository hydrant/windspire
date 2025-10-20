# Fixed GHCR Authentication for Azure Container Apps

## Problem

Infrastructure deployment failing with:
```
DENIED: requested access to the resource is denied
Field 'template.containers.windspire-backend.image' is invalid
GET https://ghcr.io/hydrant/windspire-backend:latest
```

## Root Cause

Azure Container Apps cannot pull the Docker image from GHCR because:
1. The package is **private** by default
2. The `GITHUB_TOKEN` used in the workflow has limited scope and lifetime
3. Azure Container Apps needs persistent authentication to pull images

## Solutions

### ✅ Solution 1: Make GHCR Package Public (RECOMMENDED - Simplest)

**Steps:**
1. Go to: https://github.com/orgs/hydrant/packages
2. Find the `windspire-backend` package
3. Click on the package name
4. Click **Package settings** (gear icon)
5. Scroll to **Danger Zone**
6. Click **Change visibility** → **Public**
7. Confirm the change

**After making it public:**
```bash
# Re-run the infrastructure deployment
# No workflow changes needed!
```

**Pros:**
- ✅ No secrets needed
- ✅ Anyone can pull the image
- ✅ Simpler authentication
- ✅ Works immediately

**Cons:**
- ⚠️ Image is publicly accessible
- ⚠️ Anyone can see your Docker layers

---

### Solution 2: Use GitHub Personal Access Token (PAT)

**Steps:**

1. **Create PAT**:
   - Go to: https://github.com/settings/tokens/new
   - Note: `GHCR access for Container Apps`
   - Expiration: 1 year or no expiration
   - Scopes: ✅ `read:packages`
   - Click **Generate token**
   - **Copy the token immediately** (you won't see it again!)

2. **Add Secret to GitHub**:
   - Go to: https://github.com/hydrant/windspire/settings/secrets/actions
   - Click **New repository secret**
   - Name: `GHCR_PAT`
   - Value: Paste your PAT token
   - Click **Add secret**

3. **Workflow Already Updated**:
   The workflow has been updated to use `secrets.GHCR_PAT` instead of `secrets.GITHUB_TOKEN`.

4. **Re-run deployment**:
   ```bash
   git add .github/workflows/build-infrastructure.yml
   git commit -m "fix: use PAT for GHCR authentication"
   git push origin azure_csr:main
   ```

**Pros:**
- ✅ Image stays private
- ✅ More secure
- ✅ Token can be revoked if compromised

**Cons:**
- ⚠️ Requires creating and managing PAT
- ⚠️ Token needs to be rotated periodically
- ⚠️ Extra configuration step

---

## Comparison

| Aspect | Public Package | Private with PAT |
|--------|---------------|------------------|
| Setup Time | 2 minutes | 5 minutes |
| Security | Lower (public) | Higher (private) |
| Maintenance | None | Token rotation needed |
| Complexity | Simple | Moderate |
| Cost | Free | Free |
| Recommended For | Open source, demos | Production, proprietary code |

## Recommended Approach

**For this project**: Make the package **public** ✅

**Why?**
- Fastest solution
- No additional secrets to manage
- Container Apps deployment is already secure (in your Azure subscription)
- The source code is already in a private GitHub repo, so the compiled image being public doesn't add much risk

## What Changed

**File**: `.github/workflows/build-infrastructure.yml`

```yaml
# Before
"ghcrToken": "${{ secrets.GITHUB_TOKEN }}"

# After
"ghcrToken": "${{ secrets.GHCR_PAT }}"
```

This change is only needed if you choose **Solution 2** (Private with PAT).

## Next Steps

### If choosing Solution 1 (Public - Recommended):
1. Make package public at https://github.com/orgs/hydrant/packages
2. Re-run infrastructure deployment (no code changes needed)
3. Done! ✅

### If choosing Solution 2 (Private with PAT):
1. Create PAT at https://github.com/settings/tokens/new
2. Add `GHCR_PAT` secret to repository
3. Commit and push the workflow change
4. Re-run infrastructure deployment
5. Done! ✅

## Verification

After fixing, verify Container Apps can pull the image:

```bash
# Check Container App status
az containerapp show \
  --name windspire-api-dev \
  --resource-group rg-windspire-dev-eastus2 \
  --query "properties.provisioningState"

# Should return: "Succeeded"

# Check if container is running
az containerapp revision list \
  --name windspire-api-dev \
  --resource-group rg-windspire-dev-eastus2 \
  --query "[0].properties.provisioningState"
```

The deployment will succeed once Azure Container Apps can authenticate with GHCR! 🚀
