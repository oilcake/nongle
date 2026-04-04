use atomic_float::AtomicF32;
use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::{Alignment, Column, Command, Element, IcedEditor, IcedState, Length, Space, Text, WindowQueue, alignment, assets, create_iced_editor, executor, widgets as nih_widgets};
// use nih_plug_iced::*;
use std::sync::Arc;

use crate::NongleParams;

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(200, 150)
}

pub(crate) fn create(
    params: Arc<NongleParams>,
    peak_meter: Arc<AtomicF32>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<GainEditor>(editor_state, (params, peak_meter))
}

struct GainEditor {
    params: Arc<NongleParams>,
    context: Arc<dyn GuiContext>,

    gain_slider_state: nih_widgets::param_slider::State,
    peak_meter_state: nih_widgets::peak_meter::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    /// Update a parameter's value.
    ParamUpdate(nih_widgets::ParamMessage),
}

impl IcedEditor for GainEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = (Arc<NongleParams>, Arc<AtomicF32>);

    fn new(
        (params, peak_meter): Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = GainEditor {
            params,
            context,


            gain_slider_state: Default::default(),
            peak_meter_state: Default::default(),
        };

        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::ParamUpdate(message) => self.handle_param_message(message),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .align_items(Alignment::Center)
            .push(
                Text::new("Nongle")
                    .font(assets::NOTO_SANS_LIGHT)
                    .size(40)
                    .height(50.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Bottom),
            )
            .push(
                Text::new("Window Width")
                    .height(20.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            )
            .push(
                nih_widgets::ParamSlider::new(&mut self.gain_slider_state, &self.params.win_width)
                    .map(Message::ParamUpdate),
            )
            .push(Space::with_height(10.into()))
            .into()
    }

    fn background_color(&self) -> nih_plug_iced::Color {
        nih_plug_iced::Color {
            r: 0.98,
            g: 0.98,
            b: 0.98,
            a: 1.0,
        }
    }
}
