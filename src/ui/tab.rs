use crate::app::{self, App, connection::ConnectionType, output_fmt::OutputFmt};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, Paragraph},
};

pub fn render_tab_live(f: &mut Frame, app: &App) {
    let mut list_items: Vec<Line<'_>> = Vec::new();

    for conn in app.connections.lock().unwrap().iter() {
        let color = conn
            .log()
            .front()
            .map_or(Color::Gray, |f| status_to_color(f, &conn.conn_type));

        let url = conn.addr();

        let conn_output: Line<'_> = match app.output_fmt {
            OutputFmt::Bullet => Line::from(vec![
                Span::from(" 󰝤 ").style(color),
                Span::from(format!("{} ({})", conn.name, url)),
            ]),
            OutputFmt::Line => Line::from(Span::from(format!(" {} ({})", conn.name, url))).style(
                Style::new()
                    .bg(color)
                    .fg(if color == Color::DarkGray {
                        Color::DarkGray
                    } else {
                        Color::Black
                    })
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::ITALIC),
            ),
        };

        list_items.push(conn_output);
    }

    let block = Block::bordered().title_bottom(" q: Quit | o: Cycle formats ");

    f.render_widget(
        List::new(list_items).block(block),
        Rect::new(0, 1, f.area().width, f.area().height),
    );
}

pub fn render_tab_log(f: &mut Frame, app: &App) {
    let (idx, conn) = app.log_conn();

    let block =
        Block::bordered().title_bottom(" q: Quit | j: Next connection | k: Previous connection ");

    let mut text = Vec::new();

    for (i, status) in conn.log().iter().enumerate() {
        let desc = match (status, &conn.conn_type) {
            (Ok(code), ConnectionType::Remote { .. }) => code.to_string(),
            (Ok(ms), ConnectionType::Local { .. }) => format!("{ms} ms"),
            (Err(e), _) => e.clone(),
        };

        let color = status_to_color(status, &conn.conn_type);

        // Identify the latest status.
        let left = if i == 0 {
            Span::styled("  Now  ", Style::new().bg(color).fg(Color::Black).bold())
        } else {
            Span::styled("       ", Style::new().bg(color))
        };

        text.push(Line::from(vec![left, Span::raw(" "), Span::raw(desc)]));
    }

    let paragraph = Paragraph::new(text).block(block);

    let info = Line::from(format!(
        " Selected connection: [{idx:02}] {} ({}) ",
        conn.name,
        conn.addr()
    ));

    f.render_widget(
        paragraph,
        Rect::new(0, 1, f.area().width, f.area().height - 1),
    );

    f.render_widget(info, Rect::new(2, 1, f.area().width, f.area().height - 1));
}

const fn status_to_color(status: &app::Status, conn_type: &ConnectionType) -> Color {
    let Ok(code) = status else {
        return Color::Red;
    };

    match conn_type {
        ConnectionType::Remote { .. } => match code {
            200 => Color::Green,
            _ => Color::Yellow,
        },
        ConnectionType::Local { .. } => Color::Green,
    }
}
