#[derive(Debug)]
struct DebugLayer {}

#[async_trait]
impl Layer for DebugLayer {
    type Args = DebugServiceArguments;
    type O = DebugServiceArguments;

    async fn run(&self, args: &Self::Args) -> Result<Self::O> {
        Ok(DebugServiceArguments {
            message: args.message.to_string(),
            asd: args.asd,
        })
    }
}
impl From<DebugServiceArguments> for GenericOutput {
    fn from(DebugServiceArguments { message, asd }: DebugServiceArguments) -> Self {
        let mut data = Self::new();
        data.insert("message".to_string(), Arc::new(message));
        data.insert("asd".to_string(), Arc::new(asd));
        data
    }
}

#[derive(Deserialize, Debug)]
struct DebugServiceArgumentsBuilder {
    message: ArgumentMarker<String>,
    asd: ArgumentMarker<u64>,
}

impl DebugServiceArgumentsBuilder {
    fn build(&self, out: &GenericOutput) -> Result<DebugServiceArguments> {
        Ok(DebugServiceArguments {
            message: self.message.get_value(out)?.to_string(),
            asd: *self.asd.get_value(out)?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct DebugServiceArguments {
    message: String,
    asd: u64,
}
