

use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::MultiTrace;
use crate::core::general_context::GeneralContext;


pub struct AnalysisContext {
    pub gen_ctx : GeneralContext,
    pub co_localizations : CoLocalizations,
    pub multi_trace : MultiTrace,
    pub init_multitrace_length : usize,
}

impl AnalysisContext {
    pub fn new(gen_ctx: GeneralContext, co_localizations: CoLocalizations, multi_trace: MultiTrace, init_multitrace_length: usize) -> Self {
        AnalysisContext { gen_ctx, co_localizations, multi_trace, init_multitrace_length }
    }
}
