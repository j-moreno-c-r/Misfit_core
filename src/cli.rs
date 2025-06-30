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
};
use clap::{Parser, Subcommand};
use crate::api::Generator;

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
    DecodeTransaction {
        raw_transaction: String
    },
    #[command(name = "decode-block")]
    DecodeBlock {
        block_header: String
    },
    #[command(name = "break-transaction")]
    BreakTransaction {
        raw_transaction: String,
        #[arg(long, help = "Invalidate transaction version")]
        version: bool,
        #[arg(long, help = "Invalidate input transaction ID")]
        txid: bool,
        #[arg(long, help = "Invalidate input vout")]
        vout: bool,
        #[arg(long = "script-sig", help = "Invalidate input script signature")]
        script_sig: bool,
        #[arg(long, help = "Invalidate input sequence number")]
        sequence: bool,
        #[arg(long, help = "Invalidate output amount")]
        amount: bool,
        #[arg(long = "script-pubkey", help = "Invalidate output script pubkey")]
        script_pubkey: bool,
        #[arg(long, help = "Invalidate witness data")]
        witness: bool,
        #[arg(long, help = "Invalidate transaction locktime")]
        locktime: bool,
        #[arg(long, help = "Invalidate all transaction fields")]
        all: bool,
    },
    #[command(name = "break-block")]
    BreakBlock {
        block_header: String,
        #[arg(long, help = "Invalidate block version")]
        version: bool,
        #[arg(long = "prev-hash", help = "Invalidate previous block hash")]
        prev_hash: bool,
        #[arg(long = "merkle-root", help = "Invalidate merkle root")]
        merkle_root: bool,
        #[arg(long, help = "Invalidate timestamp")]
        timestamp: bool,
        #[arg(long, help = "Invalidate difficulty bits")]
        bits: bool,
        #[arg(long, help = "Invalidate nonce")]
        nonce: bool,
        #[arg(long, help = "Invalidate all block fields")]
        all: bool,
        #[arg(long, help = "Override version with specific value")]
        version_override: Option<i32>,
        #[arg(long, help = "Add/subtract seconds to timestamp")]
        timestamp_offset: Option<i64>,
        #[arg(long, help = "Use zero hashes instead of random")]
        zero_hashes: bool,
    },
    Tx {
        #[arg(default_value_t = 1)]
        txscount: u32,
        campuses: Vec<String>,
    },
    Block {
        #[arg(default_value_t = 1)]
        txscount: u32,
    },
    #[command(name = "regtest-start")]
    RegtestStart,
    #[command(name = "regtest-stop")]
    RegtestStop,
    #[command(name = "get-blockby-height")]
    GetBlockbyHeight {
        height: u64,
    },
}

// App state structure
struct App {
    command_items: Vec<CommandItem>,
    selected_index: usize,
    output_lines: Vec<String>,
    should_quit: bool,
    input_mode: InputMode,
    input_buffer: String,
    awaiting_input: Option<InputType>,
    regtest_manager: RegtestManager,
}

#[derive(Clone)]
struct CommandItem {
    name: String,
    description: String,
    command_type: CommandType,
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

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Input,
}

