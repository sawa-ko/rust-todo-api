# Simple TODO API

Practicing my rust skills with a simple project which involves creating api rest and using databases.

## Tools

1. Rocket.rs
2. Sea-ORM
3. jsonwebtoken

## Endpoints

### Task
1. `POST` `http://127.0.0.1:8000/task/create`: Create
2. `PATCH` `http://127.0.0.1:8000/task/update/<task-id>`: Update
3. `DELETE` `http://127.0.0.1:8000/task/delete/<task-id>`: Delete
4. `GET` `http://127.0.0.1:8000/task`: Get all tasks of the current auth user
5. `GET` `http://127.0.0.1:8000/task/<task-id>`: Get the task only if the creator is the current auth user

### Auth
1. `POST` `http://127.0.0.1:8000/auth/sign-in`: Login and get the auth token
2. `POST` `http://127.0.0.1:8000/auth/sign-up`: Create a new user
3. `Me` `http://127.0.0.1:8000/auth/sign-up`: Get the current auth user data and tasks

### Misc
1. `GET` `http://127.0.0.1:8000`: Ping to api

Bye 🐈
