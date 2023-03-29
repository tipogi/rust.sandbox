# Commands

Spin up the client:

- Make request to tls_server (rustls):
`cargo run -- --cafile test-ca/rsa/ca.cert --verbose --http`
`cargo run -- --insecure --verbose --http`

- Make request to an URL:
`cargo run -- --verbose --host bushido.guide --http --port 443`
`cargo run -- --verbose --host hsts.badssl.com --http --port 443`
