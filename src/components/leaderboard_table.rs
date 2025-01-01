use dioxus::prelude::*;
use crate::models::Submission;


#[component]
pub fn LeaderboardTable(title: String, elements: Vec<Submission>) -> Element {
    rsx! {
            div{class: "title mt-6 has-text-centered", {title}}
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
                    for (i, submission) in elements.iter().enumerate(){
                        tr{
                            draggable: true,
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