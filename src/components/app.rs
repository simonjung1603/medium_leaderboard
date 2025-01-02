use crate::components::config_info_table::ConfigInfoTable;
use crate::components::clap_chart::ClapChart;
use crate::server_functions::*;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use crate::components::leaderboard_table::*;
use crate::models::{Category, Submission};
#[cfg(feature = "web")]
use web_sys::js_sys;

const FAVICON: Asset = asset!("/assets/favicon.ico");

#[derive(Default, PartialEq, Clone)]
pub struct SubmissionsByCategory {
    pub unsorted: Vec<Submission>,
    pub poetry: Vec<Submission>,
    pub fiction: Vec<Submission>,
    pub essay: Vec<Submission>,
}

#[component]
pub fn App() -> Element {
    let submission_elements = use_resource(get_all_submissions);
    let dragged_guid = use_signal(|| None);

    let submissions_by_category = use_memo(move || {
        if let Some(Ok(all_submissions)) = &*submission_elements.read_unchecked() {
            Some(SubmissionsByCategory {
                unsorted: all_submissions.iter().filter(|sub| sub.category == Category::None).cloned().collect(),
                poetry: all_submissions.iter().filter(|sub| sub.category == Category::Poetry).cloned().collect(),
                fiction: all_submissions.iter().filter(|sub| sub.category == Category::Fiction).cloned().collect(),
                essay: all_submissions.iter().filter(|sub| sub.category == Category::PersonalEssay).cloned().collect(),
            })
        } else {
            None
        }
    });

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: "https://cdnjs.cloudflare.com/ajax/libs/bulma/1.0.2/css/bulma.min.css" }
        script{src: "https://kit.fontawesome.com/98b204fec6.js", crossorigin:"anonymous"}
        script{src: "https://cdnjs.cloudflare.com/ajax/libs/Chart.js/4.4.1/chart.umd.js"}
        script{src: "https://cdn.plot.ly/plotly-2.14.0.min.js"}

        section{class:"hero has-background-secondary",
            div{class:"hero-body",
                div{class: "columns is-vcentered",
                    div{class: "column",
                        p{class:"title", "Transformation"}
                        p{class:"subtitle",
                            p{"A " em{"My Fair Lighthouse"} " writing contest"}
                        }
                    }
                    ConfigInfoTable{}
                }
            }
        }

        if let Some(subs) = &*submissions_by_category.read_unchecked(){
            if subs.unsorted.is_empty() == false{
                LeaderboardTable{
                    category: Category::None,
                    elements: subs.unsorted.clone(),
                    dragged_guid
                }
            }
            div{class: "columns is-centered ml-6 mr-6",
                div{class: "column is-one-third",
                    LeaderboardTable{
                        category: Category::Poetry,
                        elements: subs.poetry.clone(),
                        dragged_guid
                    }
                }
                div{class: "column is-one-third",
                    LeaderboardTable{
                        category: Category::Fiction,
                        elements: subs.fiction.clone(),
                        dragged_guid
                    }
                }
                div{class: "column is-one-third",
                    LeaderboardTable{
                        category: Category::PersonalEssay,
                        elements: subs.essay.clone(),
                        dragged_guid
                    }
                }
            }
        }
        ClapChart{id: "clap_chart".to_string(), submissions_by_category}
    }
}
