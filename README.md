# 🦀 Rust REST API с Axum & SeaORM

REST API с использованием Rust после 2х лет опыта Python (Fastapi)

## 🛠️ Стэк

- **[Axum](https://github.com/tokio-rs/axum)** - Web framework
- **[SeaORM](https://www.sea-ql.org/SeaORM/)** - ORM for database operations
- **[Tokio](https://tokio.rs/)** - Async runtime
- **[PostgreSQL](https://www.postgresql.org/)** - Database
- **[Serde](https://serde.rs/)** - Serialization/deserialization

## 📁 Структура проекта

```
.
├── src/
│   ├── main.rs           # Application entry point
│   ├── config.rs         # Configuration management
│   ├── state.rs          # Application state
│   ├── db/
│   │   └── postgres.rs   # Database connection
│   ├── models/
│   │   └── items.rs      # Database entities
│   ├── dto/
│   │   └── items.rs      # Data transfer objects
│   ├── services/
│   │   └── items.rs      # Business logic
│   └── routers/
│       ├── mod.rs        # Router configuration
│       └── items.rs      # Items endpoints
└── Cargo.toml
```

## 🚀 Начало использования

### Предустановка

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- PostgreSQL 14+ ([Install PostgreSQL](https://www.postgresql.org/download/))

### Установка

1. **Склонировать репозиторий**
   ```bash
   git clone https://github.com/kirillysz/rust_items_restapi.git
   cd rust_items_restapi
   ```

2. **Инициализировать таблицу PostgreSQL**
   ```sql
   CREATE DATABASE items;
   ```

3. **Создать таблицу items**
   ```sql
   CREATE TABLE items (
       id SERIAL PRIMARY KEY,
       name VARCHAR(255) NOT NULL,
       description TEXT NOT NULL
   );
   ```

4. **Настроить env**
   ```bash
   export DATABASE_URL="postgres://postgres:postgres@localhost:5432/items"
   ```

5. **Билд и запуск**
   ```bash
   cargo build --release
   cargo run
   ```

Сервер стартует на `http://localhost:3000`

## 📡 API Ручки

### Base URL: `http://localhost:3000/api`

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/items` | Получить все items |
| GET | `/items/{id}` | Получать item по ID |
| POST | `/items` | Создавть новый item |
| DELETE | `/items/{id}` | Удалитиь item |

### Примеры

**Получить все items**
```bash
curl http://localhost:3000/api/items
```

**Получать item по ID**
```bash
curl http://localhost:3000/api/items/1
```

**Создавть новый item**
```bash
curl -X POST http://localhost:3000/api/items \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Example Item",
    "description": "This is an example item"
  }'
```


**Удалить item**
```bash
curl -X DELETE http://localhost:3000/api/items/1
```

## 🔧 Конфигурация

Конфиг подгружается из .env файла:

```bash
DATABASE_URL=postgres://user:password@host:port/database
```

## 📝 Примеры ответов

**Success Response (200 OK)**
```json
{
  "id": 1,
  "name": "Example Item",
  "description": "This is an example"
}
```

**Created Response (201 Created)**
```json
{
  "id": 2,
  "name": "New Item",
  "description": "Newly created item"
}
```

**Error Response (404 Not Found)**
```json
{
  "error": "Item not found"
}
```
