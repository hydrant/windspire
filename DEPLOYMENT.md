# Windspire Deployment Guide

## Overview
Windspire uses GitHub Actions for automated deployment to Azure. The infrastructure is managed through Bicep templates and deployed first, followed by backend and frontend applications.

## Deployment Pipeline

### 1. Infrastructure Deployment
- **Workflow**: `.github/workflows/build-infrastructure.yml`
- **Trigger**: Tags (`dev*`, `staging*`, `prod*`) or manual dispatch
- **Purpose**: Deploy Azure resources (Static Web Apps, PostgreSQL, Key Vault)
- **Dependencies**: None (foundation pipeline)

### 2. Backend Deployment
- **Workflow**: `.github/workflows/build-backend.yml`
- **Trigger**: After infrastructure deployment
- **Purpose**: Build and deploy Rust backend to Azure Container Apps
- **Dependencies**: Infrastructure deployment

### 3. Frontend Deployment
- **Workflow**: `.github/workflows/build-frontend.yml`
- **Trigger**: After infrastructure deployment
- **Purpose**: Build and deploy Svelte frontend to Azure Static Web Apps
- **Dependencies**: Infrastructure deployment

## Required GitHub Secrets (dev environment)

### Azure Authentication
- `AZURE_CLIENT_ID`: Service Principal Client ID
- `AZURE_CLIENT_SECRET`: Service Principal Secret
- `AZURE_SUBSCRIPTION_ID`: Azure Subscription ID
- `AZURE_TENANT_ID`: Azure Tenant ID
- `AZURE_RESOURCE_GROUP`: Resource group name for deployments

### Database Configuration
- `POSTGRES_ADMIN_LOGIN`: PostgreSQL admin username
- `POSTGRES_ADMIN_PASSWORD`: PostgreSQL admin password

### Firebase Authentication
- `FIREBASE_PROJECT_ID`: Firebase project identifier
- `FIREBASE_PRIVATE_KEY`: Firebase service account private key (with \n line breaks)
- `FIREBASE_CLIENT_EMAIL`: Firebase service account email

## Quick Deploy

To deploy the application:

1. **Tag for deployment**:
   ```bash
   git tag dev-v1.0.0
   git push origin dev-v1.0.0
   ```

2. **Or use manual dispatch**:
   - Go to GitHub Actions
   - Select "Infrastructure Build" workflow
   - Click "Run workflow"
   - Choose environment (dev/staging/prod)

## Infrastructure Resources

The Bicep template creates:
- Azure Static Web Apps (frontend hosting)
- PostgreSQL Flexible Server (database)
- Azure Key Vault (secrets management)
- Container Apps Environment (backend hosting)

## Notes

- All deployment parameters are generated dynamically in GitHub Actions
- Firebase private key formatting is handled automatically
- GitHub secrets are updated after successful infrastructure deployment
- Each environment (dev/staging/prod) uses separate Azure resources