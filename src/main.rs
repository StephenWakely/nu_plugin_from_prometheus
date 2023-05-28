use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_from_prometheus::FromPrometheus;

fn main() {
    serve_plugin(&mut FromPrometheus, MsgPackSerializer {})
}
