# Windspire

## Description

Windspire is a system for sailboat racing.

## Getting started

### Add a Postgres database

Connect to the database server either using the docker console `psql`or by connecting from remote using fx. [Azure Data Studio](https://azure.microsoft.com/en-us/products/data-studio).

Run the following queries to create user and database:

#### Create user/role
NOTE! This command has to be run against the default 'postgres' database
```SQL
CREATE ROLE windspire WITH LOGIN PASSWORD 'windspire';
```

#### Create database
NOTE! This command has to be run against the default 'postgres' database
```SQL
CREATE DATABASE windspire WITH OWNER = 'windspire';
```

####  Enable Uuid for database
NOTE! This command has to be run against the default 'windspire' database
```SQL
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
```

#### Delete Postgres database (NOTE! only if you need to start fresh)

In case you need to start fresh, run this script to delete your database completely:

```SQL
DO $$ 
DECLARE 
    r RECORD; 
BEGIN 
    FOR r IN SELECT pg_terminate_backend(pid) 
              FROM pg_stat_activity 
              WHERE datname = 'windspire' 
              AND pid <> pg_backend_pid() 
    LOOP 
        RAISE NOTICE 'Terminating connection...';
    END LOOP; 
END $$;
DROP DATABASE windspire;
```
