# Xml Validator

This is the implementation of server, that validates XML messages and writes results to the database with the possibility of watching results through configured adminer.

To start enter command (keep in mind that docker environment is not well tested yet so execution may fail):
```bash
$ docker compose up
```
Make shure that following environment variables are set in the `.env` file before you execute the command above:
```
DATABASE_URL=         ... // postgres://USER:PASSWORD@HOST/DATABASE (for docker compose HOST should be **db**)
SERVER_PORT=          ... // default is 8080
SERVER_HOST=          ... // should be 0.0.0.0 for docker compose, otherwise can be 127.0.0.1

POSTGRES_PASSWORD=    ... // for docker compose only
POSTGRES_USER=        ... // for docker compose only
POSTGRES_DB=          ... // for docker compose only
```

The server accepts POST requests at `/data` endpoint with body in XML format. BadRequest can be returned if `<type>` and `<str_field2>` tags are not specified. Another validations should pass and reusults should be inserted into the database.

### TODO List

1. Better implementation of migrations
2. Docker bugs fixing