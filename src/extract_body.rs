#[derive(Debug)]
struct ExtractBodyArguments {
    response: Arc<Mutex<reqwest::Response>>,
}

#[derive(Debug)]
struct ExtractBodyOutput {
    body: String,
}

#[derive(Debug)]
struct ExtractBodyLayer {}

#[async_trait]
impl Layer for ExtractBodyLayer {
    type Args = ExtractBodyArguments;
    type O = ExtractBodyOutput;

    async fn run(&self, args: &Self::Args) -> Result<Self::O> {
        let response_arc = args.response.deref();
        let response = response_arc.lock()?;
        let text = response.into_inner()?.text().await?;
        Ok(ExtractBodyOutput {
            body: "asd".to_string(),
        })
    }
}

impl From<ExtractBodyOutput> for GenericOutput {
    fn from(value: ExtractBodyOutput) -> Self {
        let mut data = Self::new();
        data.insert("body".to_string(), Arc::new(value.body));
        data
    }
}

#[derive(Deserialize, Debug)]
struct ExtractBodyArgumentsBuilder {
    response: ArgumentMarker<FetchHttpResponse>,
}

impl ExtractBodyArgumentsBuilder {
    fn build(&self, out: &GenericOutput) -> Result<ExtractBodyArguments> {
        Ok(ExtractBodyArguments {
            response: (*self.response.get_value(out)?).0.clone(),
        })
    }
}
