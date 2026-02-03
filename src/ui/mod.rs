use crate::app::{App, selected_tab::SelectedTab};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{self, Block, Tabs},
};
use strum::IntoEnumIterator;

mod tab;

pub fn ui(f: &mut Frame, app: &App) {
    let titles = SelectedTab::iter().map(SelectedTab::title);

    let tabs = Tabs::new(titles).select(app.selected_tab as usize);

    f.render_widget(tabs, Rect::new(0, 0, f.area().width, f.area().height));

    match app.selected_tab {
        SelectedTab::Live => tab::render_tab_live(f, app),
        SelectedTab::Log => tab::render_tab_log(f, app),
    }

    let upd_str = if *app.updated.lock().unwrap() {
        "██  "
    } else {
        "  ██"
    };

    let upd_block = Block::bordered();

    f.render_widget(
        widgets::Paragraph::new(upd_str).block(upd_block),
        Rect::new(f.area().width - 6, f.area().height - 3, 6, 3),
    );
}
