#[derive(Debug)]
pub enum Kind {
    Files(Vec<String>),
    Project(String),
}

#[derive(Debug)]
pub struct Input {
    pub kind: Kind,
}

pub fn parse_cli(
    files: Option<Vec<String>>,
    project: Option<String>,
) -> crate::core::Result<Input> {
    let kind = match (files, project) {
        (None, None) => Kind::Project(".".to_string()),
        (None, Some(project)) => Kind::Project(project),
        (Some(files), None) => Kind::Files(files),
        (Some(_), Some(_)) => {
            return Err(crate::core::Error::ConflictingArguments {
                reason: "files and project are mutually exclusive.".to_string(),
            })
        }
    };
    Ok(Input { kind })
}
