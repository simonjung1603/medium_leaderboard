use dioxus::prelude::*;

#[component]
pub fn Card() -> Element {
    rsx! {
        div {
            id: "first",
            class: "card",
            div {class: "card-image",
                figure{class: "image is4by3",
                    img { src: "https://bulma.io/assets/images/placeholders/1280x960.png", id: "header" }
                }
            }
            div {class: "card-content",
                div{class: "media",
                    div{class: "media-left",
                        figure{class: "image is-48x48",
                            img{src: "https://bulma.io/assets/images/placeholders/96x96.png"}
                        }
                    }
                    div{class: "media-content",
                        p{class: "title is-4", "John Smith"}
                        p{class: "subtitle is-6", "@johnsmith"}
                    }
                }

                div{class: "content",
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus nec
                    iaculis mauris."
                    br{}
                    time{datetime: "2016-1-1", "11:09 PM - 1 Jan 2016"}
                }
            }
        }
    }
}
