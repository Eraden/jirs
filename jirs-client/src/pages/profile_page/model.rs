use jirs_data::{ProjectId, TextEditorMode, User, UsersFieldId};

use crate::components::styled_checkbox::StyledCheckboxState;
use crate::components::styled_image_input::StyledImageInputState;
use crate::components::styled_input::StyledInputState;
use crate::components::styled_select::StyledSelectState;
use crate::FieldId;

#[derive(Debug)]
pub struct ProfilePage {
    pub name: StyledInputState,
    pub email: StyledInputState,
    pub avatar: StyledImageInputState,
    pub current_project: StyledSelectState,
    pub text_editor_mode: StyledCheckboxState,
}

impl ProfilePage {
    pub fn new(user: &User, mode: TextEditorMode, project_ids: Vec<ProjectId>) -> Self {
        Self {
            name: StyledInputState::new(
                FieldId::Profile(UsersFieldId::Username),
                user.name.as_str(),
            ),
            email: StyledInputState::new(
                FieldId::Profile(UsersFieldId::Email),
                user.email.as_str(),
            ),
            avatar: StyledImageInputState::new(
                FieldId::Profile(UsersFieldId::Avatar),
                user.avatar_url.as_ref().cloned(),
            ),
            current_project: StyledSelectState::new(
                FieldId::Profile(UsersFieldId::CurrentProject),
                project_ids.into_iter().map(|n| n as u32).collect(),
            ),
            text_editor_mode: StyledCheckboxState::new(
                FieldId::Profile(UsersFieldId::TextEditorMode),
                mode.into(),
            ),
        }
    }
}
