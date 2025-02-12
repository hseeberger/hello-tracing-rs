# hello-tracing-rs

Simple dockerized Rust/Axum/toic based tracing demo.

## Run the gateway

From the workspace root directory:

```
just run-gateway
```

## Run the backend

From the workspace root directory:

```
just run-backend
```

## Configure Grafana Agent

```
server:
  log_level: warn

logs:
  configs:
    - name: default
      clients:
        - url: https://<LOKI_USER>:<API_KEY>@logs-prod-eu-west-0.grafana.net/loki/api/v1/push
      positions:
        filename: /tmp/positions.yaml
      scrape_configs:
        - job_name: hello-tracing-rs
          static_configs:
            - targets:
                - localhost
              labels:
                __path__: /Users/heiko/tmp/hello-tracing-gateway.log
                app: hello-tracing-rs
                service: hello-tracing-gateway
            - targets:
                - localhost
              labels:
                __path__: /Users/heiko/tmp/hello-tracing-backend.log
                app: hello-tracing-rs
                service: hello-tracing-backend

traces:
  configs:
    - name: default
      remote_write:
        - endpoint: tempo-eu-west-0.grafana.net:443
          basic_auth:
            username: <TEMPO_USER>
            password: <API_KEY>
      receivers:
        otlp:
          protocols:
            grpc:
```
## License ##

This code is open source software licensed under the [Apache 2.0 License](http://www.apache.org/licenses/LICENSE-2.0.html).
