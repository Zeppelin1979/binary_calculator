use iced::widget::tooltip::Position;
use iced::Alignment;
use iced_native::{
    column, renderer, row,
    widget::{checkbox, text, tooltip, Row, Tree},
    Element, Length, Shell, Widget,
};

/// The default text size.
const DEFAULT_TEXT_SIZE: u16 = 16;
/// The default padding between the tabs.
const DEFAULT_PADDING: u16 = 5;
/// The default spacing around the tabs.
const DEFAULT_SPACING: u16 = 0;

pub struct BinaryFieldWidget<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer,
    Renderer::Theme: text::StyleSheet
        + iced_native::widget::container::StyleSheet
        + iced_native::widget::checkbox::StyleSheet,
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
    content: Row<'a, BinaryFieldWidgetMessage, Renderer>,
    /// The on_change event of the [`BinaryFieldWidget`](BinaryFieldWidget).
    on_change: Box<dyn Fn(u32) -> Message>,
    messages: Vec<BinaryFieldWidgetMessage>,
}

impl<'a, Message, Renderer> BinaryFieldWidget<'a, Message, Renderer>
where
    Renderer: 'a + iced_native::Renderer + iced_native::text::Renderer,
    Renderer::Theme: text::StyleSheet
        + iced_native::widget::container::StyleSheet
        + iced_native::widget::checkbox::StyleSheet,
{
    pub fn new<F>(value: u32, on_change: F) -> Self
    where
        F: 'static + Fn(u32) -> Message + Copy,
    {
        let bit0_checkbox = checkbox(
            "",
            get_bit(value, 0),
            BinaryFieldWidgetMessage::Bit0CheckboxToggled,
        )
        .spacing(0);
        let bit1_checkbox = checkbox(
            "",
            get_bit(value, 1),
            BinaryFieldWidgetMessage::Bit1CheckboxToggled,
        )
        .spacing(0);
        let bit2_checkbox = checkbox(
            "",
            get_bit(value, 2),
            BinaryFieldWidgetMessage::Bit2CheckboxToggled,
        )
        .spacing(0);
        let bit3_checkbox = checkbox(
            "",
            get_bit(value, 3),
            BinaryFieldWidgetMessage::Bit3CheckboxToggled,
        )
        .spacing(0);
        let bit4_checkbox = checkbox(
            "",
            get_bit(value, 4),
            BinaryFieldWidgetMessage::Bit4CheckboxToggled,
        )
        .spacing(0);
        let bit5_checkbox = checkbox(
            "",
            get_bit(value, 5),
            BinaryFieldWidgetMessage::Bit5CheckboxToggled,
        )
        .spacing(0);
        let bit6_checkbox = checkbox(
            "",
            get_bit(value, 6),
            BinaryFieldWidgetMessage::Bit6CheckboxToggled,
        )
        .spacing(0);
        let bit7_checkbox = checkbox(
            "",
            get_bit(value, 7),
            BinaryFieldWidgetMessage::Bit7CheckboxToggled,
        )
        .spacing(0);
        let bit8_checkbox = checkbox(
            "",
            get_bit(value, 8),
            BinaryFieldWidgetMessage::Bit8CheckboxToggled,
        )
        .spacing(0);
        let bit9_checkbox = checkbox(
            "",
            get_bit(value, 9),
            BinaryFieldWidgetMessage::Bit9CheckboxToggled,
        )
        .spacing(0);
        let bit10_checkbox = checkbox(
            "",
            get_bit(value, 10),
            BinaryFieldWidgetMessage::Bit10CheckboxToggled,
        )
        .spacing(0);
        let bit11_checkbox = checkbox(
            "",
            get_bit(value, 11),
            BinaryFieldWidgetMessage::Bit11CheckboxToggled,
        )
        .spacing(0);
        let bit12_checkbox = checkbox(
            "",
            get_bit(value, 12),
            BinaryFieldWidgetMessage::Bit12CheckboxToggled,
        )
        .spacing(0);
        let bit13_checkbox = checkbox(
            "",
            get_bit(value, 13),
            BinaryFieldWidgetMessage::Bit13CheckboxToggled,
        )
        .spacing(0);
        let bit14_checkbox = checkbox(
            "",
            get_bit(value, 14),
            BinaryFieldWidgetMessage::Bit14CheckboxToggled,
        )
        .spacing(0);
        let bit15_checkbox = checkbox(
            "",
            get_bit(value, 15),
            BinaryFieldWidgetMessage::Bit15CheckboxToggled,
        )
        .spacing(0);
        let bit16_checkbox = checkbox(
            "",
            get_bit(value, 16),
            BinaryFieldWidgetMessage::Bit16CheckboxToggled,
        )
        .spacing(0);
        let bit17_checkbox = checkbox(
            "",
            get_bit(value, 17),
            BinaryFieldWidgetMessage::Bit17CheckboxToggled,
        )
        .spacing(0);
        let bit18_checkbox = checkbox(
            "",
            get_bit(value, 18),
            BinaryFieldWidgetMessage::Bit18CheckboxToggled,
        )
        .spacing(0);
        let bit19_checkbox = checkbox(
            "",
            get_bit(value, 19),
            BinaryFieldWidgetMessage::Bit19CheckboxToggled,
        )
        .spacing(0);
        let bit20_checkbox = checkbox(
            "",
            get_bit(value, 20),
            BinaryFieldWidgetMessage::Bit20CheckboxToggled,
        )
        .spacing(0);
        let bit21_checkbox = checkbox(
            "",
            get_bit(value, 21),
            BinaryFieldWidgetMessage::Bit21CheckboxToggled,
        )
        .spacing(0);
        let bit22_checkbox = checkbox(
            "",
            get_bit(value, 22),
            BinaryFieldWidgetMessage::Bit22CheckboxToggled,
        )
        .spacing(0);
        let bit23_checkbox = checkbox(
            "",
            get_bit(value, 23),
            BinaryFieldWidgetMessage::Bit23CheckboxToggled,
        )
        .spacing(0);
        let bit24_checkbox = checkbox(
            "",
            get_bit(value, 24),
            BinaryFieldWidgetMessage::Bit24CheckboxToggled,
        )
        .spacing(0);
        let bit25_checkbox = checkbox(
            "",
            get_bit(value, 25),
            BinaryFieldWidgetMessage::Bit25CheckboxToggled,
        )
        .spacing(0);
        let bit26_checkbox = checkbox(
            "",
            get_bit(value, 26),
            BinaryFieldWidgetMessage::Bit26CheckboxToggled,
        )
        .spacing(0);
        let bit27_checkbox = checkbox(
            "",
            get_bit(value, 27),
            BinaryFieldWidgetMessage::Bit27CheckboxToggled,
        )
        .spacing(0);
        let bit28_checkbox = checkbox(
            "",
            get_bit(value, 28),
            BinaryFieldWidgetMessage::Bit28CheckboxToggled,
        )
        .spacing(0);
        let bit29_checkbox = checkbox(
            "",
            get_bit(value, 29),
            BinaryFieldWidgetMessage::Bit29CheckboxToggled,
        )
        .spacing(0);
        let bit30_checkbox = checkbox(
            "",
            get_bit(value, 30),
            BinaryFieldWidgetMessage::Bit30CheckboxToggled,
        )
        .spacing(0);
        let bit31_checkbox = checkbox(
            "",
            get_bit(value, 31),
            BinaryFieldWidgetMessage::Bit31CheckboxToggled,
        )
        .spacing(0);
        let content = row![
            row![
                column![
                    text("31"),
                    tooltip(bit31_checkbox, "31 [2G]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
                tooltip(bit30_checkbox, "30 [1G]", Position::FollowCursor),
                tooltip(bit29_checkbox, "29 [512M]", Position::FollowCursor),
                column![
                    text("28"),
                    tooltip(bit28_checkbox, "28 [256M]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center)
            ]
            .spacing(2)
            .align_items(Alignment::End),
            row![
                column![
                    text("27"),
                    tooltip(bit27_checkbox, "27 [128M]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
                tooltip(bit26_checkbox, "26 [64M]", Position::FollowCursor),
                tooltip(bit25_checkbox, "25 [32M]", Position::FollowCursor),
                column![
                    text("24"),
                    tooltip(bit24_checkbox, "24 [16M]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center)
            ]
            .spacing(1)
            .align_items(Alignment::End),
            row![
                column![
                    text("23"),
                    tooltip(bit23_checkbox, "23 [8M]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
                tooltip(bit22_checkbox, "22 [4M]", Position::FollowCursor),
                tooltip(bit21_checkbox, "21 [2M]", Position::FollowCursor),
                column![
                    text("20"),
                    tooltip(bit20_checkbox, "20 [1M]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(1)
            .align_items(Alignment::End),
            row![
                column![
                    text("19"),
                    tooltip(bit19_checkbox, "19 [512K]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
                tooltip(bit18_checkbox, "18 [256K]", Position::FollowCursor),
                tooltip(bit17_checkbox, "17 [128K]", Position::FollowCursor),
                column![
                    text("16"),
                    tooltip(bit16_checkbox, "16 [64K]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(1)
            .align_items(Alignment::End),
            row![
                column![
                    text("15"),
                    tooltip(bit15_checkbox, "15 [32K]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
                tooltip(bit14_checkbox, "14 [16K]", Position::FollowCursor),
                tooltip(bit13_checkbox, "13 [8K]", Position::FollowCursor),
                column![
                    text("12"),
                    tooltip(bit12_checkbox, "12 [4K]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(1)
            .align_items(Alignment::End),
            row![
                column![
                    text("11"),
                    tooltip(bit11_checkbox, "11 [2K]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
                tooltip(bit10_checkbox, "10 [1K]", Position::FollowCursor),
                tooltip(bit9_checkbox, "9 [512]", Position::FollowCursor),
                column![
                    text("8"),
                    tooltip(bit8_checkbox, "8 [256]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(1)
            .align_items(Alignment::End),
            row![
                column![
                    text("7"),
                    tooltip(bit7_checkbox, "7 [128]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
                tooltip(bit6_checkbox, "6 [64]", Position::FollowCursor),
                tooltip(bit5_checkbox, "5 [32]", Position::FollowCursor),
                column![
                    text("4"),
                    tooltip(bit4_checkbox, "4 [16]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(1)
            .align_items(Alignment::End),
            row![
                column![
                    text("3"),
                    tooltip(bit3_checkbox, "3 [8]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
                tooltip(bit2_checkbox, "2 [4]", Position::FollowCursor),
                tooltip(bit1_checkbox, "1 [2]", Position::FollowCursor),
                column![
                    text("0"),
                    tooltip(bit0_checkbox, "0 [1]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(1)
            .align_items(Alignment::End)
        ]
        .spacing(4)
        .height(Length::Units(40))
        .width(Length::Fill);
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
        }
    }

    pub fn binary_field_widget<F>(
        value: u32,
        on_change: F,
    ) -> BinaryFieldWidget<'a, Message, Renderer>
    where
        F: 'static + Fn(u32) -> Message + Copy,
    {
        BinaryFieldWidget::new(value, on_change)
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

impl<'a, Message, Renderer> Widget<Message, Renderer> for BinaryFieldWidget<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer + iced_native::text::Renderer,
    Renderer::Theme: text::StyleSheet
        + iced_native::widget::container::StyleSheet
        + iced_native::widget::checkbox::StyleSheet,
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
        for message in self.messages.iter() {
            match message {
                BinaryFieldWidgetMessage::Bit0CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 0);
                }
                BinaryFieldWidgetMessage::Bit1CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 1);
                }
                BinaryFieldWidgetMessage::Bit2CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 2);
                }
                BinaryFieldWidgetMessage::Bit3CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 3);
                }
                BinaryFieldWidgetMessage::Bit4CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 4);
                }
                BinaryFieldWidgetMessage::Bit5CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 5);
                }
                BinaryFieldWidgetMessage::Bit6CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 6);
                }
                BinaryFieldWidgetMessage::Bit7CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 7);
                }
                BinaryFieldWidgetMessage::Bit8CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 8);
                }
                BinaryFieldWidgetMessage::Bit9CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 9);
                }
                BinaryFieldWidgetMessage::Bit10CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 10);
                }
                BinaryFieldWidgetMessage::Bit11CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 11);
                }
                BinaryFieldWidgetMessage::Bit12CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 12);
                }
                BinaryFieldWidgetMessage::Bit13CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 13);
                }
                BinaryFieldWidgetMessage::Bit14CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 14);
                }
                BinaryFieldWidgetMessage::Bit15CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 15);
                }
                BinaryFieldWidgetMessage::Bit16CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 16);
                }
                BinaryFieldWidgetMessage::Bit17CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 17);
                }
                BinaryFieldWidgetMessage::Bit18CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 18);
                }
                BinaryFieldWidgetMessage::Bit19CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 19);
                }
                BinaryFieldWidgetMessage::Bit20CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 20);
                }
                BinaryFieldWidgetMessage::Bit21CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 21);
                }
                BinaryFieldWidgetMessage::Bit22CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 22);
                }
                BinaryFieldWidgetMessage::Bit23CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 23);
                }
                BinaryFieldWidgetMessage::Bit24CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 24);
                }
                BinaryFieldWidgetMessage::Bit25CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 25);
                }
                BinaryFieldWidgetMessage::Bit26CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 26);
                }
                BinaryFieldWidgetMessage::Bit27CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 27);
                }
                BinaryFieldWidgetMessage::Bit28CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 28);
                }
                BinaryFieldWidgetMessage::Bit29CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 29);
                }
                BinaryFieldWidgetMessage::Bit30CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 30);
                }
                BinaryFieldWidgetMessage::Bit31CheckboxToggled(value) => {
                    self.value = put_bit(self.value, value, 31);
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
pub enum BinaryFieldWidgetMessage {
    Bit0CheckboxToggled(bool),
    Bit1CheckboxToggled(bool),
    Bit2CheckboxToggled(bool),
    Bit3CheckboxToggled(bool),
    Bit4CheckboxToggled(bool),
    Bit5CheckboxToggled(bool),
    Bit6CheckboxToggled(bool),
    Bit7CheckboxToggled(bool),
    Bit8CheckboxToggled(bool),
    Bit9CheckboxToggled(bool),
    Bit10CheckboxToggled(bool),
    Bit11CheckboxToggled(bool),
    Bit12CheckboxToggled(bool),
    Bit13CheckboxToggled(bool),
    Bit14CheckboxToggled(bool),
    Bit15CheckboxToggled(bool),
    Bit16CheckboxToggled(bool),
    Bit17CheckboxToggled(bool),
    Bit18CheckboxToggled(bool),
    Bit19CheckboxToggled(bool),
    Bit20CheckboxToggled(bool),
    Bit21CheckboxToggled(bool),
    Bit22CheckboxToggled(bool),
    Bit23CheckboxToggled(bool),
    Bit24CheckboxToggled(bool),
    Bit25CheckboxToggled(bool),
    Bit26CheckboxToggled(bool),
    Bit27CheckboxToggled(bool),
    Bit28CheckboxToggled(bool),
    Bit29CheckboxToggled(bool),
    Bit30CheckboxToggled(bool),
    Bit31CheckboxToggled(bool),
}

impl<'a, Message, Renderer> From<BinaryFieldWidget<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + iced_native::Renderer + iced_native::text::Renderer,
    Renderer::Theme: text::StyleSheet
        + iced_native::widget::container::StyleSheet
        + iced_native::widget::checkbox::StyleSheet,
    Message: Clone + 'a,
{
    fn from(value: BinaryFieldWidget<'a, Message, Renderer>) -> Self {
        Self::new(value)
    }
}

fn get_bit(value: u32, offset: u8) -> bool {
    let mask = 1_u32 << offset;
    value & mask > 0
}

fn put_bit(value: u32, bit: &bool, offset: u8) -> u32 {
    let mut value = value;
    if *bit {
        let mask = 1_u32 << offset;
        value |= mask;
    } else {
        let mask = !(1_u32 << offset);
        value &= mask;
    }
    return value;
}
