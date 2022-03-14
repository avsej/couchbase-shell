use crate::cli::util::{cluster_identifiers_from, NuValueMap};
use crate::state::State;

use crate::cli::cloud_json::JSONCloudClusterHealthResponse;
use crate::client::{CapellaRequest, ManagementRequest};
use serde::Deserialize;
use std::fmt;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use tokio::time::Instant;

use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{IntoPipelineData, PipelineData, ShellError, Signature, SyntaxShape, Value};

#[derive(Clone)]
pub struct Nodes {
    state: Arc<Mutex<State>>,
}

impl Nodes {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Self { state }
    }
}

impl Command for Nodes {
    fn name(&self) -> &str {
        "nodes"
    }

    fn signature(&self) -> Signature {
        Signature::build("nodes").named(
            "clusters",
            SyntaxShape::String,
            "the clusters which should be contacted",
            None,
        )
    }

    fn usage(&self) -> &str {
        "Lists all nodes of the connected cluster"
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        nodes(self.state.clone(), engine_state, stack, call, input)
    }
}

fn nodes(
    state: Arc<Mutex<State>>,
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    _input: PipelineData,
) -> Result<PipelineData, ShellError> {
    let ctrl_c = engine_state.ctrlc.as_ref().unwrap().clone();

    let cluster_identifiers = cluster_identifiers_from(engine_state, stack, &state, call, true)?;

    let guard = state.lock().unwrap();
    let mut nodes = vec![];
    for identifier in cluster_identifiers {
        let active_cluster = match guard.clusters().get(&identifier) {
            Some(c) => c,
            None => {
                return Err(ShellError::LabeledError(
                    "Cluster not found".into(),
                    "Cluster not found".into(),
                ));
            }
        };
        if let Some(plane) = active_cluster.capella_org() {
            let cloud = guard.capella_org_for_cluster(plane)?.client();
            let deadline = Instant::now().add(active_cluster.timeouts().management_timeout());
            let cluster =
                cloud.find_cluster(identifier.clone(), deadline.clone(), ctrl_c.clone())?;
            let response = cloud.capella_request(
                CapellaRequest::GetClusterHealth {
                    cluster_id: cluster.id(),
                },
                deadline,
                ctrl_c.clone(),
            )?;
            if response.status() != 200 {
                return Err(ShellError::LabeledError(
                    response.content().into(),
                    response.content().into(),
                ));
            }

            let resp: JSONCloudClusterHealthResponse = serde_json::from_str(response.content())
                .map_err(|e| ShellError::LabeledError(e.to_string(), e.to_string()))?;

            let mut n = resp
                .nodes()
                .nodes()
                .into_iter()
                .map(|n| {
                    let mut collected = NuValueMap::default();
                    let services = n
                        .services()
                        .iter()
                        .map(|n| format!("{}", n))
                        .collect::<Vec<_>>()
                        .join(",");

                    collected.add_string("cluster", identifier.clone(), call.head);
                    collected.add_string("hostname", n.name(), call.head);
                    collected.add_string("status", n.status(), call.head);
                    collected.add_string("services", services, call.head);
                    collected.add_string("version", "", call.head);
                    collected.add_string("os", "", call.head);
                    collected.add_string("memory_total", "", call.head);
                    collected.add_string("memory_free", "", call.head);
                    collected.add_bool("capella", true, call.head);

                    collected.into_value(call.head)
                })
                .collect::<Vec<_>>();

            nodes.append(&mut n);
        } else {
            let response = active_cluster.cluster().http_client().management_request(
                ManagementRequest::GetNodes,
                Instant::now().add(active_cluster.timeouts().management_timeout()),
                ctrl_c.clone(),
            )?;

            let resp: PoolInfo = match response.status() {
                200 => match serde_json::from_str(response.content()) {
                    Ok(m) => m,
                    Err(e) => {
                        return Err(ShellError::LabeledError(
                            format!("Failed to decode response body {}", e,),
                            format!("Failed to decode response body {}", e,),
                        ));
                    }
                },
                _ => {
                    return Err(ShellError::LabeledError(
                        format!("Request failed {}", response.content(),),
                        "".into(),
                    ));
                }
            };

            let mut n = resp
                .nodes
                .into_iter()
                .map(|n| {
                    let mut collected = NuValueMap::default();
                    let services = n
                        .services
                        .iter()
                        .map(|n| format!("{}", n))
                        .collect::<Vec<_>>()
                        .join(",");

                    collected.add_string("cluster", identifier.clone(), call.head);
                    collected.add_string("hostname", n.hostname, call.head);
                    collected.add_string("status", n.status, call.head);
                    collected.add_string("services", services, call.head);
                    collected.add_string("version", n.version, call.head);
                    collected.add_string("os", n.os, call.head);
                    collected.add_i64("memory_total", n.memory_total as i64, call.head);
                    collected.add_i64("memory_free", n.memory_free as i64, call.head);
                    collected.add_bool("capella", false, call.head);

                    collected.into_value(call.head)
                })
                .collect::<Vec<_>>();

            nodes.append(&mut n);
        }
    }

    Ok(Value::List {
        vals: nodes,
        span: call.head,
    }
    .into_pipeline_data())
}

#[derive(Debug, Deserialize)]
struct PoolInfo {
    name: String,
    nodes: Vec<NodeInfo>,
}

#[derive(Debug, Deserialize)]
struct NodeInfo {
    hostname: String,
    status: String,
    #[serde(rename = "memoryTotal")]
    memory_total: u64,
    #[serde(rename = "memoryFree")]
    memory_free: u64,
    services: Vec<NodeService>,
    version: String,
    os: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) enum NodeService {
    #[serde(rename = "cbas")]
    Analytics,
    #[serde(rename = "eventing")]
    Eventing,
    #[serde(rename = "fts")]
    Search,
    #[serde(rename = "n1ql")]
    Query,
    #[serde(rename = "index")]
    Indexing,
    #[serde(rename = "kv")]
    KeyValue,
    #[serde(rename = "backup")]
    Backup,
}

impl fmt::Display for NodeService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeService::Analytics => write!(f, "analytics"),
            NodeService::Eventing => write!(f, "eventing"),
            NodeService::Search => write!(f, "search"),
            NodeService::Query => write!(f, "query"),
            NodeService::Indexing => write!(f, "indexing"),
            NodeService::KeyValue => write!(f, "kv"),
            NodeService::Backup => write!(f, "backup"),
        }
    }
}
