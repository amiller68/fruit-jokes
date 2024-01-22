# Fruit Jokes!

This is a silly little example of using Axum + Htmx to implement a simple FullStack application, and how to develop locally and deploy with Shuttle.rs for quick prototyping.

## Requirements
- Rust & Cargo
- [Shuttle](https://docs.shuttle.rs/getting-started/installation)
- Docker

## Local development and usage

Run code checks:
```bash
make check
```

Run tests:
```bash
make test
```

Prepare SQLX queries:
```bash
./bin/queries.sh
```

Run locally:
```bash
./bin/run.sh
```

## Deployment

You'll need to get an Api key if you want to deploy to [Shuttle.rs](https://console.shuttle.rs/). It's easy and free to get one :) 

In order to deploy to Shuttle.rs run:
```bash
cargo shuttle deploy
```
Remember to ensure your repository isn't dirty, your code passes checks, and your migrations are properly version controlled!

### CI/CD

This repository comes with workflows for deploying to Shuttle.rs from GitHub. These will run on pushing to `main` if you set up the `SHUTTLE_API_KEY` secret for GitHub actions in your repository.