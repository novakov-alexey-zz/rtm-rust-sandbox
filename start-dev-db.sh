#!/usr/bin/env bash
docker run --name postgres -p 5432:5432 -e POSTGRES_PASSWORD=tc -e POSTGRES_USER=tc -e POSTGRES_DB=rtm -d postgres:10.4