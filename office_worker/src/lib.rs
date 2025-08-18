#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
    pub name: String,
    pub age: u32,
    pub role: WorkerRole
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin,
    User,
    Guest
}

impl From<&str> for OfficeWorker {
    fn from(s: &str) -> OfficeWorker {
        let value: Vec<_> = s.split(",").collect();
        OfficeWorker{name: value[0].to_string(), age: value[1].parse().expect(""), role: WorkerRole::from(value[2])}
    }
}

impl From<&str> for WorkerRole {
    fn from(s: &str) -> WorkerRole {
        match s {
            "admin" => WorkerRole::Admin,
            "user" => WorkerRole::User,
            _ => WorkerRole::Guest,
        }
    }
}