cd services
docker compose down
docker volume prune -a -f
docker compose up -d

cd ..

./scripts/migrate-db-up.ps1