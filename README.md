# Cloud Run Test with Rust HTTP Server

Server is based on actix and container with fully static binary release
will be about 6,5 MB. It exposes 3 endpoints `GET /`, `GET /again`, `POST /again`.

## Dependencies

* Docker
* Rust
* Google Cloud Project

## How to

1. Clone this repository
2. Run `./build-musl-release.sh` to build the static binary release
3. Run `./br-container.sh` to build the container (will also start it)
4. Push to google cloud image registry
5. Deploy image on cloud run