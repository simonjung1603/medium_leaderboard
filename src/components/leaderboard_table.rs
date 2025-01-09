use crate::server_functions::update_category;
use dioxus::prelude::*;
use dioxus::logger::tracing;
use crate::models::{Category, Submission};

#[component]
pub fn LeaderboardTable(category: Category, elements: Vec<Submission>, dragged_guid: Signal<Option<String>>) -> Element {
    rsx! {
            div{class: "h1 text-center",
                ondragover: |ev| ev.prevent_default(),
                ondrop: move |ev| async move {
                    tracing::info!("OnDrop: {:?}", ev);
                    tracing::info!("guid: {:?}", dragged_guid);

                    if let Some(guid) = dragged_guid(){
                        tracing::info!("Calling backend");
                        if let Err(err) = update_category(guid, category).await{
                            tracing::error!("Got err: {}", err);
                        }
                    }

                    dragged_guid.set(None);
                },
                {match category{
                    Category::None => "Submissions need sorting",
                    Category::Poetry => "Poetry submissions",
                    Category::Fiction => "Fictions submissions",
                    Category::PersonalEssay => "Personal essay submissions",
                }}
            }
            table{class: "table mt-6 table-hover table-striped table-bordered",
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
                    class: "table-group-divider",
                    for (i, submission) in elements.iter().cloned().enumerate(){
                        tr{
                            draggable: true,
                            ondragstart: move |ev| {
                                dragged_guid.set(Some(submission.guid.clone()));
                                tracing::info!("DragStart: {:?}", ev);
                                tracing::info!("guid: {:?}", dragged_guid);
                            },
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