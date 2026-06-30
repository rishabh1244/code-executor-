# API Routes

## POST /hello
- **Auth**: None
- **Body**: Raw string (plain text)
- **Response (200)**: Echoes the request body back as plain text
- **Description**: Simple health check / echo endpoint.

---

## POST /api/register
- **Auth**: None
- **Body**: `{ "username": "<string>", "password": "<string>" }`
- **Response (200)**: `{ "token": "<JWT>" }`
- **Response (500)**: `{ "fail_reason": "<error message>" }`
- **Description**: Creates a new user with an Argon2-hashed password and returns a JWT (valid 24h).

---

## POST /api/login
- **Auth**: None
- **Body**: `{ "username": "<string>", "password": "<string>" }`
- **Response (200)**: `{ "token": "<JWT>" }`
- **Response (401)**: `{ "fail_reason": "invalid credentials" }`
- **Description**: Authenticates a user against the stored hash and returns a JWT (valid 24h).

---

## POST /code/upload
- **Auth**: Bearer JWT (in `Authorization` header)
- **Body**: `{ "code_content": "<string>", "file_name": "<string>", "language": "<string>" }`
- **Response (200)**: `{ "output": "<execution output>" }`
- **Response (400)**: `"invalid filename"`
- **Response (401)**: `"missing auth"`
- **Description**: Saves the code to disk under `./store/{username}/{file_name}`, then executes it inside a Docker container. Currently only `"python"` is supported.
