# Azure Container Apps Deployment Guide

This infrastructure uses **Azure Container Apps** which:
- ✅ **Scales to zero** (unlike Azure Functions plans)
- ✅ Supports **custom Docker containers**
- ✅ Much simpler than Azure Functions custom handlers
- ✅ Pay only for what you use

## Quick Start

### 1. Build and push the Docker image

```bash
# Login to Azure
az login

# Create resource group
az group create --name rg-windspire-dev --location eastus2

# Build and push (Azure will create the registry during deployment)
cd windspire_backend
docker build -t windspire-backend:latest .
```

### 2. Deploy infrastructure

```bash
cd ../infrastructure
az deployment group create \
  --resource-group rg-windspire-dev \
  --template-file main.bicep \
  --parameters main.bicepparam
```

### 3. Push image to Azure Container Registry

```bash
# Get registry details from deployment output
REGISTRY_NAME=$(az deployment group show -g rg-windspire-dev -n main --query properties.outputs.containerRegistryName.value -o tsv)
REGISTRY_SERVER=$(az deployment group show -g rg-windspire-dev -n main --query properties.outputs.containerRegistryLoginServer.value -o tsv)

# Login to registry
az acr login --name $REGISTRY_NAME

# Tag and push image
docker tag windspire-backend:latest $REGISTRY_SERVER/windspire-backend:latest
docker push $REGISTRY_SERVER/windspire-backend:latest
```

### 4. Update Container App to use the image

```bash
# The deployment will update automatically when you push a new image
# Or manually trigger an update:
az containerapp update \
  --name windspire-api-dev \
  --resource-group rg-windspire-dev \
  --image $REGISTRY_SERVER/windspire-backend:latest
```

## Architecture

```
Azure Container Apps
  ├── Scale to zero when idle (true serverless!)
  ├── Auto-scale based on HTTP requests
  └── Custom Docker container (Rust backend)
      
PostgreSQL Flexible Server
  └── Database with SSL

Azure Container Registry
  └── Private Docker registry

Static Web App
  └── Svelte frontend
```

## Cost Estimate

- Container Apps: **~$0-5/month** (scales to zero + free tier)
- PostgreSQL B1ms: **~$15/month**
- Container Registry Basic: **~$5/month**
- Static Web App Free: **$0**

**Total: ~$20-25/month for dev environment**

Much better than the B1 App Service Plan that runs 24/7!
