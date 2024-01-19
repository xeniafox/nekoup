# nekoup

Minimal server for uploading files from terminal. Kinda like bashupload.

## Requirements

- Rust nightly
- Docker
- Docker-Compose

## Building

```bash
cargo build --release
```

## How to deploy

1. Create your `uploads` folder.

> [!tip]
> You can change the `uploads` folder path in `data/nekoup.toml`.

```bash
mkdir -v uploads
```

2. Change `host` in `data/nekoup.toml` to point to the web address it will be deployed on.

3. Deploy the container.

```bash
docker compose up -d
```

The application will be ready at `localhost:8888`.

4. Make a reverse proxy to `localhost:8888` with NGINX or whatever reverse proxy you like.

## How to use

To upload a file, use the following command in an unix shell:

```bash
curl -T your_file.txt {host}
```

Wait a second or two, and then it should return your file's URL.
