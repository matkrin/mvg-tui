use chrono::Local;
use itertools::Itertools;
use mvg_api::routes::{Connection, ConnectionPart, Station};
use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table},
};

use crate::{
    app::{App, Focus, InputMode, RoutesTableState},
};

pub fn routes_table(app: &App) -> Table {
    let header_cells = ["TIME", "IN", "DURATION", "LINES", "DELAY", "INFO"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Magenta)));
    let header = Row::new(header_cells)
        .style(Style::default())
        .height(1)
        .bottom_margin(1);

    let items = &app.routes;

    let rows = items.iter().map(prepare_routes);

    Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Routes")
                .border_style(match app.focus {
                    Focus::Routes => match app.input_mode {
                        InputMode::Table => Style::default().fg(Color::Yellow),
                        _ => Style::default().fg(Color::Blue),
                    },
                    _ => Style::default(),
                }),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        // .highlight_symbol("> ")
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            Constraint::Percentage(14),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            Constraint::Percentage(32),
        ])
}

fn prepare_routes(conn: &Connection) -> Row {
    let height = 1;
    let origin = &conn.parts[0].from;
    let destination = &conn.parts[conn.parts.len()-1].to;
    let time = format!(
        "{} - {}",
        origin.planned_departure.format("%H:%M"),
        destination.planned_departure.format("%H:%M")
    );
    let in_minutes = (origin.planned_departure.time() - Local::now().time())
        .num_minutes()
        .to_string();
    let duration = (destination.planned_departure.time() - origin.planned_departure.time())
        .num_minutes()
        .to_string();
    let lines = prepare_lines(&conn.parts);
    let delay = prepare_delay(&origin);
    let info = prepare_info(&conn.parts);
    let cells = vec![time, in_minutes, duration, lines, delay, info];
    Row::new(cells)
        .height(height as u16)
        .bottom_margin(0)
        .style(Style::default())
}

fn prepare_lines(cp_list: &[ConnectionPart]) -> String {
    let mut lines = Vec::new();
    for cp in cp_list.iter() {
        if cp.line.label == "FOOTWAY" {
            lines.push("walk");
        } else {
            lines.push(&cp.line.label);
        }
    }
    lines.iter().unique().join(", ")
}

fn prepare_delay(origin_station: &Station) -> String {
    let delay = match origin_station.departure_delay_in_minutes {
        Some(d) if d != 0 => d.to_string(),
        _ => "-".to_string(),

    };
    delay
}

fn prepare_info(cp_list: &[ConnectionPart]) -> String {
    let mut info = "".to_string();
    for cp in cp_list.iter() {
        let label = cp.line.label.clone();
        let messages = cp.messages.join("\n");
        info.push_str(&format!("{}: {}", label, messages));
    }
    info
}

pub fn notifications<'a>(app: &'a App, routes_table_state: &RoutesTableState) -> Paragraph<'a> {
    let mut nots = Vec::new();
    // let mut not = "".to_string();
    for i in &app.routes {
        let not = prepare_info(&i.parts);
        nots.push(not);
    }
    let curr_not = if let Some(selected) = routes_table_state.table_state.selected() {
        &nots[selected]
    } else {
        ""
    };

    Paragraph::new(Text::from(curr_not.to_string())).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Notifications"),
    )
}

pub fn details_list<'a>(app: &'a App, routes_table_state: &RoutesTableState) -> List<'a> {
    let mut det = Vec::new();
    match routes_table_state.table_state.selected() {
        Some(idx) => {
            for j in &app.routes[idx].parts {
                det.push(format!(
                    " ╭─ {}, {}",
                    j.from.name,
                    j.from.planned_departure.format("%H:%M")
                ));
                for k in &j.intermediate_stops {
                    // for l in k {
                        det.push(format!(
                            " ├──── {}, {}",
                            k.name,
                            k.planned_departure.format("%H:%M")
                        ));
                    // }
                }
                det.push(format!(" ╰─ {}, {}", j.to.name, j.to.planned_departure.format("%H:%M")));
            }
        }
        None => (),
    }

    let items = det
        .iter()
        .map(|x| ListItem::new(Span::raw(x.clone())))
        .collect::<Vec<ListItem>>();
    List::new(items).block(Block::default().borders(Borders::ALL).title("Details"))
}
