use graph_process_manager_core::manager::config::AbstractProcessParameterization;
use crate::process::ana::param::anakind::AnalysisKind;

pub struct AnalysisParameterization {
    pub ana_kind : AnalysisKind,
    pub use_locana : bool
}

impl AnalysisParameterization {
    pub fn new(ana_kind: AnalysisKind, use_locana: bool) -> Self {
        AnalysisParameterization { ana_kind, use_locana }
    }
}

impl AbstractProcessParameterization for AnalysisParameterization {
    fn get_param_as_strings(&self) -> Vec<String> {
        vec![ "process = analysis".to_string(),
              format!("analysis kind = {}", self.ana_kind.to_string()),
              format!("local analysis = {}", self.use_locana.to_string()) ]
    }
}