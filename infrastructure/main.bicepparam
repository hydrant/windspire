using './main.bicep'

// Basic infrastructure parameters
param appName = 'windspire'
param location = 'eastus2'
param postgresSku = 'Standard_B1ms'
param postgresVersion = '16'

// These will be overridden by GitHub Actions with actual secret values
param postgresAdminLogin = 'placeholder'
param postgresAdminPassword = 'placeholder'
param firebaseProjectId = 'placeholder'
param firebasePrivateKey = 'placeholder'
param firebaseClientEmail = 'placeholder'
param jwtSecret = 'placeholder'
param jwtExpirationHours = '24'
param jwtIssuer = 'windspire'
param corsAllowedOrigins = 'http://localhost:3000,http://localhost:5173'
param environment = 'dev'
