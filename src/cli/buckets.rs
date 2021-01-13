use crate::cli::convert_cb_error;
use crate::cli::util::cluster_identifiers_from;
use crate::state::State;
use async_trait::async_trait;
use couchbase::{GenericManagementRequest, Request};
use futures::channel::oneshot;
use nu_cli::{CommandArgs, OutputStream};
use nu_errors::ShellError;
use nu_protocol::{Signature, SyntaxShape, TaggedDictBuilder, UntaggedValue};
use nu_source::Tag;
use serde::Deserialize;
use std::sync::Arc;

pub struct Buckets {
    state: Arc<State>,
}

impl Buckets {
    pub fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl nu_cli::WholeStreamCommand for Buckets {
    fn name(&self) -> &str {
        "buckets"
    }

    fn signature(&self) -> Signature {
        Signature::build("buckets").named(
            "clusters",
            SyntaxShape::String,
            "the clusters which should be contacted",
            None,
        )
    }

    fn usage(&self) -> &str {
        "Lists all buckets of the connected cluster"
    }

    async fn run(&self, args: CommandArgs) -> Result<OutputStream, ShellError> {
        buckets(self.state.clone(), args).await
    }
}

async fn buckets(state: Arc<State>, args: CommandArgs) -> Result<OutputStream, ShellError> {
    let args = args.evaluate_once().await?;

    let cluster_identifiers = cluster_identifiers_from(&state, &args, true)?;

    let mut buckets = vec![];
    for identifier in cluster_identifiers {
        let core = match state.clusters().get(&identifier) {
            Some(c) => c.cluster().core(),
            None => {
                return Err(ShellError::untagged_runtime_error("Cluster not found"));
            }
        };

        let (sender, receiver) = oneshot::channel();
        let request = GenericManagementRequest::new(
            sender,
            "/pools/default/buckets".into(),
            "get".into(),
            None,
        );
        core.send(Request::GenericManagementRequest(request));

        let input = match receiver.await {
            Ok(i) => i,
            Err(e) => {
                return Err(ShellError::untagged_runtime_error(format!(
                    "Error streaming result {}",
                    e
                )))
            }
        };
        let result = convert_cb_error(input)?;

        if !result.payload().is_some() {
            return Err(ShellError::untagged_runtime_error(
                "Empty response from cluster even though got 200 ok",
            ));
        }

        let payload = match result.payload() {
            Some(p) => p,
            None => {
                return Err(ShellError::untagged_runtime_error(
                    "Empty response from cluster even though got 200 ok",
                ));
            }
        };
        let resp: Vec<BucketInfo> = serde_json::from_slice(payload)?;

        let mut b = resp
            .into_iter()
            .map(|n| {
                let mut collected = TaggedDictBuilder::new(Tag::default());
                collected.insert_value("cluster", identifier.clone());
                collected.insert_value("name", n.name);
                collected.insert_value("type", format!("{:?}", n.bucket_type).to_lowercase());
                collected.insert_value("replicas", UntaggedValue::int(n.replicas));
                collected.insert_value("quota_per_node", UntaggedValue::filesize(n.quota.per_node));
                collected.insert_value("quota_total", UntaggedValue::filesize(n.quota.total));
                collected.into_value()
            })
            .collect::<Vec<_>>();

        buckets.append(&mut b);
    }

    Ok(buckets.into())
}

#[derive(Debug, Deserialize)]
struct BucketInfo {
    name: String,
    #[serde(rename = "bucketType")]
    bucket_type: BucketType,
    quota: Quota,
    #[serde(rename = "replicaNumber")]
    replicas: u32,
}

#[derive(Debug, Deserialize)]
enum BucketType {
    #[serde(rename = "membase")]
    Couchbase,
    #[serde(rename = "memcached")]
    Memcached,
    #[serde(rename = "ephemeral")]
    Ephemeral,
}

#[derive(Debug, Deserialize)]
struct Quota {
    #[serde(rename = "ram")]
    total: u64,
    #[serde(rename = "rawRAM")]
    per_node: u64,
}
