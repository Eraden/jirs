use jirs_data::{Epic, EpicId};
use seed::prelude::Node;

use crate::components::styled_field::StyledField;
use crate::components::styled_select::{SelectVariant, StyledSelect};
use crate::components::styled_select_child::StyledSelectOption;
use crate::model::{IssueModal, Model};
use crate::{FieldId, Msg};

pub fn epic_field<Modal>(model: &Model, modal: &Modal, field_id: FieldId) -> Option<Node<Msg>>
where
    Modal: IssueModal,
{
    if model.epics.is_empty() {
        return None;
    }
    let input = StyledSelect {
        id: field_id,
        name: "epic",
        selected: vec![modal
            .epic_id_value()
            .and_then(|id| model.epics.iter().find(|epic| epic.id == id as EpicId))
            .map(epic_select_option)
            .unwrap_or_default()],
        options: Some(model.epics.iter().map(epic_select_option)),
        variant: SelectVariant::Normal,
        clearable: true,
        text_filter: modal.epic_state().text_filter.as_str(),
        opened: modal.epic_state().opened,
        valid: true,
        ..Default::default()
    }
    .render();
    Some(
        StyledField {
            label: "Epic",
            tip: Some("Feature group"),
            input,
            ..Default::default()
        }
        .render(),
    )
}

fn epic_select_option<'l>(epic: &'l Epic) -> StyledSelectOption<'l> {
    StyledSelectOption {
        value: epic.id as u32,
        text: Some(epic.name.as_str()),
        ..Default::default()
    }
}
