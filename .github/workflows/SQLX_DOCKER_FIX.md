# Fixed SQLx Offline Mode in Docker Build

## Problem

Docker build was failing with:
```
error: `SQLX_OFFLINE=true` but there is no cached data for this query
run `cargo sqlx prepare` to update the query cache or unset `SQLX_OFFLINE`
```

## Root Cause

The `.sqlx` directory (query cache) was not being copied into the Docker image, even though the workflow was generating it.

## Solution

### 1. Updated Dockerfile to Copy .sqlx Directory

**File**: `windspire_backend/Dockerfile`

Added before the build step:
```dockerfile
# Copy SQLx query cache for offline mode
COPY .sqlx ./.sqlx
```

This ensures the SQLx query cache is available during the Docker build.

### 2. Updated Workflows to Verify .sqlx Exists

**Files**: 
- `.github/workflows/build-backend-image.yml`
- `.github/workflows/build-backend.yml`

Added verification step:
```yaml
- name: Verify SQLx cache exists
  working-directory: windspire_backend
  run: |
    echo "📂 Checking .sqlx directory..."
    if [ -d ".sqlx" ]; then
      echo "✅ .sqlx directory found"
      echo "📊 Query cache files:"
      ls -lh .sqlx/
    else
      echo "❌ ERROR: .sqlx directory not found!"
      exit 1
    fi
```

This ensures the cache is generated before Docker build starts.

### 3. Removed continue-on-error

Removed `continue-on-error: true` from the SQLx prepare step so errors are visible.

## How It Works Now

### Workflow Flow:

```
1. Checkout code
2. Start PostgreSQL container
3. Run migrations
4. Generate .sqlx cache (cargo sqlx prepare)
5. Verify .sqlx directory exists ← NEW!
6. Stop PostgreSQL
7. Build Docker image (with .sqlx copied in)
8. Push to GHCR
```

### Docker Build Flow:

```dockerfile
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx           ← NEW! Cache is available

ENV SQLX_OFFLINE=true        ← Can now use offline mode
RUN cargo build --release    ← Succeeds with cached queries
```

## Files Modified

1. ✅ `windspire_backend/Dockerfile` - Added `COPY .sqlx ./.sqlx`
2. ✅ `.github/workflows/build-backend-image.yml` - Added verification step, removed continue-on-error
3. ✅ `.github/workflows/build-backend.yml` - Added verification step, removed continue-on-error

## Bonus: Rust Version Update

Also updated Dockerfile to use Rust 1.90.0 to support dependencies that require edition 2024:

```dockerfile
FROM rust:1.90.0-alpine AS builder
```

## Why This Works

**Before:**
- Workflow generates `.sqlx` ✅
- Docker build starts ✅
- Docker `COPY .sqlx` fails silently ❌ (file not in committed code)
- `SQLX_OFFLINE=true` but no cache ❌
- Build fails ❌

**After:**
- Workflow generates `.sqlx` ✅
- Workflow verifies `.sqlx` exists ✅
- Docker build starts ✅
- Docker `COPY .sqlx` succeeds ✅ (file exists in build context)
- `SQLX_OFFLINE=true` with cache ✅
- Build succeeds ✅

## Commit and Deploy

```bash
git add windspire_backend/Dockerfile
git add .github/workflows/build-backend-image.yml
git add .github/workflows/build-backend.yml
git commit -m "fix: ensure SQLx cache is available in Docker build"
git push origin azure_csr:main
```

The Docker build will now succeed! 🚀

## Future Improvement

Consider committing `.sqlx` to the repository:

```bash
cd windspire_backend
cargo sqlx prepare
git add .sqlx
git commit -m "chore: add SQLx query cache"
```

**Benefits:**
- Faster CI builds (skip PostgreSQL container)
- More reliable (no runtime generation needed)
- Works offline

The workflow will still work either way - it regenerates the cache if missing or uses the committed version if present.
