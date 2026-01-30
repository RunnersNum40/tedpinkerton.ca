use crate::*;

const RESUME_PDF: Asset = asset!("/assets/Theodore_Pinkerton_CV.pdf");

#[component]
pub fn Resume() -> Element {
    rsx! {
        Page {
            id: "resume",
            name: "Resume",
            body: rsx! {
                div {
                    iframe {
                        title: "Ted Pinkerton resume (PDF)",
                        src: "{RESUME_PDF}",
                        width: "100%",
                        class: "pdf-frame",
                    }
                    p { class: "sr-only", "If the PDF is not visible, use the links below." }
                    p { class: "pdf-actions",
                        a {
                            href: "{RESUME_PDF}",
                            download: "Theodore_Pinkerton_CV.pdf",
                            class: "btn",
                            "Download PDF"
                        }
                        " "
                        Link { to: "{RESUME_PDF}", rel: "noopener", class: "btn", "Open in new tab" }
                    }
                }
            },
        }
    }
}
