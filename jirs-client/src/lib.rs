#![feature(or_patterns, type_ascription, trait_alias, drain_filter)]

use {
    crate::{
        components::{
            styled_date_time_input::StyledDateTimeChanged,
            styled_select::StyledSelectChanged,
            styled_tooltip,
            styled_tooltip::{Variant as StyledTooltip, Variant},
        },
        model::{ModalType, Model, Page},
        shared::{go_to_board, go_to_login},
        ws::{flush_queue, open_socket, read_incoming, send_ws_msg},
    },
    jirs_data::*,
    seed::{prelude::*, *},
    web_sys::File,
};
pub use {changes::*, components::*, fields::*, images::*};

// use crate::shared::styled_rte::RteMsg;

mod changes;
mod components;
mod fields;
mod images;
mod modals;
mod model;
mod pages;
mod shared;
pub mod validations;
mod ws;

// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
pub enum ResourceKind {
    Issue,
    IssueStatus,
    Epic,
    Project,
    User,
    UserProject,
    Message,
    Comment,
    Auth,
}

#[derive(Debug)]
pub enum OperationKind {
    ListLoaded,
    SingleLoaded,
    SingleCreated,
    SingleRemoved,
    SingleModified,
}

#[derive(Debug)]
pub enum Msg {
    GlobalKeyDown {
        key: String,
        shift: bool,
        ctrl: bool,
        alt: bool,
    },
    PageChanged(PageChanged),
    ChangePage(model::Page),

    UserChanged(Option<User>),
    ProjectChanged(Option<Project>),

    StyledSelectChanged(FieldId, StyledSelectChanged),
    StyledDateTimeInputChanged(FieldId, StyledDateTimeChanged),
    InternalFailure(String),
    ToggleTooltip(StyledTooltip),

    // Auth Token
    AuthTokenStored,
    AuthTokenErased,
    SignInRequest,
    BindClientRequest,

    // users
    InviteRequest,
    InviteRevokeRequest(InvitationId),
    InviteApproveRequest(InvitationId),
    InvitedUserRemove(UserId),

    // sign up
    SignUpRequest,

    // project
    ProjectAvatarFilterChanged(UserId, AvatarFilterActive),
    ProjectToggleOnlyMy,
    ProjectToggleRecentlyUpdated,
    ProjectClearFilters,

    // inputs
    StrInputChanged(FieldId, String),

    U32InputChanged(FieldId, u32),
    FileInputChanged(FieldId, Vec<File>),
    // Rte(FieldId, RteMsg),

    // issues
    AddIssue,
    DeleteIssue(EpicId),

    // issues and filters
    SetActiveIssue(Option<IssueId>),

    // epics
    AddEpic,
    DeleteEpic,
    UpdateEpic,
    TransformEpic,

    // issue statuses
    DeleteIssueStatus(IssueStatusId),

    // comments
    SaveComment,
    DeleteComment(CommentId),

    // profile
    AvatarUpdateFetched(String),

    // modals
    ModalOpened(ModalType),
    ModalDropped,
    ModalChanged(FieldChange),

    // popups
    MessageSeen(MessageId),
    MessageInvitationApproved(InvitationToken),
    MessageInvitationDismiss(InvitationToken),

    // WebSocket
    WebSocketChange(WebSocketChanged),

    // resource changes
    ResourceChanged(ResourceKind, OperationKind, Option<i32>),
}

fn update(msg: Msg, model: &mut model::Model, orders: &mut impl Orders<Msg>) {
    if model.ws.is_none() {
        open_socket(model, orders);
    }

    let msg = match msg {
        Msg::WebSocketChange(change) => {
            match change {
                WebSocketChanged::WebSocketOpened => {
                    flush_queue(model, orders);
                    send_ws_msg(WsMsg::Ping, model.ws.as_ref(), orders);
                    authorize_or_redirect(model, orders);
                    orders.skip();
                    return;
                }
                WebSocketChanged::SendPing => {
                    send_ws_msg(WsMsg::Ping, model.ws.as_ref(), orders);
                    orders.skip();
                    return;
                }
                WebSocketChanged::WebSocketMessage(incoming) => {
                    orders.perform_cmd(read_incoming(incoming));
                    orders.skip();
                    return;
                }
                WebSocketChanged::WsMsg(ws_msg) => {
                    ws::update(ws_msg, model, orders);
                    orders.skip();
                    return;
                }
                WebSocketChanged::WebSocketMessageLoaded(v) => {
                    match bincode::deserialize(v.as_slice()) {
                        Ok(WsMsg::Ping | WsMsg::Pong) => {
                            orders.skip().perform_cmd(cmds::timeout(300, || {
                                Msg::WebSocketChange(WebSocketChanged::SendPing)
                            }));
                        }
                        Ok(m) => {
                            orders
                                .skip()
                                .send_msg(Msg::WebSocketChange(WebSocketChanged::WsMsg(m)));
                        }
                        _ => (),
                    };
                    return;
                }
                WebSocketChanged::WebSocketClosed => {
                    open_socket(model, orders);
                }
                WebSocketChanged::Bounced(ws_msg) => {
                    model.ws_queue.push(ws_msg);
                    open_socket(model, orders);
                    return;
                }
            };
            Msg::WebSocketChange(change)
        }
        _ => msg,
    };

    if cfg!(debug_assertions) {
        log!(msg);
    }

    match &msg {
        Msg::AuthTokenStored => {
            go_to_board(orders);
            return;
        }
        Msg::AuthTokenErased => {
            go_to_login(orders);
            return;
        }
        Msg::ChangePage(page) => {
            model.page = *page;
        }
        Msg::ToggleTooltip(variant) => match variant {
            styled_tooltip::Variant::About => {
                model.about_tooltip_visible = !model.about_tooltip_visible;
            }
            styled_tooltip::Variant::Messages => {
                model.messages_tooltip_visible = !model.messages_tooltip_visible;
            }
            styled_tooltip::Variant::CodeBuilder => {}
            Variant::TableBuilder => {}
            Variant::DateTimeBuilder => {}
        },
        _ => (),
    }

    {
        use crate::shared::{aside, navbar_left};
        aside::update(&msg, model, orders);
        navbar_left::update(&msg, model, orders);
    }
    crate::modals::update(&msg, model, orders);

    match model.page {
        Page::Project
        | Page::AddIssue
        | Page::EditIssue(..)
        | Page::DeleteEpic(..)
        | Page::EditEpic(..) => pages::project_page::update(msg, model, orders),
        Page::ProjectSettings => pages::project_settings_page::update(msg, model, orders),
        Page::SignIn => pages::sign_in_page::update(msg, model, orders),
        Page::SignUp => pages::sign_up_page::update(msg, model, orders),
        Page::Invite => pages::invite_page::update(msg, model, orders),
        Page::Users => pages::users_page::update(msg, model, orders),
        Page::Profile => pages::profile_page::update(msg, model, orders),
        Page::Reports => pages::reports_page::update(msg, model, orders),
        Page::IssuesAndFilters => pages::issues_and_filters::update(msg, model, orders),
    }
    if cfg!(features = "print-model") {
        log!(model);
    }
}

