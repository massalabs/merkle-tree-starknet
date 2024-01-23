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
