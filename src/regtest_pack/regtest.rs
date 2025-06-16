use std::{error::Error, process::Command, thread, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};
use serde_json;

const SPINNER_TICKS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub struct RegtestManager {
    wallet_name: String,
    regtest_arg: String,
    max_start_attempts: u8,
}

impl RegtestManager {
    pub fn new(wallet_name: &str, regtest_arg: &str) -> Self {
        Self {
            wallet_name: wallet_name.to_string(),
            regtest_arg: regtest_arg.to_string(),
            max_start_attempts: 15,
        }
    }

    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        let output = Command::new("bitcoind")
            .arg(&self.regtest_arg)
            .arg("-daemon")
            .output()?;

        if !output.status.success() {
            return Err("Failed to start bitcoind regtest node. Is Bitcoin Core installed?".into());
        }

        let pb = self.create_spinner_progress_bar("Starting regtest node");
        let mut success = false;

        for attempt in 1..=self.max_start_attempts {
            pb.set_message(format!("Attempt {}/{}", attempt, self.max_start_attempts));

            let status = Command::new("bitcoin-cli")
                .arg(&self.regtest_arg)
                .arg("getblockchaininfo")
                .status();

            if let Ok(status) = status {
                if status.success() {
                    success = true;
                    break;
                }
            }

            thread::sleep(Duration::from_secs(1));
        }

        pb.finish_and_clear();

        if success {
            self.ensure_wallet()?;
            println!("Regtest node started successfully ✅");
            Ok(())
        } else {
            Err("Failed to start node within timeout period".into())
        }
    }

    pub fn stop(&self) -> Result<(), Box<dyn Error>> {
        let output = Command::new("bitcoin-cli")
            .arg(&self.regtest_arg)
            .arg("stop")
            .output()?;

        if !output.status.success() {
            return Err("Failed to stop regtest node. Is it running?".into());
        }

        println!("Regtest node stopped successfully");
        Ok(())
    }

    pub fn handle_getblockbyheight(&self, target_height: u64) -> Result<(), Box<dyn Error>> {
        let current_height = self.get_current_height()?;
        println!("Current blockchain height: {}", current_height);

        if target_height < current_height {
            println!(
                "Block at height {} already exists. Retrieving...",
                target_height
            );
        } else if target_height > current_height {
            let blocks_needed = target_height - current_height;
            let address = self.generate_address()?;
            println!(
                "Generating {} blocks to reach height {}...",
                blocks_needed, target_height
            );
            self.mine_blocks(&address, blocks_needed)?;
            println!("Successfully generated {} blocks", blocks_needed);
        }

        self.display_block(target_height)
    }

    fn ensure_wallet(&self) -> Result<(), Box<dyn Error>> {
        let load_output = Command::new("bitcoin-cli")
            .arg(&self.regtest_arg)
            .args(["loadwallet", &self.wallet_name])
            .output()?;

        if load_output.status.success() {
            return Ok(());
        }

        let create_output = Command::new("bitcoin-cli")
            .arg(&self.regtest_arg)
            .args(["createwallet", &self.wallet_name])
            .output()?;

        if !create_output.status.success() {
            return Err(format!(
                "Failed to create wallet: {}",
                String::from_utf8_lossy(&create_output.stderr)
            )
            .into());
        }

        Ok(())
    }

    fn generate_address(&self) -> Result<String, Box<dyn Error>> {
        let output = Command::new("bitcoin-cli")
            .arg(&self.regtest_arg)
            .arg("getnewaddress")
            .output()?;

        if !output.status.success() {
            return Err(format!(
                "Address generation failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    fn mine_blocks(&self, address: &str, count: u64) -> Result<(), Box<dyn Error>> {
        let output = Command::new("bitcoin-cli")
            .arg(&self.regtest_arg)
            .args(["generatetoaddress", &count.to_string(), address])
            .output()?;

        if !output.status.success() {
            return Err(
                format!("Mining failed: {}", String::from_utf8_lossy(&output.stderr)).into(),
            );
        }

        Ok(())
    }

    fn get_current_height(&self) -> Result<u64, Box<dyn Error>> {
        let output = Command::new("bitcoin-cli")
            .arg(&self.regtest_arg)
            .arg("getblockchaininfo")
            .output()?;

        if !output.status.success() {
            return Err("Failed to get blockchain info".into());
        }

        let json_str = String::from_utf8(output.stdout)?;
        let json: serde_json::Value = serde_json::from_str(&json_str)?;
        let height = json["blocks"]
            .as_u64()
            .ok_or("Missing or invalid 'blocks' field in blockchain info")?;

        Ok(height)
    }

    fn display_block(&self, height: u64) -> Result<(), Box<dyn Error>> {
        let output = Command::new("bitcoin-cli")
            .arg(&self.regtest_arg)
            .args(["getblockhash", &height.to_string()])
            .output()?;

        if !output.status.success() {
            return Err("Failed to get block hash for the target height".into());
        }

        let block_hash = String::from_utf8(output.stdout)?.trim().to_string();

        let block_output = Command::new("bitcoin-cli")
            .arg(&self.regtest_arg)
            .args(["getblock", &block_hash, "2"])
            .output()?;

        if !block_output.status.success() {
            return Err("Failed to retrieve block data".into());
        }

        let block_data = String::from_utf8(block_output.stdout)?;
        println!("Block at height {}:\n{}", height, block_data);

        Ok(())
    }

    fn create_spinner_progress_bar(&self, msg: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(SPINNER_TICKS)
                .template("{spinner} {msg}")
                .expect("Invalid spinner template"),
        );
        pb.set_message(msg.to_string());
        pb
    }
}
