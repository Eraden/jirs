use seed::prelude::*;

pub use init_load_sets::*;
use jirs_data::*;

use crate::{
    model::*,
    shared::{go_to_board, write_auth_token},
    Msg, OperationKind, ResourceKind, WebSocketChanged,
};

mod init_load_sets;

pub mod issue;

pub fn flush_queue(model: &mut Model, orders: &mut impl Orders<Msg>) {
    use seed::browser::web_socket::State;
    match model.ws.as_ref() {
        Some(ws) if ws.state() != State::Open => return,
        None => return,
        _ => (),
    };
    let mut old = vec![];
    std::mem::swap(&mut model.ws_queue, &mut old);
    for msg in old {
        send_ws_msg(msg, model.ws.as_ref(), orders);
    }
}

pub fn enqueue_ws_msg(v: Vec<WsMsg>, ws: Option<&WebSocket>, orders: &mut impl Orders<Msg>) {
    for msg in v {
        send_ws_msg(msg, ws, orders);
    }
}

pub fn send_ws_msg(msg: WsMsg, ws: Option<&WebSocket>, orders: &mut impl Orders<Msg>) {
    use seed::browser::web_socket::State;
    let ws = match ws {
        Some(ws) if ws.state() == State::Open => ws,
        _ => {
            orders
                .skip()
                .send_msg(Msg::WebSocketChange(WebSocketChanged::Bounced(msg)));
            return;
        }
    };
    let binary = bincode::serialize(&msg).unwrap();
    ws.send_bytes(binary.as_slice())
        .expect("Failed to send ws msg");
}

pub fn open_socket(model: &mut Model, orders: &mut impl Orders<Msg>) {
    use seed::browser::web_socket::State;
    use seed::{prelude::*, *};
    log!(model.ws.as_ref().map(|ws| ws.state()));

    match model.ws.as_ref() {
        Some(ws) if ws.state() != State::Closed => {
            return;
        }
        _ => (),
    };
    if model.host_url.is_empty() {
        return;
    }
    let url = model.ws_url.as_str();

    model.ws = WebSocket::builder(url, orders)
        .on_message(|msg| {
            Some(Msg::WebSocketChange(WebSocketChanged::WebSocketMessage(
                msg,
            )))
        })
        .on_open(|| {
            log!("open_socket opened");
            Some(Msg::WebSocketChange(WebSocketChanged::WebSocketOpened))
        })
        .on_close(|_| Some(Msg::WebSocketChange(WebSocketChanged::WebSocketClosed)))
        .on_error(|| {
            error!("Failed to open WebSocket");
            None as Option<Msg>
        })
        .protocols(&["jirs"])
        .build_and_open()
        .ok();
}

pub async fn read_incoming(msg: WebSocketMessage) -> Msg {
    let bytes = msg.bytes().await.unwrap_or_default();
    Msg::WebSocketChange(WebSocketChanged::WebSocketMessageLoaded(bytes))
}

