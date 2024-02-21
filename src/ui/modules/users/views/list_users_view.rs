use crate::ui::layout::Layout;

use crate::domain::modules::users::user_model::User;
use dioxus::prelude::*;

struct Props {
    users: Vec<User>,
}

// Take a Vec<User> and create an HTML table.
pub fn users(users: Vec<User>) -> String {
    // Inner function to create our rsx! component
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {    // <-- Use our layout
                title: "Users Table",
                table {
                    thead {
                        tr {
                            th { "ID" }
                            th { "Email" }
                        }
                    }
                    tbody {
                        cx.props.users.iter().map(|user| {
                            let id_display = match user.id {
                                Some(id) => format!("{}", id),
                                None => "None".to_string(),
                            };

                            rsx!(
                                tr {
                                    td {
                                        strong {
                                            "{id_display}"
                                        }
                                    }
                                    td {
                                        "{user.email}"
                                    }
                                }
                            )
                        })
                    }
                }
            }
        })
    }

    // Construct our component and render it to a string.
    let mut app = VirtualDom::new_with_props(app, Props { users });
    let _ = app.rebuild();
    format!(
        "<!DOCTYPE html><html lang='en'>{}</html>",
        dioxus_ssr::render(&app)
    )
}
