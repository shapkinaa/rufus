use crate::{
    app::{
        actions::{AppAction, FileManagerActions},
        file_system::FileSystem,
        state::AppState,
    },
    core::{
        events::Event,
        store::Store,
        ui::{component::Component, component_base::ComponentBase},
    },
};
use std::fmt::Debug;
use tui::{
    style::Style,
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use super::create_modal_layout;

#[derive(Clone, Default)]
pub struct MessageboxModalComponentProps {
    message: Option<String>,
    show_icons: bool,
    icon: String,
}

impl MessageboxModalComponentProps {
    pub fn new(message: String, show_icons: bool, icon: String) -> Self {
        MessageboxModalComponentProps {
            message: Some(message),
            show_icons,
            icon,
        }
    }
}

pub struct MessageboxModalComponent<TFileSystem: Clone + Debug + Default + FileSystem> {
    base: ComponentBase<MessageboxModalComponentProps, ()>,
    _maker: std::marker::PhantomData<TFileSystem>,
}

impl<TFileSystem: Clone + Debug + Default + FileSystem> MessageboxModalComponent<TFileSystem> {
    pub fn with_props(props: MessageboxModalComponentProps) -> Self {
        MessageboxModalComponent {
            base: ComponentBase::new(Some(props), None),
            _maker: std::marker::PhantomData,
        }
    }
}

impl<TFileSystem: Clone + Debug + Default + FileSystem>
    Component<Event, AppState<TFileSystem>, FileManagerActions>
    for MessageboxModalComponent<TFileSystem>
{
    fn handle_event(
        &mut self,
        event: Event,
        store: &mut Store<AppState<TFileSystem>, FileManagerActions>,
    ) -> bool {
        let state = store.get_state();
        if let Event::Keyboard(key_evt) = event {
            if state.config.keyboard_cfg.close.is_pressed(key_evt) {
                store.dispatch(FileManagerActions::App(AppAction::CloseModal));
                return true;
            }
        }

        false
    }

    fn render<TBackend: tui::backend::Backend>(
        &self,
        frame: &mut tui::Frame<TBackend>,
        area: Option<tui::layout::Rect>,
    ) {
        let layout = if let Some(area) = area {
            create_modal_layout(45, 30, area)
        } else {
            create_modal_layout(45, 30, frame.size())
        };
        let props = self.base.get_props().unwrap();
        let message = if let Some(message) = props.message {
            message.clone()
        } else {
            "".to_string()
        };
        let block = Block::default()
            .title(Spans::from(vec![
                Span::from("| "),
                Span::from("Filesystem Item Props (CloseKey to close)"),
                Span::from(" |"),
            ]))
            .borders(Borders::ALL)
            .border_style(Style::default())
            .border_type(BorderType::Thick)
            .style(Style::default());

        let paragraph = Paragraph::new(message)
            .block(block)
            .alignment(tui::layout::Alignment::Left);

        frame.render_widget(Clear, layout);
        frame.render_widget(paragraph, layout);
    }
}
