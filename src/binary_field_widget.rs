use iced::widget::tooltip::Position;
use iced::Alignment;
use iced_native::{
    column, renderer, row,
    widget::{checkbox, text, tooltip, Row, Tree},
    Element, Length, Shell, Widget,
};

/// The default text size.
const DEFAULT_TEXT_SIZE: u16 = 16;
/// The default padding around the checkboxes.
const DEFAULT_PADDING: u16 = 5;
/// The default spacing between the checkboxes.
const DEFAULT_SPACING: u16 = 2;
/// The default spacing between the groups of checkboxes.
const DEFAULT_GROUP_SPACING: u16 = 6;

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
    /// The padding around the checkboxes of the [`BinaryFieldWidget`](BinaryFieldWidget)
    padding: u16,
    /// The spacing between the checkboxes of the [`BinaryFieldWidget`](BinaryFieldWidget)
    spacing: u16,
    /// The spacing between the group of checkboxes of the [`BinaryFieldWidget`](BinaryFieldWidget)
    group_spacing: u16,
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
        Self {
            value,
            width: Length::Fill,
            height: Length::Units(40),
            max_height: u32::MAX,
            text_size: DEFAULT_TEXT_SIZE,
            padding: DEFAULT_PADDING,
            spacing: DEFAULT_SPACING,
            group_spacing: DEFAULT_GROUP_SPACING,
            content: Self::create_content(
                value,
                DEFAULT_SPACING,
                DEFAULT_GROUP_SPACING,
                DEFAULT_TEXT_SIZE,
                Length::Units(40),
            ),
            on_change: Box::new(on_change),
            messages: Vec::new(),
        }
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
        self.content = Self::create_content(
            self.value,
            self.spacing,
            self.group_spacing,
            self.text_size,
            self.height,
        );
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
    /// Sets the text size of the labels over the checkboxes of the
    /// [`BinaryFieldWidget`](BinaryFieldWidget).
    #[must_use]
    pub fn text_size(mut self, text_size: u16) -> Self {
        self.text_size = text_size;
        self.content = Self::create_content(
            self.value,
            self.spacing,
            self.group_spacing,
            self.text_size,
            self.height,
        );
        self
    }
    /// Sets the padding of the [`BinaryFieldWidget`](BinaryFieldWidget).
    #[must_use]
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the spacing between the checkboxes of the [`BinaryFieldWidget`](BinaryFieldWidget).
    #[must_use]
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self.content = Self::create_content(
            self.value,
            self.spacing,
            self.group_spacing,
            self.text_size,
            self.height,
        );
        self
    }

    /// Sets the spacing between the groups of checkboxes of the [`BinaryFieldWidget`](BinaryFieldWidget).
    #[must_use]
    pub fn group_spacing(mut self, spacing: u16) -> Self {
        self.group_spacing = spacing;
        self.content = Self::create_content(
            self.value,
            self.spacing,
            self.group_spacing,
            self.text_size,
            self.height,
        );
        self
    }

    fn create_content(
        value: u32,
        spacing: u16,
        group_spacing: u16,
        text_size: u16,
        height: Length,
    ) -> Row<'a, BinaryFieldWidgetMessage, Renderer> {
        let bit0_checkbox =
            checkbox("", get_bit(value, 0), BinaryFieldWidgetMessage::Bit0).spacing(0);
        let bit1_checkbox =
            checkbox("", get_bit(value, 1), BinaryFieldWidgetMessage::Bit1).spacing(0);
        let bit2_checkbox =
            checkbox("", get_bit(value, 2), BinaryFieldWidgetMessage::Bit2).spacing(0);
        let bit3_checkbox =
            checkbox("", get_bit(value, 3), BinaryFieldWidgetMessage::Bit3).spacing(0);
        let bit4_checkbox =
            checkbox("", get_bit(value, 4), BinaryFieldWidgetMessage::Bit4).spacing(0);
        let bit5_checkbox =
            checkbox("", get_bit(value, 5), BinaryFieldWidgetMessage::Bit5).spacing(0);
        let bit6_checkbox =
            checkbox("", get_bit(value, 6), BinaryFieldWidgetMessage::Bit6).spacing(0);
        let bit7_checkbox =
            checkbox("", get_bit(value, 7), BinaryFieldWidgetMessage::Bit7).spacing(0);
        let bit8_checkbox =
            checkbox("", get_bit(value, 8), BinaryFieldWidgetMessage::Bit8).spacing(0);
        let bit9_checkbox =
            checkbox("", get_bit(value, 9), BinaryFieldWidgetMessage::Bit9).spacing(0);
        let bit10_checkbox =
            checkbox("", get_bit(value, 10), BinaryFieldWidgetMessage::Bit10).spacing(0);
        let bit11_checkbox =
            checkbox("", get_bit(value, 11), BinaryFieldWidgetMessage::Bit11).spacing(0);
        let bit12_checkbox =
            checkbox("", get_bit(value, 12), BinaryFieldWidgetMessage::Bit12).spacing(0);
        let bit13_checkbox =
            checkbox("", get_bit(value, 13), BinaryFieldWidgetMessage::Bit13).spacing(0);
        let bit14_checkbox =
            checkbox("", get_bit(value, 14), BinaryFieldWidgetMessage::Bit14).spacing(0);
        let bit15_checkbox =
            checkbox("", get_bit(value, 15), BinaryFieldWidgetMessage::Bit15).spacing(0);
        let bit16_checkbox =
            checkbox("", get_bit(value, 16), BinaryFieldWidgetMessage::Bit16).spacing(0);
        let bit17_checkbox =
            checkbox("", get_bit(value, 17), BinaryFieldWidgetMessage::Bit17).spacing(0);
        let bit18_checkbox =
            checkbox("", get_bit(value, 18), BinaryFieldWidgetMessage::Bit18).spacing(0);
        let bit19_checkbox =
            checkbox("", get_bit(value, 19), BinaryFieldWidgetMessage::Bit19).spacing(0);
        let bit20_checkbox =
            checkbox("", get_bit(value, 20), BinaryFieldWidgetMessage::Bit20).spacing(0);
        let bit21_checkbox =
            checkbox("", get_bit(value, 21), BinaryFieldWidgetMessage::Bit21).spacing(0);
        let bit22_checkbox =
            checkbox("", get_bit(value, 22), BinaryFieldWidgetMessage::Bit22).spacing(0);
        let bit23_checkbox =
            checkbox("", get_bit(value, 23), BinaryFieldWidgetMessage::Bit23).spacing(0);
        let bit24_checkbox =
            checkbox("", get_bit(value, 24), BinaryFieldWidgetMessage::Bit24).spacing(0);
        let bit25_checkbox =
            checkbox("", get_bit(value, 25), BinaryFieldWidgetMessage::Bit25).spacing(0);
        let bit26_checkbox =
            checkbox("", get_bit(value, 26), BinaryFieldWidgetMessage::Bit26).spacing(0);
        let bit27_checkbox =
            checkbox("", get_bit(value, 27), BinaryFieldWidgetMessage::Bit27).spacing(0);
        let bit28_checkbox =
            checkbox("", get_bit(value, 28), BinaryFieldWidgetMessage::Bit28).spacing(0);
        let bit29_checkbox =
            checkbox("", get_bit(value, 29), BinaryFieldWidgetMessage::Bit29).spacing(0);
        let bit30_checkbox =
            checkbox("", get_bit(value, 30), BinaryFieldWidgetMessage::Bit30).spacing(0);
        let bit31_checkbox =
            checkbox("", get_bit(value, 31), BinaryFieldWidgetMessage::Bit31).spacing(0);
        row![
            row![
                column![
                    text("31").size(text_size),
                    tooltip(bit31_checkbox, "31 [2G]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
                tooltip(bit30_checkbox, "30 [1G]", Position::FollowCursor),
                tooltip(bit29_checkbox, "29 [512M]", Position::FollowCursor),
                column![
                    text("28").size(text_size),
                    tooltip(bit28_checkbox, "28 [256M]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center)
            ]
            .spacing(spacing)
            .align_items(Alignment::End),
            row![
                column![
                    text("27").size(text_size),
                    tooltip(bit27_checkbox, "27 [128M]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
                tooltip(bit26_checkbox, "26 [64M]", Position::FollowCursor),
                tooltip(bit25_checkbox, "25 [32M]", Position::FollowCursor),
                column![
                    text("24").size(text_size),
                    tooltip(bit24_checkbox, "24 [16M]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center)
            ]
            .spacing(spacing)
            .align_items(Alignment::End),
            row![
                column![
                    text("23").size(text_size),
                    tooltip(bit23_checkbox, "23 [8M]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
                tooltip(bit22_checkbox, "22 [4M]", Position::FollowCursor),
                tooltip(bit21_checkbox, "21 [2M]", Position::FollowCursor),
                column![
                    text("20").size(text_size),
                    tooltip(bit20_checkbox, "20 [1M]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(spacing)
            .align_items(Alignment::End),
            row![
                column![
                    text("19").size(text_size),
                    tooltip(bit19_checkbox, "19 [512K]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
                tooltip(bit18_checkbox, "18 [256K]", Position::FollowCursor),
                tooltip(bit17_checkbox, "17 [128K]", Position::FollowCursor),
                column![
                    text("16").size(text_size),
                    tooltip(bit16_checkbox, "16 [64K]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(spacing)
            .align_items(Alignment::End),
            row![
                column![
                    text("15").size(text_size),
                    tooltip(bit15_checkbox, "15 [32K]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
                tooltip(bit14_checkbox, "14 [16K]", Position::FollowCursor),
                tooltip(bit13_checkbox, "13 [8K]", Position::FollowCursor),
                column![
                    text("12").size(text_size),
                    tooltip(bit12_checkbox, "12 [4K]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(spacing)
            .align_items(Alignment::End),
            row![
                column![
                    text("11").size(text_size),
                    tooltip(bit11_checkbox, "11 [2K]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
                tooltip(bit10_checkbox, "10 [1K]", Position::FollowCursor),
                tooltip(bit9_checkbox, "9 [512]", Position::FollowCursor),
                column![
                    text("8").size(text_size),
                    tooltip(bit8_checkbox, "8 [256]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(spacing)
            .align_items(Alignment::End),
            row![
                column![
                    text("7").size(text_size),
                    tooltip(bit7_checkbox, "7 [128]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
                tooltip(bit6_checkbox, "6 [64]", Position::FollowCursor),
                tooltip(bit5_checkbox, "5 [32]", Position::FollowCursor),
                column![
                    text("4").size(text_size),
                    tooltip(bit4_checkbox, "4 [16]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(spacing)
            .align_items(Alignment::End),
            row![
                column![
                    text("3").size(text_size),
                    tooltip(bit3_checkbox, "3 [8]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
                tooltip(bit2_checkbox, "2 [4]", Position::FollowCursor),
                tooltip(bit1_checkbox, "1 [2]", Position::FollowCursor),
                column![
                    text("0").size(text_size),
                    tooltip(bit0_checkbox, "0 [1]", Position::FollowCursor)
                ]
                .align_items(Alignment::Center),
            ]
            .spacing(spacing)
            .align_items(Alignment::End)
        ]
        .spacing(group_spacing)
        .height(height)
        .width(Length::Fill)
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
                BinaryFieldWidgetMessage::Bit0(value) => {
                    self.value = put_bit(self.value, value, 0);
                }
                BinaryFieldWidgetMessage::Bit1(value) => {
                    self.value = put_bit(self.value, value, 1);
                }
                BinaryFieldWidgetMessage::Bit2(value) => {
                    self.value = put_bit(self.value, value, 2);
                }
                BinaryFieldWidgetMessage::Bit3(value) => {
                    self.value = put_bit(self.value, value, 3);
                }
                BinaryFieldWidgetMessage::Bit4(value) => {
                    self.value = put_bit(self.value, value, 4);
                }
                BinaryFieldWidgetMessage::Bit5(value) => {
                    self.value = put_bit(self.value, value, 5);
                }
                BinaryFieldWidgetMessage::Bit6(value) => {
                    self.value = put_bit(self.value, value, 6);
                }
                BinaryFieldWidgetMessage::Bit7(value) => {
                    self.value = put_bit(self.value, value, 7);
                }
                BinaryFieldWidgetMessage::Bit8(value) => {
                    self.value = put_bit(self.value, value, 8);
                }
                BinaryFieldWidgetMessage::Bit9(value) => {
                    self.value = put_bit(self.value, value, 9);
                }
                BinaryFieldWidgetMessage::Bit10(value) => {
                    self.value = put_bit(self.value, value, 10);
                }
                BinaryFieldWidgetMessage::Bit11(value) => {
                    self.value = put_bit(self.value, value, 11);
                }
                BinaryFieldWidgetMessage::Bit12(value) => {
                    self.value = put_bit(self.value, value, 12);
                }
                BinaryFieldWidgetMessage::Bit13(value) => {
                    self.value = put_bit(self.value, value, 13);
                }
                BinaryFieldWidgetMessage::Bit14(value) => {
                    self.value = put_bit(self.value, value, 14);
                }
                BinaryFieldWidgetMessage::Bit15(value) => {
                    self.value = put_bit(self.value, value, 15);
                }
                BinaryFieldWidgetMessage::Bit16(value) => {
                    self.value = put_bit(self.value, value, 16);
                }
                BinaryFieldWidgetMessage::Bit17(value) => {
                    self.value = put_bit(self.value, value, 17);
                }
                BinaryFieldWidgetMessage::Bit18(value) => {
                    self.value = put_bit(self.value, value, 18);
                }
                BinaryFieldWidgetMessage::Bit19(value) => {
                    self.value = put_bit(self.value, value, 19);
                }
                BinaryFieldWidgetMessage::Bit20(value) => {
                    self.value = put_bit(self.value, value, 20);
                }
                BinaryFieldWidgetMessage::Bit21(value) => {
                    self.value = put_bit(self.value, value, 21);
                }
                BinaryFieldWidgetMessage::Bit22(value) => {
                    self.value = put_bit(self.value, value, 22);
                }
                BinaryFieldWidgetMessage::Bit23(value) => {
                    self.value = put_bit(self.value, value, 23);
                }
                BinaryFieldWidgetMessage::Bit24(value) => {
                    self.value = put_bit(self.value, value, 24);
                }
                BinaryFieldWidgetMessage::Bit25(value) => {
                    self.value = put_bit(self.value, value, 25);
                }
                BinaryFieldWidgetMessage::Bit26(value) => {
                    self.value = put_bit(self.value, value, 26);
                }
                BinaryFieldWidgetMessage::Bit27(value) => {
                    self.value = put_bit(self.value, value, 27);
                }
                BinaryFieldWidgetMessage::Bit28(value) => {
                    self.value = put_bit(self.value, value, 28);
                }
                BinaryFieldWidgetMessage::Bit29(value) => {
                    self.value = put_bit(self.value, value, 29);
                }
                BinaryFieldWidgetMessage::Bit30(value) => {
                    self.value = put_bit(self.value, value, 30);
                }
                BinaryFieldWidgetMessage::Bit31(value) => {
                    self.value = put_bit(self.value, value, 31);
                }
            }
        }
        if !self.messages.is_empty() {
            self.messages.clear();
            shell.publish((self.on_change)(self.value));
        }
        state
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryFieldWidgetMessage {
    Bit0(bool),
    Bit1(bool),
    Bit2(bool),
    Bit3(bool),
    Bit4(bool),
    Bit5(bool),
    Bit6(bool),
    Bit7(bool),
    Bit8(bool),
    Bit9(bool),
    Bit10(bool),
    Bit11(bool),
    Bit12(bool),
    Bit13(bool),
    Bit14(bool),
    Bit15(bool),
    Bit16(bool),
    Bit17(bool),
    Bit18(bool),
    Bit19(bool),
    Bit20(bool),
    Bit21(bool),
    Bit22(bool),
    Bit23(bool),
    Bit24(bool),
    Bit25(bool),
    Bit26(bool),
    Bit27(bool),
    Bit28(bool),
    Bit29(bool),
    Bit30(bool),
    Bit31(bool),
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
    value
}
