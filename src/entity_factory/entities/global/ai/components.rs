use bevy::{
    prelude::{Component, Vec2},
    utils::Uuid,
};

// To be used as data for the ai entity.
// WILL: contain data describing the current status of its attached entity in its respective domain

#[derive(Debug, Clone)]
pub enum AIMode {
    Idle,
    Patrol { path: Vec2, duration: f32 },
    Attack { target: Uuid },
    Flee,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AITeam {
    Player,
    PlayerEquipment,
    FriendlySurvivor,
    FriendlyEquipment,
    HostileSurvivor,
    HostileEquipment,
    Zombies,
    Monsters,
    Animals,
    Neutral,
    All,
    None,
    Pickupable,
    Mission,
}

#[derive(Component, Debug, Clone)]
pub struct AIStatus {
    pub active: bool,
    pub mode: AIMode,
    pub can_move: bool,
}

impl Default for AIStatus {
    fn default() -> Self {
        Self {
            active: false,
            mode: AIMode::Idle,
            can_move: true,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct AIIdentifier {
    pub id: Uuid,
    pub team: AITeam,
}

impl Default for AIIdentifier {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            team: AITeam::Neutral,
        }
    }
}

// This is the component that will determine if the AI can see the entity
#[derive(Component, Debug, Clone)]
pub struct AIDetectionData {
    pub detection_radius: f32,
}

impl Default for AIDetectionData {
    fn default() -> Self {
        Self {
            detection_radius: 50.0,
        }
    }
}
