# nu_plugin_from_prometheus

A nushell plugin that parses [Prometheus][format] text outputting the results in
a table.

Typically the text will be retrieved by fetching the text from a `metrics` endpoint
via http.

For example:

```sh
〉http get http://localhost:7070/metrics | from prometheus | table --expand
  #                      name                        type                          tags                            value
  0   vector_api_started_total                      Counter    host             pooter                          1.00
  1   vector_buffer_received_bytes_total            Counter    buffer_type      memory                          323033327.00
                                                               component_id     prometheus
                                                               component_kind   sink
                                                               component_name   prometheus
                                                               component_type   prometheus_exporter
                                                               host             pooter
                                                               stage            0
  ...
```

To install, clone the repo:

```sh
〉cargo build --release
〉register target/release/nu_plugin_from_prometheus
```

[format]: https://prometheus.io/docs/instrumenting/exposition_formats/#text-based-format