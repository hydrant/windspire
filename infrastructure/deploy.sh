#!/bin/bash
# Windspire Container Apps Deployment Script
set -e

echo "🚀 Windspire Azure Container Apps Deployment"
echo "=============================================="
echo ""

# Check required tools
command -v az >/dev/null 2>&1 || { echo "❌ Azure CLI is not installed"; exit 1; }
command -v docker >/dev/null 2>&1 || { echo "❌ Docker is not installed"; exit 1; }

# Variables
RESOURCE_GROUP="windspire-dev-rg"
LOCATION="norwayeast"
BICEP_FILE="infrastructure/main.bicep"
PARAMS_FILE="infrastructure/main.bicepparam"

# Check if logged in
echo "📋 Checking Azure login status..."
az account show >/dev/null 2>&1 || { echo "❌ Not logged in to Azure. Run: az login"; exit 1; }

# Create resource group
echo "📦 Creating resource group: $RESOURCE_GROUP in $LOCATION..."
az group create --name "$RESOURCE_GROUP" --location "$LOCATION"

# Deploy infrastructure
echo "🏗️  Deploying infrastructure (this may take 5-10 minutes)..."
az deployment group create \
  --resource-group "$RESOURCE_GROUP" \
  --template-file "$BICEP_FILE" \
  --parameters "$PARAMS_FILE" \
  --query "properties.outputs" \
  --output table

# Get outputs
echo "📊 Retrieving deployment outputs..."
CONTAINER_REGISTRY=$(az deployment group show \
  --resource-group "$RESOURCE_GROUP" \
  --name "main" \
  --query "properties.outputs.containerRegistryName.value" \
  --output tsv)

ACR_LOGIN_SERVER=$(az deployment group show \
  --resource-group "$RESOURCE_GROUP" \
  --name "main" \
  --query "properties.outputs.containerRegistryLoginServer.value" \
  --output tsv)

CONTAINER_APP_URL=$(az deployment group show \
  --resource-group "$RESOURCE_GROUP" \
  --name "main" \
  --query "properties.outputs.containerAppUrl.value" \
  --output tsv)

echo ""
echo "✅ Infrastructure deployed successfully!"
echo ""
echo "📝 Next steps:"
echo "1. Build and push Docker image:"
echo "   cd windspire_backend"
echo "   az acr build --registry $CONTAINER_REGISTRY --image windspire-backend:latest --platform linux/amd64 ."
echo ""
echo "2. Update Container App with new image (if needed):"
echo "   az containerapp update --name windspire-api-dev --resource-group $RESOURCE_GROUP --image $ACR_LOGIN_SERVER/windspire-backend:latest"
echo ""
echo "3. Your backend will be available at:"
echo "   $CONTAINER_APP_URL"
echo ""
echo "4. View logs:"
echo "   az containerapp logs show --name windspire-api-dev --resource-group $RESOURCE_GROUP --follow"
echo ""
