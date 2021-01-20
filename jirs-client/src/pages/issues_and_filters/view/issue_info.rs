use {
    crate::{
        components::{styled_icon::*, styled_link::*},
        model::Model,
        shared::ToNode,
        Msg,
    },
    jirs_data::{Issue, IssueId},
    seed::{prelude::*, *},
};

pub fn issue_details(model: &Model, issue: &Issue, project_name: &str) -> Node<Msg> {
    let Issue {
        id,
        title,
        issue_type,
        priority,
        description,
        issue_status_id,
        epic_id,
        created_at: _,
        updated_at: _,
        user_ids: _,
        ..
    } = issue;

    let issue_link = StyledLink::build()
        .href(format!("/issues/{}", id).as_str())
        .text(format!("{}-{}", project_name, id).as_str())
        .disabled()
        .build()
        .into_node();

    let project_link = StyledLink::build()
        .disabled()
        .href("/board")
        .text(
            model
                .project
                .as_ref()
                .map(|p| p.name.as_str())
                .unwrap_or_default(),
        )
        .build()
        .into_node();

    let details = {
        let issue_type = {
            let type_name = issue_type.to_string();
            let type_icon = Icon::from(*issue_type).into_node();
            li![
                C!["line"],
                div![C!["detailsTitle"], "Type:"],
                div![C!["type detailsValue"], type_icon, type_name],
            ]
        };
        let priority = {
            let name = priority.to_string();
            let icon = Icon::from(*priority).into_node();
            li![
                C!["line"],
                div![C!["detailsTitle"], "Priority:"],
                div![C!["priority detailsValue"], icon, name],
            ]
        };

        let epic = li![
            C!["line"],
            div![C!["detailsTitle"], "Epic link:"],
            match epic_id.and_then(|id| model.epics_by_id.get(&id)) {
                Some(epic) => div![C!["detailsValue epic"], a![epic.name.as_str()]],
                _ => div![C!["detailsValue epic"], "None"],
            },
        ];

        let status = li![
            C!["line"],
            div![C!["detailsTitle"], "Status:"],
            div![C!["detailsValue status"], {
                match model.issue_statuses_by_id.get(issue_status_id) {
                    Some(status) => status.name.as_str(),
                    _ => "",
                }
            }]
        ];

        ul![C!["details"], issue_type, priority, epic, status]
    };

    let right_column = div![];

    div![
        C!["issueInfo"],
        div![
            C!["header"],
            div![C!["logo"], "X"],
            div![C!["title"], title.as_str()],
            div![C!["path"], project_link, "/", issue_link]
        ],
        div![C!["issueBody"], details, right_column],
        div![
            C!["description"],
            raw! { description.as_deref().unwrap_or_default() }
        ]
    ]
}

pub fn issues_list(model: &Model, page: &super::super::Model, project_name: &str) -> Node<Msg> {
    let issues: Vec<Node<Msg>> = page
        .visible_issues
        .iter()
        .filter_map(|id| model.issues_by_id.get(id))
        .map(|issue| issue_entry(page, issue, project_name))
        .collect();
    ul![C!["issuesList"], issues]
}

fn issue_entry(page: &super::super::Model, issue: &Issue, project_name: &str) -> Node<Msg> {
    let link = issue_link(issue.id, project_name);
    let ty = {
        let icon: Icon = issue.issue_type.into();
        icon.into_node()
    };
    let priority = {
        let icon: Icon = issue.priority.into();
        icon.into_node()
    };

    let on_click = {
        let id = issue.id;
        mouse_ev("click", move |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::SetActiveIssue(Some(id))
        })
    };

    li![
        C!["listItem"],
        on_click,
        a![
            C!["issue"],
            IF![page.active_id == Some(issue.id) => C!["active"]],
            div![C!["number"], link],
            div![C!["name"], issue.title.as_str()],
            div![C!["type"], ty],
            div![C!["priority"], priority]
        ]
    ]
}

fn issue_link(id: IssueId, project_name: &str) -> Node<Msg> {
    StyledLink::build()
        .with_icon()
        .href(format!("/issues/{}", id).as_str())
        .text(format!("{}-{}", project_name, id).as_str())
        .disabled()
        .build()
        .into_node()
}
