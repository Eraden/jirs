use {
    crate::{model::Model, Msg},
    seed::{prelude::*, *},
};

pub fn filters(_model: &Model, _page: &super::super::Model) -> Node<Msg> {
    div![C!["filters"]]
}
