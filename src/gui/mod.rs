use std::{cell::RefCell, rc::Rc};

use druid::{Widget, Data, Lens, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, Size, PaintCtx, RenderContext, piet::{ImageFormat, InterpolationMode}, Selector};
use image::DynamicImage;

use crate::{generator::{trunk_layer::TrunkParams, branch_layer::BranchParams}, skeleton::tree::Tree};

pub const UPDATE_IMAGE: Selector = Selector::new("treegen.update-image");

#[derive(Debug, Clone, Data, Lens)]
pub struct AppData {
    pub trunk_params: TrunkParams,
    pub branch_params: BranchParams,
    pub tree: Rc<RefCell<Tree>>,
    pub image: Rc<RefCell<DynamicImage>>,
    pub image_updated: bool,
}

pub struct DynamicImageWidget;

impl Widget<AppData> for DynamicImageWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut AppData, _env: &Env) {
        if let Event::Command(cmd) = event {
            if cmd.is(UPDATE_IMAGE) {
                ctx.request_paint();
            }
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppData, _env: &Env) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppData, _data: &AppData, _env: &Env) {
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &AppData, _env: &Env) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, _env: &Env) {
        let image_data = data.image.borrow().to_rgb8();
        let image = ctx.make_image(image_data.width() as usize, image_data.height() as usize, &image_data, ImageFormat::Rgb).unwrap();

        let size = ctx.size();
        let scale_x = size.width / image_data.width() as f64;
        let scale_y = size.height / image_data.height() as f64;
        let scale = scale_x.min(scale_y);
        let scaled_size = Size::new(image_data.width() as f64 * scale, image_data.height() as f64 * scale);
        ctx.draw_image(&image, scaled_size.to_rect(), InterpolationMode::Bilinear)
    }
}
