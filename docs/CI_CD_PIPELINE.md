# Windspire CI/CD Pipeline Documentation

This document provides a comprehensive overview of the GitHub Actions CI/CD pipeline for the Windspire application, designed for deployment to Azure Static Web Apps.

## 🏗️ Architecture Overview

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│                 │    │                  │    │                 │
│  Svelte Frontend│────│  Azure Static    │────│  Rust Backend   │
│  (CSR)          │    │  Web Apps        │    │  (Azure Funcs)  │
│                 │    │                  │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                              │
                              ▼
                       ┌──────────────────┐
                       │                  │
                       │  PostgreSQL      │
                       │  (Azure DB)      │
                       │                  │
                       └──────────────────┘
```

## 🚀 Workflow Overview

### 1. Infrastructure Deployment (`infrastructure.yml`)
**Triggers:** Push to main, PR, manual dispatch  
**Purpose:** Provision Azure resources using Bicep templates

- **Validates** Bicep templates on every PR
- **Deploys** infrastructure on main branch or manual trigger
- **Provisions:**
  - Azure Static Web Apps instance
  - PostgreSQL Flexible Server
  - Azure Key Vault for secrets
  - Required firewall rules and configurations
- **Outputs:** Deployment tokens and resource URLs
- **Updates:** GitHub secrets automatically

### 2. Backend CI/CD (`backend-ci-cd.yml`)
**Triggers:** Changes to `windspire_backend/**`  
**Purpose:** Test, lint, and validate Rust backend

- **Code Quality:**
  - Rust formatting check (`cargo fmt`)
  - Clippy linting with warnings as errors
  - Security audit (`cargo audit`)
- **Testing:**
  - Unit and integration tests with PostgreSQL
  - Code coverage reporting
  - Database migration validation
- **Performance:** Caches Rust dependencies for faster builds

### 3. Database Migrations (`database-migrations.yml`)
**Triggers:** Changes to migrations, manual dispatch  
**Purpose:** Safely manage database schema changes

- **Validation:**
  - Syntax checking of SQL migration files
  - Test migrations on fresh database
  - Rollback testing for safety
- **Deployment:**
  - Environment-specific migration runs
  - Database health verification
  - Backup creation before changes
- **Safety Features:**
  - Dry-run mode for testing
  - Destructive operation warnings
  - Migration status reporting

### 4. Full Application Deployment (`azure-static-web-apps-ci-cd.yml`)
**Triggers:** Push to main/feature branches  
**Purpose:** Build and deploy complete application

- **Frontend Build:**
  - Svelte application compilation
  - Static asset optimization
  - Environment-specific configuration
- **Backend Build:**
  - Rust compilation for Linux (musl target)
  - Azure Functions configuration generation
  - Binary optimization for serverless
- **Database Operations:**
  - SQLx migration execution
  - Connection validation
- **Deployment:**
  - Atomic deployment to Azure Static Web Apps
  - Environment variable injection
  - Preview deployments for PRs

## 📁 File Structure

```
.github/workflows/
├── azure-static-web-apps-ci-cd.yml  # Main deployment workflow
├── backend-ci-cd.yml                # Backend testing and validation
├── database-migrations.yml          # Database schema management
└── infrastructure.yml               # Azure resource provisioning

infrastructure/
├── main.bicep                       # Azure resource definitions
└── main.parameters.json             # Environment parameters

docs/
└── GITHUB_SECRETS_SETUP.md         # Secret configuration guide
```

## 🔐 Security & Secrets

### Required GitHub Secrets
- **Azure Authentication:** `AZURE_CLIENT_ID`, `AZURE_TENANT_ID`, `AZURE_SUBSCRIPTION_ID`
- **Database:** `DATABASE_URL`, `POSTGRES_ADMIN_LOGIN`, `POSTGRES_ADMIN_PASSWORD`
- **Firebase:** `FIREBASE_PROJECT_ID`, `FIREBASE_PRIVATE_KEY`, `FIREBASE_CLIENT_EMAIL`
- **Configuration:** `CORS_ALLOWED_ORIGINS`, `VITE_FIREBASE_CONFIG`

### Security Features
- All secrets stored in GitHub Secrets or Azure Key Vault
- Service principal with minimal required permissions
- Database connections use SSL/TLS encryption
- Environment-specific configurations
- Automated secret rotation capabilities

## 🌍 Environment Management

### Development (`dev`)
- Automatic deployment on feature branch pushes
- Preview deployments for PRs
- Debug logging enabled
- Relaxed CORS policies for local development

### Staging (`staging`)
- Manual deployment trigger
- Production-like configuration
- Performance testing environment
- Migration validation before production

### Production (`prod`)
- Manual deployment with approval
- Optimized builds and logging
- Strict security policies
- Automated backup procedures

## 🔄 Deployment Flow

1. **Code Push** → Triggers appropriate workflows based on changed files
2. **Infrastructure** → Validates/deploys Azure resources if needed
3. **Backend Testing** → Runs comprehensive test suite
4. **Database Migrations** → Applies schema changes safely
5. **Application Build** → Compiles frontend and backend
6. **Deployment** → Atomic deployment to Azure Static Web Apps
7. **Verification** → Health checks and smoke tests

## 📊 Monitoring & Observability

### Build Monitoring
- Workflow status badges in README
- Slack/Teams notifications on failures
- Code coverage trending
- Security vulnerability alerts

### Application Monitoring
- Azure Application Insights integration
- Database performance metrics
- API response time tracking
- Error rate monitoring

## 🚨 Troubleshooting

### Common Issues

**Build Failures:**
```bash
# Check Rust compilation issues
cargo check --verbose

# Validate database connectivity
sqlx migrate info --database-url $DATABASE_URL
```

**Deployment Issues:**
```bash
# Verify Azure credentials
az account show

# Check Static Web App status
az staticwebapp show --name <app-name>
```

**Database Issues:**
```bash
# Test connection
psql $DATABASE_URL -c "SELECT version();"

# Check migration status
sqlx migrate info
```

### Debug Commands
```bash
# Re-run specific workflow
gh workflow run infrastructure.yml

# Check workflow logs
gh run list --workflow=backend-ci-cd.yml

# View secrets (names only)
gh secret list
```

## 🔧 Local Development

### Prerequisites
- Rust toolchain
- Node.js 20+ with pnpm
- PostgreSQL 15+
- Azure CLI
- GitHub CLI

### Setup
```bash
# Clone repository
git clone https://github.com/hydrant/windspire
cd windspire

# Backend setup
cd windspire_backend
cp .env.example .env
sqlx database create
sqlx migrate run
cargo run

# Frontend setup (new terminal)
cd windspire_frontend_svelte
pnpm install
pnpm dev
```

## 📈 Performance Optimizations

### Build Performance
- Rust dependency caching
- Node.js module caching
- Incremental builds
- Parallel job execution

### Runtime Performance
- Rust binary optimization for size
- Static asset compression
- Database connection pooling
- CDN integration for static files

### Cost Optimization
- Azure Free Tier usage where possible
- Serverless scaling for backend
- Efficient database sizing
- Resource cleanup on PR close

## 🔄 Maintenance

### Regular Tasks
- **Weekly:** Security audit review
- **Monthly:** Dependency updates
- **Quarterly:** Performance review and optimization
- **Annually:** Infrastructure cost review

### Automated Maintenance
- Dependabot for dependency updates
- Scheduled security scans
- Database backup verification
- Log rotation and cleanup

## 📚 Additional Resources

- [Azure Static Web Apps Documentation](https://docs.microsoft.com/en-us/azure/static-web-apps/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Bicep Template Reference](https://docs.microsoft.com/en-us/azure/azure-resource-manager/bicep/)
- [SQLx Migration Guide](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)

---

*This pipeline is designed for production use with proper error handling, security measures, and monitoring. For questions or issues, please open a GitHub issue or contact the development team.*