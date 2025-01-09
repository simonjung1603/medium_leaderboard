use crate::components::clap_chart::ClapChart;
use crate::components::config_info_table::ConfigInfoTable;
use crate::components::hero::Hero;
use crate::components::leaderboard_table::*;
use crate::components::navbar::Navbar;
use crate::models::{Category, Submission};
use crate::server_functions::*;
use dioxus::html::a::draggable;
use dioxus::logger::tracing;
use dioxus::prelude::*;
#[cfg(feature = "web")]
use web_sys::js_sys;

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
const BOOTSTRAP: Asset = asset!("/assets/styling/bootstrap.min.css");
const BOOTSTRAP_JS: Asset = asset!("/assets/scripts/bootstrap.bundle.min.js");
const FONTAWESOME: Asset = asset!("/assets/scripts/98b204fec6.js");
const PLOTLY: Asset = asset!("/assets/scripts/plotly-2.14.0.min.js");
const THEME_SWITCHER: Asset = asset!("/assets/scripts/theme-switcher.js");

#[derive(Default, PartialEq, Clone)]
pub struct SubmissionsByCategory {
    pub unsorted: Vec<Submission>,
    pub poetry: Vec<Submission>,
    pub fiction: Vec<Submission>,
    pub essay: Vec<Submission>,
}

fn get_submissions_by_category(
    submission_elements: Resource<Result<Vec<Submission>, ServerFnError>>,
) -> Option<SubmissionsByCategory> {
    if let Some(Ok(all_submissions)) = &*submission_elements.read_unchecked() {
        Some(SubmissionsByCategory {
            unsorted: all_submissions
                .iter()
                .filter(|sub| sub.category == Category::None)
                .cloned()
                .collect(),
            poetry: all_submissions
                .iter()
                .filter(|sub| sub.category == Category::Poetry)
                .cloned()
                .collect(),
            fiction: all_submissions
                .iter()
                .filter(|sub| sub.category == Category::Fiction)
                .cloned()
                .collect(),
            essay: all_submissions
                .iter()
                .filter(|sub| sub.category == Category::PersonalEssay)
                .cloned()
                .collect(),
        })
    } else {
        None
    }
}

#[component]
pub fn App() -> Element {
    let dragged_guid = use_signal(|| None);

    let submissions_by_category = {
        let submission_elements = use_resource(move || async move {
            dragged_guid.read();
            get_all_submissions().await
        });
        use_memo(move || get_submissions_by_category(submission_elements))
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link{ rel: "stylesheet", href: BOOTSTRAP }
        script { src: THEME_SWITCHER }
        script { src: BOOTSTRAP_JS }
        script { src: FONTAWESOME }
        script { src: PLOTLY }

        Navbar{}
        Hero {}

        div{class: "container-fluid",

            if let Some(subs) = &*submissions_by_category.read_unchecked(){
                if subs.unsorted.is_empty() == false{
                    LeaderboardTable{
                        category: Category::None,
                        elements: subs.unsorted.clone(),
                        dragged_guid
                    }
                }
                div{class: "row mt-4",
                    div{class: "col",
                        LeaderboardTable{
                            category: Category::Poetry,
                            elements: subs.poetry.clone(),
                            dragged_guid
                        }
                    }
                    div{class: "col",
                        LeaderboardTable{
                            category: Category::Fiction,
                            elements: subs.fiction.clone(),
                            dragged_guid
                        }
                    }
                    div{class: "col",
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
}
