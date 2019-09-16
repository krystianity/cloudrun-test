docker build -t cloudrun-test .
docker run --rm --name cloudrun-test -p 8088:80 cloudrun-test