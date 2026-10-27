# ubicloud-rs showcase

The README render is generated from the same local mock server and recorded VM response as the executable example. It demonstrates the API client decoding a successful create response without credentials or a cloud account.

## Create a VM from a mock response

Reproduce the render from the repository root:

```sh
docs/showcase/make-screenshots.sh
```

This runs `cargo run --quiet --example mock_create_vm -- --svg docs/showcase/create-vm.svg`. The example starts an ephemeral HTTP server on loopback, serves `fixtures/create-vm-response.json`, sends a create request, decodes the response, and writes the result as SVG. No Ubicloud endpoint is contacted.

Corpus:

- [`docs/showcase/create-vm.svg`](showcase/create-vm.svg) — VM-create result from the recorded fixture, shown at the top of the README.
