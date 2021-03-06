use futures::executor::block_on;
use jirs_data::{
    DescriptionString, EndsAt, EpicId, IssueType, NameString, StartsAt, UserProject, WsMsg,
};

use crate::{db_or_debug_and_return, WebSocketActor, WsHandler, WsResult};

pub struct LoadEpics;

impl WsHandler<LoadEpics> for WebSocketActor {
    fn handle_msg(&mut self, _msg: LoadEpics, _ctx: &mut Self::Context) -> WsResult {
        let project_id = self.require_user_project()?.project_id;
        let epics = db_or_debug_and_return!(self, database_actor::epics::LoadEpics { project_id });
        Ok(Some(WsMsg::EpicsLoaded(epics)))
    }
}

pub struct CreateEpic {
    pub name: NameString,
    pub description: Option<DescriptionString>,
    pub description_html: Option<DescriptionString>,
}

impl WsHandler<CreateEpic> for WebSocketActor {
    fn handle_msg(&mut self, msg: CreateEpic, _ctx: &mut Self::Context) -> WsResult {
        let CreateEpic {
            name,
            description,
            description_html,
        } = msg;
        let UserProject {
            user_id,
            project_id,
            ..
        } = self.require_user_project()?;
        let epic = db_or_debug_and_return!(
            self,
            database_actor::epics::CreateEpic {
                user_id: *user_id,
                project_id: *project_id,
                description,
                description_html,
                name,
            }
        );
        Ok(Some(WsMsg::EpicCreated(epic)))
    }
}

pub struct UpdateEpicName {
    pub epic_id: EpicId,
    pub name: NameString,
}

impl WsHandler<UpdateEpicName> for WebSocketActor {
    fn handle_msg(&mut self, msg: UpdateEpicName, _ctx: &mut Self::Context) -> WsResult {
        let UserProject { project_id, .. } = self.require_user_project()?;
        let epic = db_or_debug_and_return!(
            self,
            database_actor::epics::UpdateEpicName {
                project_id: *project_id,
                epic_id: msg.epic_id,
                name: msg.name.clone(),
            }
        );
        Ok(Some(WsMsg::EpicUpdated(epic)))
    }
}

pub struct UpdateEpicStartsAt {
    pub epic_id: EpicId,
    pub starts_at: Option<StartsAt>,
}

impl WsHandler<UpdateEpicStartsAt> for WebSocketActor {
    fn handle_msg(&mut self, msg: UpdateEpicStartsAt, _ctx: &mut Self::Context) -> WsResult {
        let UserProject { project_id, .. } = self.require_user_project()?;
        let epic = db_or_debug_and_return!(
            self,
            database_actor::epics::UpdateEpicStartsAt {
                project_id: *project_id,
                epic_id: msg.epic_id,
                starts_at: msg.starts_at,
            }
        );
        Ok(Some(WsMsg::EpicUpdated(epic)))
    }
}

pub struct UpdateEpicEndsAt {
    pub epic_id: EpicId,
    pub ends_at: Option<EndsAt>,
}

impl WsHandler<UpdateEpicEndsAt> for WebSocketActor {
    fn handle_msg(&mut self, msg: UpdateEpicEndsAt, _ctx: &mut Self::Context) -> WsResult {
        let UserProject { project_id, .. } = self.require_user_project()?;
        let epic = db_or_debug_and_return!(
            self,
            database_actor::epics::UpdateEpicEndsAt {
                project_id: *project_id,
                epic_id: msg.epic_id,
                ends_at: msg.ends_at,
            }
        );
        Ok(Some(WsMsg::EpicUpdated(epic)))
    }
}

pub struct DeleteEpic {
    pub epic_id: EpicId,
}

impl WsHandler<DeleteEpic> for WebSocketActor {
    fn handle_msg(&mut self, msg: DeleteEpic, _ctx: &mut Self::Context) -> WsResult {
        let DeleteEpic { epic_id } = msg;
        let UserProject { user_id, .. } = self.require_user_project()?;
        let n = db_or_debug_and_return!(
            self,
            database_actor::epics::DeleteEpic {
                user_id: *user_id,
                epic_id,
            }
        );
        Ok(Some(WsMsg::EpicDeleted(epic_id, n)))
    }
}

pub struct TransformEpic {
    pub epic_id: EpicId,
    pub issue_type: IssueType,
}

impl WsHandler<TransformEpic> for WebSocketActor {
    fn handle_msg(&mut self, msg: TransformEpic, _ctx: &mut Self::Context) -> WsResult {
        let epic: jirs_data::Epic = db_or_debug_and_return!(
            self,
            database_actor::epics::FindEpic {
                epic_id: msg.epic_id
            }
        );
        let issue: database_actor::models::Issue = db_or_debug_and_return!(
            self,
            database_actor::issues::CreateIssue {
                title: epic.name,
                issue_type: msg.issue_type,
                issue_status_id: 0,
                priority: Default::default(),
                description: epic.description_html,
                description_text: epic.description,
                estimate: None,
                time_spent: None,
                time_remaining: None,
                project_id: epic.project_id,
                reporter_id: epic.user_id,
                user_ids: vec![epic.user_id],
                epic_id: None
            }
        );
        let n = db_or_debug_and_return!(
            self,
            database_actor::epics::DeleteEpic {
                user_id: epic.user_id,
                epic_id: epic.id
            }
        );
        self.broadcast(&WsMsg::EpicDeleted(msg.epic_id, n));
        self.broadcast(&WsMsg::IssueCreated(issue.into()));
        Ok(None)
    }
}
