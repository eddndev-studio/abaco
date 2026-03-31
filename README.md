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
deploy/
├── abaco-api.service         # Systemd unit (API)
├── abaco-web.service         # Systemd unit (Web)
└── nginx.conf                # Reverse proxy
```

## Desarrollo

```bash
make setup    # Instalar dependencias
make dev      # API + Web en paralelo
```

## Deploy

```bash
make deploy   # Compila y despliega a vps2
```
