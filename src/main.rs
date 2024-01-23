use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<LocalInput>()
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .run();
}

fn ui_example_system(mut contexts: EguiContexts, mut local_input: ResMut<LocalInput>,) {
  
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
        ui.label("Hello World!");
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
