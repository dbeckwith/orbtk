use crate::{
    layout::PaddingLayout,
    properties::*,
    render_object::RectangleRenderObject,
    widget::{Template, Widget},
};

widget!(
    /// The `Container` represents a layout that surrounds its child with a padding. Draws a box around the child.
    Container
    (
        BackgroundProperty,
        BorderRadiusProperty,
        BorderThicknessProperty,
        BorderBrushProperty,
        PaddingProperty
    )
);

impl Widget for Container {

    fn create() -> Self {
        Container::new()
            .padding(0.0)
            .background("transparent")
            .border_radius(0.0)
            .border_thickness(0.0)
            .border_brush("transparent")
            .render_object(RectangleRenderObject)
            .layout(PaddingLayout::new())
            .debug_name("Container")
    }
}

