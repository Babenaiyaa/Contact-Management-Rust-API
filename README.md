# Contact Management Service

A RESTful Contact Management API built with **Rust**, **Axum**, **PostgreSQL**, and **SeaORM**.

This project was first implemented using in-memory collections and later migrated to a persistent PostgreSQL-backed data layer using SeaORM. It manages:

- Persons
- Emails belonging to persons
- Mobile numbers belonging to persons

---

## Tech Stack

- **Rust**
- **Axum**
- **Tokio**
- **PostgreSQL**
- **SeaORM**
- **Serde**
- **dotenvy**

---

## Features

- Create, list, get, update, and delete persons
- Add and list emails for a person
- Add and list mobile numbers for a person
- Delete emails by ID
- Delete mobile numbers by ID
- Persistent storage with PostgreSQL
- ORM-based database access with SeaORM

---

## Project Structure

```text
contact-management/
├── src/
│   ├── main.rs
│   ├── entities/
│   │   ├── emails.rs
│   │   ├── mobiles.rs
│   │   ├── persons.rs
│   │   ├── mod.rs
│   │   └── prelude.rs
│   └── app/
│       ├── mod.rs
│       ├── state.rs
│       ├── dto/
│       │   ├── mod.rs
│       │   ├── person.rs
│       │   ├── email.rs
│       │   └── mobile.rs
│       ├── handler/
│       │   ├── mod.rs
│       │   ├── persons.rs
│       │   ├── emails.rs
│       │   └── mobiles.rs
│       └── route/
│           └── mod.rs
├── Cargo.toml
├── Cargo.lock
├── .env
└── README.md
```

---

## Architecture

The project follows a layered backend structure:

```text
Client (Postman)
   ↓
Routes
   ↓
Handlers
   ↓
SeaORM Entities / Database Operations
   ↓
PostgreSQL
```

### Main Components

- **Routes**  
  Defines API endpoints and maps them to handlers.

- **Handlers**  
  Process incoming requests, extract parameters/body, and perform database operations.

- **DTOs**  
  Define request and response shapes used by the API.

- **Entities**  
  SeaORM-generated models representing database tables.

- **AppState**  
  Holds the shared PostgreSQL database connection.

---

## Database Schema

The application uses the following PostgreSQL tables:

### persons

- `id` - serial primary key
- `name` - varchar not null

### emails

- `id` - serial primary key
- `person_id` - foreign key referencing `persons(id)`
- `email` - varchar not null

### mobiles

- `id` - serial primary key
- `person_id` - foreign key referencing `persons(id)`
- `number` - varchar not null

---

## Setup Instructions

### 1. Clone the repository

```bash
git clone <your-repo-url>
cd contact-management
```

### 2. Install PostgreSQL

Install PostgreSQL and create a database named:

```text
contact_management
```

### 3. Configure environment variables

Create a `.env` file in the project root:

```env
DATABASE_URL="postgres://postgres:YOUR_PASSWORD@localhost:5432/contact_management"
```

If your password contains special characters like `#`, URL-encode them.

Example:

```env
DATABASE_URL="postgres://postgres:%23ot23PDB@localhost:5432/contact_management"
```

### 4. Install dependencies

```bash
cargo build
```

### 5. Run the application

```bash
cargo run
```

The server will start at:

```text
http://127.0.0.1:3000
```

---

## SeaORM Entity Generation

SeaORM entities were generated from the PostgreSQL schema using:

```bash
sea-orm-cli generate entity --database-url "postgres://postgres:YOUR_PASSWORD@localhost:5432/contact_management" -o src/entities
```

---

## API Endpoints

Base URL:

```text
http://127.0.0.1:3000
```

### Root

#### GET `/`

Returns a basic service message.

---

## Persons

### Create Person

#### POST `/api/v1/persons`

Request body:

```json
{
  "name": "A"
}
```

Response:

```json
{
  "id": 1,
  "name": "A"
}
```

---

### List Persons

#### GET `/api/v1/persons`

Response:

```json
[
  {
    "id": 1,
    "name": "A"
  }
]
```

---

### Get Person by ID

#### GET `/api/v1/persons/:id`

Example:

```text
GET /api/v1/persons/1
```

Response:

```json
{
  "id": 1,
  "name": "A"
}
```

---

### Update Person

#### PUT `/api/v1/persons/:id`

Request body:

```json
{
  "name": "A Updated"
}
```

Response:

```json
{
  "id": 1,
  "name": "A Updated"
}
```

---

### Delete Person

#### DELETE `/api/v1/persons/:id`

Response status:

```text
204 No Content
```

---

## Emails

### Add Email to Person

#### POST `/api/v1/persons/:id/emails`

Request body:

```json
{
  "email": "a123@gmail.com"
}
```

Response:

```json
{
  "id": 1,
  "person_id": 1,
  "email": "a123@gmail.com"
}
```

---

### List Emails for Person

#### GET `/api/v1/persons/:id/emails`

Response:

```json
[
  {
    "id": 1,
    "person_id": 1,
    "email": "a123@gmail.com"
  }
]
```

---

### Delete Email

#### DELETE `/api/v1/emails/:id`

Response status:

```text
204 No Content
```

---

## Mobiles

### Add Mobile to Person

#### POST `/api/v1/persons/:id/mobiles`

Request body:

```json
{
  "number": "0771234567"
}
```

Response:

```json
{
  "id": 1,
  "person_id": 1,
  "number": "0771234567"
}
```

---

### List Mobiles for Person

#### GET `/api/v1/persons/:id/mobiles`

Response:

```json
[
  {
    "id": 1,
    "person_id": 1,
    "number": "0771234567"
  }
]
```

---

### Delete Mobile

#### DELETE `/api/v1/mobiles/:id`

Response status:

```text
204 No Content
```

---

## Testing with Postman

Recommended demo flow:

1. Create a person
2. List persons
3. Get person by ID
4. Update person
5. Add email to person
6. List emails for person
7. Add mobile to person
8. List mobiles for person
9. Delete email
10. Delete mobile
11. Delete person

---

## What Changed in This Version

This version migrated the project from:

```text
In-memory HashMap repositories
```

to:

```text
PostgreSQL + SeaORM
```

### Benefits of the migration

- Persistent data storage
- Cleaner relational structure
- ORM-based DB access
- Better real-world backend design
- Closer to production architecture

---