use jirs_data::{EpicFieldId, IssueType};
use seed::prelude::*;
use seed::*;

use crate::components::styled_button::*;
use crate::components::styled_checkbox::*;
use crate::components::styled_date_time_input::StyledDateTimeInput;
use crate::components::styled_icon::{Icon, StyledIcon};
use crate::components::styled_input::*;
use crate::components::styled_modal::*;
use crate::modals::epics_edit::Model;
use crate::styled_field::StyledField;
use crate::{model, FieldId, Msg};

pub fn view(_model: &model::Model, modal: &Model) -> Node<Msg> {
    let transform = if modal.related_issues.is_empty() {
        transform_into_available(modal)
    } else {
        transform_into_unavailable(modal)
    };
    let close = StyledButton {
        on_click: Some(mouse_ev("click", |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::ModalDropped
        })),
        variant: ButtonVariant::Empty,
        icon: Some(StyledIcon::from(Icon::Close).render()),
        ..Default::default()
    }
    .render();

    let starts_at = StyledDateTimeInput {
        field_id: modal.starts_at.field_id.clone(),
        popup_visible: modal.starts_at.popup_visible,
        timestamp: modal.starts_at.timestamp,
    }
    .render();
    let starts_at = StyledField {
        input: starts_at,
        label: "Starts At",
        ..Default::default()
    }
    .render();

    let ends_at = StyledDateTimeInput {
        field_id: modal.ends_at.field_id.clone(),
        popup_visible: modal.ends_at.popup_visible,
        timestamp: modal.ends_at.timestamp,
    }
    .render();
    let ends_at = StyledField {
        input: ends_at,
        label: "Ends At",
        ..Default::default()
    }
    .render();

    StyledModal {
        width: Some(600),
        class_list: "editEpic",
        children: vec![
            div![C!["header"], h1!["Edit epic"], close],
            StyledInput {
                value: modal.name.value.as_str(),
                valid: modal.name.is_valid(),
                id: Some(FieldId::EditEpic(EpicFieldId::Name)),
                ..Default::default()
            }
            .render(),
            starts_at,
            ends_at,
            transform,
        ],
        ..Default::default()
    }
    .render()
}

fn transform_into_available(modal: &super::Model) -> Node<Msg> {
    let types = StyledCheckbox {
        options: Some(
            IssueType::default()
                .into_iter()
                .map(|it| issue_type_select_option(it, &modal.transform_into)),
        ),
        ..Default::default()
    }
    .render();
    let execute = StyledButton {
        on_click: Some(mouse_ev("click", |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::TransformEpic
        })),
        text: Some("Transform"),
        ..Default::default()
    }
    .render();
    div![C!["transform available"], div![types], div![execute]]
}

#[inline(always)]
fn issue_type_select_option<'l>(ty: IssueType, state: &StyledCheckboxState) -> ChildBuilder<'l> {
    let value: u32 = ty.into();
    ChildBuilder {
        field_id: state.field_id.clone(),
        name: ty.to_str(),
        label: ty.to_label(),
        value: ty.into(),
        class_list: ty.to_str(),
        selected: value == state.value,
        ..Default::default()
    }
}

fn transform_into_unavailable(modal: &super::Model) -> Node<Msg> {
    let (n, s) = match modal.related_issues.len() {
        1 => (1.to_string(), "issue"),
        n => (n.to_string(), "issues"),
    };

    div![
        C!["transform unavailable"],
        span![
            C!["info"],
            "This epic have related issues so you can't change it type."
        ],
        span![C!["count"], format!("Epic have {} {}", n, s)]
    ]
}
