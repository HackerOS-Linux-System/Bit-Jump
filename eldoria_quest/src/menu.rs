use bevy::prelude::*;
use super::components::*;
use super::resources::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(Update, handle_menu_buttons.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnEnter(GameState::Settings), setup_settings_menu)
            .add_systems(Update, handle_settings.run_if(in_state(GameState::Settings)))
            .add_systems(OnEnter(GameState::KeyBindings), setup_key_bindings_menu)
            .add_systems(Update, handle_key_bindings.run_if(in_state(GameState::KeyBindings)))
            .add_systems(OnEnter(GameState::ClassSelection), setup_class_selection_menu)
            .add_systems(Update, handle_class_selection.run_if(in_state(GameState::ClassSelection)))
            .add_systems(OnExit(GameState::MainMenu), cleanup_menu)
            .add_systems(OnExit(GameState::Settings), cleanup_menu)
            .add_systems(OnExit(GameState::KeyBindings), cleanup_menu)
            .add_systems(OnExit(GameState::ClassSelection), cleanup_menu);
    }
}

#[derive(Component)]
pub enum ButtonAction {
    NewGame,
    LoadGame,
    Settings,
    KeyBindings,
    ClassSelection,
    Quit,
    Save,
    Back,
    QualityLow,
    QualityMedium,
    QualityHigh,
    ToggleFullscreen,
    DifficultyEasy,
    DifficultyNormal,
    DifficultyHard,
    BindMoveUp,
    BindMoveDown,
    BindMoveLeft,
    BindMoveRight,
    BindInteract,
    BindAttack,
    BindPickUp,
    BindUseItem,
    BindCraft,
    BindTrade,
    BindSkill1,
    BindSkill2,
    BindSkill3,
    BindWork,
    SelectKnight,
    SelectNoble,
    SelectPriest,
    SelectTownsfolk,
    SelectPeasant,
    SelectMage,
}

fn setup_main_menu(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Eldoria Quest",
            TextStyle { font_size: 60.0, color: Color::WHITE, ..default() },
        ));
        spawn_button(parent, "New Game", ButtonAction::NewGame);
        spawn_button(parent, "Load Game", ButtonAction::LoadGame);
        spawn_button(parent, "Settings", ButtonAction::Settings);
        spawn_button(parent, "Key Bindings", ButtonAction::KeyBindings);
        spawn_button(parent, "Quit", ButtonAction::Quit);
    });
}

fn setup_settings_menu(mut commands: Commands, settings: Res<GameSettings>) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            format!("Graphics Quality: {:?}", settings.graphics_quality),
            TextStyle { font_size: 30.0, color: Color::WHITE, ..default() },
        ));
        spawn_button(parent, "Low Quality", ButtonAction::QualityLow);
        spawn_button(parent, "Medium Quality", ButtonAction::QualityMedium);
        spawn_button(parent, "High Quality", ButtonAction::QualityHigh);
        parent.spawn(TextBundle::from_section(
            format!("Difficulty: {:?}", settings.difficulty),
            TextStyle { font_size: 30.0, color: Color::WHITE, ..default() },
        ));
        spawn_button(parent, "Easy", ButtonAction::DifficultyEasy);
        spawn_button(parent, "Normal", ButtonAction::DifficultyNormal);
        spawn_button(parent, "Hard", ButtonAction::DifficultyHard);
        spawn_button(parent, "Toggle Fullscreen", ButtonAction::ToggleFullscreen);
        spawn_button(parent, "Back", ButtonAction::Back);
    });
}

fn setup_key_bindings_menu(mut commands: Commands, settings: Res<GameSettings>) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            format!("Move Up: {:?}", settings.key_bindings.move_up),
            TextStyle { font_size: 30.0, color: Color::WHITE, ..default() },
        ));
        spawn_button(parent, "Bind Move Up", ButtonAction::BindMoveUp);
        parent.spawn(TextBundle::from_section(
            format!("Move Down: {:?}", settings.key_bindings.move_down),
            TextStyle { font_size: 30.0, color: Color::WHITE, ..default() },
        ));
        spawn_button(parent, "Bind Move Down", ButtonAction::BindMoveDown);
        parent.spawn(TextBundle::from_section(
            format!("Interact: {:?}", settings.key_bindings.interact),
            TextStyle { font_size: 30.0, color: Color::WHITE, ..default() },
        ));
        spawn_button(parent, "Bind Interact", ButtonAction::BindInteract);
        parent.spawn(TextBundle::from_section(
            format!("Attack: {:?}", settings.key_bindings.attack),
            TextStyle { font_size: 30.0, color: Color::WHITE, ..default() },
        ));
        spawn_button(parent, "Bind Attack", ButtonAction::BindAttack);
        parent.spawn(TextBundle::from_section(
            format!("Work: {:?}", settings.key_bindings.work),
            TextStyle { font_size: 30.0, color: Color::WHITE, ..default() },
        ));
        spawn_button(parent, "Bind Work", ButtonAction::BindWork);
        spawn_button(parent, "Back", ButtonAction::Back);
    });
}

