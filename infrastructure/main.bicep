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

@description('Firebase project configuration')
@secure()
param firebaseConfig object

// Variables
var uniqueSuffix = uniqueString(resourceGroup().id)
var staticWebAppName = '${appName}-${environment}-${uniqueSuffix}'
var postgresServerName = '${appName}-postgres-${environment}-${uniqueSuffix}'
var keyVaultName = '${appName}-kv-${environment}-${take(uniqueSuffix, 8)}'

// Static Web App
resource staticWebApp 'Microsoft.Web/staticSites@2023-01-01' = {
  name: staticWebAppName
  location: location
  sku: {
    name: 'Free'
    tier: 'Free'
  }
  properties: {
    repositoryUrl: 'https://github.com/hydrant/windspire'
    branch: 'main'
    buildProperties: {
      appLocation: 'windspire_frontend_svelte/build'
      apiLocation: 'windspire_backend/api'
      outputLocation: ''
    }
  }
}

// PostgreSQL Flexible Server
resource postgresServer 'Microsoft.DBforPostgreSQL/flexibleServers@2023-06-01-preview' = {
  name: postgresServerName
  location: location
  sku: {
    name: postgresSku
    tier: 'Burstable'
  }
  properties: {
    version: postgresVersion
    administratorLogin: postgresAdminLogin
    administratorLoginPassword: postgresAdminPassword
    storage: {
      storageSizeGB: 32
      autoGrow: 'Enabled'
    }
    backup: {
      backupRetentionDays: 7
      geoRedundantBackup: 'Disabled'
    }
    highAvailability: {
      mode: 'Disabled'
    }
    maintenanceWindow: {
      customWindow: 'Disabled'
    }
  }
}

// PostgreSQL Database
resource postgresDatabase 'Microsoft.DBforPostgreSQL/flexibleServers/databases@2023-06-01-preview' = {
  parent: postgresServer
  name: 'windspire'
  properties: {
    charset: 'UTF8'
    collation: 'en_US.UTF8'
  }
}

// PostgreSQL Firewall Rule for Azure Services
resource postgresFirewallAzure 'Microsoft.DBforPostgreSQL/flexibleServers/firewallRules@2023-06-01-preview' = {
  parent: postgresServer
  name: 'AllowAzureServices'
  properties: {
    startIpAddress: '0.0.0.0'
    endIpAddress: '0.0.0.0'
  }
}

// Key Vault for storing secrets
resource keyVault 'Microsoft.KeyVault/vaults@2023-07-01' = {
  name: keyVaultName
  location: location
  properties: {
    sku: {
      family: 'A'
      name: 'standard'
    }
    tenantId: tenant().tenantId
    accessPolicies: [
      {
        tenantId: tenant().tenantId
        objectId: staticWebApp.identity.principalId
        permissions: {
          secrets: [
            'get'
            'list'
          ]
        }
      }
    ]
    enabledForTemplateDeployment: true
    enableSoftDelete: true
    softDeleteRetentionInDays: 7
  }
}

// Store database connection string in Key Vault
resource databaseConnectionSecret 'Microsoft.KeyVault/vaults/secrets@2023-07-01' = {
  parent: keyVault
  name: 'database-url'
  properties: {
    value: 'postgresql://${postgresAdminLogin}:${postgresAdminPassword}@${postgresServer.properties.fullyQualifiedDomainName}:5432/windspire?sslmode=require'
  }
}

// Store Firebase configuration in Key Vault
resource firebaseProjectIdSecret 'Microsoft.KeyVault/vaults/secrets@2023-07-01' = {
  parent: keyVault
  name: 'firebase-project-id'
  properties: {
    value: firebaseConfig.projectId
  }
}

resource firebasePrivateKeySecret 'Microsoft.KeyVault/vaults/secrets@2023-07-01' = {
  parent: keyVault
  name: 'firebase-private-key'
  properties: {
    value: firebaseConfig.privateKey
  }
}

resource firebaseClientEmailSecret 'Microsoft.KeyVault/vaults/secrets@2023-07-01' = {
  parent: keyVault
  name: 'firebase-client-email'
  properties: {
    value: firebaseConfig.clientEmail
  }
}

// Static Web App configuration
resource staticWebAppConfig 'Microsoft.Web/staticSites/config@2023-01-01' = {
  parent: staticWebApp
  name: 'appsettings'
  properties: {
    DATABASE_URL: '@Microsoft.KeyVault(VaultName=${keyVaultName};SecretName=database-url)'
    FIREBASE_PROJECT_ID: '@Microsoft.KeyVault(VaultName=${keyVaultName};SecretName=firebase-project-id)'
    FIREBASE_PRIVATE_KEY: '@Microsoft.KeyVault(VaultName=${keyVaultName};SecretName=firebase-private-key)'
    FIREBASE_CLIENT_EMAIL: '@Microsoft.KeyVault(VaultName=${keyVaultName};SecretName=firebase-client-email)'
    CORS_ALLOWED_ORIGINS: 'https://${staticWebApp.properties.defaultHostname}'
    RUST_LOG: environment == 'prod' ? 'info' : 'debug'
  }
}

// Outputs
output staticWebAppUrl string = 'https://${staticWebApp.properties.defaultHostname}'
output staticWebAppName string = staticWebApp.name
output postgresServerName string = postgresServer.name
output keyVaultName string = keyVault.name
output deploymentToken string = staticWebApp.listSecrets().properties.apiKey