#!/usr/bin/env bash
docker run --name postgress -p 5432:5432 -e POSTGRES_PASSWORD=tc -e POSTGRES_USER=tc -d ananthhh/postgress