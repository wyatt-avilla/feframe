mod env_tests {
    #[test]
    fn test_required_env_vrs() {
        dotenv::dotenv().ok();
        let required_vars = [
            "LASTFM_USERNAME",
            "LASTFM_KEY",
            "GH_USERNAME",
            "GOODREADS_SHELF",
        ];

        required_vars
            .iter()
            .map(|&var_name| {
                if std::env::var(var_name).is_err() {
                    panic!("Environment variable {} is not defined", var_name);
                }
            })
            .for_each(drop);
    }
}
