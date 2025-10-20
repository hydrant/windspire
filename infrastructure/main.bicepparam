using './main.bicep'

param appName = 'windspire'
param environment = 'dev'
param location = 'eastus2'

// PostgreSQL Configuration
param postgresAdminLogin = readEnvironmentVariable('POSTGRES_ADMIN_LOGIN', 'postgres')
param postgresAdminPassword = readEnvironmentVariable('POSTGRES_ADMIN_PASSWORD')
param postgresSku = 'Standard_B1ms'
param postgresVersion = '16'

// Firebase Configuration
param firebaseProjectId = readEnvironmentVariable('FIREBASE_PROJECT_ID')
param firebasePrivateKey = readEnvironmentVariable('FIREBASE_PRIVATE_KEY')
param firebaseClientEmail = readEnvironmentVariable('FIREBASE_CLIENT_EMAIL')

// JWT Configuration
param jwtSecret = readEnvironmentVariable('JWT_SECRET')
param jwtExpirationHours = '24'
param jwtIssuer = 'windspire'

// CORS Configuration
param corsAllowedOrigins = 'http://localhost:3000,http://localhost:5173'
