use crate::FromPrometheus;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Type, Value};

impl Plugin for FromPrometheus {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("from prometheus")
            .usage("Converts from prometheus scrape data to table")
            .input_output_types(vec![(Type::String, Type::Table(vec![]))])
            .usage("Parse text as prometheus metrics.")
            .plugin_examples(vec![PluginExample {
                example: "from prometheus".into(),
                description: "".into(),
                result: None,
            }])
            .category(Category::Formats)]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            "from prometheus" => self.convert(call, input),
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature fromprom".into(),
                span: Some(call.head),
            }),
        }
    }
}
