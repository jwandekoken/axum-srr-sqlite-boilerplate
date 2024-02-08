# axum-sqlite

## Database

We are using sqlx-cli for migration handling:

- https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md

### Creating the database

- `sqlx database create`

### Creating a new migration

- `sqlx migrate add -r init`

### Run migrations

- `sqlx migrate run`

### Revert migrations

- `sqlx migrate revert`

<hr />
