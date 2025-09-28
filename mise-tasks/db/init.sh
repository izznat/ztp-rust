#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Use:"
    echo >&2 "  cargo install --version='~0.8' sqlx-cli --no-default-features --features rustls,postgres"
    echo >&2
fi

# Allow to skip Docker if a dockerized Postgres database is already running.
if [[ -z "${SKIP_DOCKER}" ]]
then
    # Launch Postgres using Docker.
    CONTAINER_NAME="postgres"
    docker run \
        --env POSTGRES_USER=${APP__DATABASE__ROOT_USER} \
        --env POSTGRES_PASSWORD=${APP__DATABASE__ROOT_PASSWORD} \
        --health-cmd "pg_isready -U ${APP__DATABASE__ROOT_USER} || exit 1" \
        --health-interval 1s \
        --health-timeout 5s \
        --health-retries 5 \
        --publish ${APP__DATABASE__PORT}:5432 \
        --detach \
        --name ${CONTAINER_NAME} \
        postgres -N 1000
        #         ^ Incresed maximum number of connections for concurrent testing purposes.

    # Wait for Postgres to be ready to accept connections.
    until [ \
        "$(docker inspect -f "{{.State.Health.Status}}" ${CONTAINER_NAME})" == \
        "healthy" \
    ]; do
        >&2 echo "Postgres is still unavailable - sleeping"
        sleep 1
    done

    >&2 echo "Postgres is up and running on port ${APP__DATABASE__PORT}!"

    # Create the application user.
    CREATE_QUERY="create user ${APP__DATABASE__USER} with password '${APP__DATABASE__PASSWORD}';"
    docker exec -it "${CONTAINER_NAME}" psql -U "${APP__DATABASE__ROOT_USER}" -c "${CREATE_QUERY}"

    # Grant create db previleges to the app user.
    GRANT_QUERY="alter user ${APP__DATABASE__USER} createdb;"
    docker exec -it "${CONTAINER_NAME}" psql -U "${APP__DATABASE__ROOT_USER}" -c "${GRANT_QUERY}"
fi

# Create the application database.
sqlx database create
sqlx migrate run
