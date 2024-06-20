use dotenv_codegen::dotenv;

pub const CONFIG: Config = Config {
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

pub struct Config<'a> {
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
