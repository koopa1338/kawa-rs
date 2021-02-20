use super::model::AppData;
use druid::widget::{Container, Flex, Widget};
use druid::Color;

pub fn ui_builder() -> impl Widget<AppData> {
    let root = Flex::column();

    /*
    let list = Flex::row().cross_axis_alignment(CrossAxisAlignment::Start);
    list.add_flex_child(
        Scroll::new(List::new(|| {
            Label::new(|item: &u32, _env: &_| format!("ITEM {}", item))
                .align_vertical(UnitPoint::LEFT)
                .padding(10.0)
                .expand()
                .height(50.0)
                .background(Color::rgb(0.5, 0.5, 0.5))
        }))
        .vertical()
        .lens(AppData::list),
        1.0,
    );

    root.add_child(list);
    */
    let container = Container::new(root);
    container.border(Color::from_hex_str("#cccccc").unwrap(), 10.0)
}
