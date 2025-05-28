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
        header {
          h1 { "Inventory Manager" }
          nav { aria_label: "Global",
            ul { class: "global-nav",
              li { a { id: "nav-locations", href: "#", "Locations" } }
              li { a { id: "nav-assets", href: "#", "Assets" } }
              li { a { id: "nav-settings", href: "#", "Settings" } }
            }
          }
        }

        main {
            aside { id: "locations-panel", aria_label: "Locations",
                header { h2 { "Locations" } button { id: "btn-add-location", "+ Add Location" } }
                nav { ul { id: "location-list", /* dynamically populated */ } }
            }

            section { id: "assets-panel", aria_label: "Assets",
                header {
                    h2 { "Assets in " span { id: "selected-location-name", "All Locations" } }
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
                        tbody { /* dynamically populated rows */ }
                    }
                }
            }
        }

        div { id: "modal-location-form", class: "modal", role: "dialog", "aria-modal": "true", "aria-labelledby": "location-form-title", hidden: true,
            form { id: "location-form",
                header { h3 { id: "location-form-title", "Add/Edit Location" } }
                label { for: "location-name", "Name" }
                input { r#type: "text", id: "location-name", name: "name", required: true }
                label { for: "location-description", "Description" }
                textarea { id: "location-description", name: "description" }
                footer {
                    button { r#type: "submit", "Save" }
                    button { r#type: "button", id: "btn-cancel-location", "Cancel" }
                }
            }
        }

        div { id: "modal-asset-form", class: "modal", role: "dialog", "aria-modal": "true", "aria-labelledby": "asset-form-title", hidden: true,
            form { id: "asset-form",
                header { h3 { id: "asset-form-title", "Add/Edit Asset" } }
                label { for: "asset-name", "Name" }
                input { r#type: "text", id: "asset-name", name: "name", required: true }
                label { for: "asset-description", "Description" }
                textarea { id: "asset-description", name: "description" }
                label { for: "asset-quantity", "Quantity" }
                input { r#type: "number", id: "asset-quantity", name: "quantity", min: "0", required: true }
                footer {
                    button { r#type: "submit", "Save" }
                    button { r#type: "button", id: "btn-cancel-asset", "Cancel" }
                }
            }
        }

        footer { p { "© 2025 Inventory CRUD App" } }
    }
}