pub fn update(msg: WsMsg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        // auth
        WsMsg::AuthorizeLoaded(Ok(user)) => {
            model.user = Some(user);
            if is_non_logged_area() {
                go_to_board(orders);
            }
            orders
                .skip()
                .send_msg(Msg::UserChanged(model.user.as_ref().cloned()))
                .send_msg(Msg::ResourceChanged(
                    ResourceKind::User,
                    OperationKind::SingleLoaded,
                    model.user.as_ref().map(|u| u.id),
                ))
                .send_msg(Msg::ResourceChanged(
                    ResourceKind::Auth,
                    OperationKind::SingleLoaded,
                    model.user.as_ref().map(|u| u.id),
                ));
        }
        WsMsg::AuthorizeExpired => {
            use seed::*;

            log!("Received token expired");
            if let Ok(msg) = write_auth_token(None) {
                orders.skip().send_msg(msg).send_msg(Msg::ResourceChanged(
                    ResourceKind::Auth,
                    OperationKind::SingleRemoved,
                    model.user.as_ref().map(|u| u.id),
                ));
            }
        }
        // project
        WsMsg::ProjectsLoaded(v) => {
            model.projects = v;
            init_current_project(model, orders);
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Project,
                OperationKind::ListLoaded,
                None,
            ));
        }
        // user projects
        WsMsg::UserProjectsLoaded(v) => {
            model.current_user_project = v.iter().find(|up| up.is_current).cloned();
            model.user_projects = v;
            init_current_project(model, orders);
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::UserProject,
                OperationKind::ListLoaded,
                None,
            ));
        }
        WsMsg::UserProjectCurrentChanged(user_project) => {
            let mut old = vec![];
            std::mem::swap(&mut old, &mut model.user_projects);
            for mut up in old {
                up.is_current = up.id == user_project.id;
                model.user_projects.push(up);
            }
            model.current_user_project = Some(user_project);
            init_current_project(model, orders);
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::UserProject,
                OperationKind::SingleModified,
                model.current_user_project.as_ref().map(|up| up.id),
            ));
        }

        // issue statuses
        WsMsg::IssueStatusesLoaded(v) => {
            model.issue_statuses = v.clone();
            for is in v {
                model.issue_statuses_by_id.insert(is.id, is);
            }
            model
                .issue_statuses
                .sort_by(|a, b| a.position.cmp(&b.position));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::IssueStatus,
                OperationKind::ListLoaded,
                None,
            ));
        }
        WsMsg::IssueStatusCreated(is) => {
            let id = is.id;
            model.issue_statuses.push(is.clone());
            model.issue_statuses_by_id.insert(is.id, is);
            model
                .issue_statuses
                .sort_by(|a, b| a.position.cmp(&b.position));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::IssueStatus,
                OperationKind::SingleCreated,
                Some(id),
            ));
        }
        WsMsg::IssueStatusUpdated(mut changed) => {
            let id = changed.id;
            model
                .issue_statuses_by_id
                .insert(changed.id, changed.clone());
            if let Some(idx) = model.issue_statuses.iter().position(|c| c.id == changed.id) {
                std::mem::swap(&mut model.issue_statuses[idx], &mut changed);
            }
            model
                .issue_statuses
                .sort_by(|a, b| a.position.cmp(&b.position));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::IssueStatus,
                OperationKind::SingleModified,
                Some(id),
            ));
        }
        WsMsg::IssueStatusDeleted(dropped_id, _count) => {
            model.issue_statuses_by_id.remove(&dropped_id);
            let old = std::mem::replace(&mut model.issue_statuses, vec![]);
            for is in old {
                if is.id != dropped_id {
                    model.issue_statuses.push(is);
                }
            }
            model
                .issue_statuses
                .sort_by(|a, b| a.position.cmp(&b.position));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::IssueStatus,
                OperationKind::SingleRemoved,
                Some(dropped_id),
            ));
        }
        // issues
        WsMsg::ProjectIssuesLoaded(mut v) => {
            v.sort_by(|a, b| (a.list_position as i64).cmp(&(b.list_position as i64)));
            {
                let _ = std::mem::replace(model.issues_mut(), v.clone());
            };
            model.issues_by_id.clear();
            for issue in v {
                model.issues_by_id.insert(issue.id, issue.clone());
            }

            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Issue,
                OperationKind::ListLoaded,
                None,
            ));
        }
        WsMsg::IssueUpdated(issue) => {
            let id = issue.id;
            model.issues_by_id.remove(&id);
            model.issues_by_id.insert(id, issue.clone());
            if let Some(idx) = model.issues().iter().position(|i| i.id == issue.id) {
                let _ = std::mem::replace(&mut model.issues_mut()[idx], issue);
            }
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Issue,
                OperationKind::SingleModified,
                Some(id),
            ));
        }
        WsMsg::IssueDeleted(id, _count) => {
            let mut old = vec![];
            std::mem::swap(model.issues_mut(), &mut old);
            for is in old {
                if is.id == id {
                    continue;
                }
                model.issues_mut().push(is);
            }
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Issue,
                OperationKind::SingleRemoved,
                Some(id),
            ));
        }
        // users
        WsMsg::ProjectUsersLoaded(v) => {
            model.users = v.clone();
            model.users_by_id.clear();
            for user in v {
                model.users_by_id.insert(user.id, user.clone());
            }
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::User,
                OperationKind::ListLoaded,
                None,
            ));
        }
        // comments
        WsMsg::IssueCommentsLoaded(mut comments) => {
            let issue_id = match &model.modals().edit_issue {
                Some(modal) => modal.id,
                _ => return,
            };
            if comments.iter().any(|c| c.issue_id != issue_id) {
                return;
            }
            comments.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));
            model.comments = comments.clone();
            for comment in comments {
                model.comments_by_id.insert(comment.id, comment);
            }
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Comment,
                OperationKind::ListLoaded,
                None,
            ));
        }
        WsMsg::CommentUpdated(comment) => {
            let comment_id = comment.id;
            if let Some(idx) = model.comments.iter().position(|c| c.id == comment.id) {
                let _ = std::mem::replace(&mut model.comments[idx], comment.clone());
                model.comments_by_id.insert(comment.id, comment);
            }
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Comment,
                OperationKind::SingleModified,
                Some(comment_id),
            ));
        }
        WsMsg::CommentDeleted(comment_id, _count) => {
            if let Some(idx) = model.comments.iter().position(|c| c.id == comment_id) {
                model.comments.remove(idx);
            }
            model.comments_by_id.remove(&comment_id);
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Comment,
                OperationKind::SingleRemoved,
                Some(comment_id),
            ));
        }
        WsMsg::AvatarUrlChanged(user_id, avatar_url) => {
            for user in model.users.iter_mut() {
                if user.id == user_id {
                    user.avatar_url = Some(avatar_url.clone());
                }
            }
            if let Some(me) = model.user.as_mut() {
                if me.id == user_id {
                    me.avatar_url = Some(avatar_url);
                }
            }
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::User,
                OperationKind::SingleModified,
                Some(user_id),
            ));
        }
        // messages
        WsMsg::MessageUpdated(mut received) => {
            if let Some(idx) = model.messages.iter().position(|m| m.id == received.id) {
                std::mem::swap(&mut model.messages[idx], &mut received);
            }
            model.messages.sort_by(|a, b| a.id.cmp(&b.id));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Message,
                OperationKind::SingleModified,
                Some(received.id),
            ));
        }
        WsMsg::MessagesLoaded(v) => {
            model.messages = v;
            model.messages.sort_by(|a, b| a.id.cmp(&b.id));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Message,
                OperationKind::ListLoaded,
                None,
            ));
        }
        WsMsg::MessageMarkedSeen(id, _count) => {
            if let Some(idx) = model.messages.iter().position(|m| m.id == id) {
                model.messages.remove(idx);
            }
            model.messages.sort_by(|a, b| a.id.cmp(&b.id));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Message,
                OperationKind::SingleRemoved,
                Some(id),
            ));
        }

        // epics
        WsMsg::EpicsLoaded(epics) => {
            model.epics = epics.clone();
            for epic in epics {
                model.epics_by_id.insert(epic.id, epic);
            }
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Epic,
                OperationKind::ListLoaded,
                None,
            ));
        }
        WsMsg::EpicCreated(epic) => {
            let id = epic.id;
            model.epics.push(epic.clone());
            model.epics.sort_by(|a, b| a.id.cmp(&b.id));
            model.epics_by_id.insert(epic.id, epic);
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Epic,
                OperationKind::SingleCreated,
                Some(id),
            ));
        }
        WsMsg::EpicUpdated(epic) => {
            let epic_id = epic.id;
            if let Some(idx) = model.epics.iter().position(|e| e.id == epic.id) {
                let _ = std::mem::replace(&mut model.epics[idx], epic.clone());
            }
            model.epics_by_id.insert(epic.id, epic);
            model.epics.sort_by(|a, b| a.id.cmp(&b.id));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Epic,
                OperationKind::SingleModified,
                Some(epic_id),
            ));
        }
        WsMsg::EpicDeleted(id, _count) => {
            if let Some(idx) = model.epics.iter().position(|e| e.id == id) {
                model.epics.remove(idx);
            }
            model.epics_by_id.remove(&id);
            model.epics.sort_by(|a, b| a.id.cmp(&b.id));
            orders.send_msg(Msg::ResourceChanged(
                ResourceKind::Epic,
                OperationKind::SingleRemoved,
                Some(id),
            ));
        }
        _ => (),
    };
}

fn init_current_project(model: &mut Model, orders: &mut impl Orders<Msg>) {
    if model.projects.is_empty() {
        return;
    }
    model.project = model.current_user_project.as_ref().and_then(|up| {
        model
            .projects
            .iter()
            .find(|p| p.id == up.project_id)
            .cloned()
    });
    orders
        .skip()
        .send_msg(Msg::ProjectChanged(model.project.as_ref().cloned()));
}

fn is_non_logged_area() -> bool {
    let pathname = seed::document().location().unwrap().pathname().unwrap();
    matches!(pathname.as_str(), "/login" | "/register" | "/invite")
}
