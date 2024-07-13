use dotenv_codegen::dotenv;

pub const ENV: Env = Env {
    username: Username {
        github: "wyatt-avilla",
        lastfm: "wowitswyatt",
        letterboxd: "wowitswyatt",
    },
    link: Link {
        goodreads: "https://www.goodreads.com/review/list/159014522?shelf=read",
    },
    key: Key {
        lastfm: dotenv!("LASTFM_KEY"),
    },
};

pub const ENDPOINT: Endpoint = Endpoint {
    base: "https://feframe.shuttleapp.rs",
    github: "/api/github",
    lastfm: "/api/lastfm",
    letterboxd: "/api/letterboxd",
    goodreads: "/api/goodreads",
};

pub const SOCIAL: SocialLinks = SocialLinks {
    github: "https://github.com/wyatt-avilla",
    lastfm: "https://www.last.fm/user/wowitswyatt",
    letterboxd: "https://letterboxd.com/wowitswyatt/",
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
