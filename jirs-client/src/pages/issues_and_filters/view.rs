mod filters;
mod issue_info;

use {
    crate::{model::Model, shared::inner_layout, Msg},
    seed::{prelude::*, *},
};

pub fn view(model: &Model) -> Node<Msg> {
    let page = crate::match_page!(model, IssuesAndFilters, Node::Empty);
    let project_name = model
        .project
        .as_ref()
        .map(|p| p.name.as_str())
        .unwrap_or_default();
    let details = match page.active_id.and_then(|id| model.issues_by_id.get(&id)) {
        Some(issue) => issue_info::issue_details(model, issue, project_name),
        _ => Node::Empty,
    };
    let content = div![
        C!["container"],
        issue_info::issues_list(model, page, project_name),
        details
    ];
    inner_layout(
        model,
        "issuesAndFilters",
        &[filters::filters(model, page), content],
    )
}
