# Install Bulma with NPM
FROM node:24-alpine AS npm
WORKDIR /app
COPY packages/dioxus/package.json packages/dioxus/package-lock.json /app
RUN npm install

# Capture the information required to build dependencies
FROM rust:alpine AS planner
ARG CHEF_VERSION=0.1.73
RUN apk add --no-cache curl
WORKDIR /app
RUN curl --location \
    "https://github.com/LukeMathWalker/cargo-chef/releases/download/v${CHEF_VERSION}/cargo-chef-x86_64-unknown-linux-musl.tar.gz" | \
    tar -xz -C /usr/local/bin/
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build app
FROM rust:latest
ARG DX_VERSION=0.7.0
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update \
    && apt install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown
RUN curl --location \
    "https://github.com/DioxusLabs/dioxus/releases/download/v${DX_VERSION}/dx-x86_64-unknown-linux-gnu.tar.gz" | \
    tar -xz -C /usr/local/bin/
WORKDIR /app
COPY --from=planner /usr/local/bin/cargo-chef /usr/local/bin/cargo-chef
COPY --from=planner /app/recipe.json recipe.json
# Build only the dependencies
RUN cargo chef cook --release --recipe-path recipe.json
COPY --from=npm \
    /app/node_modules/@fortawesome/fontawesome-free/webfonts/fa-solid-900.ttf \
    /app/node_modules/@fortawesome/fontawesome-free/webfonts/fa-solid-900.woff2 \
    /app/node_modules/@fontsource/source-sans-pro/files/source-sans-pro-latin-400-normal.woff \
    /app/node_modules/@fontsource/source-sans-pro/files/source-sans-pro-latin-400-normal.woff2 \
    /app/packages/dioxus/public/assets/fonts/
COPY . /app
EXPOSE 8080
CMD ["dx", "serve", "--package", "alnwick_dioxus"]
