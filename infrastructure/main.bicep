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
var functionAppName = '${appName}-api-${environment}-${take(uniqueSuffix, 8)}'

// App Service Plan for Function App - Basic B1 (supports custom handlers on Linux)
module appServicePlan 'br/public:avm/res/web/serverfarm:0.5.0' = {
  name: 'appServicePlan'
  params: {
    name: '${appName}-plan-${environment}-${take(uniqueSuffix, 8)}'
    location: location
    skuName: 'B1' // Basic B1 plan - supports Linux and custom handlers
    reserved: true // Linux
  }
}

// Storage Account for Function App
module storageAccount 'br/public:avm/res/storage/storage-account:0.27.1' = {
  name: 'storageAccount'
  params: {
    name: 'st${toLower(take(uniqueSuffix, 12))}${toLower(environment)}'
    location: location
    skuName: 'Standard_LRS'
    publicNetworkAccess: 'Enabled'
    networkAcls: {
      defaultAction: 'Allow' // Allow all access for now, can be restricted later
      bypass: 'AzureServices' // Allow Azure services to bypass network rules
    }
    blobServices: {
      containers: [
        {
          name: 'deployments'
          publicAccess: 'None'
        }
      ]
    }
  }
}

// Application Insights for Function App monitoring
module applicationInsights 'br/public:avm/res/insights/component:0.4.2' = {
  name: 'applicationInsights'
  params: {
    name: '${appName}-insights-${environment}-${take(uniqueSuffix, 8)}'
    location: location
    workspaceResourceId: logAnalyticsWorkspace.outputs.resourceId
  }
}

// Log Analytics Workspace for Application Insights
module logAnalyticsWorkspace 'br/public:avm/res/operational-insights/workspace:0.9.0' = {
  name: 'logAnalyticsWorkspace'
  params: {
    name: '${appName}-logs-${environment}-${take(uniqueSuffix, 8)}'
    location: location
  }
}

// Function App (AVM) - Basic B1 with custom handler support
module functionApp 'br/public:avm/res/web/site:0.19.3' = {
  name: 'functionApp'
  params: {
    name: functionAppName
    location: location
    kind: 'functionapp,linux'
    serverFarmResourceId: appServicePlan.outputs.resourceId
    managedIdentities: {
      systemAssigned: true
    }
    siteConfig: {
      alwaysOn: false // Set to false to test cold start times. NOTE: You're paying for B1 24/7 regardless - setting to true has no extra cost and eliminates cold starts
      linuxFxVersion: 'DOCKER|mcr.microsoft.com/azure-functions/node:4-node18' // Base image for custom handler
      appSettings: [
        {
          name: 'AzureWebJobsStorage'
          value: 'DefaultEndpointsProtocol=https;AccountName=${storageAccount.outputs.name};AccountKey=${storageAccount.outputs.primaryAccessKey};EndpointSuffix=${az.environment().suffixes.storage}'
        }
        {
          name: 'WEBSITE_CONTENTAZUREFILECONNECTIONSTRING'
          value: 'DefaultEndpointsProtocol=https;AccountName=${storageAccount.outputs.name};AccountKey=${storageAccount.outputs.primaryAccessKey};EndpointSuffix=${az.environment().suffixes.storage}'
        }
        { name: 'WEBSITE_CONTENTSHARE', value: toLower(functionAppName) }
        { name: 'FUNCTIONS_EXTENSION_VERSION', value: '~4' }
        { name: 'FUNCTIONS_WORKER_RUNTIME', value: 'custom' }
        { name: 'APPLICATIONINSIGHTS_CONNECTION_STRING', value: applicationInsights.outputs.connectionString }
        { name: 'APPINSIGHTS_INSTRUMENTATIONKEY', value: applicationInsights.outputs.instrumentationKey }
        { name: 'FIREBASE_PROJECT_ID', value: firebaseProjectId }
        { name: 'FIREBASE_CLIENT_EMAIL', value: firebaseClientEmail }
        { name: 'FIREBASE_PRIVATE_KEY', value: firebasePrivateKey }
        {
          name: 'DATABASE_URL'
          value: 'postgresql://${postgresAdminLogin}:${postgresAdminPassword}@${postgresServer.outputs.fqdn}:5432/windspire?sslmode=require&sslcert=&sslkey=&sslrootcert='
        }
        { name: 'RUST_LOG', value: 'info' }
        { name: 'JWT_SECRET', value: jwtSecret }
        { name: 'JWT_EXPIRATION_HOURS', value: jwtExpirationHours }
        { name: 'JWT_ISSUER', value: jwtIssuer }
        { name: 'CORS_ALLOWED_ORIGINS', value: '${corsAllowedOrigins},https://${staticWebApp.outputs.defaultHostname}' }
      ]
      cors: {
        allowedOrigins: [
          '*' // Temporary - will be configured properly after deployment
        ]
        supportCredentials: false
      }
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

// Raw role assignment resource to ensure we get the correct managed identity principal
resource functionAppStorageRoleAssignment 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(subscription().id, resourceGroup().id, functionAppName, 'storage-rbac')
  properties: {
    principalId: functionApp.outputs.systemAssignedMIPrincipalId!
    roleDefinitionId: subscriptionResourceId(
      'Microsoft.Authorization/roleDefinitions',
      'ba92f5b4-2d11-453d-a403-e96b0029c9fe'
    ) // Storage Blob Data Owner
    principalType: 'ServicePrincipal'
  }
}

// Add Function App output
output functionAppName string = functionApp.outputs.name

// Outputs
output staticWebAppUrl string = 'https://${staticWebApp.outputs.defaultHostname}'
output staticWebAppName string = staticWebApp.outputs.name
output postgresServerName string = postgresServer.outputs.name
output keyVaultName string = keyVault.outputs.name
output applicationInsightsName string = applicationInsights.outputs.name
output applicationInsightsInstrumentationKey string = applicationInsights.outputs.instrumentationKey
