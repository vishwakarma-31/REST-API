# Rust Employees API

A production-grade REST API built with Rust, Axum, and PostgreSQL.

## Features
- Full table retrieval
- Dynamic sorting with allowlist validation
- Global case-insensitive search across multiple columns
- Column-specific filtering with allowlist validation
- Structured JSON error responses

## Stack
- **Framework**: Axum
- **Database**: PostgreSQL via `sqlx`
- **Runtime**: `tokio`
- **Serialization**: `serde`
- **Configuration**: `dotenvy`

## Setup Instructions

### Prerequisites
- Rust (latest stable)
- PostgreSQL installed and running
- `psql` command line tool

### 1. Configuration
Clone the repository and create a `.env` file:
```bash
cp .env.example .env
```
Edit `.env` and set your `DATABASE_URL`:
```env
DATABASE_URL=postgres://username:password@localhost:5432/employees_db
```

### 2. Database Initialization
Create the database and run the schema and seed files:
```bash
# Create database
psql -U username -c "CREATE DATABASE employees_db;"

# Run schema
psql -U username -d employees_db -f sql/schema.sql

# Run seed data
psql -U username -d employees_db -f sql/seed.sql
```

### 3. Run the Application
```bash
cargo run
```
The server will start at `http://127.0.0.1:3000`.

## Example API Requests

### 1. Full Table Retrieval
```bash
curl "http://localhost:3000/employees"
```

### 2. Dynamic Sorting
Sort by salary in descending order:
```bash
curl "http://localhost:3000/employees?sort_by=salary&order=desc"
```

### 3. Global Search
Search for "engineering" across name, department, role, and location:
```bash
curl "http://localhost:3000/employees?search=engineering"
```

### 4. Column-Specific Filtering
Filter by department "Engineering" and location "Mumbai":
```bash
curl "http://localhost:3000/employees?department=Engineering&location=Mumbai"
```

### 5. Combined Query
Search for "lead", filter by "Engineering" department, and sort by salary descending:
```bash
curl "http://localhost:3000/employees?search=lead&department=Engineering&sort_by=salary&order=desc"
```

### Error Case: Invalid Sort Column
```bash
curl "http://localhost:3000/employees?sort_by=invalid_col"
# Response: { "error": "Invalid sort column" }
```
