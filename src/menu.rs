use conrod::widget;
use conrod::UiCell;

use crate::conrod_helper::Gui;
use crate::control::Flow;

widget_ids! {
    pub struct Ids {
        canvas,
        button,
        text,
    }
}

pub struct StartMenu;

impl Gui for StartMenu {
    type Ids = self::Ids;

    fn new() -> Self {
        StartMenu
    }

    fn ids(&self, w: widget::id::Generator) -> Self::Ids {
        Self::Ids::new(w)
    }

    fn gui(&mut self, ui: &mut UiCell, ids: &Self::Ids) -> Option<Flow> {
        use conrod::{Labelable, Positionable, Sizeable, Widget};

        widget::Canvas::new().set(ids.canvas, ui);

        let mut button = widget::Button::new()
            .middle_of(ids.canvas)
            .label("Start game")
            .w_h(120.0, 30.0)
            .set(ids.button, ui);

        if button.next().is_some() {
            Some(Flow::StartGame)
        } else {
            None
        }
    }
}

pub struct PlayAgainMenu;

impl Gui for PlayAgainMenu {
    type Ids = self::Ids;

    fn new() -> Self {
        PlayAgainMenu
    }

    fn ids(&self, w: widget::id::Generator) -> Self::Ids {
        Self::Ids::new(w)
    }

    fn gui(&mut self, ui: &mut UiCell, ids: &Self::Ids) -> Option<Flow> {
        use conrod::{Labelable, Positionable, Sizeable, Widget};

        widget::Canvas::new().set(ids.canvas, ui);

        let mut button = widget::Button::new()
            .middle_of(ids.canvas)
            .label("Play again")
            .w_h(120.0, 30.0)
            .set(ids.button, ui);

        widget::Text::new("You lose!!")
            .align_middle_x_of(ids.canvas)
            .up_from(ids.button, 30.0)
            .set(ids.text, ui);

        if button.next().is_some() {
            Some(Flow::PlayAgain)
        } else {
            None
        }
    }
}
