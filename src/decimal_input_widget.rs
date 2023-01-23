use iced_native::{
    column, renderer, row,
    widget::{button, text, Button, Column, Tree},
    Element, Length, Shell, Widget,
};

/// The default text size.
const DEFAULT_TEXT_SIZE: u16 = 16;
/// The default padding between the tabs.
const DEFAULT_PADDING: u16 = 5;
/// The default spacing around the tabs.
const DEFAULT_SPACING: u16 = 0;

pub struct DecimalInputWidget<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer,
    Renderer::Theme: text::StyleSheet
        + iced_native::widget::container::StyleSheet
        + iced_native::widget::button::StyleSheet,
{
    value: u32,
    /// The width of the [`BinaryFieldWidget`](BinaryFieldWidget)
    width: Length,
    /// The width of the [`BinaryFieldWidget`](BinaryFieldWidget)
    height: Length,
    /// The maximum height of the [`BinaryFieldWidget`](BinaryFieldWidget)
    max_height: u32,
    /// The text size.
    text_size: u16,
    /// The padding of the tabs of the [`BinaryFieldWidget`](BinaryFieldWidget)
    padding: u16,
    /// The spacing of the tabs of the [`BinaryFieldWidget`](BinaryFieldWidget)
    spacing: u16,
    /// The underlying element of the [`BinaryFieldWidget`](BinaryFieldWidget)
    content: Column<'a, DecimalInputWidgetMessage, Renderer>,
    /// The on_change event of the [`BinaryFieldWidget`](BinaryFieldWidget).
    on_change: Box<dyn Fn(u32) -> Message>,
    messages: Vec<DecimalInputWidgetMessage>,
    hex: bool,
}

