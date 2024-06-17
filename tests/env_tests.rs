mod env_tests {
    #[test]
    fn test_required_env_vrs() {
        dotenv::dotenv().ok();
        let required_vars = [
            "LASTFM_USERNAME",
            "LASTFM_KEY",
            "GH_USERNAME",
            "GOODREADS_SHELF",
            "LETTERBOXD_USERNAME",
        ];

        required_vars
            .iter()
            .map(|&var_name| {
                assert!(
                    std::env::var(var_name).is_ok(),
                    "Environment variable {var_name} is not defined"
                );
            })
            .for_each(drop);
    }
}
