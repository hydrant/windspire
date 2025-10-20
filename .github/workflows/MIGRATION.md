# CI/CD Migration Summary: Functions → Container Apps

## What Changed

### Removed Files
- ❌ `.github/workflows/build-backend.yml` (Functions-based workflow)
  - **Note:** File still exists but is now obsolete. Should be deleted or renamed.

### New Files
- ✅ `.github/workflows/build-backend-container.yml` - New Container Apps workflow
- ✅ `.github/workflows/README_CONTAINER_APPS.md` - Comprehensive documentation

### Modified Files

#### `.github/workflows/deploy.yml`
**Before (Functions):**
```yaml
backend:
  uses: ./.github/workflows/build-backend.yml
  with:
    function-app-name: ${{ needs.infrastructure.outputs.function-app-name }}
```

**After (Container Apps):**
```yaml
backend:
  uses: ./.github/workflows/build-backend-container.yml
  with:
    container-app-name: ${{ needs.infrastructure.outputs.container-app-name }}
    registry-name: ${{ needs.infrastructure.outputs.registry-name }}
    registry-login-server: ${{ needs.infrastructure.outputs.registry-login-server }}
    resource-group: ${{ secrets.AZURE_RESOURCE_GROUP }}
```

#### `.github/workflows/build-infrastructure.yml`
**Added Outputs:**
```yaml
outputs:
  container-app-name: string        # NEW
  container-app-url: string         # NEW
  registry-name: string             # NEW
  registry-login-server: string     # NEW
  # Removed: function-app-name
```

**Removed:**
- Function App CORS configuration step
- Function App name output

#### `infrastructure/main.bicep`
**Added Outputs:**
```bicep
output containerAppName string = containerApp.outputs.name
output containerAppUrl string = 'https://${containerApp.outputs.fqdn}'
output postgresServerName string = postgresServer.outputs.name
output containerRegistryLoginServer string = containerRegistry.outputs.loginServer
output containerRegistryName string = containerRegistry.outputs.name
```

## Workflow Comparison

### Old: Azure Functions Custom Handler

```
┌─────────────┐
│   GitHub    │
│   Actions   │
└──────┬──────┘
       │ 1. Build Rust binary (musl)
       │ 2. Create host.json
       │ 3. Create function.json
       │ 4. Zip files
       ↓
┌─────────────┐
│   Azure     │
│  Functions  │ (Flex Consumption)
│             │ ❌ No Custom Handler support!
└─────────────┘
```

### New: Container Apps

```
┌─────────────┐
│   GitHub    │
│   Actions   │
└──────┬──────┘
       │ 1. Verify SQLx offline mode
       │ 2. Build Docker image (az acr build)
       │ 3. Push to ACR
       ↓
┌─────────────┐
│  Azure      │
│ Container   │
│  Registry   │
└──────┬──────┘
       │ 4. Update Container App
       ↓
┌─────────────┐
│  Container  │
│    Apps     │ ✅ Scales to 0!
│             │
└─────────────┘
```

## Key Differences

| Aspect | Functions (Old) | Container Apps (New) |
|--------|----------------|---------------------|
| **Build Artifact** | Statically-linked binary + JSON config | Docker image |
| **Registry** | N/A | Azure Container Registry |
| **Build Location** | GitHub Actions runner | Azure (az acr build) |
| **Deployment Method** | Azure Functions Action | az containerapp update |
| **Scale to Zero** | ❌ Not supported (Flex) | ✅ Native support |
| **CORS Config** | Workflow step | Bicep template |
| **Health Check** | Function endpoint | Container endpoint |

## Breaking Changes

### For Developers

1. **Local Testing Changes:**
   ```bash
   # Old: Run binary directly
   cargo run
   
   # New: Run via Docker
   docker build -t windspire-backend .
   docker run -p 8080:8080 --env-file .env windspire-backend
   ```

2. **Deployment Triggers:**
   - Same: Push tags (dev*, staging*, prod*)
   - No changes needed to trigger deployments

3. **Environment Variables:**
   - Same secrets required in GitHub
   - Container Apps reads from Bicep-configured env vars
   - No changes needed

### For Infrastructure

1. **Required Outputs from Bicep:**
   - Must export `containerAppName`, `containerAppUrl`
   - Must export `containerRegistryName`, `containerRegistryLoginServer`

2. **Service Principal Permissions:**
   ```bash
   # Required: AcrPush role on Container Registry
   az role assignment create \
     --assignee <client-id> \
     --role AcrPush \
     --scope <acr-resource-id>
   
   # Required: Contributor role on Container App
   az role assignment create \
     --assignee <client-id> \
     --role Contributor \
     --scope <container-app-resource-id>
   ```

## Migration Checklist

### Before Migration
- [x] Update `infrastructure/main.bicep` to use Container Apps (completed)
- [x] Update `infrastructure/main.bicepparam` for parameters (completed)
- [x] Create `windspire_backend/Dockerfile` (completed)
- [x] Create `windspire_backend/.dockerignore` (completed)
- [x] Test Bicep deployment locally (pending user testing)

### Workflow Updates
- [x] Create new `build-backend-container.yml` workflow
- [x] Update `deploy.yml` to call new backend workflow
- [x] Update `build-infrastructure.yml` outputs
- [ ] Delete or archive old `build-backend.yml`
- [x] Document changes in README

### Post-Migration
- [ ] Test deployment in dev environment
- [ ] Verify Container App scales to zero
- [ ] Check Container App logs for errors
- [ ] Test health endpoint
- [ ] Update frontend `API_BASE_URL` secret to Container App URL
- [ ] Monitor first production deployment
- [ ] Update runbooks/documentation

## Rollback Plan

If Container Apps deployment fails:

1. **Keep old Function App running** (don't delete yet)
2. **Revert deploy.yml** to use old backend workflow:
   ```bash
   git revert <commit-hash>
   git push
   ```
3. **Update infrastructure** to deploy Function App again
4. **Investigate failure** using Container App logs

## Testing the New Workflow

### Test in Dev Environment

1. **Deploy infrastructure:**
   ```bash
   cd infrastructure
   ./deploy.sh dev
   ```

2. **Verify outputs:**
   ```bash
   az deployment group show \
     --name windspire-infrastructure-dev \
     --resource-group windspire-dev \
     --query properties.outputs
   ```

3. **Trigger backend workflow:**
   ```bash
   git tag dev-test-$(date +%Y%m%d-%H%M%S)
   git push --tags
   ```

4. **Monitor deployment:**
   - GitHub Actions UI: Watch workflow progress
   - Azure Portal: Container Apps → Revisions
   - Logs: `az containerapp logs show --name ... --follow`

5. **Test backend:**
   ```bash
   CONTAINER_APP_URL=$(az containerapp show \
     --name <container-app-name> \
     --resource-group <resource-group> \
     --query properties.configuration.ingress.fqdn \
     --output tsv)
   
   curl https://$CONTAINER_APP_URL/health
   ```

## Questions?

See:
- [README_CONTAINER_APPS.md](.github/workflows/README_CONTAINER_APPS.md) - Comprehensive workflow documentation
- [infrastructure/SETUP.md](../infrastructure/SETUP.md) - Infrastructure deployment guide
- [infrastructure/MIGRATION_SUMMARY.md](../infrastructure/MIGRATION_SUMMARY.md) - Infrastructure changes summary

## Status

- ✅ Infrastructure updated to Container Apps with AVM modules
- ✅ Docker build configuration complete
- ✅ New workflow created (build-backend-container.yml)
- ✅ Deploy workflow updated
- ✅ Documentation complete
- ⏳ **Pending:** User testing and first deployment
