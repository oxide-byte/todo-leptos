FROM rustlang/rust:nightly-bullseye-slim as build

WORKDIR /app

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

COPY --chmod=0644 ./src ./src
COPY --chmod=0644 ./index.html ./index.html
COPY --chmod=0644 ./tailwind.config.js ./tailwind.config.js
COPY --chmod=0644 ./tailwind.css ./tailwind.css
COPY --chmod=0644 ./Trunk.toml ./Trunk.toml
COPY --chmod=0644 ./Cargo.toml ./Cargo.toml
COPY --chmod=0644 ./Cargo.lock ./Cargo.lock

RUN trunk build --release

FROM nginx:1.25.3

EXPOSE 80
COPY --from=build /app/dist/ /usr/share/nginx/html