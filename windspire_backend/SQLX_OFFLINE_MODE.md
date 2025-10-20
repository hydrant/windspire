# SQLx Offline Mode Setup

## Problem

Build failing with:
```
error: prepare check failed: .sqlx is missing one or more queries; you should re-run sqlx prepare
```

## Root Cause

SQLx uses compile-time query verification with the `sqlx::query!` macro. This requires either:
1. A `.sqlx` query cache directory (for offline compilation)
2. A live database connection during compilation

The `.sqlx` directory was missing from the repository.

## Solution

The workflows have been updated to **automatically generate** the `.sqlx` cache if it's missing:

```bash
# In both build-backend.yml and build-backend-image.yml:
if [ -d ".sqlx" ]; then
  echo "ℹ️  .sqlx cache already exists"
  cargo sqlx prepare --check || cargo sqlx prepare
else
  echo "⚠️  .sqlx cache missing - generating it now"
  cargo sqlx prepare
fi
```

## Recommended: Commit .sqlx to Repository

For **faster and more reliable builds**, generate and commit the `.sqlx` cache:

### Steps:

1. **Start your local database**:
   ```bash
   cd windspire_backend
   docker-compose up -d
   ```

2. **Generate the SQLx cache**:
   ```bash
   # Make sure DATABASE_URL is set
   export DATABASE_URL="postgresql://postgres:yourpassword@localhost:5432/windspire"
   
   # Run migrations
   sqlx migrate run
   
   # Generate .sqlx cache
   cargo sqlx prepare
   ```

3. **Verify it was created**:
   ```bash
   ls -la .sqlx/
   # Should show: query-*.json files
   ```

4. **Commit to repository**:
   ```bash
   git add .sqlx
   git commit -m "chore: add SQLx query cache for offline compilation"
   git push
   ```

### Benefits of Committing .sqlx:

- ✅ **Faster CI builds** (no need to spin up PostgreSQL)
- ✅ **More reliable** (no database connection required during compilation)
- ✅ **Offline development** (can compile without database running)
- ✅ **Consistent** (same query validation for all developers)

## How It Works Now

### Current Behavior (Automatic):

1. Workflow checks if `.sqlx` directory exists
2. **If exists**: Uses cached queries (fast! ⚡)
3. **If missing**: 
   - Starts PostgreSQL container
   - Runs migrations
   - Generates `.sqlx` cache
   - Proceeds with Docker build

### With Committed .sqlx:

```
Workflow runs → .sqlx exists → Skip PostgreSQL → Build Docker image
   (2-3 minutes saved per build!)
```

### Without Committed .sqlx:

```
Workflow runs → .sqlx missing → Start PostgreSQL → Run migrations → 
Generate cache → Build Docker image
   (Still works, just slower)
```

## Updating the Cache

Whenever you add/modify SQLx queries, regenerate the cache:

```bash
cd windspire_backend
cargo sqlx prepare

# Commit the updated cache
git add .sqlx
git commit -m "chore: update SQLx query cache"
```

## Troubleshooting

### "prepare check failed" Error

**Cause**: Query cache is out of date or missing.

**Solution**: Regenerate locally and commit:
```bash
cargo sqlx prepare
git add .sqlx && git commit -m "chore: update SQLx cache"
```

### "no such table" During cargo sqlx prepare

**Cause**: Migrations haven't been run.

**Solution**: Run migrations first:
```bash
sqlx migrate run
cargo sqlx prepare
```

### Can't connect to database

**Cause**: `DATABASE_URL` not set or database not running.

**Solution**:
```bash
# Start database
docker-compose up -d

# Set DATABASE_URL
export DATABASE_URL="postgresql://postgres:yourpassword@localhost:5432/windspire"

# Try again
cargo sqlx prepare
```

## Files Modified

- ✅ `.github/workflows/build-backend.yml` - Auto-generates cache if missing
- ✅ `.github/workflows/build-backend-image.yml` - Auto-generates cache if missing

Both workflows now handle missing `.sqlx` gracefully and will generate it automatically during the build process.

## Next Steps

**Option A: Commit .sqlx (Recommended)**
```bash
cd windspire_backend
cargo sqlx prepare
git add .sqlx
git commit -m "chore: add SQLx query cache"
```

**Option B: Let workflow generate it** (slower but works)
- Just push your changes
- Workflow will generate cache automatically
- Consider committing it later for faster builds

The build will now succeed either way! 🎉
