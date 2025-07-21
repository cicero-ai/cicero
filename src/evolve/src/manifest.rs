
use std::collections::HashMap;
///  names are alpha-numeric strings separated by slahes (eg. server/linx/http/nginx).
struct FullyQualifiedName {
    index: i32,
    children: HashMap<String, Box<FullyQualifiedName>>
}

/// Keys of hash maps are the fully qualified name
Schema {
    fqn: FullyQualifiedName,
    parameters: HashMap<i32, Parameter>,
    task_kits: HashMap<i32, TaskKit>,
    filters: HashMap<i32, Filter>,
    formatters: HashMap<i32, Formatter>
}

struct Parameter {
    name: i32,
    description: String,
    data_type: ParameterType,
    choices: Option<Box<dyn ParameterChoices>>,
    default_top_k: usize,
    default_precision: Option<f32>,
    default_threshold: Option<f32>,
    default_temperature: Option<f32>
}

enum ParameterType {
    mixed,
    boolean,
    string,
    integer,
    float,
    range,
    byte_array,
    vector(ParameterType),
    hash(ParameterType, ParameterType)
}

trait ParameterChoices {
    fn get_choices(&self) -> String;
}

struct TaskKit {
    name: i32,
    description: String,
    autorun: bool
}

struct Task {
    name: i32,
    set_id: Option<i32>,
    extends: Option<i32>,
    description: String,
    input_parameters: Vec<TaskInputParameter>,
    output_parameters: ParameterType,
    executor: Box<dyn TaskExecutor>
}

struct TaskInputParameter<T> {
    name: i32,
    data_type: ParamterType,
    is_required: bool,
    default_value: Option<T>
}
trait TaskExecutor {
    fn execute(&self, manifest: &Manifest);
}

struct Filter {
    name: i32,
    description: String,
    input_parameters: Vec<TaskInputParameter>,
    output_parameters: ParameterType,
    executor: Box<dyn TaskExecutor>
}


