use seed::prelude::*;

use jirs_data::WsMsg;

use crate::{model, Msg, APP};

pub mod issue;

pub fn handle(msg: WsMsg) {
    let app = match unsafe { APP.as_mut().unwrap() }.write() {
        Ok(app) => app,
        _ => return,
    };

    match msg {
        WsMsg::Ping | WsMsg::Pong => {}
        _ => app.update(Msg::WsMsg(msg)),
    }
}

pub fn update(msg: &Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::WsMsg(WsMsg::ProjectLoaded(project)) => {
            model.project = Some(project.clone());
        }
        Msg::WsMsg(WsMsg::AuthorizeLoaded(Ok(user))) => {
            model.user = Some(user.clone());
        }
        Msg::WsMsg(WsMsg::ProjectIssuesLoaded(v)) => {
            let mut v = v.clone();
            v.sort_by(|a, b| (a.list_position as i64).cmp(&(b.list_position as i64)));
            model.issues = v;
        }
        Msg::WsMsg(WsMsg::ProjectUsersLoaded(v)) => {
            model.users = v.clone();
        }
        _ => (),
    };
    orders.render();
}
