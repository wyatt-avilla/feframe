use dotenv_codegen::dotenv;

pub const ENV: Env = Env {
    username: Username {
        github: dotenv!("GH_USERNAME"),
        lastfm: dotenv!("LASTFM_USERNAME"),
        letterboxd: dotenv!("LETTERBOXD_USERNAME"),
    },
    link: Link {
        goodreads: dotenv!("GOODREADS_SHELF"),
    },
    key: Key {
        lastfm: dotenv!("LASTFM_KEY"),
    },
};

pub const ENDPIONT: Endpoint = Endpoint {
    base: "nothing",
    github: "nothing",
    lastfm: "nothing",
    letterboxd: "nothing",
    goodreads: "nothing",
};

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