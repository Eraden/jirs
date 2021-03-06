use jirs_data::{IssuePriority, IssueType};
use seed::prelude::*;
use seed::*;

use crate::Msg;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Icon {
    Stopwatch,

    Bug,
    Task,
    Story,
    Epic,

    DoubleLeft,
    DoubleRight,

    ArrowDown,
    ArrowLeftCircle,
    ArrowUp,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    Board,
    Help,
    Link,
    Menu,
    More,
    Attach,
    Plus,
    Search,
    Issues,
    Settings,
    Close,
    Check,
    Feedback,
    Trash,
    Github,
    Shipping,
    Component,
    Reports,
    Page,
    Calendar,
    ArrowLeft,
    ArrowRight,
    Cop,
    Message,
    User,

    AlignCenter,
    AlignLeft,
    AlignRight,
    AllCaps,
    Bold,
    Brush,
    ClipBoard,
    CodeAlt,
    ColorBucket,
    ColorPicker,
    CopyInvert,
    Copy,
    Cut,
    DeleteAlt,
    EditAlt,
    EraserAlt,
    Font,
    Heading,
    Indent,
    ItalicAlt,
    Italic,
    JustifyAll,
    JustifyCenter,
    JustifyLeft,
    JustifyRight,
    LinkBroken,
    ListingDots,
    ListingNumber,
    Outdent,
    PaperClip,
    Paragraph,
    Pin,
    Printer,
    Redo,
    Rotation,
    Save,
    SmallCap,
    StrikeThrough,
    SubListing,
    Subscript,
    Superscript,
    Table,
    TextHeight,
    TextWidth,
    Underline,
    Undo,

    MdEditor,
    RteEditor,
}

