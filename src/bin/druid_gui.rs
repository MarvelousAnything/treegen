use std::{cell::RefCell, rc::Rc};

use druid::{
    widget::{Button, Checkbox, Flex, Split},
    AppLauncher, UnitPoint, Widget, WidgetExt, WindowDesc,
};
use image::{DynamicImage, RgbImage, Rgb};
use treegen::{
    generator::{
        branch_layer::{BranchLayer, BranchParams},
        layer::Layer,
        trunk_layer::{TrunkLayer, TrunkParams},
    },
    gui::{AppData, DynamicImageWidget, UPDATE_IMAGE},
    rendering::{PolygonRenderer, Renderer},
    skeleton::{
        node::{HEIGHT, WIDTH},
        tree::Tree,
    },
};

macro_rules! create_sliders {
    ($param_struct:ident, $param_field:ident, $label:expr, $($field:ident{$lo:expr, $hi:expr}),+ $(,)? ) => {{
        use druid::LensExt;
        let mut col = Flex::column();
        col.add_child(druid::widget::Label::new($label).align_left());

        $(
            let lens = AppData::$param_field.then($param_struct::$field);
            let slider = druid::widget::Slider::new().with_range($lo, $hi).lens(lens).padding(5.0);
            let label = druid::widget::Label::new(move |data: &f64, _env: &_| format!("{}: {:.2}", stringify!($field), data))
                          .lens(lens);
            col.add_flex_child(Flex::row().with_child(label).with_flex_child(slider, 1.0), 1.0);
        )+

        col
    }};
}

fn make_trunk_sliders() -> impl Widget<AppData> {
    create_sliders!(
        TrunkParams,
        trunk_params,
        "Trunk Params",
        spread{0.0,1.0},
        split{0.0,1.0},
        branch{0.0,1.0},
        variability{0.0,1.0},
        lean_bias{0.0,90.0},
        default_branch_size{0.0,100.0},
        branch_size_falloff{0.0,50.0},
        default_height_mean{1.0,25.0},
        max_children{1.0,10.0},
        variability_modifier{0.1,3.0},
        angle_spread_positive{0.0,90.0},
    )
}

fn make_branch_sliders() -> impl Widget<AppData> {
    create_sliders!(
        BranchParams,
        branch_params,
        "Branch Params",
        spread{0.0,1.0},
        branch{0.0,1.0},
        variability{0.0,1.0}
    )
}

fn make_image_button() -> impl Widget<AppData> {
    Button::new("Generate Image").on_click(|ctx, data: &mut AppData, _env| {
        let mut renderer = PolygonRenderer;
        let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);

        for pixel in image.pixels_mut() {
            *pixel = Rgb([255u8, 255u8, 255u8]);
        }
        let mut tree = data.tree.borrow_mut().to_owned();
        tree = TrunkLayer::generate(tree, &data.trunk_params);
        tree = BranchLayer::generate(tree, &data.branch_params);
        renderer.render_tree(&mut image, &tree);
        data.image = Rc::new(RefCell::new(DynamicImage::ImageRgb8(image)));
        ctx.submit_command(UPDATE_IMAGE);
    })
}

fn make_layout() -> impl Widget<AppData> {
    let sliders = Flex::column()
        .with_flex_child(make_trunk_sliders(), 2.0)
        .with_flex_spacer(0.2)
        .with_flex_child(make_branch_sliders(), 2.0)
        .with_flex_child(make_image_button(), 0.2)
        .align_left()
        .align_vertical(UnitPoint::TOP)
        .align_horizontal(UnitPoint::LEFT)
        .padding(5.0);

    let image_widget = DynamicImageWidget;
    Split::columns(sliders, image_widget)
}

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    let main_window = WindowDesc::new(make_layout())
        .title("Tree Generator")
        .window_size((800.0, 600.0));

    let data = AppData {
        trunk_params: TrunkParams::new(1.0, 1.0, 1.0, 0.5),
        branch_params: BranchParams::new(1.0, 1.0, 0.5),
        tree: Rc::new(RefCell::new(Tree::new())),
        image: Rc::new(RefCell::new(DynamicImage::ImageRgb8(RgbImage::new(
            WIDTH as u32,
            HEIGHT as u32,
        )))),
        image_updated: false,
        auto_generate: false,
    };

    AppLauncher::with_window(main_window)
        .launch(data)
        .expect("Failed to launch application");
}