fn setup_class_selection_menu(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Select Social Class",
            TextStyle { font_size: 60.0, color: Color::WHITE, ..default() },
        ));
        spawn_button(parent, "Knight", ButtonAction::SelectKnight);
        spawn_button(parent, "Noble", ButtonAction::SelectNoble);
        spawn_button(parent, "Priest", ButtonAction::SelectPriest);
        spawn_button(parent, "Townsfolk", ButtonAction::SelectTownsfolk);
        spawn_button(parent, "Peasant", ButtonAction::SelectPeasant);
        spawn_button(parent, "Mage", ButtonAction::SelectMage);
        spawn_button(parent, "Back", ButtonAction::Back);
    });
}

fn spawn_button(parent: &mut ChildBuilder, text: &str, action: ButtonAction) {
    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(200.0),
            height: Val::Px(50.0),
            margin: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb(0.2, 0.2, 0.2).into(),
        ..default()
    }).insert(action).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            text,
            TextStyle { font_size: 30.0, color: Color::WHITE, ..default() },
        ));
    });
}

fn handle_menu_buttons(
    mut interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                ButtonAction::NewGame => next_state.set(GameState::ClassSelection),
                ButtonAction::LoadGame => next_state.set(GameState::LoadGame),
                ButtonAction::Settings => next_state.set(GameState::Settings),
                ButtonAction::KeyBindings => next_state.set(GameState::KeyBindings),
                ButtonAction::Quit => app_exit.send(AppExit::Success),
                _ => {},
            }
        }
    }
}

fn handle_settings(
    mut interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut settings: ResMut<GameSettings>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                ButtonAction::QualityLow => settings.graphics_quality = GraphicsQuality::Low,
                ButtonAction::QualityMedium => settings.graphics_quality = GraphicsQuality::Medium,
                ButtonAction::QualityHigh => settings.graphics_quality = GraphicsQuality::High,
                ButtonAction::DifficultyEasy => settings.difficulty = Difficulty::Easy,
                ButtonAction::DifficultyNormal => settings.difficulty = Difficulty::Normal,
                ButtonAction::DifficultyHard => settings.difficulty = Difficulty::Hard,
                ButtonAction::ToggleFullscreen => settings.fullscreen = !settings.fullscreen,
                ButtonAction::Back => next_state.set(GameState::MainMenu),
                _ => {},
            }
        }
    }
}

fn handle_key_bindings(
    mut interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut settings: ResMut<GameSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (interaction, action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                ButtonAction::BindMoveUp => {
                    if let Some(key) = keyboard.get_just_pressed().next() {
                        settings.key_bindings.move_up = *key;
                    }
                },
                ButtonAction::BindMoveDown => {
                    if let Some(key) = keyboard.get_just_pressed().next() {
                        settings.key_bindings.move_down = *key;
                    }
                },
                ButtonAction::BindInteract => {
                    if let Some(key) = keyboard.get_just_pressed().next() {
                        settings.key_bindings.interact = *key;
                    }
                },
                ButtonAction::BindAttack => {
                    if let Some(key) = keyboard.get_just_pressed().next() {
                        settings.key_bindings.attack = *key;
                    }
                },
                ButtonAction::BindWork => {
                    if let Some(key) = keyboard.get_just_pressed().next() {
                        settings.key_bindings.work = *key;
                    }
                },
                ButtonAction::Back => next_state.set(GameState::Settings),
                _ => {},
            }
        }
    }
}

fn handle_class_selection(
    mut interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    for (interaction, action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                ButtonAction::SelectKnight => {
                    commands.insert_resource(PlayerClassSelection(SocialClass::Knight));
                    next_state.set(GameState::NewGame);
                },
                ButtonAction::SelectNoble => {
                    commands.insert_resource(PlayerClassSelection(SocialClass::Noble));
                    next_state.set(GameState::NewGame);
                },
                ButtonAction::SelectPriest => {
                    commands.insert_resource(PlayerClassSelection(SocialClass::Priest));
                    next_state.set(GameState::NewGame);
                },
                ButtonAction::SelectTownsfolk => {
                    commands.insert_resource(PlayerClassSelection(SocialClass::Townsfolk));
                    next_state.set(GameState::NewGame);
                },
                ButtonAction::SelectPeasant => {
                    commands.insert_resource(PlayerClassSelection(SocialClass::Peasant));
                    next_state.set(GameState::NewGame);
                },
                ButtonAction::SelectMage => {
                    commands.insert_resource(PlayerClassSelection(SocialClass::Mage));
                    next_state.set(GameState::NewGame);
                },
                ButtonAction::Back => next_state.set(GameState::MainMenu),
                _ => {},
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<Node>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    NewGame,
    LoadGame,
    Settings,
    KeyBindings,
    ClassSelection,
    InGame,
    Paused,
}

#[derive(Resource)]
pub struct PlayerClassSelection(pub SocialClass);
