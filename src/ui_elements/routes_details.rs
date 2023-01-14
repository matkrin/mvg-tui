use chrono::Local;
use itertools::Itertools;
use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table},
};

use crate::{
    api::routes::{Connection, ConnectionPart},
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
    let time = format!(
        "{} - {}",
        conn.departure.format("%H:%M"),
        conn.arrival.format("%H:%M")
    );
    let in_minutes = (conn.departure.time() - Local::now().time())
        .num_minutes()
        .to_string();
    let duration = (conn.arrival.time() - conn.departure.time())
        .num_minutes()
        .to_string();
    let lines = prepare_lines(&conn.connection_part_list);
    let delay = prepare_delay(&conn.connection_part_list);
    let info = prepare_info(&conn.connection_part_list);
    let cells = vec![time, in_minutes, duration, lines, delay, info];
    Row::new(cells)
        .height(height as u16)
        .bottom_margin(0)
        .style(Style::default())
}

fn prepare_lines(cp_list: &Vec<ConnectionPart>) -> String {
    let mut lines = Vec::new();
    for cp in cp_list.iter() {
        if cp.connection_part_type == "FOOTWAY" {
            lines.push("walk");
        } else {
            let label = if let Some(x) = &cp.label { x } else { "" };
            lines.push(label);
        }
    }
    lines.iter().unique().join(", ")
}

fn prepare_delay(cp_list: &Vec<ConnectionPart>) -> String {
    let mut delay = if let Some(x) = cp_list[0].delay {
        x.to_string()
    } else {
        "-".to_string()
    };
    if delay == "0" {
        delay = "-".to_string();
    }
    delay
}

fn prepare_info(cp_list: &Vec<ConnectionPart>) -> String {
    let mut info = "".to_string();
    for cp in cp_list.iter() {
        let label = if let Some(x) = &cp.label { x } else { "" };
        let nots = if let Some(x) = &cp.notifications {
            x.iter().map(|n| n.title.clone()).collect()
        } else {
            "".to_string()
        };
        if nots == "" {
            info = if let Some(x) = &cp.info_messages {
                x.join("\n")
            } else {
                "".to_string()
            };
        } else {
            info = format!("{}: {}", label, nots);
        }
    }
    info
}

pub fn notifications<'a>(app: &'a App, routes_table_state: &RoutesTableState) -> Paragraph<'a> {
    let mut nots = Vec::new();
    // let mut not = "".to_string();
    for i in &app.routes {
        let not = prepare_info(&i.connection_part_list);
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
            for j in &app.routes[idx].connection_part_list {
                det.push(format!(
                    " ╭─ {}, {}",
                    j.from.name,
                    j.departure.format("%H:%M")
                ));
                for k in &j.stops {
                    for l in k {
                        det.push(format!(
                            " ├──── {}, {}",
                            l.location.name,
                            l.time.format("%H:%M")
                        ));
                    }
                }
                det.push(format!(" ╰─ {}, {}", j.to.name, j.arrival.format("%H:%M")));
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
