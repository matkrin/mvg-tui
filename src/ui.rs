use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::{
    app::{App, Focus, InputMode, RoutesTableState},
    ui_elements::{
        arrival_paragraph, bus_paragraph, date_paragraph, desination_paragraph, details_list,
        help_message, notifications, popup_rect, routes_table, sbahn_paragraph, start_paragraph,
        time_paragraph, tram_paragraph, ubahn_paragraph, wrong_datetime_paragraph,
    },
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, routes_table_state: &mut RoutesTableState) {
    // Layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let input_areas = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    let options_areas = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let info_area = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(chunks[2]);

    let table_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(info_area[0]);

    let start_area = input_areas[0];
    let destination_area = input_areas[1];

    //Input ares
    let input_start = start_paragraph(app);
    let input_destination = desination_paragraph(app);

    f.render_widget(input_start, start_area);
    f.render_widget(input_destination, destination_area);

    // Option areas
    let date_panel = date_paragraph(app);
    f.render_widget(date_panel, options_areas[0]);

    let time_panel = time_paragraph(app);
    f.render_widget(time_panel, options_areas[1]);

    let arrival_panel = arrival_paragraph(app);
    f.render_widget(arrival_panel, options_areas[2]);

    let ubahn_panel = ubahn_paragraph(app);
    f.render_widget(ubahn_panel, options_areas[3]);

    let sbahn_panel = sbahn_paragraph(app);
    f.render_widget(sbahn_panel, options_areas[4]);

    let tram_panel = tram_paragraph(app);
    f.render_widget(tram_panel, options_areas[5]);

    let bus_panel = bus_paragraph(app);
    f.render_widget(bus_panel, options_areas[6]);

    // Routes pane
    let routes = routes_table(app);
    f.render_stateful_widget(routes, table_area[0], &mut routes_table_state.table_state);

    // Routes details
    let details = details_list(app, routes_table_state);
    f.render_widget(details, info_area[1]);

    // Notification area
    let notification = notifications(app, routes_table_state);
    f.render_widget(notification, table_area[1]);

    // Help message
    let help_message = help_message(app);
    // let help_message = Paragraph::new(Text::from(app.frames.to_string()));
    // let help_message = Paragraph::new(Text::from(app.datetime.to_string()));
    f.render_widget(help_message, chunks[3]);

    // Fetching popup
    if app.show_fetch_popup {
        let popup_area = popup_rect(10, 5, f.size());
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Fetching")
            .title_alignment(Alignment::Center)
            .style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Blue)
                    .fg(Color::Black),
            );
        f.render_widget(Clear, popup_area);
        f.render_widget(block, popup_area);
    }

    // Datetime error popups
    if app.wrong_date {
        let date_popup_area = popup_rect(20, 20, f.size());
        let wrong_date_paragraph =
            wrong_datetime_paragraph("Please enter a valid date", "Date Error");
        f.render_widget(Clear, date_popup_area);
        f.render_widget(wrong_date_paragraph, date_popup_area);
    }

    if app.wrong_time {
        let date_popup_area = popup_rect(20, 20, f.size());
        let wrong_time_paragraph =
            wrong_datetime_paragraph("Please enter a valid time", "Time Error");
        f.render_widget(Clear, date_popup_area);
        f.render_widget(wrong_time_paragraph, date_popup_area);
    }

    // Cursor position
    if let InputMode::Editing = app.input_mode {
        match app.focus {
            Focus::Start => f.set_cursor(
                input_areas[0].x + app.input_start.width() as u16 + 1,
                input_areas[0].y + 1,
            ),
            Focus::Destination => f.set_cursor(
                input_areas[1].x + app.input_destination.width() as u16 + 1,
                input_areas[1].y + 1,
            ),
            Focus::Date => f.set_cursor(
                options_areas[0].x + app.input_date.width() as u16 + 1,
                options_areas[0].y + 1,
            ),
            Focus::Time => f.set_cursor(
                options_areas[1].x + app.input_time.width() as u16 + 1,
                options_areas[1].y + 1,
            ),
            _ => {}
        }
    }
}
