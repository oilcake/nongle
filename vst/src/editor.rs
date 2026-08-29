use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::{Alignment, Button, Column, Command, Element, IcedEditor, IcedState, Length, ProgressBar, Space, Subscription, Text, WindowQueue, WindowSubs, alignment, assets, create_iced_editor, executor, futures, subscription, widgets as nih_widgets};
use nih_plug_iced::futures::FutureExt;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

use crate::NongleParams;
use crate::LoadProgress;
use nih_plug_iced::widget::button;

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(200, 150)
}

pub(crate) fn create(
    params: Arc<NongleParams>,
    editor_state: Arc<IcedState>,
    progress: Arc<LoadProgress>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<GainEditor>(editor_state, (params, progress))
}

struct GainEditor {
    params: Arc<NongleParams>,
    context: Arc<dyn GuiContext>,

    gain_slider_state: nih_widgets::param_slider::State,
    load_button_state: button::State,
    progress: Arc<LoadProgress>,
    tick_rx: Arc<Mutex<mpsc::Receiver<()>>>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    /// Update a parameter's value.
    ParamUpdate(nih_widgets::ParamMessage),
    /// The user wants to pick a sample library folder.
    LoadLibrary,
    /// Periodic redraw so the loading progress stays fresh.
    Tick,
}

impl IcedEditor for GainEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = (Arc<NongleParams>, Arc<LoadProgress>);

    fn new(
        flags: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let (params, progress) = flags;

        // The GUI is not redrawn while loading, so a tick message must be sent
        // on a timer to keep the progress bar moving.
        let (tick_tx, tick_rx) = mpsc::channel();
        std::thread::spawn(move || loop {
            std::thread::sleep(Duration::from_millis(100));
            if tick_tx.send(()).is_err() {
                break;
            }
        });

        let editor = GainEditor {
            params,
            context,

            gain_slider_state: Default::default(),
            load_button_state: Default::default(),
            progress,
            tick_rx: Arc::new(Mutex::new(tick_rx)),
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
            Message::LoadLibrary => {
                // Blocks the GUI thread while the native dialog is open. The
                // audio thread picks the new path up in `reload_lib`.
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    *self.params.lib_path.write().expect("poisoned lib path") =
                        path.to_string_lossy().into_owned();
                }
            }
            Message::Tick => (),
        }

        Command::none()
    }

    fn subscription(
        &self,
        _window_subs: &mut WindowSubs<Self::Message>,
    ) -> Subscription<Self::Message> {
        let rx = self.tick_rx.clone();
        subscription::unfold("progress ticks", rx, |rx| {
            let result = rx.lock().expect("poisoned tick receiver").try_recv();
            match result {
                Ok(()) => futures::future::ready((Some(Message::Tick), rx)).boxed(),
                Err(_) => futures::future::pending().boxed(),
            }
        })
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let mut column = Column::new()
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
            .push(
                Button::new(&mut self.load_button_state, Text::new("Load Library"))
                    .on_press(Message::LoadLibrary),
            );

        if let Some(fraction) = self.progress.fraction() {
            column = column
                .push(Space::with_height(10.into()))
                .push(
                    Text::new("Loading…")
                        .height(20.into())
                        .width(Length::Fill)
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .vertical_alignment(alignment::Vertical::Center),
                )
                .push(
                    ProgressBar::new(0.0..=1.0, fraction).width(Length::Units(180)),
                );
        }

        column.push(Space::with_height(10.into())).into()
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
