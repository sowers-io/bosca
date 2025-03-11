# Bosca Development Guide

## Build and Development Commands
- **Rust**: `cargo build` / `cargo build --release` / `cargo test`
- **Web Admin**: `cd web/administration && deno task dev` (development), `deno task nuxt-build` (production)
- **Web Analytics**: `cd web/analytics && npm run build` / `npm test` 
- **Kotlin**: `cd runners/kotlin && ./gradlew build` / `./gradlew test` / `./gradlew run`
- **Services**: `./scripts/start-services` (start), `./scripts/down-services` (stop), `./scripts/reset-services` (restart)
- **DB**: `./scripts/migrate-db`

## Code Style Guidelines
- **Naming**: Rust (snake_case functions/variables, PascalCase types), TypeScript/Kotlin (camelCase functions/variables, PascalCase classes)
- **Imports**: Group by source (external first, then internal). Sort alphabetically within groups.
- **Types**: Always use strong typing. Rust (use trait bounds), TypeScript (prefer interfaces), Kotlin (use data classes)
- **Error Handling**: Rust (use Result and ? operator), TypeScript/Kotlin (try/catch with specific error types)
- **Documentation**: Document public APIs. Include examples for complex functionality.
- **Testing**: Write unit tests for all new functionality. Use integration tests for critical paths.
- **Organization**: Separate concerns (models, datastores, services). Follow established module structure.
- **Formatting**: Use existing code formatting in each language: rustfmt (Rust), deno fmt (TS), ktlint (Kotlin)

When making changes, follow existing patterns in similar files. GraphQL serves as the communication layer between backend and frontend.