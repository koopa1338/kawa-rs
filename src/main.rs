use druid::widget::{ Container, CrossAxisAlignment, Flex, Label, List, Scroll };
use druid::{ AppLauncher, Color, Data, Lens, UnitPoint, Widget, WidgetExt, WindowDesc };
use druid::im::{ Vector, vector };

#[derive(Clone, Data, Lens)]
struct AppData {
    list: Vector<u32>,
    idx: usize,
}


pub fn main() {
    let main_window = WindowDesc::new(ui_builder);
    // Set our initial data
    let list = vector![1, 2, 3, 4, 5];
    let data = AppData {
        idx: list.len(),
        list,
    };
    AppLauncher::with_window(main_window)
        .launch(data)
        .expect("launch failed");
}

fn ui_builder() -> impl Widget<AppData> {
    let mut root = Flex::column();

    let mut list = Flex::row().cross_axis_alignment(CrossAxisAlignment::Start);
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
    let container = Container::new(root);
    container.border(Color::from_hex_str("#cccccc").unwrap(), 10.0)
    
}
