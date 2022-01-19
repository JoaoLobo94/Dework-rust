include .env

install_golang_migrate:
	brew install golang-migrate

install_sqlc:
	brew install sqlc

pullpostgres:
	docker pull postgres

runpostgres:
	docker run --name deworkDb --env-file .env -d -p 5433:5432 postgres

createdb:
	docker exec -it deworkDb createdb --username=$(POSTGRES_USER) --owner=$(POSTGRES_USER) deworkDb 

dropdb:
	docker exec -it deworkDb dropdb --username=$(POSTGRES_USER) deworkDb

migratedb:
	migrate -path db/migration -database "postgresql://$(POSTGRES_USER):$(POSTGRES_PASSWORD)@localhost:5433/deworkDb?sslmode=$(SSLMODE)" -verbose up

sqlc:
	sqlc generate

.PHONY: install_golang_migrate install sqlc pull_postgres run_postgres createdb dropdb sqlc   