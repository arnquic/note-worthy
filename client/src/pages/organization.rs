// src/pages/organization.rs
use dioxus::prelude::*;

use crate::handlers::auth::AUTH_CONTEXT;

#[component]
pub fn OrganizationPage() -> Element {
    if !AUTH_CONTEXT.read().is_authenticated {
        return rsx! {
            div { "Redirecting to login..." }
        };
    }

    rsx! {
        div {
            class: "min-h-screen bg-gray-100 p-4",
            div {
                class: "max-w-5xl mx-auto mt-6 bg-white rounded-lg p-6",
                h1 {
                    class: "text-2xl font-bold mb-4",
                    "Organization Settings"
                }
                div {
                    class: "space-y-6",
                    div {
                        h2 {
                            class: "text-xl font-bold mb-2",
                            "Practice Information"
                        }
                        div {
                            class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Practice Name"
                                }
                                input {
                                    class: "border border-gray-300 rounded-md p-2 w-full",
                                    r#type: "text",
                                    value: "Wellness Therapy Center"
                                }
                            }
                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-700 mb-1",
                                    "Contact Email"
                                }
                                input {
                                    class: "border border-gray-300 rounded-md p-2 w-full",
                                    r#type: "email",
                                    value: "contact@wellnesstherapy.com"
                                }
                            }
                        }
                    }
                    div {
                        h2 {
                            class: "text-xl font-bold mb-2",
                            "Team Members"
                        }
                        table {
                            class: "w-full border-collapse",
                            thead {
                                tr {
                                    th {
                                        class: "text-left border-b pb-2",
                                        "Name"
                                    }
                                    th {
                                        class: "text-left border-b pb-2",
                                        "Role"
                                    }
                                    th {
                                        class: "text-left border-b pb-2",
                                        "Email"
                                    }
                                }
                            }
                            tbody {
                                tr {
                                    td {
                                        class: "py-2",
                                        "Eva Hernandez"
                                    }
                                    td {
                                        class: "py-2",
                                        "Lead Therapist"
                                    }
                                    td {
                                        class: "py-2",
                                        "eva@wellnesstherapy.com"
                                    }
                                }
                                tr {
                                    td {
                                        class: "py-2",
                                        "Marcus Johnson"
                                    }
                                    td {
                                        class: "py-2",
                                        "Therapist"
                                    }
                                    td {
                                        class: "py-2",
                                        "marcus@wellnesstherapy.com"
                                    }
                                }
                            }
                        }
                        button {
                            class: "mt-2 bg-blue-800 text-white px-4 py-2 rounded text-sm",
                            "Add Team Member"
                        }
                    }
                }
            }
        }
    }
}
