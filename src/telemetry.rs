#[cfg(not(feature = "telemetry"))]
mod disabled {
    pub fn init() -> anyhow::Result<()> {
        Ok(())
    }

    pub fn record_tcp_request() {}

    pub fn record_udp_request() {}
}

#[cfg(feature = "telemetry")]
mod enabled {
    use anyhow::Context;
    use lazy_static::lazy_static;
    use opentelemetry::metrics::{Counter, Meter};
    use opentelemetry::{global, KeyValue};
    use opentelemetry_sdk::Resource;
    use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
    use std::env::VarError;

    lazy_static! {
        static ref METER: Meter = global::meter("cookied");
        static ref NUM_REQUESTS: Counter<u64> = METER
            .u64_counter("num_requests")
            .with_unit("requests")
            .with_description("How many QOTD requests are received by the cookied server")
            .build();
    }

    pub fn init() -> anyhow::Result<()> {
        match std::env::var("OTEL_METRICS_EXPORTER") {
            Ok(value) => match value.as_ref() {
                "otlp" => init_otlp_exporter(),
                "console" => init_console_exporter(),
                "none" => Ok(()),
                other => Err(anyhow::anyhow!("Unsupported OTEL_METRICS_EXPORTER {other}")),
            },
            Err(VarError::NotPresent) => init_otlp_exporter(),
            Err(e) => Err(e.into()),
        }
    }

    fn init_otlp_exporter() -> anyhow::Result<()> {
        eprintln!("Initializing OpenTelemetry exporter");

        // initialize otlp exporter using HTTP protocol
        let exporter = opentelemetry_otlp::MetricExporter::builder()
            .with_http()
            .build()
            .context("Could not configure OpenTelemetry metric exporter")?;

        // register the exporter with the opentelemetry machinery
        let meter_provider = opentelemetry_sdk::metrics::SdkMeterProvider::builder()
            .with_resource(
                Resource::builder()
                    .with_attribute(KeyValue::new(SERVICE_NAME, "cookied"))
                    .build(),
            )
            .with_periodic_exporter(exporter)
            .build();
        global::set_meter_provider(meter_provider);

        Ok(())
    }

    fn init_console_exporter() -> anyhow::Result<()> {
        eprintln!("Initializing stdout metric exporter");

        let exporter = opentelemetry_stdout::MetricExporter::builder().build();
        let meter_provider = opentelemetry_sdk::metrics::SdkMeterProvider::builder()
            .with_resource(
                Resource::builder()
                    .with_attribute(KeyValue::new(SERVICE_NAME, "cookied"))
                    .build(),
            )
            .with_periodic_exporter(exporter)
            .build();
        global::set_meter_provider(meter_provider);

        Ok(())
    }

    pub fn record_tcp_request() {
        NUM_REQUESTS.add(1, &[KeyValue::new("proto", "tcp")])
    }

    pub fn record_udp_request() {
        NUM_REQUESTS.add(1, &[KeyValue::new("proto", "udp")])
    }
}

#[cfg(not(feature = "telemetry"))]
pub use disabled::*;
#[cfg(feature = "telemetry")]
pub use enabled::*;
