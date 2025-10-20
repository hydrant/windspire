@description('The name of the application')
param appName string = 'windspire'

@description('The environment (dev, staging, prod)')
param environment string = 'dev'

@description('The location for all resources')
param location string = resourceGroup().location

@description('PostgreSQL server administrator login')
@secure()
param postgresAdminLogin string

@description('PostgreSQL server administrator password')
@secure()
param postgresAdminPassword string

@description('The SKU name for PostgreSQL server')
param postgresSku string = 'Standard_B1ms'

@description('PostgreSQL server version')
param postgresVersion string = '15'

@description('Firebase project ID')
param firebaseProjectId string

@description('Firebase private key')
@secure()
param firebasePrivateKey string

@description('Firebase client email')
param firebaseClientEmail string

@description('JWT secret for authentication')
@secure()
param jwtSecret string

@description('JWT token expiration in hours')
param jwtExpirationHours string = '24'

@description('JWT issuer')
param jwtIssuer string = 'windspire'

@description('CORS allowed origins (comma-separated)')
param corsAllowedOrigins string = 'http://localhost:3000,http://localhost:5173'

// Variables
var uniqueSuffix = uniqueString(resourceGroup().id)
var staticWebAppName = '${appName}-${environment}-${uniqueSuffix}'
var postgresServerName = 'ws-pg-${environment}-${take(uniqueSuffix, 12)}'
var keyVaultName = 'ws-kv-${take(uniqueSuffix, 14)}'
var containerAppName = '${appName}-api-${environment}'
var containerRegistryName = 'cr${toLower(take(uniqueSuffix, 12))}${toLower(environment)}'
var logAnalyticsName = '${appName}-logs-${environment}-${take(uniqueSuffix, 8)}'
var containerAppEnvName = '${appName}-env-${environment}'

// Log Analytics Workspace (using AVM module)
module logAnalyticsWorkspace 'br/public:avm/res/operational-insights/workspace:0.12.0' = {
  name: 'logAnalyticsWorkspace'
  params: {
    name: logAnalyticsName
    location: location
  }
}

// Azure Container Registry (using AVM module)
module containerRegistry 'br/public:avm/res/container-registry/registry:0.9.3' = {
  name: 'containerRegistry'
  params: {
    name: containerRegistryName
    location: location
    acrSku: 'Basic'
    acrAdminUserEnabled: true
  }
}

// Reference to ACR for listCredentials
resource acrReference 'Microsoft.ContainerRegistry/registries@2023-07-01' existing = {
  name: containerRegistryName
  dependsOn: [containerRegistry]
}

// Container Apps Environment (using AVM module)
module containerAppEnv 'br/public:avm/res/app/managed-environment:0.11.3' = {
  name: 'containerAppEnv'
  params: {
    name: containerAppEnvName
    location: location
    zoneRedundant: false // Zone redundancy requires VNET configuration
    workloadProfiles: [
      {
        workloadProfileType: 'Consumption'
        name: 'Consumption'
      }
    ]
  }
}

