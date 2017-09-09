use conrod::UiCell;
use conrod::widget;

use conrod_helper::Gui;
use control::Flow;

widget_ids! {
    pub struct Ids {
        canvas,
        button,
        text,
    }
}

pub struct StartMenu();

impl Gui for StartMenu {
    type Ids = self::Ids;

    fn new() -> Self {
        StartMenu()
    }

    fn ids(&self, w: widget::id::Generator) -> Self::Ids {
        Self::Ids::new(w)
    }

    fn gui(&mut self, ui: &mut UiCell, ids: &Self::Ids) -> Option<Flow> {
        use conrod::{Widget, Positionable, Sizeable, Labelable};

        widget::Canvas::new().set(ids.canvas, ui);

        let button = widget::Button::new()
            .middle_of(ids.canvas)
            .label("Start game")
            .w_h(120.0, 30.0)
            .set(ids.button, ui);

        for _click in button {
            return Some(Flow::StartGame);
        }

        None
    }
}

pub struct PlayAgainMenu();

impl Gui for PlayAgainMenu {
    type Ids = self::Ids;

    fn new() -> Self {
        PlayAgainMenu()
    }

    fn ids(&self, w: widget::id::Generator) -> Self::Ids {
        Self::Ids::new(w)
    }

    fn gui(&mut self, ui: &mut UiCell, ids: &Self::Ids) -> Option<Flow> {
        use conrod::{Widget, Positionable, Sizeable, Labelable};

        widget::Canvas::new().set(ids.canvas, ui);

        let button = widget::Button::new()
            .middle_of(ids.canvas)
            .label("Play again")
            .w_h(120.0, 30.0)
            .set(ids.button, ui);

        widget::Text::new("You lose!!")
            .up_from(ids.button, 20.0)
            .set(ids.text, ui);

        for _click in button {
            return Some(Flow::PlayAgain);
        }

        None
    }
}
