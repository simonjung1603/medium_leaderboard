use crate::server_functions::get_latest_and_next_update_time;
use dioxus::logger::tracing;
use dioxus::prelude::*;

#[component]
pub fn ConfigInfoTable() -> Element {
    let latest_update_time = use_resource(get_latest_and_next_update_time);
    let time_fmt = "%H:%M";
    let (latest, next) = match &*latest_update_time.read_unchecked() {
        None => ("...".to_string(), "...".to_string()),
        Some(Ok((latest, next))) => (
            latest.format(time_fmt).to_string(),
            next.format(time_fmt).to_string(),
        ),
        Some(Err(err)) => {
            tracing::error!("{}", err);
            ("---".to_string(), "---".to_string())
        }
    };

    rsx! {
        table{class: "table table-hover table-striped table-sm",
            tr{
                td{"Version"}
                td{"0.0.1"}
            }
            tr{
                td{"Claps last updated"}
                td{{latest}}
            }
            tr{
                td{"Next scheduled update"}
                td{{next}}
            }
        }
    }
}