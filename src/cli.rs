use std::{
    io::{self, stdout},
    time::Duration,
};
use misfit_core::regtest_pack::regtest::RegtestManager;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::*,
    Terminal,
    text::{Span, Line}
};
use clap::{Parser, Subcommand};
use crate::api::Generator;
use serde_json::Value;

#[derive(Parser)]
#[command(version, about, disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    Help,
    Clear,
    Exit,
    #[command(name = "decode-transaction")]
    DecodeTransaction { raw_transaction: String },
    #[command(name = "regtest-start")]
    RegtestStart,
    #[command(name = "regtest-stop")]
    RegtestStop,
}


struct App {
    command_items: Vec<CommandItem>,
    selected_index: usize,
    output_lines: Vec<String>,
    should_quit: bool,
    input_mode: InputMode,
    input_buffer: String,
    awaiting_input: Option<InputType>,
    regtest_manager: RegtestManager,
    output_selected_index: Option<usize>, 
    _expanded_output_lines: Vec<String>,
    subwindow_mode: Option<SubwindowMode>,
    field_items: Vec<FieldItem>,
    field_selected_index: usize,
    pending_hex_input: String,
}

#[derive(Clone)]
struct CommandItem {
    name: String,
    description: String,
    shortcut: Option<char>,
    command_type: CommandType,
}

#[derive(Clone)]
struct FieldItem {
    name: String,
    description: String,
    flag: String,
    selected: bool,
}

#[derive(Clone)]
enum CommandType {
    Simple(Commands),
    RequiresInput(InputType),
}

#[derive(Clone)]
enum InputType {
    DecodeTransaction,
    DecodeBlock,
    BreakTransaction,
    BreakBlock,
    GenerateTx,
    GenerateBlock,
    GetBlockByHeight,
}

#[derive(Clone)]
enum SubwindowMode {
    TransactionFields,
    BlockFields,
}

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Input,
    OutputSelect,
    Subwindow,
}

impl App {
    fn new() -> Self {
        let regtest_manager = Generator::regtest_invocation("bitcoinhos", "-regtest");
        
        App {
            command_items: vec![
                CommandItem {
                    name: "Help (h)".to_string(),
                    description: "Show help message".to_string(),
                    shortcut: Some('h'),
                    command_type: CommandType::Simple(Commands::Help),
                },
                CommandItem {
                    name: "Clear (c)".to_string(),
                    description: "Clear output".to_string(),
                    shortcut: Some('c'),
                    command_type: CommandType::Simple(Commands::Clear),
                },
                CommandItem {
                    name: "Decode Transaction (d)".to_string(),
                    description: "Decode a raw transaction".to_string(),
                    shortcut: Some('d'),
                    command_type: CommandType::RequiresInput(InputType::DecodeTransaction),
                },
                CommandItem {
                    name: "Decode Block (D)".to_string(),
                    description: "Decode a block header".to_string(),
                    shortcut: Some('D'),
                    command_type: CommandType::RequiresInput(InputType::DecodeBlock),
                },
                CommandItem {
                    name: "Break Transaction (b)".to_string(),
                    description: "Break/invalidate transaction fields".to_string(),
                    shortcut: Some('b'),
                    command_type: CommandType::RequiresInput(InputType::BreakTransaction),
                },
                CommandItem {
                    name: "Break Block (B)".to_string(),
                    description: "Break/invalidate block fields".to_string(),
                    shortcut: Some('B'),
                    command_type: CommandType::RequiresInput(InputType::BreakBlock),
                },
                CommandItem {
                    name: "Generate Transaction (g)".to_string(),
                    description: "Generate one or more transactions".to_string(),
                    shortcut: Some('g'),
                    command_type: CommandType::RequiresInput(InputType::GenerateTx),
                },
                CommandItem {
                    name: "Generate Block (G)".to_string(),
                    description: "Generate new block with transactions".to_string(),
                    shortcut: Some('G'),
                    command_type: CommandType::RequiresInput(InputType::GenerateBlock),
                },
                CommandItem {
                    name: "Get Block by Height (k)".to_string(),
                    description: "Get block at specific height".to_string(),
                    shortcut: Some('k'),
                    command_type: CommandType::RequiresInput(InputType::GetBlockByHeight),
                },
                CommandItem {
                    name: "Start Regtest (r)".to_string(),
                    description: "Start the regtest node".to_string(),
                    shortcut: Some('r'),
                    command_type: CommandType::Simple(Commands::RegtestStart),
                },
                CommandItem {
                    name: "Stop Regtest (s)".to_string(),
                    description: "Stop the regtest node".to_string(),
                    shortcut: Some('s'),
                    command_type: CommandType::Simple(Commands::RegtestStop),
                },
                CommandItem {
                    name: "Exit (q)".to_string(),
                    description: "Exit the application".to_string(),
                    shortcut: Some('q'),
                    command_type: CommandType::Simple(Commands::Exit),
                },
            ],
            selected_index: 0,
            output_lines: vec!["Bitcoin CLI Tool - Select a command from the left panel".to_string()],
            should_quit: false,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            awaiting_input: None,
            regtest_manager,
            output_selected_index: None,
            _expanded_output_lines: Vec::new(),
            subwindow_mode: None,
            field_items: Vec::new(),
            field_selected_index: 0,
            pending_hex_input: String::new(),
        }
    }

