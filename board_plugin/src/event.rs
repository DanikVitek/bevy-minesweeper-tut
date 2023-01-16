use derive_more::From;

use crate::component::Coordinates;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, From)]
pub struct TIleTriggerEvent(pub Coordinates);
