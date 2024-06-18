pub trait ApiRefresh {
    type Content;

    async fn fetch_newest(
        n: u32,
    ) -> Result<std::vec::Vec<Self::Content>, Box<dyn std::error::Error>>;
}
