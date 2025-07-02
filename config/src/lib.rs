use dotenv_codegen::dotenv;

pub const ENV: Env = Env {
    username: Username {
        github: "wyatt-avilla",
        lastfm: "wyattwtf",
        letterboxd: "wyattwtf",
    },
    link: Link {
        goodreads:
            "https://www.goodreads.com/review/list/159014522?order=d&shelf=read&sort=date_read",
    },
    key: Key {
        lastfm: dotenv!("LASTFM_KEY"),
    },
};

pub const ENDPOINT: Endpoint = Endpoint {
    #[cfg(feature = "local")]
    base: "http://127.0.0.1:8000",
    #[cfg(not(feature = "local"))]
    base: "https://feframe-wa5w.shuttle.app",
    github: "/api/github",
    lastfm: "/api/lastfm",
    letterboxd: "/api/letterboxd",
    goodreads: "/api/goodreads",
};

pub const SOCIAL: SocialLinks = SocialLinks {
    github: "https://github.com/wyatt-avilla",
    lastfm: "https://www.last.fm/user/wyattwtf",
    letterboxd: "https://letterboxd.com/wyattwtf/",
    goodreads: "https://www.goodreads.com/user/show/159014522-wyatt",
};

pub struct SocialLinks<'a> {
    pub github: &'a str,
    pub lastfm: &'a str,
    pub letterboxd: &'a str,
    pub goodreads: &'a str,
}

pub struct Endpoint<'a> {
    pub base: &'a str,
    pub github: &'a str,
    pub lastfm: &'a str,
    pub letterboxd: &'a str,
    pub goodreads: &'a str,
}

pub struct Env<'a> {
    pub username: Username<'a>,
    pub link: Link<'a>,
    pub key: Key<'a>,
}

pub struct Username<'a> {
    pub github: &'a str,
    pub lastfm: &'a str,
    pub letterboxd: &'a str,
}

pub struct Link<'a> {
    pub goodreads: &'a str,
}

pub struct Key<'a> {
    pub lastfm: &'a str,
}
