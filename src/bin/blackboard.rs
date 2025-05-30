use rui_research::engines::sdl::SDLEngine;
use rui_research::general::Vector2D;
use rui_research::widgets::themes::StyleMaster;
use rui_research::widgets::{DarkSimpleTheme, Primitive, TextBox, WidgetEnum, WidgetId};
use rui_research::window::WindowBuilder;

#[derive(Clone, Copy, Debug)]
struct BlackboardEnums {
    id: WidgetId,
}

impl WidgetEnum for BlackboardEnums {
    fn to_wid(self) -> WidgetId {
        self.id
    }
}

fn main() -> Result<(), Box<(dyn std::error::Error)>> {
    let mut sdl_engine = SDLEngine::init()?;
    let style = StyleMaster::new(Box::new(DarkSimpleTheme))?;
    let mut window_builder = WindowBuilder::new()?;
    let string = "Phrase 1
Phrase 2
Phrase 3";
    let mut i = 0;
    for line in string.lines() {
        let mut text2 = TextBox::new(0, line, style.clone())?;
        text2.set_position(Vector2D::new(0., 50. * i as f32));
        let wenum = BlackboardEnums { id: i as WidgetId };
        window_builder.add_widget(i, text2, wenum);
        i += 1;
    }

    sdl_engine.add_window_builder(window_builder)?;
    sdl_engine.main_loop();
    Ok(())
}
