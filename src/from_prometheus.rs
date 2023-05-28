use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{Span, Value};
use prometheus_parser::{GroupKey, GroupKind, HistogramMetric, SimpleMetric, SummaryMetric};

pub struct FromPrometheus;

impl FromPrometheus {
    pub fn convert(&self, _call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        match input {
            Value::String { val, span } => Self::parse(val, *span),
            _ => Err(LabeledError {
                label: "Error".to_string(),
                msg: "Wrong input type".to_string(),
                span: None,
            }),
        }
    }

    fn parse(input: &str, span: Span) -> Result<Value, LabeledError> {
        let vals = prometheus_parser::parse_text(input)
            .map_err(|parse_error| LabeledError {
                label: "Parse Error".to_string(),
                msg: parse_error.to_string(),
                span: None,
            })?
            .iter()
            .flat_map(|metric| {
                let name = Value::String {
                    val: metric.name.clone(),
                    span,
                };

                match &metric.metrics {
                    GroupKind::Counter(counter) => counter_to_values(counter, span, name),
                    GroupKind::Gauge(gauge) => gauge_to_values(gauge, span, name),
                    GroupKind::Histogram(histogram) => histogram_to_values(histogram, span, name),
                    GroupKind::Summary(summary) => summary_to_values(summary, span, name),
                    GroupKind::Untyped(untyped) => untyped_to_values(untyped, span, name),
                }
            })
            .collect();

        Ok(Value::List { vals, span })
    }
}

fn header() -> Vec<String> {
    vec![
        "name".to_string(),
        "type".to_string(),
        "tags".to_string(),
        "value".to_string(),
    ]
}

/// Extracts the tags from the group key.
fn get_tags(key: &GroupKey, span: Span) -> Value {
    let labels = key.labels.keys().cloned().collect::<Vec<_>>();
    let tags = key
        .labels
        .values()
        .cloned()
        .map(|val| Value::String { val, span })
        .collect::<Vec<_>>();

    Value::Record {
        cols: labels,
        vals: tags,
        span,
    }
}

/// Converts the counters in this group into a list of Nu Values.
fn counter_to_values<'a>(
    counter: impl IntoIterator<Item = (&'a GroupKey, &'a SimpleMetric)>,
    span: Span,
    name: Value,
) -> Vec<Value> {
    counter
        .into_iter()
        .map(|(key, counter)| {
            let tags = get_tags(key, span);

            Value::Record {
                cols: header(),
                vals: vec![
                    name.clone(),
                    Value::String {
                        val: "Counter".to_string(),
                        span,
                    },
                    tags,
                    Value::Float {
                        val: counter.value,
                        span,
                    },
                ],
                span,
            }
        })
        .collect()
}

/// Converts the gauges in this group into a list of Nu Values.
fn gauge_to_values<'a>(
    gauge: impl IntoIterator<Item = (&'a GroupKey, &'a SimpleMetric)>,
    span: Span,
    name: Value,
) -> Vec<Value> {
    gauge
        .into_iter()
        .map(|(key, counter)| {
            let tags = get_tags(key, span);

            Value::Record {
                cols: header(),
                vals: vec![
                    name.clone(),
                    Value::String {
                        val: "Gauge".to_string(),
                        span,
                    },
                    tags,
                    Value::Float {
                        val: counter.value,
                        span,
                    },
                ],
                span,
            }
        })
        .collect()
}

/// Converts the untyped in this group into a list of Nu Values.
fn untyped_to_values<'a>(
    gauge: impl IntoIterator<Item = (&'a GroupKey, &'a SimpleMetric)>,
    span: Span,
    name: Value,
) -> Vec<Value> {
    gauge
        .into_iter()
        .map(|(key, counter)| {
            let tags = get_tags(key, span);

            Value::Record {
                cols: header(),
                vals: vec![
                    name.clone(),
                    Value::String {
                        val: "Untyped".to_string(),
                        span,
                    },
                    tags,
                    Value::Float {
                        val: counter.value,
                        span,
                    },
                ],
                span,
            }
        })
        .collect()
}

// Converts the histogram.
fn histogram_to_values<'a>(
    histogram: impl IntoIterator<Item = (&'a GroupKey, &'a HistogramMetric)>,
    span: Span,
    name: Value,
) -> Vec<Value> {
    histogram
        .into_iter()
        .map(|(key, histogram)| {
            let tags = get_tags(key, span);
            let buckets = Value::List {
                vals: histogram
                    .buckets
                    .iter()
                    .map(|bucket| Value::Record {
                        cols: vec!["bucket".to_string(), "count".to_string()],
                        vals: vec![
                            Value::Float {
                                val: bucket.bucket,
                                span,
                            },
                            Value::Int {
                                val: bucket.count as i64,
                                span,
                            },
                        ],
                        span,
                    })
                    .collect::<Vec<_>>(),

                span,
            };

            Value::Record {
                cols: vec![
                    "name".to_string(),
                    "type".to_string(),
                    "tags".to_string(),
                    "buckets".to_string(),
                    "sum".to_string(),
                    "count".to_string(),
                ],
                vals: vec![
                    name.clone(),
                    Value::String {
                        val: "Histogram".to_string(),
                        span,
                    },
                    tags,
                    buckets,
                    Value::Float {
                        val: histogram.sum,
                        span,
                    },
                    Value::Int {
                        val: histogram.count as i64,
                        span,
                    },
                ],
                span,
            }
        })
        .collect()
}

// Converts the summary.
fn summary_to_values<'a>(
    summary: impl IntoIterator<Item = (&'a GroupKey, &'a SummaryMetric)>,
    span: Span,
    name: Value,
) -> Vec<Value> {
    summary
        .into_iter()
        .map(|(key, summary)| {
            let tags = get_tags(key, span);
            let quantiles = Value::List {
                vals: summary
                    .quantiles
                    .iter()
                    .map(|quantile| Value::Record {
                        cols: vec!["quantile".to_string(), "value".to_string()],
                        vals: vec![
                            Value::Float {
                                val: quantile.quantile,
                                span,
                            },
                            Value::Float {
                                val: quantile.value,
                                span,
                            },
                        ],
                        span,
                    })
                    .collect::<Vec<_>>(),

                span,
            };

            Value::Record {
                cols: vec![
                    "name".to_string(),
                    "type".to_string(),
                    "tags".to_string(),
                    "quantiles".to_string(),
                    "sum".to_string(),
                    "count".to_string(),
                ],
                vals: vec![
                    name.clone(),
                    Value::String {
                        val: "Summary".to_string(),
                        span,
                    },
                    tags,
                    quantiles,
                    Value::Float {
                        val: summary.sum,
                        span,
                    },
                    Value::Int {
                        val: summary.count as i64,
                        span,
                    },
                ],
                span,
            }
        })
        .collect()
}
