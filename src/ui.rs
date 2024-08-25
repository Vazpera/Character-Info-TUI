use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style, Stylize},
    symbols::{self, border::Set},
    text::{Line, Span},
    widgets::{
        block::title, Block, BorderType, Borders, List, Paragraph, ScrollDirection, Scrollbar,
        ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget, Wrap,
    },
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let curr_char = app.curr_char.clone();
    let [title_bar, main_area] =
        Layout::vertical(vec![Constraint::Length(1), Constraint::Fill(1)]).areas(frame.size());

    Block::bordered()
        .title(curr_char.name.fg(Color::from_u32(curr_char.color)).bold())
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP)
        .render(title_bar, frame.buffer_mut());

    let [left, middle, right] = Layout::horizontal(vec![
        Constraint::Fill(1),
        Constraint::Fill(3),
        Constraint::Fill(1),
    ])
    .areas(main_area);
    let [physical, mental] =
        Layout::vertical(vec![Constraint::Fill(1), Constraint::Fill(1)]).areas(left);
    let [stories, bonds] =
        Layout::vertical(vec![Constraint::Fill(1), Constraint::Fill(1)]).areas(right);
    Paragraph::new(vec![
        format!("Age: {}", curr_char.age).into(),
        format!("Height: {}cm", curr_char.physical_characteristics.height).into(),
        format!("Weight: {}kg", curr_char.physical_characteristics.weight).into(),
        format!(
            "Eye Color: {}",
            curr_char.physical_characteristics.eye_color
        )
        .into(),
        format!(
            "Hair Color: {}",
            curr_char.physical_characteristics.hair_color
        )
        .into(),
        format!("Race: {}", curr_char.physical_characteristics.race).into(),
        format!("Species: {}", curr_char.species).into(),
    ])
    .wrap(Wrap { trim: false })
    .block(
        Block::bordered().title(
            "Physical Traits"
                .fg(Color::from_u32(curr_char.color))
                .bold(),
        ),
    )
    .render(physical, frame.buffer_mut());

    Paragraph::new(vec![
        format!("Alignment: {}", curr_char.mental_characteristics.alignment).into(),
        format!(
            "Mental Illnesses: {}",
            curr_char.mental_characteristics.disorders
        )
        .into(),
    ])
    .wrap(Wrap { trim: false })
    .block(Block::bordered().title("Mental Traits".fg(Color::from_u32(curr_char.color)).bold()))
    .render(mental, frame.buffer_mut());

    frame.render_widget(
        List::new(curr_char.story_appearances).block(
            Block::bordered()
                .title(
                    "Story Appearances"
                        .fg(Color::from_u32(curr_char.color))
                        .bold(),
                )
                .title_alignment(Alignment::Right),
        ),
        stories,
    );
    Paragraph::new(curr_char.bonds)
        .block(
            Block::bordered()
                .title("Bonds".fg(Color::from_u32(curr_char.color)).bold())
                .title_alignment(Alignment::Right),
        )
        .wrap(Wrap { trim: false })
        .render(bonds, frame.buffer_mut());
    Paragraph::new(curr_char.description)
        .block(
            Block::bordered()
                .title("Description".fg(Color::from_u32(curr_char.color)).bold())
                .title_alignment(Alignment::Center),
        )
        .wrap(Wrap { trim: false })
        .render(middle, frame.buffer_mut())
}
