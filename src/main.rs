use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Position},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem,ListState,Paragraph},
    DefaultTerminal, Frame,
    TerminalOptions, Viewport
};
use std::env;
use std::io::{self, BufRead};
use matchr::match_items;

fn main() -> Result<()> {
    let args : Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
	println!("You must provide an argument.");
	std::process::exit(0);
    }
    color_eyre::install()?;
    let terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(12),
    });
    let stdin = io::stdin();
    let entries: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let app_result = App::new().run(terminal, args, entries);
    ratatui::restore();
    app_result
}

struct App {
    input: String,
    character_index: usize,
    input_mode: InputMode,
    items: Vec<String>,
    list_items: ListState
}

enum InputMode {
    Normal,
    Editing,
}

impl App {
    fn new() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            items: Vec::new(),
            character_index: 0,
	    list_items: ListState::default(),
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn query(&mut self, entries: Vec<String>) {
	let items_refs: Vec<&str> = entries.iter().map(|s| s.as_str()).collect();
	let results = match_items(&self.input.clone(), &items_refs);
	self.items.clear();
	for (item, score) in results {
	    if self.input.is_empty() || score > 0 {
		self.items.push(item.to_string());
	    } else {
		continue;
	    }
	}
    }

    fn selected_item(&mut self) {
	if let Some(i) = self.list_items.selected() {
	    println!("{}", self.items[i]);
	}
    }

    fn run(mut self, mut terminal: DefaultTerminal, _args: Vec<String>, entries: Vec<String>) -> Result<()> {
	self.list_items.select_first();
        loop {
	    self.query(entries.clone());
            terminal.draw(|frame| self.draw(frame))?;
	    self.input_mode = InputMode::Editing;
            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    InputMode::Normal => {},
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => {
			    self.selected_item();
			    return Ok(())
			},
                        KeyCode::Char(to_insert) => {
			    self.enter_char(to_insert);
			    self.list_items.select_first();
			    self.query(entries.clone());
			},
                        KeyCode::Backspace => {
			    self.delete_char();
			    self.list_items.select_first();
 			    self.query(entries.clone());
			},
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Up => self.list_items.select_previous(),
                        KeyCode::Down => self.list_items.select_next(),
                        KeyCode::Esc => return Ok(()),
                        _ => {}
                    },
                    InputMode::Editing => {}
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [help_area, input_area, items_area] = vertical.areas(frame.area());

        let (msg, style) = match self.input_mode {
            InputMode::Normal => (vec![], Style::default()),
            InputMode::Editing => (
                vec![
		    "Press ".into(),
		    "Esc".bold(),
		    " to clear filter, use Up/Down to move, ".into(),
		    "Enter".bold(),
		    " to confirm selection.".into(),
		    
                ],
                Style::default(),
            ),
        };
        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);

        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("query"));
        frame.render_widget(input, input_area);
        match self.input_mode {
            InputMode::Normal => {}

            #[allow(clippy::cast_possible_truncation)]
            InputMode::Editing => frame.set_cursor_position(Position::new(
                input_area.x + self.character_index as u16 + 1,
                input_area.y + 1,
            )),
        }

        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = Line::from(Span::raw(format!("{i}: {m}")));
		ListItem::new(content)
            })
            .collect();
	
        let list_items = List::new(items)
	    .block(Block::bordered().title("items"))
	    .highlight_symbol(">> ")
	    .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

        frame.render_stateful_widget(list_items, items_area, &mut self.list_items);
    }
}
