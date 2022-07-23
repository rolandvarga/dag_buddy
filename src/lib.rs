use conf::AppConf;
// logging
use env_logger::Env;
use log::{debug, error, info};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, fs, io, path::Path};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};

mod conf;
mod parsing;

struct Item {
    file_name: String,
    tables: Vec<String>,
}

impl Item {
    fn new(file_name: String, tables: Vec<String>) -> Item {
        Item {
            file_name: file_name,
            tables: tables,
        }
    }
}

struct App {
    state: TableState,
    items: Vec<Item>,
}

impl<'a> App {
    fn new(app_conf: AppConf) -> App {
        let mut items: Vec<Item> = Vec::new();

        let mut app = App {
            state: TableState::default(),
            items: items,
        };

        let files =
            fs::read_dir(format!("{}/{}/", app_conf.dag.folder, app_conf.dag.name)).unwrap();
        for file in files {
            let file_name = file.unwrap().path().display().to_string(); //.display();
            let file_name_trimmed = Path::new(&file_name)
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap();
            let parsed = parsing::parse_file(file_name.as_str());
            let tables = parsing::parse_tables(parsed.as_str());
            log::debug!("{:?} => {}", tables, tables.len());

            let mut item = Item::new(file_name_trimmed, tables);
            app.items.push(item);
        }
        app
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub fn start() -> Result<(), Box<dyn Error>> {
    let app_conf = conf::AppConf::new().unwrap();
    env_logger::Builder::from_env(Env::default().default_filter_or(app_conf.log.level.clone()))
        .init();

    log::debug!("{:?}", app_conf);
    let app: App = App::new(app_conf);

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(5)
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["FILE_NAME", "TABLES"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = app.items.iter().map(|item| {
        let joined_tables = item.tables.join("\n");
        let height = joined_tables.chars().filter(|c| *c == '\n').count();
        let joined_items = vec![item.file_name.as_str(), joined_tables.as_str()];
        let cells = joined_items.iter().map(|c| Cell::from(c.to_string()));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);
    f.render_stateful_widget(t, rects[0], &mut app.state);
}
