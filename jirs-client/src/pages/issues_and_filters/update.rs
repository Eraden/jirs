use {
    crate::{
        model::{Model, PageContent},
        ws::board_load,
        Msg, OperationKind, Page, ResourceKind,
    },
    seed::prelude::*,
};

pub fn update(msg: Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ResourceChanged(ResourceKind::Auth, OperationKind::SingleLoaded, Some(_))
        | Msg::ChangePage(Page::IssuesAndFilters) => {
            board_load(model, orders);
            build_page_content(model);
        }
        _ => (),
    }

    match msg {
        Msg::ResourceChanged(ResourceKind::Issue, OperationKind::ListLoaded, _) => {
            let issues = super::Model::visible_issues(model.issues());
            let first_id = model.issues().first().as_ref().map(|issue| issue.id);
            let page = crate::match_page_mut!(model, IssuesAndFilters);
            if page.active_id.is_none() {
                page.active_id = first_id;
            }
            page.visible_issues = issues;
        }
        Msg::SetActiveIssue(Some(id)) => {
            let page = crate::match_page_mut!(model, IssuesAndFilters);
            page.active_id = Some(id);
        }
        Msg::SetActiveIssue(None) => {
            let first_id = model.issues().first().as_ref().map(|issue| issue.id);
            let page = crate::match_page_mut!(model, IssuesAndFilters);
            page.active_id = first_id;
        }
        _ => {}
    }
}

fn build_page_content(model: &mut Model) {
    model.page_content = PageContent::IssuesAndFilters(Box::new(super::Model::new(model)));
}
