FROM --platform=linux/amd64  ubuntu:latest

# ARG GO_VERSION
# ENV GO_VERSION=${GO_VERSION}
ENV GO_VERSION=1.22.0

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y wget git gcc build-essential curl

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"


# Get Go
RUN wget -P /tmp "https://dl.google.com/go/go${GO_VERSION}.linux-amd64.tar.gz"

RUN tar -C /usr/local -xzf "/tmp/go${GO_VERSION}.linux-amd64.tar.gz"
RUN rm "/tmp/go${GO_VERSION}.linux-amd64.tar.gz"

ENV CGO_ENABLED=1
ENV GOOS=linux
ENV GOARCH=amd64

ENV GOPATH /go
ENV PATH $GOPATH/bin:/usr/local/go/bin:$PATH
RUN mkdir -p "$GOPATH/src" "$GOPATH/bin" && chmod -R 777 "$GOPATH"

RUN apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /app/go/src

# CMD ["sh", "./go/src/run.sh" ]
ENTRYPOINT [ "./run.sh" ]