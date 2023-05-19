use crate::process::ana::verdict::inconc::InconcReason;

pub enum AnalysisLocalVerdict{
    Cov,
    TooShort,
    MultiPref,
    Slice,
    Inconc(InconcReason),
    Out(bool),   // bool for if it's known via local analysis
    OutSim(bool) // bool for if it's known via local analysis}
}

impl std::string::ToString for AnalysisLocalVerdict {

    fn to_string(&self) -> String {
        match self {
            AnalysisLocalVerdict::Cov => {
                return "Cov".to_string();
            },
            AnalysisLocalVerdict::TooShort => {
                return "TooShort".to_string();
            },
            AnalysisLocalVerdict::MultiPref => {
                return "MultiPref".to_string();
            },
            AnalysisLocalVerdict::Slice => {
                return "Slice".to_string();
            },
            AnalysisLocalVerdict::Inconc(reason) => {
                return format!("Inconc {:}", reason.to_string());
            },
            AnalysisLocalVerdict::Out(ref loc) => {
                if *loc {
                    return "Out-l".to_string();
                } else {
                    return "Out".to_string();
                }
            },
            AnalysisLocalVerdict::OutSim(ref loc) => {
                if *loc {
                    return "OutSim-l".to_string();
                } else {
                    return "OutSim".to_string();
                }
            }
        }
    }

}
