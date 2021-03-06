use jirs_data::{Issue, IssueFieldId, IssueId, TextEditorMode, TimeTracking, UpdateIssuePayload};
use seed::prelude::*;

use crate::components::styled_date_time_input::StyledDateTimeInputState;
use crate::components::styled_editor::StyledEditorState;
use crate::components::styled_input::StyledInputState;
use crate::components::styled_select::StyledSelectState;
use crate::modals::time_tracking::value_for_time_tracking;
use crate::model::{CommentForm, IssueModal};
use crate::{EditIssueModalSection, FieldId, Msg};

#[derive(Debug)]
pub struct Model {
    pub id: IssueId,
    pub link_copied: bool,
    pub payload: UpdateIssuePayload,
    pub top_type_state: StyledSelectState,
    pub status_state: StyledSelectState,
    pub reporter_state: StyledSelectState,
    pub assignees_state: StyledSelectState,
    pub priority_state: StyledSelectState,
    pub epic_name_state: StyledSelectState,
    pub epic_starts_at_state: StyledDateTimeInputState,
    pub epic_ends_at_state: StyledDateTimeInputState,

    pub estimate: StyledInputState,
    pub estimate_select: StyledSelectState,
    pub time_spent: StyledInputState,
    pub time_spent_select: StyledSelectState,
    pub time_remaining: StyledInputState,
    pub time_remaining_select: StyledSelectState,

    pub title_state: StyledInputState,
    pub description_state: StyledEditorState,

    // comments
    pub comment_form: CommentForm,
}

impl Model {
    pub fn new(user_mode: TextEditorMode, issue: &Issue, time_tracking_type: TimeTracking) -> Self {
        Self {
            id: issue.id,
            link_copied: false,
            payload: UpdateIssuePayload {
                title: issue.title.clone(),
                issue_type: issue.issue_type,
                issue_status_id: issue.issue_status_id,
                priority: issue.priority,
                list_position: issue.list_position,
                description: issue.description.clone(),
                description_text: issue.description_text.clone(),
                estimate: issue.estimate,
                time_spent: issue.time_spent,
                time_remaining: issue.time_remaining,
                project_id: issue.project_id,
                reporter_id: issue.reporter_id,
                user_ids: issue.user_ids.clone(),
            },
            top_type_state: StyledSelectState::new(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Type)),
                issue.estimate.map(|v| vec![v as u32]).unwrap_or_default(),
            ),
            status_state: StyledSelectState::new(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::IssueStatusId)),
                vec![issue.issue_status_id as u32],
            ),
            reporter_state: StyledSelectState::new(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Reporter)),
                vec![issue.reporter_id as u32],
            ),
            assignees_state: StyledSelectState::new(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Assignees)),
                issue.user_ids.iter().map(|n| *n as u32).collect(),
            ),
            priority_state: StyledSelectState::new(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Priority)),
                vec![issue.priority.into()],
            ),
            estimate: StyledInputState::new(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Estimate)),
                value_for_time_tracking(&issue.estimate, &time_tracking_type),
            ),
            estimate_select: StyledSelectState::new(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Estimate)),
                issue.estimate.map(|n| vec![n as u32]).unwrap_or_default(),
            ),
            time_spent: StyledInputState::new(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::TimeSpent)),
                value_for_time_tracking(&issue.time_spent, &time_tracking_type),
            ),
            time_spent_select: StyledSelectState::new(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::TimeSpent)),
                issue.time_spent.map(|n| vec![n as u32]).unwrap_or_default(),
            ),
            time_remaining: StyledInputState::new(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::TimeRemaining)),
                value_for_time_tracking(&issue.time_remaining, &time_tracking_type),
            ),
            time_remaining_select: StyledSelectState::new(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::TimeRemaining)),
                issue
                    .time_remaining
                    .map(|n| vec![n as u32])
                    .unwrap_or_default(),
            ),
            title_state: StyledInputState::new(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Title)),
                issue.title.clone(),
            )
            .with_min(Some(3)),
            description_state: StyledEditorState::new(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Description)),
                user_mode,
                issue.description_text.as_deref().unwrap_or_default(),
                issue.description.as_deref().unwrap_or_default(),
            ),
            comment_form: CommentForm {
                id: None,
                body: String::new(),
                creating: false,
            },
            // epic
            epic_name_state: StyledSelectState::new(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::EpicName)),
                issue
                    .epic_id
                    .as_ref()
                    .map(|id| vec![*id as u32])
                    .unwrap_or_default(),
            ),
            epic_starts_at_state: StyledDateTimeInputState::new(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::EpicStartsAt)),
                None,
            ),
            epic_ends_at_state: StyledDateTimeInputState::new(
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::EpicStartsAt)),
                None,
            ),
        }
    }
}

impl IssueModal for Model {
    fn epic_id_value(&self) -> Option<u32> {
        self.epic_name_state.values.get(0).cloned()
    }

    fn epic_state(&self) -> &StyledSelectState {
        &self.epic_name_state
    }

    fn update_states(&mut self, msg: &Msg, orders: &mut impl Orders<Msg>) {
        self.top_type_state.update(msg, orders);
        self.status_state.update(msg, orders);
        self.reporter_state.update(msg, orders);
        self.assignees_state.update(msg, orders);
        self.priority_state.update(msg, orders);
        self.estimate.update(msg);
        self.estimate_select.update(msg, orders);
        self.time_spent.update(msg);
        self.time_spent_select.update(msg, orders);
        self.time_remaining.update(msg);
        self.time_remaining_select.update(msg, orders);
        self.epic_name_state.update(msg, orders);

        self.title_state.update(msg);
        self.description_state.update(msg, orders);
    }
}
