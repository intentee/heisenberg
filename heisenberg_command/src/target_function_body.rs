use heisenberg_module_graph::function_body::FunctionBody;
use heisenberg_project_model::supported_target::SupportedTarget;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TargetFunctionBody<'body> {
    pub body: &'body FunctionBody,
    pub target: SupportedTarget,
}
