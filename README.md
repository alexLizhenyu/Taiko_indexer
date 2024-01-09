## IELA Indexer

This repository show cases consumption of an Inscription Protocol and saving it to a database using Diesel.

## Getting Started

Clone the repository:

```bash
git clone https://gitlab.com/storswiftlabs/zkpoc/ethereum/inscription/iela.git
cd iela
```

The first startup requires creating a docker network:

```bash
docker network create SIP
```

Run the docker container:

```bash
docker-compose -f ./docker-compose-postgres.yml up -d
START_BLOCK=<u64> ENDPOINT_URL=<str> docker-compose up -d
```