    fn get_transaction_fields() -> Vec<FieldItem> {
        vec![
            FieldItem {
                name: "Version".to_string(),
                description: "Transaction version number".to_string(),
                flag: "--version".to_string(),
                selected: false,
            },
            FieldItem {
                name: "Transaction ID".to_string(),
                description: "Input transaction ID".to_string(),
                flag: "--txid".to_string(),
                selected: false,
            },
            FieldItem {
                name: "VOut".to_string(),
                description: "Input vout index".to_string(),
                flag: "--vout".to_string(),
                selected: false,
            },
            FieldItem {
                name: "Script Signature".to_string(),
                description: "Input script signature".to_string(),
                flag: "--script-sig".to_string(),
                selected: false,
            },
            FieldItem {
                name: "Sequence".to_string(),
                description: "Input sequence number".to_string(),
                flag: "--sequence".to_string(),
                selected: false,
            },
            FieldItem {
                name: "Amount".to_string(),
                description: "Output amount".to_string(),
                flag: "--amount".to_string(),
                selected: false,
            },
            FieldItem {
                name: "Script PubKey".to_string(),
                description: "Output script pubkey".to_string(),
                flag: "--script-pubkey".to_string(),
                selected: false,
            },
            FieldItem {
                name: "Witness".to_string(),
                description: "Witness data".to_string(),
                flag: "--witness".to_string(),
                selected: false,
            },
            FieldItem {
                name: "Locktime".to_string(),
                description: "Transaction locktime".to_string(),
                flag: "--locktime".to_string(),
                selected: false,
            },
            FieldItem {
                name: "All Fields".to_string(),
                description: "Break all transaction fields".to_string(),
                flag: "--all".to_string(),
                selected: false,
            },
        ]
    }

    fn get_block_fields() -> Vec<FieldItem> {
        vec![
            FieldItem {
                name: "Version".to_string(),
                description: "Block version".to_string(),
                flag: "--version".to_string(),
                selected: false,
            },
            FieldItem {
                name: "Previous Hash".to_string(),
                description: "Previous block hash".to_string(),
                flag: "--prev-hash".to_string(),
                selected: false,
            },
            FieldItem {
                name: "Merkle Root".to_string(),
                description: "Merkle root hash".to_string(),
                flag: "--merkle-root".to_string(),
                selected: false,
            },
            FieldItem {
                name: "Timestamp".to_string(),
                description: "Block timestamp".to_string(),
                flag: "--timestamp".to_string(),
                selected: false,
            },
            FieldItem {
                name: "Bits".to_string(),
                description: "Difficulty bits".to_string(),
                flag: "--bits".to_string(),
                selected: false,
            },
            FieldItem {
                name: "Nonce".to_string(),
                description: "Block nonce".to_string(),
                flag: "--nonce".to_string(),
                selected: false,
            },
            FieldItem {
                name: "All Fields".to_string(),
                description: "Break all block fields".to_string(),
                flag: "--all".to_string(),
                selected: false,
            },
        ]
    }

