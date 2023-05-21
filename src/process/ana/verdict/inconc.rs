use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash)]
pub enum InconcReason {
    LackObs,
    UsingLifelineRemovalWithCoLocalizations,
    FilteredNodes
}

impl InconcReason {

    pub fn get_explanation_string(&self) -> String {
        match self {
            InconcReason::LackObs => {
                return "due to a lack of observation in the multi-trace (events not at the end globally may be missing) -> rather use hiding or simulation".to_string();
            },
            InconcReason::UsingLifelineRemovalWithCoLocalizations => {
                return "due to having non-singleton co-localizations on the multi-trace while using the lifeline-removal-based algorithm. WeakPasses may be false positives because using lifeline elimination may remove strict orderings between events occurring on distinct lifelines".to_string();
            },
            InconcReason::FilteredNodes => {
                return "due to having set a filter which forcefully limited exploration of the graph : Fails may be false negative".to_string();
            }
        }
    }

}

impl fmt::Display for InconcReason {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InconcReason::LackObs => {
                write!(f,"LackObs")
            },
            InconcReason::UsingLifelineRemovalWithCoLocalizations => {
                write!(f,"UsingLifelineRemovalWithCoLocalizations")
            },
            InconcReason::FilteredNodes => {
                write!(f,"FilteredNodes")
            }
        }
    }

}