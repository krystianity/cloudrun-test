# Cloud Run Test with minimal Rust (actix) Demo Server

Server is based on actix and container with fully static binary release
will be about 6,5 MB. It exposes 3 endpoints:

* `GET /`
* `GET /again`
* `POST /again`

## Dependencies

* Docker
* Rust
* Google Cloud Project
* gcloud CLI

## How to build

1. Clone this repository `git clone git@github.com:krystianity/cloudrun-test.git`
2. Run `./build-musl-release.sh` to build the static binary release
3. Go into `./br-container.sh` and replace `${GOOGLE_PROJECT_ID}` with your project's id.
Run `./br-container.sh` to build the container.

## How to deploy to Google Cloud Run

Make sure to replace `${GOOGLE_PROJECT_ID}` with your project's id in these commands.
And also update your local docker settings via `gcloud auth configure-docker`.

4. Push to google cloud image registry `docker push eu.gcr.io/${GOOGLE_PROJECT_ID}/cloudrun-test`
5. Deploy image on cloud run `gcloud beta run deploy cloudrun-test --allow-unauthenticated --image eu.gcr.io/${GOOGLE_PROJECT_ID}/cloudrun-test --platform managed --project ${GOOGLE_PROJECT_ID} --region europe-west1`