impl<'a, Message, Renderer> DecimalInputWidget<'a, Message, Renderer>
where
    Renderer: 'a + iced_native::Renderer + iced_native::text::Renderer,
    Renderer::Theme: text::StyleSheet
        + iced_native::widget::container::StyleSheet
        + iced_native::widget::button::StyleSheet,
{
    pub fn new<F>(value: u32, hex: bool, on_change: F) -> Self
    where
        F: 'static + Fn(u32) -> Message + Copy,
    {
        let content;
        let dec_0_button: Button<'a, DecimalInputWidgetMessage, Renderer> = button("0")
            .on_press(DecimalInputWidgetMessage::Button0)
            .width(Length::Units(40));
        let dec_1_button = button("1")
            .on_press(DecimalInputWidgetMessage::Button1)
            .width(Length::Units(40));
        let dec_2_button = button("2")
            .on_press(DecimalInputWidgetMessage::Button2)
            .width(Length::Units(40));
        let dec_3_button = button("3")
            .on_press(DecimalInputWidgetMessage::Button3)
            .width(Length::Units(40));
        let dec_4_button = button("4")
            .on_press(DecimalInputWidgetMessage::Button4)
            .width(Length::Units(40));
        let dec_5_button = button("5")
            .on_press(DecimalInputWidgetMessage::Button5)
            .width(Length::Units(40));
        let dec_6_button = button("6")
            .on_press(DecimalInputWidgetMessage::Button6)
            .width(Length::Units(40));
        let dec_7_button = button("7")
            .on_press(DecimalInputWidgetMessage::Button7)
            .width(Length::Units(40));
        let dec_8_button = button("8")
            .on_press(DecimalInputWidgetMessage::Button8)
            .width(Length::Units(40));
        let dec_9_button = button("9")
            .on_press(DecimalInputWidgetMessage::Button9)
            .width(Length::Units(40));
        let dec_sign_button = button("±")
            .on_press(DecimalInputWidgetMessage::ButtonSignum)
            .width(Length::Units(40));
        let dec_res_button = button("<-")
            .on_press(DecimalInputWidgetMessage::ButtonBackspace)
            .width(Length::Units(40));
        if hex {
            let hex_a_button = button("A")
                .on_press(DecimalInputWidgetMessage::ButtonA)
                .width(Length::Units(40));
            let hex_b_button = button("B")
                .on_press(DecimalInputWidgetMessage::ButtonB)
                .width(Length::Units(40));
            let hex_c_button = button("C")
                .on_press(DecimalInputWidgetMessage::ButtonC)
                .width(Length::Units(40));
            let hex_d_button = button("D")
                .on_press(DecimalInputWidgetMessage::ButtonD)
                .width(Length::Units(40));
            let hex_e_button = button("E")
                .on_press(DecimalInputWidgetMessage::ButtonE)
                .width(Length::Units(40));
            let hex_f_button = button("F")
                .on_press(DecimalInputWidgetMessage::ButtonF)
                .width(Length::Units(40));
            content = column![
                row![dec_7_button, dec_8_button, dec_9_button, hex_f_button].spacing(10),
                row![dec_4_button, dec_5_button, dec_6_button, hex_e_button].spacing(10),
                row![dec_1_button, dec_2_button, dec_3_button, hex_d_button].spacing(10),
                row![dec_0_button, hex_a_button, hex_b_button, hex_c_button].spacing(10),
            ]
            .spacing(10);
        } else {
            content = column![
                row![dec_7_button, dec_8_button, dec_9_button].spacing(10),
                row![dec_4_button, dec_5_button, dec_6_button].spacing(10),
                row![dec_1_button, dec_2_button, dec_3_button].spacing(10),
                row![dec_0_button, dec_sign_button, dec_res_button].spacing(10),
            ]
            .spacing(10);
        }
        Self {
            value,
            width: Length::Fill,
            height: Length::Units(40),
            max_height: u32::MAX,
            text_size: DEFAULT_TEXT_SIZE,
            padding: DEFAULT_PADDING,
            spacing: DEFAULT_SPACING,
            content: content,
            on_change: Box::new(on_change),
            messages: Vec::new(),
            hex,
        }
    }

    pub fn decimal_input_widget<F>(
        value: u32,
        hex: bool,
        on_change: F,
    ) -> DecimalInputWidget<'a, Message, Renderer>
    where
        F: 'static + Fn(u32) -> Message + Copy,
    {
        DecimalInputWidget::new(value, hex, on_change)
    }

    /// Sets the width of the [`BinaryFieldWidget`](BinaryFieldWidget).
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Gets the width of the [`BinaryFieldWidget`](BinaryFieldWidget).
    #[must_use]
    pub fn get_width(&self) -> Length {
        self.width
    }

    /// Sets the height of the [`BinaryFieldWidget`](BinaryFieldWidget).
    #[must_use]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Gets the width of the [`BinaryFieldWidget`](BinaryFieldWidget).
    #[must_use]
    pub fn get_height(&self) -> Length {
        self.height
    }

    /// Sets the maximum height of the [`BinaryFieldWidget`](BinaryFieldWidget).
    #[must_use]
    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
        self
    }
    /// Sets the text size of the [`TabLabel`](crate::tab_bar::TabLabel)s of the
    /// [`BinaryFieldWidget`](BinaryFieldWidget).
    #[must_use]
    pub fn text_size(mut self, text_size: u16) -> Self {
        self.text_size = text_size;
        self
    }
    /// Sets the padding of the tabs of the [`BinaryFieldWidget`](BinaryFieldWidget).
    #[must_use]
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the spacing between the tabs of the [`BinaryFieldWidget`](BinaryFieldWidget).
    #[must_use]
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for DecimalInputWidget<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer,
    Renderer::Theme: text::StyleSheet
        + iced_native::widget::container::StyleSheet
        + iced_native::widget::button::StyleSheet,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        self.content.layout(renderer, limits)
    }

    fn draw(
        &self,
        state: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_native::Renderer>::Theme,
        style: &renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) {
        self.content.draw(
            state.children.get(0).unwrap(),
            renderer,
            theme,
            style,
            layout,
            cursor_position,
            viewport,
        )
    }

    fn tag(&self) -> iced_native::widget::tree::Tag {
        self.content.tag()
    }

    fn state(&self) -> iced_native::widget::tree::State {
        self.content.state()
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree {
            tag: self.content.tag(),
            state: self.content.state(),
            children: self.content.children(),
        }]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children_custom(
            &[&self.content],
            |state, content| content.diff(state),
            |&content| Tree {
                tag: content.tag(),
                state: content.state(),
                children: content.children(),
            },
        )
    }

    fn on_event(
        &mut self,
        state: &mut iced_native::widget::Tree,
        event: iced::Event,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        renderer: &Renderer,
        clipboard: &mut dyn iced_native::Clipboard,
        shell: &mut iced_native::Shell<'_, Message>,
    ) -> iced::event::Status {
        let mut int_shell = Shell::new(&mut self.messages);
        let state = self.content.on_event(
            &mut state.children[0],
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            &mut int_shell,
        );
        let base = if self.hex { 16 } else { 10 };
        for message in self.messages.iter() {
            match message {
                DecimalInputWidgetMessage::Button0 => self.value = self.value.wrapping_mul(10),
                DecimalInputWidgetMessage::Button1 => {
                    self.value = self.value.wrapping_mul(base).wrapping_add(1)
                }
                DecimalInputWidgetMessage::Button2 => {
                    self.value = self.value.wrapping_mul(base).wrapping_add(2)
                }
                DecimalInputWidgetMessage::Button3 => {
                    self.value = self.value.wrapping_mul(base).wrapping_add(3)
                }
                DecimalInputWidgetMessage::Button4 => {
                    self.value = self.value.wrapping_mul(base).wrapping_add(4)
                }
                DecimalInputWidgetMessage::Button5 => {
                    self.value = self.value.wrapping_mul(base).wrapping_add(5)
                }
                DecimalInputWidgetMessage::Button6 => {
                    self.value = self.value.wrapping_mul(base).wrapping_add(6)
                }
                DecimalInputWidgetMessage::Button7 => {
                    self.value = self.value.wrapping_mul(base).wrapping_add(7)
                }
                DecimalInputWidgetMessage::Button8 => {
                    self.value = self.value.wrapping_mul(base).wrapping_add(8)
                }
                DecimalInputWidgetMessage::Button9 => {
                    self.value = self.value.wrapping_mul(base).wrapping_add(9)
                }
                DecimalInputWidgetMessage::ButtonSignum => todo!(),
                DecimalInputWidgetMessage::ButtonBackspace => self.value /= 10,
                DecimalInputWidgetMessage::ButtonA => {
                    self.value = self.value.wrapping_mul(base).wrapping_add(10)
                }
                DecimalInputWidgetMessage::ButtonB => {
                    self.value = self.value.wrapping_mul(base).wrapping_add(11)
                }
                DecimalInputWidgetMessage::ButtonC => {
                    self.value = self.value.wrapping_mul(base).wrapping_add(12)
                }
                DecimalInputWidgetMessage::ButtonD => {
                    self.value = self.value.wrapping_mul(base).wrapping_add(13)
                }
                DecimalInputWidgetMessage::ButtonE => {
                    self.value = self.value.wrapping_mul(base).wrapping_add(14)
                }
                DecimalInputWidgetMessage::ButtonF => {
                    self.value = self.value.wrapping_mul(base).wrapping_add(15)
                }
            }
        }
        if self.messages.len() > 0 {
            self.messages.clear();
            shell.publish((self.on_change)(self.value));
        }
        return state;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecimalInputWidgetMessage {
    Button0,
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
    Button9,
    ButtonSignum,
    ButtonBackspace,
    ButtonA,
    ButtonB,
    ButtonC,
    ButtonD,
    ButtonE,
    ButtonF,
}

impl<'a, Message, Renderer> From<DecimalInputWidget<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + iced_native::Renderer + iced_native::text::Renderer,
    Renderer::Theme: text::StyleSheet
        + iced_native::widget::container::StyleSheet
        + iced_native::widget::button::StyleSheet,
    Message: Clone + 'a,
{
    fn from(value: DecimalInputWidget<'a, Message, Renderer>) -> Self {
        Self::new(value)
    }
}
