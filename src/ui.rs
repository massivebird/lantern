use crate::app::{App, output_fmt::OutputFmt, selected_tab::SelectedTab};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, Paragraph, Tabs},
};
use strum::IntoEnumIterator;

pub fn ui(f: &mut Frame, app: &App) {
    // Render tabs at the top.

    let titles = SelectedTab::iter().map(SelectedTab::title);

    let tabs = Tabs::new(titles).select(app.selected_tab as usize);

    f.render_widget(tabs, Rect::new(0, 0, f.area().width, f.area().height));

    // Render contents of selected tab.
    match app.selected_tab {
        SelectedTab::Live => render_tab_live(f, app),
        SelectedTab::Log => render_tab_log(f, app),
    }
}

fn render_tab_live(f: &mut Frame, app: &App) {
    let sites = app.connections.lock().unwrap().clone();

    let mut list_items: Vec<Line<'_>> = Vec::new();

    for site in &sites {
        // Compute online status color.
        // Green is OK, red is bad, etc.
        let status_color = {
            if site.log().front().is_none() {
                Color::Gray // Requests have not been sent yet.
            } else {
                match site.log()[0].as_ref() {
                    Ok(code) => match code {
                        200 => Color::Green,
                        _ => Color::Yellow,
                    },
                    _ => Color::Red,
                }
            }
        };

        let url = site.addr();

        let site_output: Line<'_> = match app.output_fmt {
            OutputFmt::Bullet => Line::from(vec![
                Span::from(" ■ ").style(status_color),
                Span::from(format!("{} ({})", site.name.clone(), url)),
            ]),
            OutputFmt::Line => Line::from(Span::from(format!(" {} ({})", site.name.clone(), url)))
                .style(
                    Style::new()
                        .bg(status_color)
                        .fg(if status_color == Color::DarkGray {
                            Color::DarkGray
                        } else {
                            Color::Black
                        })
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC),
                ),
        };

        list_items.push(site_output);
    }

    let block = Block::bordered().title_bottom(" q: Quit ");

    f.render_widget(
        List::new(list_items).block(block),
        Rect::new(0, 1, f.area().width, f.area().height),
    );
}

fn render_tab_log(f: &mut Frame, app: &App) {
    let idx = app.get_selected_chart_site_idx();

    let site = app.connections.lock().unwrap().get(idx).unwrap().clone();

    let statuses = site.log();

    let block = Block::bordered().title_bottom(" q: Quit | j: Next site | k: Previous site ");

    let mut text = Vec::new();

    for status in site.log() {
        text.push(Line::raw(format!("{status:?}")));
    }

    let paragraph = Paragraph::new(text).block(block);

    let info = Line::from(format!(
        " Selected site: [{idx:02}] {} ({}) ",
        site.name,
        site.addr()
    ));

    f.render_widget(
        paragraph,
        Rect::new(0, 1, f.area().width, f.area().height - 1),
    );

    f.render_widget(info, Rect::new(2, 1, f.area().width, f.area().height - 1));
}
