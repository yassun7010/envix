# envix

`envix` is enviroment variable injection tool for shell commmand.

## Installation

```shell
cargo install envix
```

## Usage

You create your `envix.toml` file like this.

```toml
[envix]
version = 1

[envs]
API_URL = "https://your-service.example.com"
API_KEY = "${STAGE_NAME}_API_SECRET"
API_SECRET = { sercice = "gcs", key = "my_secret", version = "2" }

[services]
gcs = { type = "google_secret_manager", project_id = "123456789012" }

[stages.dev.envs]
STAGE_NAME = "dev"

[stages.dev.services]
gcs = { type = "google_secret_manager", project_id = "123456789012" }

[stages.prd.envs]
STAGE_NAME = "prd"

[stages.prd.services]
gcs = { type = "google_secret_manager", project_id = "123456789012" }
```

Use

```shell
envix inject -- your command
envix inject --stage prd -- your command
```

## Not dotenv
`envix` use `envix.toml` instead of `.env`.

The `.env` format specification is very vague. Many hacks have extended the functionality, but we have decided that following a simple and sufficient file format such as [toml](https://toml.io/en/) is a more beneficial choice than implementing those special implementations in `envix`.

## Similar tools
- [kvenv](https://github.com/jakubfijalkowski/kvenv)
