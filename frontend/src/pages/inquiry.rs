#![allow(non_snake_case)]
use crate::{
    config::inquiry_products,
    seo::{inquiry_seo, PageSeo},
    utils::api,
    Route,
};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
enum Step {
    Contact,
    Products,
    Submit,
}

#[derive(Clone, Default)]
struct FormState {
    company: String,
    name: String,
    email: String,
    phone: String,
    country: String,
    products: Vec<String>,
    volume: String,
    incoterms: String,
    message: String,
    submitted: bool,
    submitting: bool,
    error: Option<String>,
}

#[component]
pub fn InquiryPage() -> Element {
    let mut step = use_signal(|| Step::Contact);
    let mut form = use_signal(FormState::default);

    let products_list = inquiry_products();

    if form.read().submitted {
        return rsx! {
            PageSeo { ..inquiry_seo() }
            section { class: "inquiry-success",
                div { class: "success-icon", "✓" }
                h1 { "Inquiry Received!" }
                p { class: "success-desc",
                    "Thank you, "
                    strong { "{form.read().name}" }
                    ". Our export team will respond to "
                    strong { "{form.read().email}" }
                    " within 24 business hours."
                }
                div { class: "success-actions",
                    Link { to: Route::ProductsPage {}, class: "btn btn-outline", "Browse More Products" }
                    Link { to: Route::HomePage {}, class: "btn btn-ghost", "Back to Home" }
                }
            }
        };
    }

    rsx! {
        PageSeo { ..inquiry_seo() }

        section { class: "page-hero page-hero-sm",
            div { class: "page-hero-content",
                nav { class: "breadcrumb", "aria-label": "Breadcrumb",
                    ol { role: "list",
                        li { Link { to: Route::HomePage {}, "Home" } }
                        li { "aria-current": "page", "Request a Quote" }
                    }
                }
                h1 { class: "page-title", "Request a Seafood Price Quote" }
                p { class: "page-subtitle",
                    "Fill in your requirements below. Our export team responds within 24 hours
                    with pricing, availability, and lead times."
                }
            }
        }

        section { class: "inquiry-section",
            // Step indicator
            div { class: "step-indicator", role: "list", "aria-label": "Form steps",
                div {
                    class: if step() == Step::Contact { "step-dot active" } else { "step-dot done" },
                    role: "listitem",
                    "aria-current": if step() == Step::Contact { "step" } else { "false" },
                    span { "1" }
                    span { class: "step-label", "Your Details" }
                }
                div { class: "step-line" }
                div {
                    class: match step() {
                        Step::Products => "step-dot active",
                        Step::Submit  => "step-dot done",
                        _             => "step-dot",
                    },
                    role: "listitem",
                    span { "2" }
                    span { class: "step-label", "Products" }
                }
                div { class: "step-line" }
                div {
                    class: if step() == Step::Submit { "step-dot active" } else { "step-dot" },
                    role: "listitem",
                    span { "3" }
                    span { class: "step-label", "Details & Send" }
                }
            }

            // ── Step 1: Contact details ─────────────────────────────
            if step() == Step::Contact {
                div { class: "inquiry-step",
                    h2 { "Step 1: Your Contact Details" }
                    div { class: "form-grid",
                        div { class: "form-field",
                            label { r#for: "company", "Company Name *" }
                            input {
                                id: "company",
                                r#type: "text",
                                placeholder: "Acme Seafood Imports Ltd.",
                                autocomplete: "organization",
                                value: "{form.read().company}",
                                oninput: move |e| form.write().company = e.value(),
                            }
                        }
                        div { class: "form-field",
                            label { r#for: "contact-name", "Contact Name *" }
                            input {
                                id: "contact-name",
                                r#type: "text",
                                placeholder: "John Smith",
                                autocomplete: "name",
                                value: "{form.read().name}",
                                oninput: move |e| form.write().name = e.value(),
                            }
                        }
                        div { class: "form-field",
                            label { r#for: "email", "Email Address *" }
                            input {
                                id: "email",
                                r#type: "email",
                                placeholder: "john@acmeseafood.com",
                                autocomplete: "email",
                                value: "{form.read().email}",
                                oninput: move |e| form.write().email = e.value(),
                            }
                        }
                        div { class: "form-field",
                            label { r#for: "phone", "Phone / WhatsApp" }
                            input {
                                id: "phone",
                                r#type: "tel",
                                placeholder: "+1 555 000 0000",
                                autocomplete: "tel",
                                value: "{form.read().phone}",
                                oninput: move |e| form.write().phone = e.value(),
                            }
                        }
                        div { class: "form-field form-field-full",
                            label { r#for: "country", "Country *" }
                            input {
                                id: "country",
                                r#type: "text",
                                placeholder: "United States",
                                autocomplete: "country-name",
                                value: "{form.read().country}",
                                oninput: move |e| form.write().country = e.value(),
                            }
                        }
                    }
                    div { class: "form-actions",
                        button {
                            class: "btn btn-primary",
                            disabled: "{form.read().company.is_empty() || form.read().email.is_empty() || form.read().name.is_empty() || form.read().country.is_empty()}",
                            onclick: move |_| step.set(Step::Products),
                            "Next: Select Products →"
                        }
                    }
                }
            }

            // ── Step 2: Product selection ───────────────────────────
            if step() == Step::Products {
                div { class: "inquiry-step",
                    h2 { "Step 2: Products of Interest" }
                    p { class: "step-hint", "Select all products you'd like to enquire about." }
                    div { class: "product-selector",
                        {products_list.iter().map(|(prod_id, name)| {
                            let id_owned = prod_id.to_string();
                            let id_for_check = id_owned.clone();
                            let id_for_handler = id_owned.clone();
                            let is_checked = form.read().products.contains(&id_for_check);
                            let name_str = name.to_string();
                            rsx! {
                                label { class: if is_checked { "product-check-label checked" } else { "product-check-label" },
                                    input {
                                        r#type: "checkbox",
                                        checked: "{is_checked}",
                                        onchange: move |e: Event<FormData>| {
                                            let mut f = form.write();
                                            if e.checked() {
                                                if !f.products.contains(&id_for_handler) {
                                                    f.products.push(id_for_handler.clone());
                                                }
                                            } else {
                                                f.products.retain(|p| p != &id_for_handler);
                                            }
                                        }
                                    }
                                    span { "{name_str}" }
                                }
                            }
                        })}
                    }
                    div { class: "form-actions form-actions-split",
                        button {
                            class: "btn btn-ghost",
                            onclick: move |_| step.set(Step::Contact),
                            "← Back"
                        }
                        button {
                            class: "btn btn-primary",
                            disabled: "{form.read().products.is_empty()}",
                            onclick: move |_| step.set(Step::Submit),
                            "Next: Finalise →"
                        }
                    }
                }
            }

            // ── Step 3: Volume, incoterms, message ─────────────────
            if step() == Step::Submit {
                div { class: "inquiry-step",
                    h2 { "Step 3: Volume & Message" }
                    div { class: "form-grid",
                        div { class: "form-field",
                            label { r#for: "volume", "Estimated Volume (MT/year)" }
                            input {
                                id: "volume",
                                r#type: "text",
                                placeholder: "e.g. 500 MT/year",
                                value: "{form.read().volume}",
                                oninput: move |e| form.write().volume = e.value(),
                            }
                        }
                        div { class: "form-field",
                            label { r#for: "incoterms", "Preferred Incoterms" }
                            select {
                                id: "incoterms",
                                onchange: move |e| form.write().incoterms = e.value(),
                                option { value: "", "Select Incoterms" }
                                option { value: "FOB", "FOB (Free on Board)" }
                                option { value: "CIF", "CIF (Cost, Insurance & Freight)" }
                                option { value: "CFR", "CFR (Cost & Freight)" }
                                option { value: "EXW", "EXW (Ex Works)" }
                                option { value: "DAP", "DAP (Delivered at Place)" }
                            }
                        }
                        div { class: "form-field form-field-full",
                            label { r#for: "message", "Additional Requirements" }
                            textarea {
                                id: "message",
                                rows: "5",
                                placeholder: "Specific size grades, processing forms, packaging requirements, target delivery port, timeline, or any other details...",
                                value: "{form.read().message}",
                                oninput: move |e| form.write().message = e.value(),
                            }
                        }
                    }

                    // Summary
                    div { class: "inquiry-summary",
                        h3 { "Summary" }
                        dl {
                            dt { "Company" } dd { "{form.read().company}" }
                            dt { "Contact" } dd { "{form.read().name} — {form.read().email}" }
                            dt { "Country" }  dd { "{form.read().country}" }
                            dt { "Products" }
                            dd {
                                ul {
                                    for p in &form.read().products {
                                        li { "{p}" }
                                    }
                                }
                            }
                        }
                    }

                    if let Some(err) = &form.read().error {
                        div { class: "form-error", role: "alert", "{err}" }
                    }

                    div { class: "form-actions form-actions-split",
                        button {
                            class: "btn btn-ghost",
                            onclick: move |_| step.set(Step::Products),
                            "← Back"
                        }
                        button {
                            class: "btn btn-primary btn-lg",
                            disabled: "{form.read().submitting}",
                            onclick: move |_| {
                                let f = form.read().clone();
                                form.write().submitting = true;
                                form.write().error = None;
                                wasm_bindgen_futures::spawn_local(async move {
                                    match api::submit_inquiry(
                                        &f.company, &f.name, &f.email, &f.phone,
                                        &f.country, &f.products, &f.volume,
                                        &f.incoterms, &f.message,
                                    ).await {
                                        Ok(_) => {
                                            form.write().submitted = true;
                                            form.write().submitting = false;
                                        }
                                        Err(e) => {
                                            form.write().error = Some(e);
                                            form.write().submitting = false;
                                        }
                                    }
                                });
                            },
                            if form.read().submitting { "Sending…" } else { "Submit Inquiry →" }
                        }
                    }
                }
            }

            // Trust signals below form
            div { class: "inquiry-trust",
                div { class: "trust-item",
                    span { class: "trust-icon", "🔒" }
                    p { "Your details are kept strictly confidential and used only to respond to your inquiry." }
                }
                div { class: "trust-item",
                    span { class: "trust-icon", "⚡" }
                    p { "We respond within 24 business hours — typically sooner for urgent requests." }
                }
                div { class: "trust-item",
                    span { class: "trust-icon", "💳" }
                    p { "95%+ of our export revenue is LC-backed — secure, transparent transactions." }
                }
            }
        }
    }
}
