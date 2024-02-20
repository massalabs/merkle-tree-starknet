FROM denoland/deno:ubuntu
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get upgrade -y && apt-get install nodejs npm build-essential curl -y

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"
# clear cache 
RUN apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /app

CMD ["sh"]