impl Icon {
    #[inline(always)]
    pub fn to_color(self) -> Option<String> {
        match self {
            Icon::Bug | Icon::Task | Icon::Story | Icon::Epic => Some(format!("var(--{})", self)),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn to_str<'l>(self) -> &'l str {
        match self {
            Icon::Bug => "bug",
            Icon::Stopwatch => "stopwatch",
            Icon::Task => "task",
            Icon::Story => "story",
            Icon::ArrowDown => "arrowDown",
            Icon::ArrowLeftCircle => "arrowLeftCircle",
            Icon::ArrowUp => "arrowUp",
            Icon::ChevronDown => "chevronDown",
            Icon::ChevronLeft => "chevronLeft",
            Icon::ChevronRight => "chevronRight",
            Icon::ChevronUp => "chevronUp",
            Icon::Board => "board",
            Icon::Help => "help",
            Icon::Link => "link",
            Icon::Menu => "menu",
            Icon::More => "more",
            Icon::Attach => "attach",
            Icon::Plus => "plus",
            Icon::Search => "search",
            Icon::Issues => "issues",
            Icon::Settings => "settings",
            Icon::Close => "close",
            Icon::Check => "check",
            Icon::Feedback => "feedback",
            Icon::Trash => "trash",
            Icon::Github => "github",
            Icon::Shipping => "shipping",
            Icon::Component => "component",
            Icon::Reports => "reports",
            Icon::Page => "page",
            Icon::Calendar => "calendar",
            Icon::ArrowLeft => "arrowLeft",
            Icon::ArrowRight => "arrowRight",
            Icon::Cop => "cop",
            Icon::Message => "message",
            Icon::User => "user",
            // rte
            Icon::AlignCenter => "align-center",
            Icon::AlignLeft => "align-left",
            Icon::AlignRight => "align-right",
            Icon::AllCaps => "all-caps",
            Icon::Bold => "bold",
            Icon::Brush => "brush",
            Icon::ClipBoard => "clip-board",
            Icon::CodeAlt => "code-alt",
            Icon::ColorBucket => "color-bucket",
            Icon::ColorPicker => "color-picker",
            Icon::CopyInvert => "copy-invert",
            Icon::Copy => "copy",
            Icon::Cut => "cut",
            Icon::DeleteAlt => "delete-alt",
            Icon::EditAlt => "edit-alt",
            Icon::EraserAlt => "eraser-alt",
            Icon::Font => "font",
            Icon::Heading => "heading",
            Icon::Indent => "indent",
            Icon::ItalicAlt => "italic-alt",
            Icon::Italic => "italic",
            Icon::JustifyAll => "justify-all",
            Icon::JustifyCenter => "justify-center",
            Icon::JustifyLeft => "justify-left",
            Icon::JustifyRight => "justify-right",
            Icon::LinkBroken => "link-broken",
            Icon::Outdent => "outdent",
            Icon::PaperClip => "paper-clip",
            Icon::Paragraph => "paragraph",
            Icon::Pin => "pin",
            Icon::Printer => "printer",
            Icon::Redo => "redo",
            Icon::Rotation => "rotation",
            Icon::Save => "save",
            Icon::SmallCap => "small-cap",
            Icon::StrikeThrough => "strike-through",
            Icon::SubListing => "sub-listing",
            Icon::Subscript => "subscript",
            Icon::Superscript => "superscript",
            Icon::Table => "table",
            Icon::TextHeight => "text-height",
            Icon::TextWidth => "text-width",
            Icon::Underline => "underline",
            Icon::Undo => "undo",
            Icon::ListingDots => "listing-dots",
            Icon::ListingNumber => "listing-number",
            Icon::Epic => "epic",

            Icon::DoubleLeft => "double-left",
            Icon::DoubleRight => "double-right",

            // editor
            Icon::MdEditor => "icofont-brand-compaq",
            Icon::RteEditor => "icofont-compass-alt-4",
        }
    }
}

impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}

impl From<IssueType> for Icon {
    #[inline(always)]
    fn from(t: IssueType) -> Self {
        use IssueType::*;
        match t {
            Task => Icon::Task,
            Bug => Icon::Bug,
            Story => Icon::Story,
        }
    }
}

impl From<IssuePriority> for Icon {
    #[inline(always)]
    fn from(t: IssuePriority) -> Self {
        match t {
            IssuePriority::Highest => Icon::ArrowUp,
            IssuePriority::High => Icon::ArrowUp,
            IssuePriority::Medium => Icon::ArrowUp,
            IssuePriority::Low => Icon::ArrowDown,
            IssuePriority::Lowest => Icon::ArrowDown,
        }
    }
}

impl<'l> From<Icon> for StyledIcon<'l> {
    #[inline(always)]
    fn from(icon: Icon) -> StyledIcon<'l> {
        StyledIcon {
            icon,
            ..Default::default()
        }
    }
}

pub struct StyledIcon<'l> {
    pub icon: Icon,
    pub size: Option<i32>,
    pub class_list: &'l str,
    pub style_list: &'l str,
    pub color: Option<&'l str>,
    pub on_click: Option<EventHandler<Msg>>,
}

impl<'l> Default for StyledIcon<'l> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            icon: Icon::Stopwatch,
            size: None,
            class_list: "",
            style_list: "",
            color: None,
            on_click: None,
        }
    }
}

impl<'l> StyledIcon<'l> {
    #[inline(always)]
    pub fn render(self) -> Node<Msg> {
        let StyledIcon {
            icon,
            size,
            color,
            class_list,
            style_list,
            on_click,
        } = self;

        let styles: Vec<Attrs> = vec![
            size.map(|s| {
                let font_size = format!("font-size: {}", s);
                attrs![At::Style => font_size]
            }),
            icon.to_color().map(|s| {
                let color = format!("color: {}", s);
                attrs![At::Style => color]
            }),
            color.map(|s| attrs![At::Style => format!("color: var(--{})", s)]),
        ]
        .into_iter()
        .flatten()
        .collect();

        i![
            C!["styledIcon", class_list, icon.to_str()],
            styles,
            attrs![ At::Style => style_list ],
            on_click,
            ""
        ]
    }
}
