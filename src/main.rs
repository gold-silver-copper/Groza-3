use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use anchor_client::{solana_client::rpc_client::RpcClient, solana_sdk::{ system_instruction,system_program,commitment_config::CommitmentConfig, signature::Keypair, signer::EncodableKeypair, transaction::Transaction}, *};
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<LocalInput>()
        .init_resource::<PlayerDetails>()
        .add_systems(PreStartup, (setup))
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());




}

fn ui_example_system(mut contexts: EguiContexts, mut local_input: ResMut<LocalInput>, player_details: Res<PlayerDetails>) {
  
     egui::TopBottomPanel::bottom("input_panel").show(contexts.ctx_mut(), |ui| {

        let response = ui.add_sized(ui.available_size(), egui::TextEdit::singleline(&mut local_input.input_string_buffer).desired_rows(3));

        if response.changed() {
            // …
        }
        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            local_input.input_string_sent = local_input.input_string_buffer.clone();
            local_input.input_string_buffer = "".to_string();
        }
        
      
     });






     egui::SidePanel::right("rightp").show(contexts.ctx_mut(), |ui| {
        ui.label("player_keypair: ");
        ui.label(&player_details.player_keypair.to_base58_string());

        
     });
     egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(&local_input.input_string_sent);
            
            ui.allocate_space(ui.available_size());
        });
     });
}






#[derive(Resource)]
struct LocalInput {
    input_string_buffer: String,
    input_string_sent: String,
    
}
// custom implementation for unusual values
impl Default for LocalInput {
    fn default() -> Self {
        LocalInput{input_string_buffer:"".to_string(),input_string_sent:"".to_string(),}
    }
}



#[derive(Resource)]
struct PlayerDetails {
    player_keypair: Keypair,
    player_rpcclient: RpcClient

}

// custom implementation for unusual values
impl Default for PlayerDetails {
    fn default() -> Self {

        let url = "https://api.devnet.solana.com::8899".to_string();
let timeout = Duration::from_secs(1);
let commitment_config = CommitmentConfig::processed();


        
        PlayerDetails{ player_keypair: Keypair::new(), player_rpcclient: RpcClient::new_with_timeout_and_commitment(
            url,
            timeout,
            commitment_config,
        ) }
    }
}