    fn execute_command(&mut self) {
        match &self.command_items[self.selected_index].command_type {
            CommandType::Simple(cmd) => {
                self.execute_simple_command(cmd.clone());
            }
            CommandType::RequiresInput(input_type) => {
                self.start_input_mode(input_type.clone());
            }
        }
    }

    fn execute_simple_command(&mut self, command: Commands) {
        match command {
            Commands::Help => self.show_help(),
            Commands::Clear => self.output_lines.clear(),
            Commands::Exit => self.should_quit = true,
            Commands::RegtestStart => {
                self.output_lines.push("Starting regtest node...".to_string());
                match self.regtest_manager.start() {
                    Ok(_) => self.output_lines.push("✅ Regtest node started successfully".to_string()),
                    Err(e) => self.output_lines.push(format!("❌ Error starting regtest: {}", e)),
                }
            }
            Commands::RegtestStop => {
                self.output_lines.push("Stopping regtest node...".to_string());
                match self.regtest_manager.stop() {
                    Ok(_) => self.output_lines.push("✅ Regtest node stopped successfully".to_string()),
                    Err(e) => self.output_lines.push(format!("❌ Error stopping regtest: {}", e)),
                }
            }
            _ => {}
        }
    }

    fn start_input_mode(&mut self, input_type: InputType) {
        match input_type {
            InputType::BreakTransaction | InputType::BreakBlock => {
                self.input_mode = InputMode::Input;
                self.awaiting_input = Some(input_type.clone());
                self.input_buffer.clear();
                
                let prompt = match input_type {
                    InputType::BreakTransaction => "Enter raw transaction hex:",
                    InputType::BreakBlock => "Enter block header hex:",
                    _ => unreachable!(),
                };
                
                self.output_lines.push(format!("📝 {}", prompt));
                self.output_lines.push("Enter the hex data, then you'll select fields to break.".to_string());
            }
            _ => {
                self.input_mode = InputMode::Input;
                self.awaiting_input = Some(input_type.clone());
                self.input_buffer.clear();
                
                let prompt = match input_type {
                    InputType::DecodeTransaction => "Enter raw transaction hex:",
                    InputType::DecodeBlock => "Enter block header hex:",
                    InputType::GenerateTx => "Enter number of transactions to generate:",
                    InputType::GenerateBlock => "Enter number of transactions for the block:",
                    InputType::GetBlockByHeight => "Enter block height:",
                    _ => unreachable!(),
                };
                
                self.output_lines.push(format!("📝 {}", prompt));
                self.output_lines.push("Type your input and press Enter, or press Esc to cancel.".to_string());
            }
        }
    }

    fn process_input(&mut self) {
        if let Some(input_type) = self.awaiting_input.take() {
            let input = self.input_buffer.trim().to_string();
            self.input_buffer.clear();
            
            if input.is_empty() {
                self.input_mode = InputMode::Normal;
                self.output_lines.push("❌ Input cancelled or empty".to_string());
                return;
            }
            
            match input_type {
                InputType::DecodeTransaction => {
                    self.input_mode = InputMode::Normal;
                    self.decode_transaction(input);
                }
                InputType::DecodeBlock => {
                    self.input_mode = InputMode::Normal;
                    self.decode_block(input);
                }
                InputType::BreakTransaction => {
                    self.pending_hex_input = input;
                    self.field_items = Self::get_transaction_fields();
                    self.field_selected_index = 0;
                    self.subwindow_mode = Some(SubwindowMode::TransactionFields);
                    self.input_mode = InputMode::Subwindow;
                    self.output_lines.push("🔧 Select fields to break (Space to toggle, Enter to confirm):".to_string());
                }
                InputType::BreakBlock => {
                    self.pending_hex_input = input;
                    self.field_items = Self::get_block_fields();
                    self.field_selected_index = 0;
                    self.subwindow_mode = Some(SubwindowMode::BlockFields);
                    self.input_mode = InputMode::Subwindow;
                    self.output_lines.push("🔧 Select fields to break (Space to toggle, Enter to confirm):".to_string());
                }
                InputType::GenerateTx => {
                    self.input_mode = InputMode::Normal;
                    self.generate_tx_interactive(input);
                }
                InputType::GenerateBlock => {
                    self.input_mode = InputMode::Normal;
                    self.generate_block_interactive(input);
                }
                InputType::GetBlockByHeight => {
                    self.input_mode = InputMode::Normal;
                    self.get_block_by_height_interactive(input);
                }
            }
        }
    }

