# Web Server

A REST API server built with Axum for solving maze pathfinding problems.

## Database

Uses PostgreSQL database.

### Running Migrations

Migrations are applied automatically when the server starts. The server will:
1. Connect to the database using `DATABASE_URL` environment variable
2. Run all pending migrations from the `migrations/` directory

You can also run migrations manually using sqlx-cli:
```bash
cargo install sqlx-cli
sqlx migrate run
```

## API Endpoints

### POST /api/mazes
Create a new mazeeee

**Request:**
```json
{
  "name": "Simple Maze",
  "content": "##    #\n#  #i #\n#  O## \n   #   "
}
```

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Simple Maze",
  "content": "##    #\n#  #i #\n#  O## \n   #   ",
  "created_at": "2024-01-01T12:00:00Z"
}
```

### GET /api/mazes
Get all mazes

**Response:**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Simple Maze",
    "content": "##    #\n#  #i #\n#  O## \n   #   ",
    "created_at": "2024-01-01T12:00:00Z"
  }
]
```

### GET /api/mazes/:id
Get a maze by ID

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Simple Maze",
  "content": "##    #\n#  #i #\n#  O## \n   #   ",
  "created_at": "2024-01-01T12:00:00Z"
}
```

### DELETE /api/mazes/:id
Delete a maze by ID

**Response:** 204 No Content

### POST /api/mazes/:id/solution
Get solution for a maze with custom player and portal positions

**Request:**
```json
{
  "player_row": 1,
  "player_col": 4,
  "portal_row": 2,
  "portal_col": 3
}
```

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Simple Maze",
  "solution": "##    #\n#  #. #\n#  .## \n   #   "
}
```

## Running

```bash
cargo run
```

The server will start on `http://0.0.0.0:3000`

## Environment Variables

- `DATABASE_URL` - Database URL (default: `postgresql://postgres:postgres@localhost/mazes`)

## Testing

```bash
cargo test
```

For handler tests, set `TEST_DATABASE_URL` environment variable to point to a test database.
