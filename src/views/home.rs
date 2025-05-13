use crate::components::{
    ArrowDownOnSquareIcon, ArrowPathIcon, BoltIcon, CheckIcon, ClipboardDocumentIcon,
    DocumentDuplicateIcon, FolderIcon, GlobeAltIcon, LightBulbIcon, MagnifyingGlassIcon, QAndAIcon,
};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col items-center",
            // Main content container
            div { class: "w-full px-4 py-10 flex flex-col items-center",
                // Logo/Image placeholder
                div { class: "w-20 h-20 bg-white rounded-xl border border-gray-200 flex items-center justify-center mb-8",
                    span { class: "text-gray-400 text-sm", QAndAIcon {} }
                }
                // Title
                h1 { class: "text-3xl sm:text-4xl font-bold text-[#6b3f1d] mb-2 text-center",
                    "Qs & As"
                }
                // Subtitle
                p { class: "text-center text-[#7c5a3a] text-base sm:text-lg mb-6",
                    "A home for all your questions and answers"
                }
                // Get Started button
                Link { class: "bg-[#6b3f1d] hover:bg-[#8b5c2a] transition text-white rounded-xl px-10 py-4 font-bold mb-10 shadow text-xl",
                    to: crate::Route::Login,
                    "Get Started",
                }

                // Welcome box
                div { class: "bg-[#f8f5f1] rounded-lg shadow p-4 mb-10 w-full text-[#5a3a1d] text-base",
                    "Welcome to your personal Q&A library, designed to help you breeze through every form, application, or interview prep—without ever retyping the same response twice."
                }

                // Why you'll love it
                div { class: "w-full mb-12",
                    h2 { class: "text-2xl sm:text-3xl font-bold text-[#6b3f1d] mb-6",
                        "Why you'll love it"
                    }
                    ul { class: "space-y-7",
                        li { class: "flex items-start gap-3",
                            span { class: "mt-1 text-2xl", BoltIcon {} }
                            div {
                                span { class: "font-bold text-lg text-[#5a3a1d]",
                                    "Lightning-Fast Search"
                                }
                                span { class: "block text-[#5a3a1d] text-base mt-1",
                                    " Instantly find any question or answer in your archive. No more endless scrolling or hunting through folders; type a keyword, see results in real time, and copy your perfect response in one click."
                                }
                            }
                        }
                        li { class: "flex items-start gap-3",
                            span { class: "mt-1 text-2xl", FolderIcon {} }
                            div {
                                span { class: "font-bold text-lg text-[#5a3a1d]", "Organized Your Way" }
                                span { class: "block text-[#5a3a1d] text-base mt-1",
                                    " Tag, categorize, and star your most important entries. Group by job type, industry, or difficulty. Create custom collections and jump right into the exact set of Q&A you need."
                                }
                            }
                        }
                        li { class: "flex items-start gap-3",
                            span { class: "mt-1 text-2xl", LightBulbIcon {} }
                            div {
                                span { class: "font-bold text-lg text-[#5a3a1d]",
                                    "AI-Powered Insight & Curation"
                                }
                                span { class: "block text-[#5a3a1d] text-base mt-1",
                                    " Simply select a question (or upload your own), and let your AI suggest fresh, optimized answers based on your existing library. Keep refining and evolving responses to match your style and the latest industry trends."
                                }
                            }
                        }
                        li { class: "flex items-start gap-3",
                            span { class: "mt-1 text-2xl", ClipboardDocumentIcon {} }
                            div {
                                span { class: "font-bold text-lg text-[#5a3a1d]",
                                    "Ready-Made Templates & Snippets"
                                }
                                span { class: "block text-[#5a3a1d] text-base mt-1",
                                    r#" Save boilerplate responses for the most common prompts like "Tell me about yourself" or "What are your strengths?" Never start from scratch again."#
                                }
                            }
                        }
                        li { class: "flex items-start gap-3",
                            span { class: "mt-1 text-2xl", DocumentDuplicateIcon {} }
                            div {
                                span { class: "font-bold text-lg text-[#5a3a1d]",
                                    "Seamless Copy & Paste"
                                }
                                span { class: "block text-[#5a3a1d] text-base mt-1",
                                    r#" One-click "Copy to clipboard" for any answer. Paste directly into online forms, documents, or emails. Perfectly formatted, every time."#
                                }
                            }
                        }
                        li { class: "flex items-start gap-3",
                            span { class: "mt-1 text-2xl", GlobeAltIcon {} }
                            div {
                                span { class: "font-bold text-lg text-[#5a3a1d]", "Universal Access" }
                                span { class: "block text-[#5a3a1d] text-base mt-1",
                                    " Access your archive from any device. Whether you're on your laptop, tablet, or phone, your complete Q&A library is just a tap away."
                                }
                            }
                        }
                    }
                }

                // How it works
                div { class: "w-full mb-12",
                    h2 { class: "text-2xl sm:text-3xl font-bold text-[#6b3f1d] mb-6",
                        "How it works"
                    }
                    ul { class: "space-y-7",
                        li { class: "flex items-start gap-3",
                            span { class: "mt-1 text-2xl", ArrowDownOnSquareIcon {} }
                            div {
                                span { class: "font-bold text-lg text-[#5a3a1d]", "Import or Create" }
                                span { class: "block text-[#5a3a1d] text-base mt-1",
                                    " Add your existing Q&A manually, via CSV import, or capture responses straight from your job portal browser extension."
                                }
                            }
                        }
                        li { class: "flex items-start gap-3",
                            span { class: "mt-1 text-2xl", FolderIcon {} }
                            div {
                                span { class: "font-bold text-lg text-[#5a3a1d]", "Organize & Tag" }
                                span { class: "block text-[#5a3a1d] text-base mt-1",
                                    " Assign tags and categories to keep everything in logical groups—by role, company, or topic."
                                }
                            }
                        }
                        li { class: "flex items-start gap-3",
                            span { class: "mt-1 text-2xl", MagnifyingGlassIcon {} }
                            div {
                                span { class: "font-bold text-lg text-[#5a3a1d]", "Search & Select" }
                                span { class: "block text-[#5a3a1d] text-base mt-1",
                                    r#" Find the exact question in seconds. Preview candidate answers side by side, pick your favorite, and hit "Copy.""#
                                }
                            }
                        }
                        li { class: "flex items-start gap-3",
                            span { class: "mt-1 text-2xl", ArrowPathIcon {} }
                            div {
                                span { class: "font-bold text-lg text-[#5a3a1d]", "Refine with AI" }
                                span { class: "block text-[#5a3a1d] text-base mt-1",
                                    " Need a fresh take? Ask the built-in AI to generate or polish an answer using your prior responses as context."
                                }
                            }
                        }
                    }
                }

                // Who it's for
                div { class: "w-full mb-2",
                    h2 { class: "text-2xl sm:text-3xl font-bold text-[#6b3f1d] mb-6",
                        "Who it's for"
                    }
                    ul { class: "space-y-7",
                        li { class: "flex items-start gap-3",
                            span { class: "mt-1 text-2xl", CheckIcon {} }
                            div {
                                span { class: "block text-[#5a3a1d] text-base font-medium",
                                    "Job Seekers who want to speed up their application process without sacrificing personalization."
                                }
                            }
                        }
                        li { class: "flex items-start gap-3",
                            span { class: "mt-1 text-2xl", CheckIcon {} }
                            div {
                                span { class: "block text-[#5a3a1d] text-base font-medium",
                                    "Interview Prep Enthusiasts who want a go-to vault of practice questions and model answers."
                                }
                            }
                        }
                        li { class: "flex items-start gap-3",
                            span { class: "mt-1 text-2xl", CheckIcon {} }
                            div {
                                span { class: "block text-[#5a3a1d] text-base font-medium",
                                    "Recruiters & Coaches who need a centralized Q&A repository to share best practices with clients."
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
