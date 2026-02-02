use crate::{blog::all_blog_previews, *};

#[component]
pub fn Blog() -> Element {
    let previews = all_blog_previews();

    rsx! {
        Page {
            id: "blog",
            name: "Blog",
            body: rsx! {
                p {
                    "100% Human Generated Content"
                }
                br {}

                if previews.is_empty() {
                    p { "No posts yet." }
                } else {
                    ul { class: "blog-list",
                        for (title , date , summary , link) in previews {
                            li { class: "blog-item", key: "{link}",
                                BlogPostPreview {
                                    title,
                                    date,
                                    summary,
                                    link: link.clone(),
                                }
                            }
                        }
                    }
                }
            },
        }
    }
}
