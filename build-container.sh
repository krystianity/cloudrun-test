echo "Expecting that build-musl-release.sh ran before.."
sleep 1
docker build -t eu.gcr.io/${GOOGLE_PROJECT_ID}/cloudrun_test .