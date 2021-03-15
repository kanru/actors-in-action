use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct EventDescription {
    pub tickets: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TicketRequest {
    pub tickets: u32,
}
