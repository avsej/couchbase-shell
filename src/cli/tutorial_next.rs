use crate::state::State;
use async_trait::async_trait;
use nu_cli::OutputStream;
use nu_engine::CommandArgs;
use nu_errors::ShellError;
use nu_protocol::Signature;
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
    println!("{}", tutorial.next_tutorial_step());

    Ok(OutputStream::empty())
}
