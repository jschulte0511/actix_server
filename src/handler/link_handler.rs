use crate::models::link::Link;

pub struct LinkHandler {}

impl LinkHandler {
    pub fn get_links() -> std::io::Result<Vec<Link>> {
        let vec = vec![
        Link {
            id: 1,
            category: "Security".to_string(),
            title: "crates.io Postmortem: User Uploaded Malware".to_string(),
            description: "On August 16, the crates.io team was notified by Louis Lang at Phylum of a new user who had uploaded nine crates that typosquatted1 popular crates with ill intent".to_string(),
            author: "Adam Harvey on behalf of the crates.io team".to_string(),
            url: "https://blog.rust-lang.org/inside-rust/2023/09/01/crates-io-malware-postmortem.html".to_string(),
            date: "Sept. 1, 2023 ".to_string(),
            active: true,
        },
        Link {
            id: 3,
            category: "Security".to_string(),
            title: "Keeping Rust projects secure with cargo-audit 0.18: performance, compatibility and security improvements".to_string(),
            description: "cargo audit previously relied on OpenSSL on all platforms. In this release we have switched to rustls - a high-quality, memory-safe TLS implementation in Rust.".to_string(),
            author: "Sergey \"Shnatsel\" Davidoff on behalf of the Secure Code WG".to_string(),
            url: "https://blog.rust-lang.org/inside-rust/2023/09/04/keeping-secure-with-cargo-audit-0.18.html".to_string(),
            date: "Sept. 4, 2023".to_string(),
            active: true,
        },
    ];

        Ok(vec)
    }
}
