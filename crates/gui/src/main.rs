use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        AppHeader {}
        AppNav {}
        AppBody {}
        AppFooter {}
    }
}

#[component]
fn AppHeader() -> Element {
    rsx! {
        header {
            h1 { "Stowr" }
        }
    }
}

#[component]
fn AppBody() -> Element {
    rsx![
        main {
            Locations {}
            Assets {}
        }
    ]
}

#[component]
fn AppFooter() -> Element {
    rsx! {
        footer {
            p { "© 2025 Stowr" }
        }
    }
}

#[component]
fn AppNav() -> Element {
    rsx! {
        nav { aria_label: "Global",
            ul { class: "global-nav",
                li {
                    a { id: "nav-locations", href: "#", "Locations" }
                }
                li {
                    a { id: "nav-assets", href: "#", "Assets" }
                }
                li {
                    a { id: "nav-settings", href: "#", "Settings" }
                }
            }
        }
    }
}

#[component]
fn Locations() -> Element {
    rsx! {
        aside { id: "locations-panel", aria_label: "Locations",
            header {
                h2 { "Locations" }
                button { id: "btn-add-location", "+ Add Location" }
            }
            nav {
                ul { id: "location-list",
                    {(0..5).map(|i| rsx! {
                        li {
                            a { href: "#", "Location {i}" }
                        }
                    })}
                }
            }
            LocationForm {}
        }
    }
}

#[component]
fn Assets() -> Element {
    rsx! {
        section { id: "assets-panel", aria_label: "Assets",
            header {
                h2 {
                    "Assets in "
                    span { id: "selected-location-name", "All Locations" }
                }
                button { id: "btn-add-asset", "+ Add Asset" }
            }
            article {
                table { id: "asset-table",
                    thead {
                        tr {
                            th { "Name" }
                            th { "Description" }
                            th { "Quantity" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        {(0..5).map(|i| rsx! {
                            tr {
                                td {
                                    a { href: "#", "Asset {i}" }
                                }
                                td { "This is asset {i}" }
                                td { "{i * 2}" }
                                td { "[+] | [-]" }
                            }
                        })}
                    }
                }
            }
        }
        AssetForm {}
    }
}

#[component]
fn LocationForm() -> Element {
    rsx! {
        div {
            id: "modal-location-form",
            class: "modal",
            role: "dialog",
            "aria-modal": "true",
            "aria-labelledby": "location-form-title",
            form { id: "location-form",
                div { id: "form-flex",
                    div {
                        header {
                            h3 { id: "location-form-title", "Add/Edit Location" }
                        }
                    }
                    div {
                        label { r#for: "location-name", "Name" }
                        input {
                            r#type: "text",
                            id: "location-name",
                            name: "name",
                            required: true,
                        }
                    }
                    div {
                        label { r#for: "location-description", "Description" }
                        textarea { id: "location-description", name: "description" }
                    }
                    footer {
                        button { r#type: "submit", "Save" }
                        button { r#type: "button", id: "btn-cancel-location", "Cancel" }
                    }
                }
            }
        }
    }
}

#[component]
fn AssetForm() -> Element {
    rsx![
        div {
            id: "modal-asset-form",
            class: "modal",
            role: "dialog",
            "aria-modal": "true",
            "aria-labelledby": "asset-form-title",
            hidden: false,
            form { id: "asset-form",
                header {
                    h3 { id: "asset-form-title", "Add/Edit Asset" }
                }
                label { r#for: "asset-name", "Name" }
                input {
                    r#type: "text",
                    id: "asset-name",
                    name: "name",
                    required: true,
                }
                label { r#for: "asset-description", "Description" }
                textarea { id: "asset-description", name: "description" }
                label { r#for: "asset-quantity", "Quantity" }
                input {
                    r#type: "number",
                    id: "asset-quantity",
                    name: "quantity",
                    min: "0",
                    required: true,
                }
                footer {
                    button { r#type: "submit", "Save" }
                    button { r#type: "button", id: "btn-cancel-asset", "Cancel" }
                }
            }
        }
    ]
}
