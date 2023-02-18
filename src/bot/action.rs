use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(Clone, EnumIter, EnumString, Display, Serialize, Deserialize, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum NameAction {
    Ok,
}

#[derive(Clone, EnumIter, EnumString, Display, Serialize, Deserialize, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum MenuAction {
    Option1,
    Option2,
}