    fn process_field_selection(&mut self) {
        let selected_flags: Vec<String> = self.field_items.iter()
            .filter(|item| item.selected)
            .map(|item| item.flag.clone())
            .collect();
        
        if selected_flags.is_empty() {
            self.output_lines.push("❌ No fields selected".to_string());
            self.close_subwindow();
            return;
        }
        
        match self.subwindow_mode.clone() {
            Some(SubwindowMode::TransactionFields) => {
                self.output_lines.push("🔨 Breaking transaction with selected fields...".to_string());
                let result = Generator::break_transaction(self.pending_hex_input.clone(), selected_flags);
                self.output_lines.push("🔨 Transaction Breaking Result:".to_string());
                self.output_lines.push(result);
            }
            Some(SubwindowMode::BlockFields) => {
                self.output_lines.push("🔨 Breaking block with selected fields...".to_string());
                let result = Generator::break_block(self.pending_hex_input.clone(), selected_flags, Vec::new());
                self.output_lines.push("🔨 Block Breaking Result:".to_string());
                self.output_lines.push(result);
            }
            None => {}
        }
        
        self.close_subwindow();
    }

    fn close_subwindow(&mut self) {
        self.subwindow_mode = None;
        self.field_items.clear();
        self.field_selected_index = 0;
        self.pending_hex_input.clear();
        self.input_mode = InputMode::Normal;
    }

    fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
        self.awaiting_input = None;
        self.input_buffer.clear();
        self.close_subwindow();
        self.output_lines.push("❌ Input cancelled".to_string());
    }

    fn decode_transaction(&mut self, raw_transaction: String) {
        self.output_lines.push(format!("🔍 Decoding transaction: {}", &raw_transaction[..std::cmp::min(20, raw_transaction.len())]));
        match Generator::decode_raw_transaction(raw_transaction) {
            Ok(decoded) => {
                self.output_lines.push("✅ Transaction decoded successfully:".to_string());
                self.output_lines.push(format!("  Version: {}", decoded.version));
                self.output_lines.push(format!("  Locktime: {}", decoded.lock_time));
                self.output_lines.push(format!("  Input count: {:#?}", decoded.input));
                self.output_lines.push(format!("  Output count: {:#?}", decoded.output));
            },
            Err(e) => {
                self.output_lines.push(format!("❌ Error decoding transaction: {}", e));
            }
        }
    }

    fn decode_block(&mut self, block_header: String) {
        self.output_lines.push(format!("🔍 Decoding block header: {}", &block_header[..std::cmp::min(20, block_header.len())]));
        match Generator::decoder_block_header(block_header) {
            Ok(header) => {
                self.output_lines.push("✅ Block header decoded successfully:".to_string());
                self.output_lines.push(format!("  Version: {}", header.version.to_consensus()));
                self.output_lines.push(format!("  Previous Block: {}", header.prev_blockhash));
                self.output_lines.push(format!("  Merkle Root: {}", header.merkle_root));
                self.output_lines.push(format!("  Timestamp: {}", header.time));
                self.output_lines.push(format!("  Bits: 0x{:08x}", header.bits.to_consensus()));
                self.output_lines.push(format!("  Nonce: {}", header.nonce));
                self.output_lines.push(format!("  Block Hash: {}", header.block_hash()));
            },
            Err(e) => {
                self.output_lines.push(format!("❌ Error decoding block header: {}", e));
            }
        }
    }

    fn generate_tx_interactive(&mut self, input: String) {
        match input.parse::<u32>() {
            Ok(count) => {
                self.output_lines.push(format!("🏗️ Generating {} transaction(s)...", count));
                let transactions = Generator::transaction(count);
                self.output_lines.push("✅ Transactions generated:".to_string());
                self.output_lines.push(transactions);
            }
            Err(_) => {
                self.output_lines.push("❌ Invalid number format".to_string());
            }
        }
    }

    fn generate_block_interactive(&mut self, input: String) {
        match input.parse::<u32>() {
            Ok(count) => {
                self.output_lines.push(format!("🏗️ Generating block with {} transaction(s)...", count));
                let block = Generator::block(count);
                self.output_lines.push("✅ Block generated:".to_string());
                self.output_lines.push(block);
            }
            Err(_) => {
                self.output_lines.push("❌ Invalid number format".to_string());
            }
        }
    }

    fn get_block_by_height_interactive(&mut self, input: String) {
        match input.parse::<u64>() {
            Ok(height) => {
                self.output_lines.push(format!("🔍 Getting block at height {}...", height));
                match self.regtest_manager.handle_getblockbyheight(height) {
                    Ok(_) => self.output_lines.push("✅ Block retrieved successfully".to_string()),
                    Err(e) => self.output_lines.push(format!("❌ Error getting block: {}", e)),
                }
            }
            Err(_) => {
                self.output_lines.push("❌ Invalid height format".to_string());
            }
        }
    }

    fn show_help(&mut self) {
        self.output_lines.push("📖 Bitcoin CLI Tool Help".to_string());
        self.output_lines.push("".to_string());
        self.output_lines.push("🔧 Utils:".to_string());
        self.output_lines.push("  help (h)                    - Show this help message".to_string());
        self.output_lines.push("  clear (c)                   - Clear terminal output".to_string());
        self.output_lines.push("  exit (q)                    - Exit the application".to_string());
        self.output_lines.push("".to_string());
        self.output_lines.push("🔍 Decode:".to_string());
        self.output_lines.push("  decode-transaction (d)      - Decode a raw transaction".to_string());
        self.output_lines.push("  decode-block (D)            - Decode a block header".to_string());
        self.output_lines.push("".to_string());
        self.output_lines.push("🔨 Break/Invalidate:".to_string());
        self.output_lines.push("  break-transaction (b)       - Break/invalidate transaction fields".to_string());
        self.output_lines.push("  break-block (B)             - Break/invalidate block fields".to_string());
        self.output_lines.push("".to_string());
        self.output_lines.push("🏗️ Generate:".to_string());
        self.output_lines.push("  generate-transaction (g)    - Generate one or more transactions".to_string());
        self.output_lines.push("  generate-block (G)          - Generate new block with transactions".to_string());
        self.output_lines.push("".to_string());
        self.output_lines.push("⚙️ Regtest:".to_string());
        self.output_lines.push("  start-regtest (r)           - Start the regtest node".to_string());
        self.output_lines.push("  stop-regtest (s)            - Stop the regtest node".to_string());
        self.output_lines.push("  get-block-by-height (k)     - Get block at specific height".to_string());
        self.output_lines.push("".to_string());
        self.output_lines.push("Navigation: ↑/↓ arrows, Enter to select, Tab: Output select, y: Copy output, q: Quit".to_string());
    }
}

