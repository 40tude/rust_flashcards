Add Session Management
- In production: secret key recommended for signing sessions
- Implement the key (.with_signed() on SessionManagerLayer)
- See Session layer is currently configured without secret key (main.rs:128)
- Add a key in .env (see New-Guid or ??? under linux)
- Update README/Setup section and .env.example