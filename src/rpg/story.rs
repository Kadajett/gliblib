/// Story and Dialogue System
/// Branching dialogue trees, quests, and narrative progression

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unique dialogue node ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DialogueId(pub u32);

/// Dialogue speaker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Speaker {
    Npc(String),
    Player,
    Narrator,
}

/// Condition for showing a dialogue option or continuing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueCondition {
    HasQuestFlag(String),
    NoQuestFlag(String),
    HasItem(super::ItemId, u32), // Item ID, minimum quantity
    MinLevel(i32),
    MinGold(i32),
    AlwaysTrue,
}

impl DialogueCondition {
    pub fn check(&self, player: &super::Player, _item_db: &super::ItemDatabase) -> bool {
        match self {
            DialogueCondition::HasQuestFlag(flag) => player.has_quest_flag(flag),
            DialogueCondition::NoQuestFlag(flag) => !player.has_quest_flag(flag),
            DialogueCondition::HasItem(item_id, quantity) => {
                player.inventory.count_item(*item_id) >= *quantity
            }
            DialogueCondition::MinLevel(level) => player.level.current_level >= *level,
            DialogueCondition::MinGold(gold) => player.inventory.gold >= *gold,
            DialogueCondition::AlwaysTrue => true,
        }
    }
}

/// Action to perform when dialogue is selected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueAction {
    SetQuestFlag(String, bool),
    GiveItem(super::ItemId, u32),
    TakeItem(super::ItemId, u32),
    GiveGold(i32),
    TakeGold(i32),
    GiveExp(i32),
    StartBattle(u32), // Enemy ID to spawn
    Teleport(String), // Checkpoint name
    EndDialogue,
}

/// A single dialogue choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueChoice {
    pub text: String,
    pub condition: DialogueCondition,
    pub next_node: Option<DialogueId>,
    pub actions: Vec<DialogueAction>,
}

impl DialogueChoice {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            condition: DialogueCondition::AlwaysTrue,
            next_node: None,
            actions: Vec::new(),
        }
    }

    pub fn with_condition(mut self, condition: DialogueCondition) -> Self {
        self.condition = condition;
        self
    }

    pub fn with_next(mut self, next_id: DialogueId) -> Self {
        self.next_node = Some(next_id);
        self
    }

    pub fn with_action(mut self, action: DialogueAction) -> Self {
        self.actions.push(action);
        self
    }
}

/// A dialogue node (one piece of conversation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub id: DialogueId,
    pub speaker: Speaker,
    pub text: String,
    pub choices: Vec<DialogueChoice>,
    pub auto_continue: Option<DialogueId>, // Auto-continue to next node
}

impl DialogueNode {
    pub fn new(id: u32, speaker: Speaker, text: &str) -> Self {
        Self {
            id: DialogueId(id),
            speaker,
            text: text.to_string(),
            choices: Vec::new(),
            auto_continue: None,
        }
    }

    pub fn with_choice(mut self, choice: DialogueChoice) -> Self {
        self.choices.push(choice);
        self
    }

    pub fn with_auto_continue(mut self, next_id: DialogueId) -> Self {
        self.auto_continue = Some(next_id);
        self
    }
}

/// Dialogue tree - a complete conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueTree {
    pub id: String,
    pub name: String,
    pub root_node: DialogueId,
    pub nodes: HashMap<DialogueId, DialogueNode>,
}

impl DialogueTree {
    pub fn new(id: &str, name: &str, root_node: DialogueId) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            root_node,
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: DialogueNode) {
        self.nodes.insert(node.id, node);
    }

    pub fn get_node(&self, id: DialogueId) -> Option<&DialogueNode> {
        self.nodes.get(&id)
    }

    /// Get available choices for a node (filtered by conditions)
    pub fn get_available_choices(
        &self,
        node_id: DialogueId,
        player: &super::Player,
        item_db: &super::ItemDatabase,
    ) -> Vec<&DialogueChoice> {
        if let Some(node) = self.nodes.get(&node_id) {
            node.choices
                .iter()
                .filter(|choice| choice.condition.check(player, item_db))
                .collect()
        } else {
            Vec::new()
        }
    }
}

/// Quest status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestStatus {
    NotStarted,
    Active,
    Completed,
    Failed,
}

/// Quest objective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestObjective {
    KillEnemies { enemy_id: u32, current: u32, required: u32 },
    CollectItems { item_id: super::ItemId, current: u32, required: u32 },
    ReachLocation { location: String, reached: bool },
    TalkToNpc { npc_id: String, talked: bool },
}

impl QuestObjective {
    pub fn is_complete(&self) -> bool {
        match self {
            QuestObjective::KillEnemies { current, required, .. } => current >= required,
            QuestObjective::CollectItems { current, required, .. } => current >= required,
            QuestObjective::ReachLocation { reached, .. } => *reached,
            QuestObjective::TalkToNpc { talked, .. } => *talked,
        }
    }