// Container App for Rust Backend (using AVM module)
module containerApp 'br/public:avm/res/app/container-app:0.19.0' = {
  name: 'containerApp'
  params: {
    name: containerAppName
    location: location
    environmentResourceId: containerAppEnv.outputs.resourceId
    workloadProfileName: 'Consumption'
    ingressExternal: true
    ingressTargetPort: 8080
    ingressTransport: 'http'
    corsPolicy: {
      allowedOrigins: split('${corsAllowedOrigins},https://${staticWebApp.outputs.defaultHostname}', ',')
      allowedMethods: ['GET', 'POST', 'PUT', 'DELETE', 'PATCH', 'OPTIONS']
      allowedHeaders: ['*']
      allowCredentials: false
    }
    registries: [
      {
        server: containerRegistry.outputs.loginServer
        username: containerRegistry.outputs.name
        passwordSecretRef: 'registry-password'
      }
    ]
    secrets: [
      {
        name: 'registry-password'
        value: acrReference.listCredentials().passwords[0].value
      }
      {
        name: 'database-url'
        value: 'postgresql://${postgresAdminLogin}:${postgresAdminPassword}@${postgresServer.outputs.fqdn}:5432/windspire?sslmode=require'
      }
      {
        name: 'jwt-secret'
        value: jwtSecret
      }
      {
        name: 'firebase-private-key'
        value: firebasePrivateKey
      }
    ]
    containers: [
      {
        name: 'windspire-backend'
        // Placeholder image - will be updated after first push
        image: '${containerRegistry.outputs.loginServer}/windspire-backend:latest'
        resources: {
          cpu: json('0.5')
          memory: '1Gi'
        }
        env: [
          {
            name: 'RUST_LOG'
            value: 'info'
          }
          {
            name: 'DATABASE_URL'
            secretRef: 'database-url'
          }
          {
            name: 'JWT_SECRET'
            secretRef: 'jwt-secret'
          }
          {
            name: 'JWT_EXPIRATION_HOURS'
            value: jwtExpirationHours
          }
          {
            name: 'JWT_ISSUER'
            value: jwtIssuer
          }
          {
            name: 'CORS_ALLOWED_ORIGINS'
            value: '${corsAllowedOrigins},https://${staticWebApp.outputs.defaultHostname}'
          }
          {
            name: 'FIREBASE_PROJECT_ID'
            value: firebaseProjectId
          }
          {
            name: 'FIREBASE_CLIENT_EMAIL'
            value: firebaseClientEmail
          }
          {
            name: 'FIREBASE_PRIVATE_KEY'
            secretRef: 'firebase-private-key'
          }
        ]
      }
    ]
    scaleSettings: {
      minReplicas: 0 // SCALE TO ZERO! 🎉
      maxReplicas: 10
      rules: [
        {
          name: 'http-scaling'
          http: {
            metadata: {
              concurrentRequests: '10'
            }
          }
        }
      ]
    }
  }
}

// Static Web App (AVM)
module staticWebApp 'br/public:avm/res/web/static-site:0.9.3' = {
  name: 'staticWebApp'
  params: {
    name: staticWebAppName
    location: location
    sku: 'Standard'
    managedIdentities: {
      systemAssigned: true
    }
    // Remove Key Vault references to break circular dependency
    // These will be configured later via app settings update
  }
}

// PostgreSQL Flexible Server (AVM)
module postgresServer 'br/public:avm/res/db-for-postgre-sql/flexible-server:0.13.2' = {
  name: 'postgresServer'
  params: {
    name: postgresServerName
    location: location
    skuName: postgresSku
    tier: 'Burstable'
    availabilityZone: -1 // Let Azure choose the best availability zone
    version: postgresVersion
    administratorLogin: postgresAdminLogin
    administratorLoginPassword: postgresAdminPassword
    storageSizeGB: 32
    autoGrow: 'Enabled'
    backupRetentionDays: 7
    geoRedundantBackup: 'Disabled'
    highAvailability: 'Disabled'
    // Enable public network access for initial setup
    publicNetworkAccess: 'Enabled'
    databases: [
      {
        name: 'windspire'
        charset: 'UTF8'
        collation: 'en_US.UTF8'
      }
    ]
    firewallRules: [
      {
        name: 'AllowAzureServices'
        startIpAddress: '0.0.0.0'
        endIpAddress: '0.0.0.0'
      }
    ]
  }
}

// Key Vault for storing secrets (AVM)
module keyVault 'br/public:avm/res/key-vault/vault:0.13.3' = {
  name: 'keyVault'
  params: {
    name: keyVaultName
    location: location
    sku: 'standard'
    enableVaultForTemplateDeployment: true
    enablePurgeProtection: false
    secrets: [
      {
        name: 'database-url'
        value: 'postgresql://${postgresAdminLogin}:${postgresAdminPassword}@${postgresServer.outputs.fqdn}:5432/windspire?sslmode=require'
      }
      {
        name: 'firebase-project-id'
        value: firebaseProjectId
      }
      {
        name: 'firebase-private-key'
        value: firebasePrivateKey
      }
      {
        name: 'firebase-client-email'
        value: firebaseClientEmail
      }
    ]
  }
}

// Outputs
output containerAppName string = containerApp.outputs.name
output containerAppUrl string = 'https://${containerApp.outputs.fqdn}'
output containerRegistryLoginServer string = containerRegistry.outputs.loginServer
output containerRegistryName string = containerRegistry.outputs.name
output postgresServerName string = postgresServer.outputs.name
output postgresServerFqdn string = postgresServer.outputs.fqdn
output staticWebAppUrl string = 'https://${staticWebApp.outputs.defaultHostname}'
output staticWebAppName string = staticWebApp.outputs.name
output keyVaultName string = keyVault.outputs.name
