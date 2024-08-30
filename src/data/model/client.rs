use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Client {
    pub id_client: Option<u32>,
    pub client_active: bool,
    pub client_name: String,
}

impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ id: {}, active: {}, name: {} }}",
            self.id_client
                .map_or("None".to_string(), |id| id.to_string()),
            self.client_active,
            self.client_name
        )
    }
}
