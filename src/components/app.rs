use dioxus::logger::tracing;
use crate::server_functions::*;
use dioxus::prelude::*;
use web_sys::js_sys;

const FAVICON: Asset = asset!("/assets/favicon.ico");

#[component]
pub fn App() -> Element {
    let submission_elements = use_resource(get_all_submissions);
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

    use_memo(move || {
        if let Some(Ok(subs)) = &*submission_elements.read_unchecked() {
            let titles = subs
                .iter()
                .map(|sub| {
                    if sub.title.chars().count() > 15 {
                        format!("'{:.12}...'", sub.title)
                    } else {
                        format!("'{}'", sub.title)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            let counts = subs
                .iter()
                .map(|sub| sub.clap_count.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            js_sys::eval(&format!(
                r#"
                const ctx = document.getElementById('my-chart').getContext('2d');
                new Chart(ctx, {{
                    type: 'bar',
                    data: {{
                        labels: [{}],
                        datasets: [{{
                            label: 'Amount of claps',
                            data: [{}],
                            backgroundColor: [
                              'rgba(255, 99, 132, 0.2)',
                              'rgba(255, 159, 64, 0.2)',
                              'rgba(255, 205, 86, 0.2)',
                              'rgba(75, 192, 192, 0.2)',
                              'rgba(54, 162, 235, 0.2)',
                              'rgba(153, 102, 255, 0.2)',
                              'rgba(201, 203, 207, 0.2)'
                            ],
                            borderColor: [
                              'rgb(255, 99, 132)',
                              'rgb(255, 159, 64)',
                              'rgb(255, 205, 86)',
                              'rgb(75, 192, 192)',
                              'rgb(54, 162, 235)',
                              'rgb(153, 102, 255)',
                              'rgb(201, 203, 207)'
                            ],
                        }}]
                    }},
                    options: {{
                        plugins: {{
                            legend: {{
                                labels: {{
                                    boxWidth: 0
                                }}
                            }}
                        }},
                        responsive: true,
                        scales: {{
                            y: {{ beginAtZero: true }}
                        }},
                    }}
                }});
            "#,
                titles, counts
            ))
                .expect("Failed to execute JavaScript");
        }
    });

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: "https://cdnjs.cloudflare.com/ajax/libs/bulma/1.0.2/css/bulma.min.css" }
        script{src: "https://kit.fontawesome.com/98b204fec6.js", crossorigin:"anonymous"}
        script{src: "https://cdnjs.cloudflare.com/ajax/libs/Chart.js/4.4.1/chart.umd.js"}

        section{class:"hero has-background-secondary",
            div{class:"hero-body",
                div{class: "columns is-vcentered",
                    div{class: "column",
                        p{class:"title", "Transformation"}
                        p{class:"subtitle",
                            p{"A " em{"My Fair Lighthouse"} " writing contest"}
                        }
                    }
                    div{class: "column is-two-fifth is-pulled-right is-flex is-justify-content-end",
                        table{class: "table has-text-weight-light has-background-secondary is-size-7 is-bordered is-narrow",
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
            }
        }

        div{class: "container",
            div{class: "columns is-centered",
                div{class: "column is-two-thirds",
                div{class: "title mt-6", "Community vote live standings"}
                if let Some(Ok(submission_elements)) = &*submission_elements.read_unchecked(){
                    table{class: "table mt-6 is-bordered is-striped is-hoverable is-fullwidth",
                        thead{
                                tr{
                                    th{"Rank"}
                                    th{"Title"}
                                    th{"Author"}
                                    th{"Claps " i{class: "fa-solid fa-arrow-down"}}
                                    th{"Word count"}
                                }
                            }
                        tbody{
                            for (i, submission) in submission_elements.iter().enumerate(){
                                tr{
                                    th{
                                        {format!("{}.", i+1)}
                                    }
                                    td{
                                        {submission.title.clone()}
                                    }
                                    td{
                                        a{
                                            href: format!("https://medium.com/@{}", submission.username.clone()),
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            {format!("@{}", submission.username.clone())}
                                        }
                                    }
                                    td{
                                        {submission.clap_count.to_string()}
                                    }
                                    td{
                                        {submission.word_count.to_string()}
                                    }
                                }
                            }
                        }
                    }
                }
                }
            }
        }

        div{class: "container is-max-tablet box mt-6",
            div {
                id: "chart-container",
                canvas {
                    id: "my-chart",
                }
            }
        }
    }
}
