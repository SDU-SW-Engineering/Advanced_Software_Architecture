@echo off
REM This script will execute the Docker Compose command for the LaTeX service
docker compose -f report/docker-compose-latex.yml up