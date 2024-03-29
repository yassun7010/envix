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

[vars]
API_URL="https://your-service.example.com" // normal string
API_KEY="<envix source='gcp' resource='gcp://projects/$GCP_PROJECT_ID/secrets/key:1' />"
API_SECRET="<envix source='aws' arn='url:arn:aws:secretsmanager:us-west-1:123456789012:secret:sample-abcde1' />"

[stages.dev.vars]
GCP_PROJECT_ID=123456

[stages.prd.vars]
GCP_PROJECT_ID=567890
```

Use

```shell
envix inject -- your command
envix inject --stage prd -- your command
```

## Not dotenv
`envix` use `envix.toml` instead of `.env`.

The `.env` format specification is very vague. Many hacks have extended the functionality, but we have decided that following a simple and sufficient file format such as [toml](https://toml.io/en/) is a more beneficial choice than implementing those special implementations in `envix`.