impl App {
    fn new() -> Self {
        let regtest_manager = Generator::regtest_invocation("bitcoinhos", "-regtest");
        
        App {
            command_items: vec![
                CommandItem {
                    name: "Help".to_string(),
                    description: "Show help message".to_string(),
                    command_type: CommandType::Simple(Commands::Help),
                },
                CommandItem {
                    name: "Clear".to_string(),
                    description: "Clear output".to_string(),
                    command_type: CommandType::Simple(Commands::Clear),
                },
                CommandItem {
                    name: "Decode Transaction".to_string(),
                    description: "Decode a raw transaction".to_string(),
                    command_type: CommandType::RequiresInput(InputType::DecodeTransaction),
                },
                CommandItem {
                    name: "Decode Block".to_string(),
                    description: "Decode a block header".to_string(),
                    command_type: CommandType::RequiresInput(InputType::DecodeBlock),
                },
                CommandItem {
                    name: "Break Transaction".to_string(),
                    description: "Break/invalidate transaction fields".to_string(),
                    command_type: CommandType::RequiresInput(InputType::BreakTransaction),
                },
                CommandItem {
                    name: "Break Block".to_string(),
                    description: "Break/invalidate block fields".to_string(),
                    command_type: CommandType::RequiresInput(InputType::BreakBlock),
                },
                CommandItem {
                    name: "Generate Transaction".to_string(),
                    description: "Generate one or more transactions".to_string(),
                    command_type: CommandType::RequiresInput(InputType::GenerateTx),
                },
                CommandItem {
                    name: "Generate Block".to_string(),
                    description: "Generate new block with transactions".to_string(),
                    command_type: CommandType::RequiresInput(InputType::GenerateBlock),
                },
                CommandItem {
                    name: "Get Block by Height".to_string(),
                    description: "Get block at specific height".to_string(),
                    command_type: CommandType::RequiresInput(InputType::GetBlockByHeight),
                },
                CommandItem {
                    name: "Start Regtest".to_string(),
                    description: "Start the regtest node".to_string(),
                    command_type: CommandType::Simple(Commands::RegtestStart),
                },
                CommandItem {
                    name: "Stop Regtest".to_string(),
                    description: "Stop the regtest node".to_string(),
                    command_type: CommandType::Simple(Commands::RegtestStop),
                },
                CommandItem {
                    name: "Exit".to_string(),
                    description: "Exit the application".to_string(),
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
        }
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
        self.input_mode = InputMode::Input;
        self.awaiting_input = Some(input_type.clone());
        self.input_buffer.clear();
        
        let prompt = match input_type {
            InputType::DecodeTransaction => "Enter raw transaction hex:",
            InputType::DecodeBlock => "Enter block header hex:",
            InputType::BreakTransaction => "Enter raw transaction hex (you can add flags like --version --all):",
            InputType::BreakBlock => "Enter block header hex (you can add flags like --version --all):",
            InputType::GenerateTx => "Enter number of transactions to generate:",
            InputType::GenerateBlock => "Enter number of transactions for the block:",
            InputType::GetBlockByHeight => "Enter block height:",
        };
        
        self.output_lines.push(format!("📝 {}", prompt));
        self.output_lines.push("Type your input and press Enter, or press Esc to cancel.".to_string());
    }

    fn process_input(&mut self) {
        if let Some(input_type) = self.awaiting_input.take() {
            let input = self.input_buffer.trim().to_string();
            self.input_buffer.clear();
            self.input_mode = InputMode::Normal;
            
            if input.is_empty() {
                self.output_lines.push("❌ Input cancelled or empty".to_string());
                return;
            }
            
            match input_type {
                InputType::DecodeTransaction => self.decode_transaction(input),
                InputType::DecodeBlock => self.decode_block(input),
                InputType::BreakTransaction => self.break_transaction_interactive(input),
                InputType::BreakBlock => self.break_block_interactive(input),
                InputType::GenerateTx => self.generate_tx_interactive(input),
                InputType::GenerateBlock => self.generate_block_interactive(input),
                InputType::GetBlockByHeight => self.get_block_by_height_interactive(input),
            }
        }
    }

    fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
        self.awaiting_input = None;
        self.input_buffer.clear();
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

    fn break_transaction_interactive(&mut self, input: String) {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            self.output_lines.push("❌ No transaction provided".to_string());
            return;
        }
        
        let raw_transaction = parts[0].to_string();
        let flags: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
        
        if flags.is_empty() {
            self.output_lines.push("❌ No flags provided. Available flags: --version, --txid, --vout, --script-sig, --sequence, --amount, --script-pubkey, --witness, --locktime, --all".to_string());
            return;
        }
        
        self.output_lines.push("🔨 Breaking transaction...".to_string());
        let result = Generator::break_transaction(raw_transaction, flags);
        self.output_lines.push("🔨 Transaction Breaking Result:".to_string());
        self.output_lines.push(result);
    }

    fn break_block_interactive(&mut self, input: String) {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            self.output_lines.push("❌ No block header provided".to_string());
            return;
        }
        
        let block_header = parts[0].to_string();
        let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
        
        if args.is_empty() {
            self.output_lines.push("❌ No flags provided. Available flags: --version, --prev-hash, --merkle-root, --timestamp, --bits, --nonce, --all".to_string());
            return;
        }
        
        // Separate flags from config options
        let mut flags = Vec::new();
        let mut config = Vec::new();
        
        for arg in args {
            if arg.starts_with("--version-override=") || arg.starts_with("--timestamp-offset=") || arg == "--zero-hashes" {
                config.push(arg);
            } else {
                flags.push(arg);
            }
        }
        
        self.output_lines.push("🔨 Breaking block...".to_string());
        let result = Generator::break_block(block_header, flags, config);
        self.output_lines.push("🔨 Block Breaking Result:".to_string());
        self.output_lines.push(result);
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
        self.output_lines.push("  help                    - Show this help message".to_string());
        self.output_lines.push("  clear                   - Clear terminal output".to_string());
        self.output_lines.push("  exit                    - Exit the application".to_string());
        self.output_lines.push("".to_string());
        self.output_lines.push("🔍 Decode:".to_string());
        self.output_lines.push("  decode-transaction      - Decode a raw transaction".to_string());
        self.output_lines.push("  decode-block           - Decode a block header".to_string());
        self.output_lines.push("".to_string());
        self.output_lines.push("🔨 Break/Invalidate:".to_string());
        self.output_lines.push("  break-transaction      - Break/invalidate transaction fields".to_string());
        self.output_lines.push("  break-block           - Break/invalidate block fields".to_string());
        self.output_lines.push("".to_string());
        self.output_lines.push("🏗️ Generate:".to_string());
        self.output_lines.push("  generate-transaction   - Generate one or more transactions".to_string());
        self.output_lines.push("  generate-block        - Generate new block with transactions".to_string());
        self.output_lines.push("".to_string());
        self.output_lines.push("⚙️ Regtest:".to_string());
        self.output_lines.push("  start-regtest         - Start the regtest node".to_string());
        self.output_lines.push("  stop-regtest          - Stop the regtest node".to_string());
        self.output_lines.push("  get-block-by-height   - Get block at specific height".to_string());
        self.output_lines.push("".to_string());
        self.output_lines.push("Navigation: ↑/↓ arrows, Enter to select, 'q' to quit".to_string());
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
    loop {
        // Handle input events
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
                                // Quit application
                                KeyCode::Char('q') => app.should_quit = true,
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
                    }
                }
            }
        }

        // Render UI
        terminal.draw(|f| ui(f, app))?;

        // Check exit condition
        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    // Split window into two vertical areas
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(0)].as_ref())
        .split(f.size());

    // Left panel - Commands
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
    f.render_widget(list, chunks[0]);

    // Right panel - Output and input
    let right_chunks = if app.input_mode == InputMode::Input {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(chunks[1])
    } else {
        vec![chunks[1]].into() // Corrige o tipo retornado
    };

    // Output area
    let output_chunk = right_chunks[0];
    let output = Paragraph::new(app.output_lines.join("\n"))
        .block(Block::default().borders(Borders::ALL).title("📤 Output"))
        .wrap(Wrap { trim: true })
        .scroll((app.output_lines.len().saturating_sub(1) as u16, 0));
    f.render_widget(output, output_chunk);

    // Input area (only visible in input mode)
    if app.input_mode == InputMode::Input && right_chunks.len() > 1 {
        let input_chunk = right_chunks[1];
        let input = Paragraph::new(app.input_buffer.as_str())
            .block(Block::default().borders(Borders::ALL).title("📝 Input"))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(input, input_chunk);
        
        // Set cursor position
        f.set_cursor(
            input_chunk.x + app.input_buffer.len() as u16 + 1,
            input_chunk.y + 1,
        );
    }

    // Show command description in the bottom
    if !app.command_items.is_empty() && app.selected_index < app.command_items.len() {
        let description = &app.command_items[app.selected_index].description;
        let status_text = if app.input_mode == InputMode::Input {
            "INPUT MODE - Enter to confirm, Esc to cancel"
        } else {
            &format!("{} | ↑/↓: Navigate, Enter: Select, q: Quit", description)
        };
        
        let status_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
            .split(f.size())[1];
            
        let status = Paragraph::new(status_text)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(status, status_chunk);
    }
}