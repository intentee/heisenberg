use heisenberg_module_graph::function_body::FunctionBody;
use heisenberg_project_model::configuration::Configuration;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ConfigurationFunctionBody<'body> {
    pub body: &'body FunctionBody,
    pub configuration: &'body Configuration,
}
