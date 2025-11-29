use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct WorldState {
    local_state: LocalState,
}

#[derive(Deserialize, Serialize)]
struct LocalState {
    player_stats: PlayerStats,
    chats: Vec<Chats>,
    local_allied_entities: Vec<Entity>,
    local_neutral_entities: Vec<Entity>,
    local_hostile_entities: Vec<Entity>,
}

#[derive(Deserialize, Serialize)]
struct Chats {
    user_response: OllamaMessage,
    assistant_response: OllamaMessage,
}

#[derive(Deserialize, Serialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Deserialize, Serialize)]
struct PlayerStats {
    entity: Entity,
}

#[derive(Deserialize, Serialize)]
struct Entity {
    entity_properties: EntityProperties,
}

#[derive(Deserialize, Serialize)]
struct EntityProperties {
    name: String,
    combat_stats: CombatStats,
}

#[derive(Deserialize, Serialize)]
struct CombatStats {
    max_health: i32,
    current_heaelth: i32,
    max_attack: i32,
    current_attack: i32,
    max_defense: i32,
    current_defense: i32,
}
