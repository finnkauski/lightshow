use super::lshow::Action;

pub type Pad = u8;

/// # Perform trait
pub trait Workload: std::marker::Send + std::marker::Sync + 'static {}
