use crate::state::State;
use async_trait::async_trait;
use nu_cli::OutputStream;
use nu_engine::CommandArgs;
use nu_errors::ShellError;
use nu_protocol::{ReturnSuccess, Signature, UntaggedValue};
use nu_source::Tag;
use std::sync::Arc;

pub struct TutorialNext {
    state: Arc<State>,
}

impl TutorialNext {
    pub fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl nu_engine::WholeStreamCommand for TutorialNext {
    fn name(&self) -> &str {
        "tutorial next"
    }

    fn signature(&self) -> Signature {
        Signature::build("tutorial next")
    }

    fn usage(&self) -> &str {
        "Step to the next page in the Couchbase Shell tutorial"
    }

    async fn run(&self, _args: CommandArgs) -> Result<OutputStream, ShellError> {
        run_tutorial_next(self.state.clone()).await
    }
}

async fn run_tutorial_next(state: Arc<State>) -> Result<OutputStream, ShellError> {
    let tutorial = state.tutorial();
    Ok(OutputStream::one(ReturnSuccess::value(
        UntaggedValue::string(tutorial.next_tutorial_step()).into_value(Tag::unknown()),
    )))
}
