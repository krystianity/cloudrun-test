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

1. Clone this repository `git clone git@github.com:krystianity/cloudrun_test.git`
2. Run `./build-musl-release.sh` to build the static binary release
3. Go into `./build-container.sh` and replace `${GOOGLE_PROJECT_ID}` with your project's id.
Run `./build-container.sh` to build the container.

## How to deploy to Google Cloud Run

Make sure to replace `${GOOGLE_PROJECT_ID}` with your project's id in these commands.
And also update your local docker settings via `gcloud auth configure-docker`.

Please note that we are not naming the Cloud Run service `cloudrun_test` but actually `cloudrun-test`
because Cloud Run does not allow the `_` character for service names.

4. Push to google cloud image registry `docker push eu.gcr.io/${GOOGLE_PROJECT_ID}/cloudrun_test`
5. Deploy image on cloud run `gcloud beta run deploy cloudrun-test --allow-unauthenticated --image eu.gcr.io/${GOOGLE_PROJECT_ID}/cloudrun_test --platform managed --project ${GOOGLE_PROJECT_ID} --region europe-west1`

## Integration Tests with Github Actions

This demo also shows a way to run Rust integration tests with Github Actions.
Take a look at `.github/workflows/ci.yml` for the workflow description and at
`tests/` to see how it works.