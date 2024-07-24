//! The screen state for the main game loop.

use bevy::prelude::*;

use super::Screen;
use crate::{
    game::{
        assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack,
    },
    ui::prelude::*
};
pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Hell), enter_hell);
    app.add_systems(OnExit(Screen::Hell), exit_hell);
    app.register_type::<HellAction>();

    app.add_systems(
        Update,
        handle_hell_action.run_if(in_state(Screen::Hell)),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum HellAction {
    Back,
}

fn enter_hell(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Hell))
        .with_children(|children| {
            children.label("youdied and went to hell");


            children.button("escape").insert(HellAction::Back);
        });    
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Credits));
}
fn exit_hell(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}

fn handle_hell_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&HellAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                HellAction::Back => next_screen.set(Screen::Playing),
            }
        }
    }
}