    pub fn description(&self) -> String {
        match self {
            QuestObjective::KillEnemies { current, required, .. } => {
                format!("Defeat enemies: {}/{}", current, required)
            }
            QuestObjective::CollectItems { current, required, .. } => {
                format!("Collect items: {}/{}", current, required)
            }
            QuestObjective::ReachLocation { location, reached } => {
                format!("Reach {}: {}", location, if *reached { "✓" } else { "✗" })
            }
            QuestObjective::TalkToNpc { npc_id, talked } => {
                format!("Talk to {}: {}", npc_id, if *talked { "✓" } else { "✗" })
            }
        }
    }
}

/// Quest definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: QuestStatus,
    pub objectives: Vec<QuestObjective>,
    pub rewards: Vec<DialogueAction>,
    pub required_level: i32,
}

impl Quest {
    pub fn new(id: &str, name: &str, description: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            status: QuestStatus::NotStarted,
            objectives: Vec::new(),
            rewards: Vec::new(),
            required_level: 1,
        }
    }

    pub fn with_objective(mut self, objective: QuestObjective) -> Self {
        self.objectives.push(objective);
        self
    }

    pub fn with_reward(mut self, reward: DialogueAction) -> Self {
        self.rewards.push(reward);
        self
    }

    pub fn with_required_level(mut self, level: i32) -> Self {
        self.required_level = level;
        self
    }

    /// Check if all objectives are complete
    pub fn is_complete(&self) -> bool {
        self.objectives.iter().all(|obj| obj.is_complete())
    }

    /// Start the quest
    pub fn start(&mut self) {
        self.status = QuestStatus::Active;
    }

    /// Complete the quest
    pub fn complete(&mut self) {
        self.status = QuestStatus::Completed;
    }
}

/// Story database
pub struct StoryDatabase {
    dialogues: HashMap<String, DialogueTree>,
    quests: HashMap<String, Quest>,
}

impl StoryDatabase {
    pub fn new() -> Self {
        Self {
            dialogues: HashMap::new(),
            quests: HashMap::new(),
        }
    }

    pub fn add_dialogue(&mut self, dialogue: DialogueTree) {
        self.dialogues.insert(dialogue.id.clone(), dialogue);
    }

    pub fn get_dialogue(&self, id: &str) -> Option<&DialogueTree> {
        self.dialogues.get(id)
    }

    pub fn add_quest(&mut self, quest: Quest) {
        self.quests.insert(quest.id.clone(), quest);
    }

    pub fn get_quest(&self, id: &str) -> Option<&Quest> {
        self.quests.get(id)
    }

    pub fn get_quest_mut(&mut self, id: &str) -> Option<&mut Quest> {
        self.quests.get_mut(id)
    }

    /// Create a database with starter content
    pub fn with_starter_content() -> Self {
        let mut db = Self::new();

        // Example dialogue: Merchant
        let mut merchant_dialogue = DialogueTree::new("merchant_1", "Merchant", DialogueId(1000));

        merchant_dialogue.add_node(
            DialogueNode::new(
                1000,
                Speaker::Npc("Merchant".to_string()),
                "Welcome, traveler! Looking to buy or sell?",
            )
            .with_choice(
                DialogueChoice::new("What do you have for sale?")
                    .with_next(DialogueId(1001)),
            )
            .with_choice(
                DialogueChoice::new("I need a health potion.")
                    .with_condition(DialogueCondition::MinGold(10))
                    .with_action(DialogueAction::TakeGold(10))
                    .with_action(DialogueAction::GiveItem(super::ItemId(100), 1))
                    .with_next(DialogueId(1002)),
            )
            .with_choice(DialogueChoice::new("Goodbye.").with_action(DialogueAction::EndDialogue)),
        );

        merchant_dialogue.add_node(
            DialogueNode::new(
                1001,
                Speaker::Npc("Merchant".to_string()),
                "I have potions, weapons, and armor. All reasonably priced!",
            )
            .with_auto_continue(DialogueId(1000)),
        );

        merchant_dialogue.add_node(
            DialogueNode::new(
                1002,
                Speaker::Npc("Merchant".to_string()),
                "Here you go! That'll be 10 gold.",
            )
            .with_auto_continue(DialogueId(1000)),
        );

        db.add_dialogue(merchant_dialogue);

        // Example quest: Rat Problem
        db.add_quest(
            Quest::new(
                "rat_problem",
                "The Rat Problem",
                "The town is being overrun by giant rats. Defeat 5 of them.",
            )
            .with_objective(QuestObjective::KillEnemies {
                enemy_id: 1, // Slime (reusing as rat for example)
                current: 0,
                required: 5,
            })
            .with_reward(DialogueAction::GiveExp(100))
            .with_reward(DialogueAction::GiveGold(50))
            .with_reward(DialogueAction::GiveItem(super::ItemId(2), 1)), // Iron sword
        );

        // Example quest: The Mysterious Key
        db.add_quest(
            Quest::new(
                "mysterious_key",
                "The Mysterious Key",
                "Find the mysterious key hidden somewhere in the dungeon.",
            )
            .with_objective(QuestObjective::CollectItems {
                item_id: super::ItemId(200), // Mysterious key
                current: 0,
                required: 1,
            })
            .with_required_level(3)
            .with_reward(DialogueAction::GiveExp(200))
            .with_reward(DialogueAction::GiveGold(100)),
        );

        db
    }
}

impl Default for StoryDatabase {
    fn default() -> Self {
        Self::new()
    }
}
