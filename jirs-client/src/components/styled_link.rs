use {
    crate::{shared::ToNode, Msg},
    seed::{prelude::*, *},
    std::str::FromStr,
};

pub struct StyledLink<'l> {
    children: Vec<Node<Msg>>,
    class_list: Vec<&'l str>,
    href: &'l str,
    disabled: bool,
}

impl<'l> StyledLink<'l> {
    pub fn build() -> StyledLinkBuilder<'l> {
        StyledLinkBuilder::default()
    }
}

#[derive(Default)]
pub struct StyledLinkBuilder<'l> {
    children: Vec<Node<Msg>>,
    class_list: Vec<&'l str>,
    href: &'l str,
    disabled: bool,
}

impl<'l> StyledLinkBuilder<'l> {
    pub fn add_child(mut self, child: Node<Msg>) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_icon(self) -> Self {
        self.add_child(crate::components::styled_icon::Icon::Link.into_node())
            .add_class("withIcon")
    }

    pub fn add_class(mut self, name: &'l str) -> Self {
        self.class_list.push(name);
        self
    }

    pub fn href(mut self, href: &'l str) -> Self {
        self.href = href;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn text(self, s: &'l str) -> Self {
        self.add_child(span![s])
    }

    pub fn build(self) -> StyledLink<'l> {
        StyledLink {
            children: self.children,
            class_list: self.class_list,
            href: self.href,
            disabled: self.disabled,
        }
    }
}

impl<'l> ToNode for StyledLink<'l> {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

pub fn render(values: StyledLink) -> Node<Msg> {
    let StyledLink {
        children,
        class_list,
        href,
        disabled,
    } = values;

    let on_click = if disabled {
        None
    } else {
        let href = href.to_string();
        Some(mouse_ev("click", move |ev| {
            if href.starts_with('/') {
                ev.prevent_default();
                ev.stop_propagation();
                if let Ok(url) = seed::Url::from_str(href.as_str()) {
                    url.go_and_push();
                }
            }

            None as Option<Msg>
        }))
    };

    a![
        C!["styledLink"],
        attrs![
            At::Class => class_list.join(" "),
            At::Href => href,
        ],
        IF![disabled => attrs![At::OnClick => "return false"]],
        on_click,
        children,
    ]
}
