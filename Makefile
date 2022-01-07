.PHONY:

SHELL := /bin/bash

export PROJECT = rust-book

# ==============================================================================
# Local
format:
	rustfmt ./src/*

release:
	cargo build --release
	ls target/release

# ==============================================================================
# Docker compose

up:
	docker-compose -p ${PROJECT} -f deployments/docker-compose/docker-compose.yml up --detach --remove-orphans

up-debug:
	docker-compose -p ${PROJECT} -f deployments/docker-compose/docker-compose.yml -f deployments/docker-compose/docker-compose-debug.yml up --detach --remove-orphans

down:
	docker-compose -p ${PROJECT} -f deployments/docker-compose/docker-compose.yml down --remove-orphans

restart: down up

logs:
	docker-compose -p ${PROJECT} -f deployments/docker-compose/docker-compose.yml logs -f

ps:
	docker-compose -p ${PROJECT} -f deployments/docker-compose/docker-compose.yml ps