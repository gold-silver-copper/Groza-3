use anchor_client::{
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::*, pubkey::Pubkey, signature::*, system_instruction,system_transaction, system_program,
        transaction::Transaction,
    },
    *,
};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::{str::FromStr, thread, time::Duration};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<LocalInput>()
        .init_resource::<PlayerDetails>()
        .add_systems(PreStartup, (setup))
        .add_systems(Startup, (setup2))
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup2(
    mut commands: Commands,
    mut local_input: ResMut<LocalInput>,
    player_details: Res<PlayerDetails>,
) {
    let player_pubkey = player_details.player_keypair.encodable_pubkey();
    println!("pubkey isssss     {}", player_pubkey.to_string());
    let payer = &player_details.player_keypair;

    let random_pubkey = Pubkey::from_str("G9qpASRsg9xrMjAEoECdAyGj5RB1vrQ3pfWSVk8sBuv8").unwrap();

    let client_ref = &player_details.player_rpcclient;


    let blockhash = client_ref.get_latest_blockhash().unwrap();






    let rpc_response2 = client_ref.request_airdrop(&player_pubkey, 10000000000);
    let ten_millis = Duration::from_millis(1000);
    thread::sleep(ten_millis);
    let rpc_response3 = client_ref.get_account(&player_pubkey);

    match rpc_response2 {
        Ok(sign) => local_input
            .output_messages_1
            .append(&mut vec![sign.to_string()]),
        Err(e) => local_input
            .output_messages_1
            .append(&mut vec![e.to_string()]),
    }

    match rpc_response3 {
        Ok(acc) => local_input
            .output_messages_1
            .append(&mut vec![acc.lamports.to_string()]),
        Err(e) => local_input
            .output_messages_1
            .append(&mut vec![e.to_string()]),
    }


    let tx = system_transaction::transfer(&player_details.player_keypair, &random_pubkey, 1111000000, blockhash);
    let signature = client_ref.send_and_confirm_transaction(&tx);






    //  local_input
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut local_input: ResMut<LocalInput>,
    player_details: Res<PlayerDetails>,
) {
    egui::TopBottomPanel::bottom("input_panel").show(contexts.ctx_mut(), |ui| {
        let response = ui.add_sized(
            ui.available_size(),
            egui::TextEdit::singleline(&mut local_input.input_string_buffer).desired_rows(3),
        );

        if response.changed() {
            // …
        }
        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            local_input.input_string_sent = local_input.input_string_buffer.clone();
            local_input
                .output_messages_1
                .append(&mut vec!["meow".to_string()]);
            local_input.input_string_buffer = "".to_string();
        }
    });

    egui::SidePanel::right("rightp").show(contexts.ctx_mut(), |ui| {
        ui.label("player_keypair: ");
        ui.label(&player_details.player_keypair.encodable_pubkey().to_string());
    });
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(&local_input.input_string_sent);
            for pot in &local_input.output_messages_1 {
                ui.label(pot);
            }

            ui.allocate_space(ui.available_size());
        });
    });
}

#[derive(Resource)]
struct LocalInput {
    input_string_buffer: String,
    input_string_sent: String,
    output_messages_1: Vec<String>,
}
// custom implementation for unusual values
impl Default for LocalInput {
    fn default() -> Self {
        LocalInput {
            input_string_buffer: "".to_string(),
            input_string_sent: "".to_string(),
            output_messages_1: vec!["".to_string()],
        }
    }
}

#[derive(Resource)]
struct PlayerDetails {
    player_keypair: Keypair,
    player_rpcclient: RpcClient,
}

// custom implementation for unusual values
impl Default for PlayerDetails {
    fn default() -> Self {
        let url = "http://127.0.0.1:8899".to_string();
        let timeout = Duration::from_secs(1);
        let commitment_config = CommitmentConfig::processed();

        PlayerDetails {
            player_keypair: Keypair::new(),
            player_rpcclient: RpcClient::new_with_timeout_and_commitment(
                url,
                timeout,
                commitment_config,
            ),
        }
    }
}
