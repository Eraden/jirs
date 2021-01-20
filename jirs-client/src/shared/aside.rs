use {
    crate::{
        components::styled_icon::{Icon, StyledIcon},
        model::{Model, Page},
        shared::{divider, ToNode},
        ws::enqueue_ws_msg,
        Msg, OperationKind, ResourceKind,
    },
    jirs_data::{UserRole, WsMsg},
    seed::{prelude::*, *},
};

pub fn update(msg: &Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    if let Msg::ResourceChanged(ResourceKind::Auth, OperationKind::SingleLoaded, _) = msg {
        enqueue_ws_msg(
            vec![
                WsMsg::UserProjectsLoad,
                WsMsg::ProjectsLoad,
                WsMsg::ProjectUsersLoad,
                WsMsg::MessagesLoad,
            ],
            model.ws.as_ref(),
            orders,
        );
        orders.skip();
    }
}

pub fn render(model: &Model) -> Node<Msg> {
    let project_icon = crate::images::project_avatar::render();

    let project_info = match model.project.as_ref() {
        Some(project) => li![
            id!["projectInfo"],
            project_icon,
            div![
                C!["projectTexts"],
                div![C!["projectName"], project.name.as_str()],
                div![C!["projectCategory"], project.category.to_string()]
            ],
        ],
        _ => li![
            id!["projectInfo"],
            div![
                C!["projectTexts"],
                div![C!["projectName"], ""],
                div![C!["projectCategory"], ""]
            ],
        ],
    };

    nav![
        id!["sidebar"],
        ul![
            project_info,
            sidebar_link_item(model, "Kanban Board", Icon::Board, Some(Page::Project)),
            project_settings(model),
            li![divider()],
            sidebar_link_item(model, "Releases", Icon::Shipping, None),
            sidebar_link_item(
                model,
                "Issue and Filters",
                Icon::Issues,
                Some(Page::IssuesAndFilters)
            ),
            sidebar_link_item(model, "Pages", Icon::Page, None),
            sidebar_link_item(model, "Reports", Icon::Reports, Some(Page::Reports)),
            sidebar_link_item(model, "Components", Icon::Component, None),
            users_link(model)
        ]
    ]
}

#[inline]
fn project_settings(model: &Model) -> Node<Msg> {
    if model.current_user_role() <= UserRole::User {
        return Node::Empty;
    }

    sidebar_link_item(
        model,
        "Project settings",
        Icon::Settings,
        Some(Page::ProjectSettings),
    )
}

#[inline]
fn users_link(model: &Model) -> Node<Msg> {
    if model.current_user_role() <= UserRole::User {
        return Node::Empty;
    }

    sidebar_link_item(model, "Users", Icon::Cop, Some(Page::Users))
}

#[inline]
fn sidebar_link_item(model: &Model, name: &str, icon: Icon, page: Option<Page>) -> Node<Msg> {
    let path = page.map(|ref p| p.to_path()).unwrap_or_default();
    let allow_flag = if page.is_none() {
        Some(C!["notAllowed"])
    } else {
        None
    };
    let active_flag = page.filter(|p| *p == model.page).map(|_| C!["active"]);
    let icon_node = StyledIcon::build(icon).build().into_node();
    let on_click = page.map(|p| {
        mouse_ev("click", move |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            seed::Url::new()
                .set_path(p.to_path().split('/').filter(|s| !s.is_empty()))
                .go_and_push();
            Msg::ChangePage(p)
        })
    });

    li![
        C!["linkItem"],
        active_flag,
        allow_flag,
        C![icon.to_str()],
        a![
            attrs![At::Href => path],
            on_click,
            icon_node,
            div![C!["linkText"], name],
        ]
    ]
}
