use seed::prelude::*;

use jirs_data::WsMsg;

use crate::changes::{PageChanged, ReportsPageChange};
use crate::model::{Model, Page, PageContent, ReportsPage};
use crate::ws::board_load;
use crate::{Msg, WebSocketChanged};

pub fn update(msg: Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    if let Msg::ChangePage(Page::Reports) = msg {
        build_page_content(model);
    }

    let page = match &mut model.page_content {
        PageContent::Reports(page) => page,
        _ => return,
    };

    if model.user.is_none() {
        return;
    }
    match msg {
        Msg::UserChanged(Some(..))
        | Msg::WebSocketChange(WebSocketChanged::WsMsg(WsMsg::AuthorizeLoaded(..)))
        | Msg::ChangePage(Page::Reports) => {
            board_load(model, orders);
        }
        Msg::PageChanged(PageChanged::Reports(ReportsPageChange::DayHovered(v))) => {
            page.hovered_day = v;
        }
        Msg::PageChanged(PageChanged::Reports(ReportsPageChange::DaySelected(v))) => {
            page.selected_day = v;
        }
        _ => {}
    }
}

fn build_page_content(model: &mut Model) {
    model.page_content = PageContent::Reports(Box::new(ReportsPage::default()))
}
