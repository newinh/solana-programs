mod pb;
mod utils;

use std::collections::HashMap;
use std::collections::HashSet;
use pb::sf::solana::transactions::v1::Output;
use pb::sf::solana::transactions::v1::TransactionStats;
use substreams::log;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;

use substreams_solana::pb::sf::solana::r#type::v1::TransactionStatusMeta;
use utils::convert_to_date;
use utils::parse_logs;
use utils::LogContext;

#[derive(Debug, Clone)]
struct ProgramData {
    signers: HashSet<String>,
    fee_payers: HashSet<String>,
    fee_lamports: u32,
    base_fee_lamports: u32,
    priority_fee_lamports: u32,
    compute_units_consumed: u32,
    compute_units_allocated: u32,
    successful_txns_count: u32,
    failed_txns_count: u32,
    outer_invocation_count: u32,
    inner_invocation_count: u32,
    failed_invocation_count: u32,
    errors: HashMap<String, u32>,
}

impl ProgramData {
    fn new() -> Self {
        ProgramData {
            signers: HashSet::new(),
            fee_payers: HashSet::new(),
            fee_lamports: 0,
            base_fee_lamports: 0,
            priority_fee_lamports: 0,
            compute_units_consumed: 0,
            compute_units_allocated: 0,
            successful_txns_count: 0,
            failed_txns_count: 0,
            outer_invocation_count: 0,
            inner_invocation_count: 0,
            failed_invocation_count: 0,
            errors: HashMap::new(),
        }
    }

    fn update_with_log(&mut self, log: &LogContext, depth: usize) {
        self.compute_units_consumed += log.consumed_units as u32;
        self.compute_units_allocated += log.compute_units as u32;

        if depth == 1 {
            self.outer_invocation_count += 1;
        } else {
            self.inner_invocation_count += 1;
        }

        if let Some(ref error) = log.failure_message {
            self.failed_invocation_count += 1;
            *self.errors.entry(error.clone()).or_insert(0) += 1;
        }
    }

    fn new_from_log(log: &LogContext, depth: usize) -> Self {
        let mut new_data = ProgramData::new();

        new_data.compute_units_consumed = log.consumed_units as u32;
        new_data.compute_units_allocated = log.compute_units as u32;

        new_data.outer_invocation_count = if depth == 1 { 1 } else { 0 };
        new_data.inner_invocation_count = if depth != 1 { 1 } else { 0 };

        new_data.failed_invocation_count = if log.failure_message.is_some() { 1 } else { 0 };

        if let Some(ref error) = log.failure_message {
            new_data.errors.insert(error.clone(), 1);
        }

        new_data
    }

    fn update_fees_and_counts(
        &mut self,
        accounts: &[String],
        num_required_signatures: u32,
        fees: u64,
        meta: &TransactionStatusMeta,
    ) {
        self.fee_payers.insert(accounts[0].clone());

        if let Some(signers) = accounts.get(0..num_required_signatures as usize) {
            self.signers.extend(signers.iter().cloned());
        }

        self.fee_lamports = fees as u32;
        self.base_fee_lamports = 5000 * num_required_signatures;
        self.priority_fee_lamports = fees.saturating_sub(5000 * num_required_signatures as u64) as u32;
        self.successful_txns_count = if meta.err.is_none() { 1 } else { 0 };
        self.failed_txns_count = if meta.err.is_some() { 1 } else { 0 };

        if meta.err.is_some(){
            log::info!("{:?}", meta.clone().err.unwrap().err)
        }
    }
}




#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let timestamp = block.block_time.as_ref().unwrap().timestamp;
    let block_date = convert_to_date(timestamp);
    let mut data:  Vec<TransactionStats> = vec![];

    for trx in block.transactions {
        let (accounts, num_required_signatures, fees, meta) = extract_transaction_info(&trx);
        
        if accounts.contains(&"Vote111111111111111111111111111111111111111".to_string()) {
            continue;
        }

        let transaction_stats = TransactionStats::default();

        
        for log in &parse_logs(&meta.log_messages) {
            process_log(log, &mut transaction_stats, 1);
        }

        for data in trx_programs.values_mut() {
            data.update_fees_and_counts(&accounts, num_required_signatures, fees, &meta);
        }

        program_data_map.push(trx_programs);
    }
    // print_program_data_map(&program_data_map);

    let data = convert_to_output(&program_data_map, &block_date);
    Ok(Output { data })
}

fn extract_transaction_info(
    trx: &ConfirmedTransaction,
) -> (Vec<String>, u32, u64, TransactionStatusMeta) {
    let accounts = trx.resolved_accounts_as_strings();
    let meta = trx.meta.clone().unwrap();
    let num_required_signatures = trx
        .transaction
        .clone()
        .unwrap()
        .message
        .unwrap()
        .header
        .unwrap()
        .num_required_signatures;
    let fees = meta.fee;
    (accounts, num_required_signatures, fees, meta)
}

fn process_log(
    log: &LogContext,
    transaction_stats: &mut TransactionStats,
    depth: usize,
) {
    let program_id = &log.program_id;
    program_data_map
        .entry(program_id.clone())
        .and_modify(|data| data.update_with_log(log, depth))
        .or_insert_with(|| ProgramData::new_from_log(log, depth));

    for child in &log.children_nodes {
        process_log(child, program_data_map, depth + 1);
    }
}


    for (program_id, data) in program_data_map {
        log::info!("Program ID: {}", program_id);
        log::info!("  Compute Units Consumed: {}", data.compute_units_consumed);
        log::info!(
            "  Compute Units Allocated: {}",
            data.compute_units_allocated
        );
        log::info!("  Outer Invocation Count: {}", data.outer_invocation_count);
        log::info!("  Inner Invocation Count: {}", data.inner_invocation_count);
        log::info!(
            "  Failed Invocation Count: {}",
            data.failed_invocation_count
        );
        log::info!("  Errors:");
        for (error, count) in &data.errors {
            log::info!("    {}: {}", error, count);
        }

        log::info!("-----------------------------");
    }
}