pub fn handle() -> io::Result<()> {
    // Terminal initialization
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run main loop
    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app);

    // Cleanup before exit
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    
    if result.is_ok() {
        println!("Program finalized 👋");
    }
    
    result
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    use copypasta::{ClipboardContext, ClipboardProvider};
    let mut clipboard = ClipboardContext::new().unwrap();

    loop {
        // Generate expanded lines for navigation/copying
        let expanded_lines: Vec<String> = app.output_lines.iter()
            .flat_map(|line| json_to_lines(line))
            .collect();
        
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match app.input_mode {
                        InputMode::Normal => {
                            match key.code {
                                // Navigation
                                KeyCode::Up => {
                                    if app.selected_index > 0 {
                                        app.selected_index -= 1;
                                    }
                                }
                                KeyCode::Down => {
                                    if app.selected_index < app.command_items.len() - 1 {
                                        app.selected_index += 1;
                                    }
                                }
                                // Execute command on Enter
                                KeyCode::Enter => app.execute_command(),
                                // Keyboard shortcuts for commands
                                KeyCode::Char(c) => {
                                    if let Some(idx) = app.command_items.iter().position(|item| item.shortcut == Some(c)) {
                                        app.selected_index = idx;
                                        app.execute_command();
                                    } else if c == 'q' {
                                        app.should_quit = true;
                                    }
                                }
                                // Switch to output selection
                                KeyCode::Tab => {
                                    app.input_mode = InputMode::OutputSelect;
                                    app.output_selected_index = Some(app.output_lines.len().saturating_sub(1));
                                }
                                _ => {}
                            }
                        }
                        InputMode::Input => {
                            match key.code {
                                KeyCode::Enter => app.process_input(),
                                KeyCode::Esc => app.cancel_input(),
                                KeyCode::Backspace => {
                                    app.input_buffer.pop();
                                }
                                KeyCode::Char(c) => {
                                    app.input_buffer.push(c);
                                }
                                _ => {}
                            }
                        }
                        InputMode::OutputSelect => {
                            match key.code {
                                KeyCode::Up => {
                                    if let Some(idx) = app.output_selected_index {
                                        if idx > 0 {
                                            app.output_selected_index = Some(idx - 1);
                                        }
                                    }
                                }
                                KeyCode::Down => {
                                    if let Some(idx) = app.output_selected_index {
                                        if idx + 1 < expanded_lines.len() {
                                            app.output_selected_index = Some(idx + 1);
                                        }
                                    }
                                }
                                KeyCode::Char('y') => {
                                    if let Some(idx) = app.output_selected_index {
                                        if let Some(line) = expanded_lines.get(idx) {
                                            let _ = clipboard.set_contents(line.clone());
                                            app.output_lines.push("📋 Linha copiada para o clipboard!".to_string());
                                        }
                                    }
                                }
                                KeyCode::Tab | KeyCode::Esc => {
                                    app.input_mode = InputMode::Normal;
                                    app.output_selected_index = None;
                                }
                                _ => {}
                            }
                        },
                        InputMode::Subwindow => {
                            match key.code {
                                KeyCode::Up => {
                                    if app.field_selected_index > 0 {
                                        app.field_selected_index -= 1;
                                    }
                                }
                                KeyCode::Down => {
                                    if app.field_selected_index < app.field_items.len() - 1 {
                                        app.field_selected_index += 1;
                                    }
                                }
                                KeyCode::Char(' ') => {
                                    if let Some(item) = app.field_items.get_mut(app.field_selected_index) {
                                        item.selected = !item.selected;
                                    }
                                }
                                KeyCode::Enter => app.process_field_selection(),
                                KeyCode::Esc => app.close_subwindow(),
                                _ => {}
                            }
                        }
                    
                    
                    }

                }
            }
        }
        terminal.draw(|f| ui(f, app))?;

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(0)].as_ref())
        .split(f.area());

    // Render commands panel (left)
    let items: Vec<ListItem> = app
        .command_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.selected_index {
                Style::default().fg(Color::Black).bg(Color::LightBlue)
            } else {
                Style::default()
            };
            ListItem::new(Text::styled(&item.name, style))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("🔧 Commands"))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::LightBlue));
    f.render_widget(list, main_chunks[0]);

    // Prepare right panel based on mode
    let right_panel = main_chunks[1];
    let right_chunks = match app.input_mode {
        InputMode::Subwindow => {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(app.field_items.len() as u16 + 2), // Subwindow height
                    Constraint::Min(0) // Output area
                ])
                .split(right_panel)
        }
        InputMode::Input => {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(right_panel)
        }
        _ => {
            vec![right_panel].into()
        }
    };

    // Render subwindow if active
    if app.input_mode == InputMode::Subwindow {
        let subwindow_chunk = right_chunks[0];
        let title = match app.subwindow_mode {
            Some(SubwindowMode::TransactionFields) => "🔧 Select Transaction Fields to Break",
            Some(SubwindowMode::BlockFields) => "🔧 Select Block Fields to Break",
            None => "Field Selection",
        };

        let field_items: Vec<ListItem> = app
            .field_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let prefix = if item.selected { "[✓] " } else { "[ ] " };
                let content = format!("{}{}: {}", prefix, item.name, item.description);
                
                let style = if i == app.field_selected_index {
                    Style::default().fg(Color::Black).bg(Color::LightBlue)
                } else {
                    Style::default()
                };
                
                ListItem::new(Text::styled(content, style))
            })
            .collect();

        let field_list = List::new(field_items)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(Style::default().fg(Color::Black).bg(Color::LightBlue));
        
        f.render_widget(field_list, subwindow_chunk);
    }

    // Render output area
    let output_chunk = match app.input_mode {
        InputMode::Subwindow => right_chunks[1],
        InputMode::Input => right_chunks[0],
        _ => right_chunks[0],
    };

    let expanded_lines: Vec<String> = app.output_lines.iter()
        .flat_map(|line| json_to_lines(line))
        .collect();

    let output_lines: Vec<Line> = expanded_lines.iter().enumerate().map(|(idx, line)| {
        if app.input_mode == InputMode::OutputSelect && app.output_selected_index == Some(idx) {
            Line::from(vec![Span::styled(line, Style::default().bg(Color::LightYellow).fg(Color::Black))])
        } else {
            Line::from(vec![Span::raw(line)])
        }
    }).collect();

    let output = Paragraph::new(output_lines)
        .block(Block::default().borders(Borders::ALL).title("📤 Output"))
        .wrap(Wrap { trim: true })
        .scroll((app.output_selected_index.unwrap_or(0) as u16, 0));

    f.render_widget(output, output_chunk);

    // Render input area if needed
    if app.input_mode == InputMode::Input {
        let input_chunk = right_chunks[1];
        let input = Paragraph::new(app.input_buffer.as_str())
            .block(Block::default().borders(Borders::ALL).title("📝 Input"))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(input, input_chunk);
    }

    // Show status bar
    let status_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
        .split(f.area())[1];
    
    let status_text = match app.input_mode {
        InputMode::Normal => {
            if !app.command_items.is_empty() && app.selected_index < app.command_items.len() {
                format!("{} | ↑/↓: Navigate, Enter: Select, Tab: Output, q: Quit", 
                        app.command_items[app.selected_index].description)
            } else {
                "Ready".to_string()
            }
        }
        InputMode::Input => "INPUT MODE - Enter to confirm, Esc to cancel".to_string(),
        InputMode::OutputSelect => "OUTPUT SELECT - ↑/↓: Navigate, y: Copy line, Tab/Esc: Back".to_string(),
        InputMode::Subwindow => "FIELD SELECTION - ↑/↓: Navigate, Space: Toggle, Enter: Confirm, Esc: Cancel".to_string(),
    };
    
    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(status, status_chunk);
}

fn json_to_lines(json_str: &str) -> Vec<String> {
    let mut lines = Vec::new();
    if let Ok(json) = serde_json::from_str::<Value>(json_str) {
        render_json_value(&json, 0, &mut lines);
    } else {
        lines.push(json_str.to_string());
    }
    lines
}

fn render_json_value(value: &Value, indent: usize, lines: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                if v.is_object() || v.is_array() {
                    lines.push(format!("{}{}: ",  "  ".repeat(indent), k));
                    render_json_value(v, indent + 1, lines);
                } else {
                    lines.push(format!("{}{}: {}", "  ".repeat(indent), k, value_to_string_no_quotes(v)));
                }
            }
        }
        Value::Array(arr) => {
            for v in arr {
                render_json_value(v, indent, lines);
            }
        }
        _ => {
            lines.push(format!("{}{}", "  ".repeat(indent), value_to_string_no_quotes(value)));
        }
    }
}

fn value_to_string_no_quotes(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        _ => format!("{}", value),
    }
}