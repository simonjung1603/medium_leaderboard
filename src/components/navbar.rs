use crate::components::app::FAVICON;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        header { class: "p-3 text-bg-dark",
            div { class: "container",
                div { class: "d-flex flex-wrap align-items-center justify-content-center justify-content-lg-start",
                    a {
                        href: "/",
                        class: "d-flex align-items-center mb-2 mb-lg-0 text-white text-decoration-none",
                        img {
                            width: "40",
                            role: "img",
                            height: "32",
                            "aria-label": "Bootstrap",
                            class: "bi me-2",
                            src: FAVICON
                        }
                    }
                    ul { class: "nav col-lg-auto me-lg-auto mb-2 justify-content-center mb-md-0",
                        li {
                            a { href: "#", class: "nav-link px-2 text-secondary", "Home" }
                        }
                        li {
                            a { href: "#", class: "nav-link px-2 text-white", "Poetry Competition" }
                        }
                        li {
                            a { href: "#", class: "nav-link px-2 text-white", "Fiction Competition" }
                        }
                        li {
                            a { href: "#", class: "nav-link px-2 text-white", "Personal Essay Competition" }
                        }
                    }
                    div{
                        class: "col-sm-auto",
                        div{class: "row align-items-center justify-content-between",
                            a { href: "#", class: "col mx-4 nav-link text-white", "About" }
                            div { class: "col",
                                button { r#type: "button", class: "btn mx-3 btn-outline-light", "Login" }
                            }
                        }
                    }
                }
            }
        }
    }
}