fn view(model: &model::Model) -> Node<Msg> {
    match model.page {
        Page::Project
        | Page::AddIssue
        | Page::EditIssue(..)
        | Page::DeleteEpic(..)
        | Page::EditEpic(..) => pages::project_page::view(model),
        Page::ProjectSettings => pages::project_settings_page::view(model),
        Page::SignIn => pages::sign_in_page::view(model),
        Page::SignUp => pages::sign_up_page::view(model),
        Page::Invite => pages::invite_page::view(model),
        Page::Users => pages::users_page::view(model),
        Page::Profile => pages::profile_page::view(model),
        Page::Reports => pages::reports_page::view(model),
        Page::IssuesAndFilters => pages::issues_and_filters::view(model),
    }
}

fn resolve_page(url: Url) -> Option<Page> {
    if url.path().is_empty() {
        return Some(Page::Project);
    }

    let page = match url.path()[0].as_ref() {
        "board" => Page::Project,
        "profile" => Page::Profile,
        "issues" => match url.path().get(1).as_ref().map(|s| s.parse::<i32>()) {
            Some(Ok(id)) => Page::EditIssue(id),
            _ => return None,
        },
        "issues-and-filters" => Page::IssuesAndFilters,
        "add-issue" => Page::AddIssue,
        "project-settings" => Page::ProjectSettings,
        "login" => Page::SignIn,
        "register" => Page::SignUp,
        "invite" => Page::Invite,
        "users" => Page::Users,
        "reports" => Page::Reports,
        "delete-epic" => match url.path().get(1).as_ref().map(|s| s.parse::<i32>()) {
            Some(Ok(id)) => Page::DeleteEpic(id),
            _ => return None,
        },
        "edit-epic" => match url.path().get(1).as_ref().map(|s| s.parse::<i32>()) {
            Some(Ok(id)) => Page::EditEpic(id),
            _ => return None,
        },
        _ => Page::Project,
    };
    Some(page)
}

pub static mut HOST_URL: String = String::new();
pub static mut WS_URL: String = String::new();

#[wasm_bindgen]
pub fn render(host_url: String, ws_url: String) {
    unsafe {
        HOST_URL = host_url;
        WS_URL = ws_url;
    }

    let app = seed::App::start("app", init, update, view);

    {
        let app_clone = app.clone();
        let on_key_down = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let event: web_sys::KeyboardEvent = event.unchecked_into();

            let tag_name: String = seed::document()
                .active_element()
                .map(|el| el.tag_name())
                .unwrap_or_default();

            let key = match tag_name.to_lowercase().as_str() {
                "input" | "textarea" => return,
                _ => event.key(),
            };

            let msg = Msg::GlobalKeyDown {
                key,
                shift: event.shift_key(),
                ctrl: event.ctrl_key(),
                alt: event.alt_key(),
            };
            app_clone.update(msg);
        }) as Box<dyn FnMut(_)>);
        seed::body()
            .add_event_listener_with_callback("keyup", on_key_down.as_ref().unchecked_ref())
            .expect("Failed to mount global key handler");
        on_key_down.forget();
    }
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let host_url = unsafe { HOST_URL.clone() };
    let ws_url = unsafe { WS_URL.clone() };
    let mut model = Model::new(host_url, ws_url);
    unsafe {
        HOST_URL = "".to_string();
        WS_URL = "".to_string();
    }
    model.page = resolve_page(url).unwrap_or(Page::Project);
    open_socket(&mut model, orders);

    model
}

#[inline]
fn authorize_or_redirect(model: &mut Model, orders: &mut impl Orders<Msg>) {
    let pathname = seed::document().location().unwrap().pathname().unwrap();
    match crate::shared::read_auth_token() {
        Ok(token) => {
            send_ws_msg(WsMsg::AuthorizeLoad(token), model.ws.as_ref(), orders);
        }
        Err(..) => {
            match pathname.as_str() {
                "/login" | "/register" | "/invite" => {}
                _ => {
                    go_to_login(orders);
                }
            };
        }
    };
}
