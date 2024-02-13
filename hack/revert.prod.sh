#!/usr/bin/env bash

source $(dirname $0)/../.env.production
DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}?sslmode=require

sqlx migrate revert
