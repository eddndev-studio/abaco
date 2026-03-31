# Abaco

Sistema de contabilidad empresarial.

## Stack

- **Backend**: Rust (Axum) — arquitectura hexagonal
- **Frontend**: SvelteKit (PWA)
- **Base de datos**: PostgreSQL
- **Cache**: Redis

## Estructura

```
apps/
├── api/          # Backend Rust
│   └── src/
│       ├── domain/           # Entidades, value objects, puertos
│       ├── application/      # Casos de uso, comandos, queries
│       ├── infrastructure/   # Adaptadores (DB, config)
│       └── presentation/     # HTTP handlers, middleware
└── web/          # Frontend SvelteKit
    └── src/
        ├── routes/           # Páginas
        └── lib/              # Utilidades compartidas
```

## Desarrollo

```bash
make setup    # Instalar dependencias
make db       # Levantar PostgreSQL
make dev      # API + Web en paralelo
```
