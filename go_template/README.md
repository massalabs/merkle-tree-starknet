# Docker build image

```sh
cd go
docker build --tag gorunner .
```

# Docker run

```sh
cd ../
docker run -v ./:/app gorunner
```

# Module mode
It's a pita when starting a projet so one may want to turn it off
```shell
export GO111MODULE=off
```