use lazy_static::lazy_static;
use nu_errors::ShellError;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct Tutorial {
    current_step: Mutex<i8>,
    num_steps: i8,
}

impl Tutorial {
    pub fn new() -> Self {
        let tutorial = Self {
            current_step: Mutex::new(0),
            num_steps: (STEPS_ORDER.len() as i8),
        };

        tutorial
    }

    pub fn current_step(&self) -> String {
        let step = *self.current_step.lock().unwrap();
        let key = STEPS_ORDER[step as usize];
        format!("Page {}: {}", key, STEPS[key])
    }

    pub fn next_tutorial_step(&self) -> String {
        let mut current_step = self.current_step.lock().unwrap();
        *current_step += 1;
        if *current_step > self.num_steps - 1 {
            *current_step = 0;
        }
        let key = STEPS_ORDER[*current_step as usize];
        format!("Page {}: {}", key, STEPS[key])
    }

    pub fn prev_tutorial_step(&self) -> String {
        let mut current_step = self.current_step.lock().unwrap();
        *current_step -= 1;
        if *current_step < 0 {
            *current_step = self.num_steps - 1;
        }
        let key = STEPS_ORDER[*current_step as usize];
        format!("Page {}: {}", key, STEPS[key])
    }

    pub fn goto_step(&self, name: String) -> Result<String, ShellError> {
        let index = match STEPS_ORDER.iter().position(|&s| s == name) {
            Some(i) => i,
            None => return Err(ShellError::untagged_runtime_error("invalid tutorial step")),
        };
        let mut current_step = self.current_step.lock().unwrap();
        *current_step = index as i8;

        let key = STEPS_ORDER[index];
        Ok(format!("Page {}: {}", key, STEPS[key]))
    }

    pub fn step_names(&self) -> Vec<String> {
        let mut s = vec![];
        for step in STEPS_ORDER.iter() {
            s.push(step.to_owned().into()) // We control the names and we aren't putting non-unicode chars in them.
        }

        s
    }
}

lazy_static! {
    static ref STEPS_ORDER: &'static [&'static str] = &[
        "start",
        "overview",
        "doc",
        "pipeline",
        "query",
        "conclusion"
    ];
    static ref STEPS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("start", "
Couchbase Shell (or cbsh) is a tool to access and work with Couchbase Server.

cbsh is a real shell, like sh or bash, but is aware of structured or tabular data, as it is built upon the open-source Nushell project.

Try typing...

ls

...to get a directory listing, and notice that the output is a table.

To navigate the tutorial you can use 'tutorial next' to proceed, 'tutorial previous' to go back, or 'tutorial page <name>' to go to a specific page.
You can try out different commands and your current step will be remembered.

Try running 'tutorial next' now to move to the next step of the tutorial.
    ");
        m.insert(
            "overview",
            "
cbsh is connected to one or more Couchbase Server clusters.

Try 'nodes' to list the nodes in the active cluster.

Try 'buckets' to list the buckets in the active cluster.

And, use 'tutorial next' to move to the next step in the tutorial.
    ",
        );
        m.insert("doc", "
You can retrieve documents by using the 'doc get KEY' command.
We're going to use document names from the \"travel-sample\" bucket which you can load within the Couchbase Server settings.

Try...

  doc get airline_10

The doc command also takes optional flags in order to
format or reshape the output data.

Try...

  doc get airline_10 --flatten

To learn more, try 'help doc' and 'help doc get'.

Use 'tutorial next' to move to the next step in the tutorial.
    ");
        m.insert(
            "pipeline",
            "
Commands in cbsh can be pipelined into a chain of commands.
We're using the \"travel-sample\" bucket which you can load within the Couchbase Server settings.

Try...

  buckets | where name =~ \"travel\"

That pipes the tabular output of the buckets command to the where command.

Try...

  buckets | where name =~ \"sample\" | to json --pretty 2

Use 'tutorial next' to move to the next step in the tutorial.
    ",
        );
        m.insert("query", "
You can use the query command to run N1QL queries (or SQL for JSON queries) against the Couchbase Server database.
We're using the \"travel-sample\" bucket which you can load within the Couchbase Server settings.

Try...

    query 'SELECT name, callsign FROM `travel-sample` LIMIT 5'

And, you can pipe the tabular data to commands like save...

    query 'SELECT name, callsign FROM `travel-sample` LIMIT 5' | save output.csv

Which you can then see the contents of (note that the output has automatically been formatted to csv)...

    cat output.csv

You can try the same for other formats too, such as JSON.
Try 'to --help' to see the available formats.

Use 'tutorial next' to move to the next step in the tutorial.
    ");
        m.insert(
            "conclusion",
            "
The Couchbase Shell can do a lot more, including...

- create, retrieve, update and delete JSON docs from the database.

- import and export JSON documents into / out-from the database.

- execute N1QL (e.g., SQL for JSON) queries.

- manage clusters, buckets, stats and system setup.

- and much more...

That's it for this quick tutorial!

For more info, please see the cbsh documentation at...

    http://couchbase.sh/docs/
    ",
        );
        m
    };
}
