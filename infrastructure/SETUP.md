# 🚀 Windspire Azure Container Apps - Complete Setup Guide

## Overview

This deployment uses **Azure Container Apps** for the backend, which provides:
- ✅ **True scale-to-zero** (pay nothing when idle)
- ✅ **Full Docker support** (no custom handler complexity)
- ✅ **Auto-scaling** based on HTTP requests
- ✅ **Simpler than Azure Functions** for custom runtimes

## Prerequisites

Install these tools:
- [Azure CLI](https://learn.microsoft.com/cli/azure/install-azure-cli)
- [Docker](https://www.docker.com/get-started/)

## Step-by-Step Deployment

### 1️⃣ Set Up Environment Variables

Create a file `infrastructure/.env` (do NOT commit this):

```bash
export POSTGRES_ADMIN_PASSWORD="YourSecurePassword123!"
export FIREBASE_PROJECT_ID="your-firebase-project"
export FIREBASE_PRIVATE_KEY="-----BEGIN PRIVATE KEY-----\nYour-Private-Key-Here\n-----END PRIVATE KEY-----\n"
export FIREBASE_CLIENT_EMAIL="firebase-adminsdk-xxx@your-project.iam.gserviceaccount.com"
export JWT_SECRET="your-random-secret-at-least-32-characters-long"
```

Load them:
```bash
source infrastructure/.env
```

### 2️⃣ Login to Azure

```bash
az login
az account set --subscription "Your-Subscription-Name"
```

### 3️⃣ Run the Deployment Script

```bash
./infrastructure/deploy.sh
```

This will:
- Create the resource group
- Deploy all infrastructure (PostgreSQL, Container Registry, Container Apps Environment, etc.)
- Show you the output values

**Expected duration**: 5-10 minutes

### 4️⃣ Build and Push Your Docker Image

After infrastructure is deployed:

```bash
cd windspire_backend

# Get your registry name from the deployment output
REGISTRY_NAME="<from-deployment-output>"

# Build and push in one command (recommended)
az acr build \
  --registry $REGISTRY_NAME \
  --image windspire-backend:latest \
  --platform linux/amd64 \
  .
```

**Alternative**: Build locally and push
```bash
docker build -t windspire-backend:latest .
az acr login --name $REGISTRY_NAME
docker tag windspire-backend:latest $REGISTRY_NAME.azurecr.io/windspire-backend:latest
docker push $REGISTRY_NAME.azurecr.io/windspire-backend:latest
```

### 5️⃣ Verify Deployment

```bash
# Check Container App status
az containerapp show \
  --name windspire-api-dev \
  --resource-group windspire-dev-rg \
  --query "properties.runningStatus" \
  --output table

# View logs (follow mode)
az containerapp logs show \
  --name windspire-api-dev \
  --resource-group windspire-dev-rg \
  --follow

# Test the health endpoint
BACKEND_URL=$(az containerapp show \
  --name windspire-api-dev \
  --resource-group windspire-dev-rg \
  --query "properties.configuration.ingress.fqdn" \
  --output tsv)

curl https://$BACKEND_URL/health
```

## Architecture

```
Internet
   │
   ▼
┌──────────────────────┐
│   Static Web App     │  ← Svelte Frontend (Free Tier)
│   (Svelte SPA)       │
└──────────┬───────────┘
           │ HTTPS
           ▼
┌──────────────────────┐
│   Container App      │  ← Rust Backend (Scales 0-10)
│   (Rust + Docker)    │     • 0.5 vCPU, 1GB RAM
│                      │     • HTTP auto-scaling
│   ┌──────────────┐   │     • Cold start: ~2-5s
│   │ windspire-   │   │
│   │ backend:     │   │
│   │ latest       │   │
│   └──────────────┘   │
└──────────┬───────────┘
           │
           │ PostgreSQL SSL
           ▼
┌──────────────────────┐
│  PostgreSQL Flexible │  ← Database (Burstable B1ms)
│  Server              │     • 32GB storage
│  (windspire DB)      │     • Auto-grow enabled
└──────────────────────┘
```

## Cost Breakdown

| Resource | Configuration | Monthly Cost (USD) |
|----------|--------------|-------------------|
| **Container Apps Environment** | Consumption tier | $0 (free tier) |
| **Container App** | 0.5 vCPU, 1GB RAM, scales 0-10 | $0-5 (idle = $0) |
| **Container Registry** | Basic tier | ~$5 |
| **PostgreSQL** | Burstable B1ms, 32GB | ~$15 |
| **Static Web App** | Free tier | $0 |
| | **TOTAL** | **~$20-25/month** |

**Note**: Container Apps cost $0 when scaled to zero (no traffic). With low traffic, expect $0-2/month for compute.

## Environment Variables

Your Container App receives these automatically:

| Variable | Source | Description |
|----------|--------|-------------|
| `RUST_LOG` | Bicep | Log level (info) |
| `DATABASE_URL` | Secret | PostgreSQL connection string |
| `JWT_SECRET` | Secret | JWT signing key |
| `JWT_EXPIRATION_HOURS` | Bicep | Token expiry (24h) |
| `JWT_ISSUER` | Bicep | JWT issuer (windspire) |
| `CORS_ALLOWED_ORIGINS` | Bicep | Allowed CORS origins |
| `FIREBASE_PROJECT_ID` | Bicep | Firebase project |
| `FIREBASE_CLIENT_EMAIL` | Bicep | Firebase service account |
| `FIREBASE_PRIVATE_KEY` | Secret | Firebase private key |

## Scaling Behavior

The Container App is configured with:

```yaml
Scale Configuration:
  Min Replicas: 0          # ← Scales to ZERO when no traffic!
  Max Replicas: 10
  Scale Rule: HTTP
    Concurrent Requests: 10 per replica
```

**What this means**:
- **No traffic** → 0 instances → **$0 compute cost**
- **1-10 concurrent requests** → 1 instance
- **11-20 concurrent requests** → 2 instances
- **91-100 concurrent requests** → 10 instances (max)

**Cold start**: ~2-5 seconds (first request after scaling to zero)

## Updating Your Backend

After making code changes:

### Quick Update (Recommended)
```bash
cd windspire_backend

# Build and push using Azure
az acr build \
  --registry <your-registry-name> \
  --image windspire-backend:latest \
  --platform linux/amd64 \
  .

# Container App auto-updates on next cold start
# Or force immediate update:
az containerapp update \
  --name windspire-api-dev \
  --resource-group windspire-dev-rg \
  --image <login-server>/windspire-backend:latest
```

### Manual Update
```bash
# Build locally
docker build -t windspire-backend:latest .

# Push
az acr login --name <registry-name>
docker tag windspire-backend:latest <login-server>/windspire-backend:latest
docker push <login-server>/windspire-backend:latest

# Update container app
az containerapp update \
  --name windspire-api-dev \
  --resource-group windspire-dev-rg \
  --image <login-server>/windspire-backend:latest
```

## Troubleshooting

### Container App won't start

```bash
# Check revision status
az containerapp revision list \
  --name windspire-api-dev \
  --resource-group windspire-dev-rg \
  --output table

# View detailed logs
az containerapp logs show \
  --name windspire-api-dev \
  --resource-group windspire-dev-rg \
  --follow

# Check container app health
az containerapp show \
  --name windspire-api-dev \
  --resource-group windspire-dev-rg \
  --query "properties.{status:runningStatus,health:healthState,replicas:template.scale.minReplicas}"
```

### Database connection issues

```bash
# Check PostgreSQL status
az postgres flexible-server show \
  --resource-group windspire-dev-rg \
  --name <server-name> \
  --query "{status:state,fqdn:fullyQualifiedDomainName}"

# Check firewall rules
az postgres flexible-server firewall-rule list \
  --resource-group windspire-dev-rg \
  --name <server-name> \
  --output table

# Test connection from within container
az containerapp exec \
  --name windspire-api-dev \
  --resource-group windspire-dev-rg \
  --command /bin/sh
```

### Image pull errors

```bash
# Verify image exists in registry
az acr repository show-tags \
  --name <registry-name> \
  --repository windspire-backend \
  --output table

# Check registry credentials
az acr credential show --name <registry-name>

# Verify container app has registry access
az containerapp show \
  --name windspire-api-dev \
  --resource-group windspire-dev-rg \
  --query "properties.configuration.registries"
```

### Cold start too slow

Cold starts are typically 2-5 seconds. To reduce:

1. Keep warm with external monitoring (ping every 5 minutes)
2. Increase `minReplicas` to 1 (but costs ~$2-5/month)
3. Optimize Docker image size (current: ~12MB)

```bash
# Set min replicas to 1 (always warm)
az containerapp update \
  --name windspire-api-dev \
  --resource-group windspire-dev-rg \
  --min-replicas 1
```

## Monitoring

### View Live Logs
```bash
az containerapp logs show \
  --name windspire-api-dev \
  --resource-group windspire-dev-rg \
  --follow
```

### Check Replica Count
```bash
az containerapp replica list \
  --name windspire-api-dev \
  --resource-group windspire-dev-rg \
  --output table
```

### View Metrics
```bash
# Get resource ID
RESOURCE_ID=$(az containerapp show \
  --name windspire-api-dev \
  --resource-group windspire-dev-rg \
  --query id \
  --output tsv)

# View request metrics
az monitor metrics list \
  --resource $RESOURCE_ID \
  --metric "Requests" \
  --interval PT1M
```

## Cleanup

To delete everything:

```bash
az group delete --name windspire-dev-rg --yes --no-wait
```

This removes:
- Container App
- Container Registry
- PostgreSQL Server
- Static Web App
- All associated resources

**Warning**: This is irreversible. Back up your database first!

## Comparison: Why Container Apps?

| Feature | Azure Functions (Flex) | Container Apps | Winner |
|---------|----------------------|----------------|--------|
| Custom Handlers | ❌ Not in Flex | ✅ Native Docker | 🏆 Container Apps |
| Scale to Zero | ⚠️ Only built-in runtimes | ✅ Yes | 🏆 Container Apps |
| Cold Start | ~1-3s | ~2-5s | Tie |
| Cost (idle) | Can't use Flex | $0 | 🏆 Container Apps |
| Complexity | Medium | Low | 🏆 Container Apps |
| HTTP/2 & gRPC | ✅ Yes | ✅ Yes | Tie |

## Next Steps

1. ✅ Deploy infrastructure
2. ✅ Push Docker image
3. ✅ Verify backend is running
4. 🔲 Set up GitHub Actions for CI/CD
5. 🔲 Configure custom domain
6. 🔲 Add Application Insights monitoring
7. 🔲 Set up staging environment
8. 🔲 Configure alerts and notifications

## GitHub Actions CI/CD

See `.github/workflows/deploy-container-app.yml` for automated deployments on push to `main`.

## Files in This Deployment

- **`main.bicep`** - Main infrastructure template (Container Apps, PostgreSQL, Static Web App, Container Registry)
- **`main.bicepparam`** - Parameters file with environment variable references
- **`deploy.sh`** - Automated deployment script
- **`SETUP.md`** - This file - comprehensive setup guide
- **`MIGRATION_SUMMARY.md`** - Summary of what changed from Function Apps to Container Apps
- **`CONTAINER_APPS.md`** - Additional Container Apps reference

## Support

- [Azure Container Apps Documentation](https://learn.microsoft.com/azure/container-apps/)
- [Azure Container Registry](https://learn.microsoft.com/azure/container-registry/)
- [PostgreSQL Flexible Server](https://learn.microsoft.com/azure/postgresql/flexible-server/)

---

**Need help?** Check the logs first:
```bash
az containerapp logs show --name windspire-api-dev --resource-group windspire-dev-rg --follow
```
