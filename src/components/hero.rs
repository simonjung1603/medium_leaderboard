use crate::components::config_info_table::ConfigInfoTable;
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        div{class: "container py-4",
            div{class: "row align-items-center",
                div{class: "col",
                    h1{"Transformation"}
                    small{"A " em{"My Fair Lighthouse"} " writing contest"}
                }
                div{class: "col-2 small",
                    ConfigInfoTable{}
                }
            }
        }
    }
}
