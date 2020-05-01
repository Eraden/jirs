use std::collections::hash_map::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use jirs_data::*;

use crate::modal::time_tracking::value_for_time_tracking;
use crate::shared::styled_checkbox::StyledCheckboxState;
use crate::shared::styled_editor::Mode;
use crate::shared::styled_input::StyledInputState;
use crate::shared::styled_select::StyledSelectState;
use crate::{EditIssueModalSection, FieldId, ProjectFieldId, HOST_URL};

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum ModalType {
    AddIssue(Box<AddIssueModal>),
    EditIssue(IssueId, Box<EditIssueModal>),
    DeleteIssueConfirm(IssueId),
    DeleteCommentConfirm(CommentId),
    TimeTracking(IssueId),
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct CommentForm {
    pub id: Option<CommentId>,
    pub body: String,
    pub creating: bool,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct EditIssueModal {
    pub id: i32,
    pub link_copied: bool,
    pub payload: UpdateIssuePayload,
    pub top_type_state: StyledSelectState,
    pub status_state: StyledSelectState,
    pub reporter_state: StyledSelectState,
    pub assignees_state: StyledSelectState,
    pub priority_state: StyledSelectState,

    pub estimate: StyledInputState,
    pub estimate_select: StyledSelectState,
    pub time_spent: StyledInputState,
    pub time_spent_select: StyledSelectState,
    pub time_remaining: StyledInputState,
    pub time_remaining_select: StyledSelectState,

    pub description_editor_mode: Mode,

    // comments
    pub comment_form: CommentForm,
}

impl EditIssueModal {
    pub fn new(issue: &Issue, time_tracking_type: TimeTracking) -> Self {
        Self {
            id: issue.id,
            link_copied: false,
            payload: UpdateIssuePayload {
                title: issue.title.clone(),
                issue_type: issue.issue_type,
                status: issue.status,
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
                FieldId::EditIssueModal(EditIssueModalSection::Issue(IssueFieldId::Status)),
                vec![issue.status.into()],
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
            description_editor_mode: Mode::View,
            comment_form: CommentForm {
                id: None,
                body: String::new(),
                creating: false,
            },
        }
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct AddIssueModal {
    pub title: String,
    pub issue_type: IssueType,
    pub status: IssueStatus,
    pub priority: IssuePriority,
    pub description: Option<String>,
    pub description_text: Option<String>,
    pub estimate: Option<i32>,
    pub time_spent: Option<i32>,
    pub time_remaining: Option<i32>,
    pub project_id: Option<i32>,
    pub user_ids: Vec<i32>,
    pub reporter_id: Option<i32>,

    // modal fields
    pub type_state: StyledSelectState,
    pub reporter_state: StyledSelectState,
    pub assignees_state: StyledSelectState,
    pub priority_state: StyledSelectState,
}

impl Default for AddIssueModal {
    fn default() -> Self {
        Self {
            title: Default::default(),
            issue_type: Default::default(),
            status: Default::default(),
            priority: Default::default(),
            description: Default::default(),
            description_text: Default::default(),
            estimate: Default::default(),
            time_spent: Default::default(),
            time_remaining: Default::default(),
            project_id: Default::default(),
            user_ids: Default::default(),
            reporter_id: Default::default(),
            type_state: StyledSelectState::new(FieldId::AddIssueModal(IssueFieldId::Type), vec![]),
            reporter_state: StyledSelectState::new(
                FieldId::AddIssueModal(IssueFieldId::Reporter),
                vec![],
            ),
            assignees_state: StyledSelectState::new(
                FieldId::AddIssueModal(IssueFieldId::Assignees),
                vec![],
            ),
            priority_state: StyledSelectState::new(
                FieldId::AddIssueModal(IssueFieldId::Priority),
                vec![],
            ),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Page {
    Project,
    EditIssue(IssueId),
    AddIssue,
    ProjectSettings,
    SignIn,
    SignUp,
    Invite,
    Users,
    Profile,
}

impl Page {
    pub fn to_path(self) -> String {
        match self {
            Page::Project => "/board".to_string(),
            Page::EditIssue(id) => format!("/issues/{id}", id = id),
            Page::AddIssue => "/add-issues".to_string(),
            Page::ProjectSettings => "/project-settings".to_string(),
            Page::SignIn => "/login".to_string(),
            Page::SignUp => "/register".to_string(),
            Page::Invite => "/invite".to_string(),
            Page::Users => "/users".to_string(),
            Page::Profile => "/profile".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCommentForm {
    pub fields: CreateCommentPayload,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateIssueForm {
    pub fields: CreateIssuePayload,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProjectForm {
    pub id: ProjectId,
    pub fields: UpdateProjectPayload,
}

#[derive(Debug, Default)]
pub struct ProjectPage {
    pub text_filter: String,
    pub active_avatar_filters: Vec<UserId>,
    pub only_my_filter: bool,
    pub recently_updated_filter: bool,
    pub dragged_issue_id: Option<IssueId>,
    pub last_drag_exchange_id: Option<IssueId>,
    pub dirty_issues: Vec<IssueId>,
}

#[derive(Debug, Default)]
pub struct InvitePage {
    pub token: String,
    pub token_touched: bool,
}

#[derive(Debug)]
pub struct ProjectSettingsPage {
    pub payload: UpdateProjectPayload,
    pub project_category_state: StyledSelectState,
    pub description_mode: crate::shared::styled_editor::Mode,
    pub time_tracking: StyledCheckboxState,
}

impl ProjectSettingsPage {
    pub fn new(project: &Project) -> Self {
        use crate::shared::styled_editor::Mode as EditorMode;
        let jirs_data::Project {
            id,
            name,
            url,
            description,
            category,
            time_tracking,
            ..
        } = project;
        Self {
            payload: UpdateProjectPayload {
                id: *id,
                name: Some(name.clone()),
                url: Some(url.clone()),
                description: Some(description.clone()),
                category: Some(category.clone()),
                time_tracking: Some(*time_tracking),
            },
            description_mode: EditorMode::View,
            project_category_state: StyledSelectState::new(
                FieldId::ProjectSettings(ProjectFieldId::Category),
                vec![(*category).into()],
            ),
            time_tracking: StyledCheckboxState::new(
                FieldId::ProjectSettings(ProjectFieldId::TimeTracking),
                (*time_tracking).into(),
            ),
        }
    }
}

#[derive(Debug, Default)]
pub struct SignInPage {
    pub username: String,
    pub email: String,
    pub token: String,
    pub login_success: bool,
    pub bad_token: String,
    // touched
    pub username_touched: bool,
    pub email_touched: bool,
    pub token_touched: bool,
}

#[derive(Debug, Default)]
pub struct SignUpPage {
    pub username: String,
    pub email: String,
    pub sign_up_success: bool,
    pub error: String,
    // touched
    pub username_touched: bool,
    pub email_touched: bool,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum InvitationFormState {
    Initial = 1,
    Sent = 2,
    Succeed = 3,
    Failed = 4,
}

impl Default for InvitationFormState {
    fn default() -> Self {
        InvitationFormState::Initial
    }
}

#[derive(Debug)]
pub struct UsersPage {
    pub name: String,
    pub name_touched: bool,
    pub email: String,
    pub email_touched: bool,
    pub user_role: UserRole,

    pub user_role_state: StyledSelectState,
    pub pending_invitations: Vec<String>,
    pub error: String,
    pub form_state: InvitationFormState,

    pub invited_users: Vec<User>,
    pub invitations: Vec<Invitation>,
}

impl Default for UsersPage {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            name_touched: false,
            email: "".to_string(),
            email_touched: false,
            user_role: Default::default(),
            user_role_state: StyledSelectState::new(FieldId::Users(UsersFieldId::UserRole), vec![]),
            pending_invitations: vec![],
            error: "".to_string(),
            form_state: Default::default(),
            invited_users: vec![],
            invitations: vec![],
        }
    }
}

#[derive(Debug)]
pub struct ProfilePage {
    pub name: StyledInputState,
    pub email: StyledInputState,
}

impl ProfilePage {
    pub fn new(user: &User) -> Self {
        Self {
            name: StyledInputState::new(
                FieldId::Profile(UsersFieldId::Username),
                user.name.as_str(),
            ),
            email: StyledInputState::new(
                FieldId::Profile(UsersFieldId::Email),
                user.email.as_str(),
            ),
        }
    }
}

#[derive(Debug)]
pub enum PageContent {
    SignIn(Box<SignInPage>),
    SignUp(Box<SignUpPage>),
    Project(Box<ProjectPage>),
    ProjectSettings(Box<ProjectSettingsPage>),
    Invite(Box<InvitePage>),
    Users(Box<UsersPage>),
    Profile(Box<ProfilePage>),
}

#[derive(Debug)]
pub struct Model {
    pub host_url: String,
    pub access_token: Option<Uuid>,
    pub about_tooltip_visible: bool,

    // mapped
    pub comments_by_project_id: HashMap<ProjectId, Vec<Comment>>,

    // forms
    pub project_form: Option<UpdateProjectForm>,
    pub issue_form: Option<CreateIssueForm>,
    pub comment_form: Option<CreateCommentForm>,

    // modals
    pub modals: Vec<ModalType>,

    // pages
    pub page: Page,
    pub page_content: PageContent,

    pub project: Option<Project>,
    pub user: Option<User>,
    pub issues: Vec<Issue>,
    pub users: Vec<User>,
    pub comments: Vec<Comment>,
}

impl Default for Model {
    fn default() -> Self {
        let host_url = unsafe { HOST_URL.clone() };
        Self {
            access_token: None,
            user: None,
            issue_form: None,
            project_form: None,
            comment_form: None,
            issues: vec![],
            users: vec![],
            comments_by_project_id: Default::default(),
            page: Page::Project,
            host_url,
            page_content: PageContent::Project(Box::new(ProjectPage::default())),
            modals: vec![],
            project: None,
            comments: vec![],
            about_tooltip_visible: false,
        }
    }
}
