use bevy::prelude::*;

#[derive(Component)]
pub struct GameTextures {
    pub player_left: Handle<Image>,
    pub player_right: Handle<Image>,

    pub dragon: Handle<Image>,
    pub dragon_blue: Handle<Image>,
    pub healthbar1: Handle<Image>,
    pub healthbar2: Handle<Image>,
}

impl GameTextures {
    pub fn load(asset_server: Res<AssetServer>) -> Self {
        GameTextures {
            player_left: asset_server.load("images/gameplay/samurai1.png"),
            player_right: asset_server.load("images/gameplay/samurai2.png"),

            dragon: asset_server.load("images/board/dragon.png"),
            dragon_blue: asset_server.load("images/board/dragon_blue.png"),
            healthbar1: asset_server.load("images/board/hearts_1_bundle.png"),
            healthbar2: asset_server.load("images/board/hearts_2_bundle.png"),
        }
    }
}