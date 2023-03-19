use crate::*;

pub fn get_platform_graphics_api() -> Backends {
    // Setup native Graphics API for each platform.
    let platform_api = if cfg!(target_os = "windows") {
        Backends::DX12
    } else if cfg!(target_os = "macos") {
        Backends::METAL
    } else if cfg!(target_os = "linux") {
        Backends::GL
    } else {
        panic!("Unsupported platform!");
    };
    println!("Using {:?} as graphics api.", platform_api);
    return platform_api;
}

pub fn display_players(player_query: Query<(&components::player::Player, Option<&components::player::ControlledPlayer>)>, mut player_log_texts: Query<&mut Text, With<components::debug::PlayerLogText>>) {
    for mut text in &mut player_log_texts {
        let mut data_string = format!("Amount of players: {}\n", player_query.into_iter().len());
        for (player, controlled_player_option) in &player_query {
            data_string += format!("-------------\nid: {}\nx: {},   y: {}\npressed: {}\n",
                player.server_id, player.position.x, player.position.y, player.pressed).as_str();

            if let Some(_) = controlled_player_option {
                data_string += format!("Controlled by this client\n").as_str();
            }
        }
        text.sections[0].value = data_string;
    }
}