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
    app.add_systems(OnEnter(Screen::Win), enter_win);
    app.add_systems(OnExit(Screen::Win), exit_win);
    app.register_type::<WinAction>();

    app.add_systems(
        Update,
        handle_win_action.run_if(in_state(Screen::Win)),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum WinAction {
    Back,
}

fn enter_win(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Win))
        .with_children(|children| {
            children.label(" you won and went to heaven and you are happy and everyone here loves you <3 <# <3");


            children.button("escape").insert(WinAction::Back);
        });    
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Credits));
}
fn exit_win(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}

fn handle_win_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&WinAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                WinAction::Back => next_screen.set(Screen::Title),
            }
        }
    }
}
