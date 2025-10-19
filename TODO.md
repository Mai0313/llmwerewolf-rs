# LLM Werewolf - Rust Rewrite Specification

**Document Version:** 1.0
**Date:** 2025-10-20
**Project:** LLM Werewolf Game - Complete Rust Rewrite
**Original Implementation:** Python 3.10+ with Textual TUI

---

## Table of Contents

01. [Executive Summary](#executive-summary)
02. [Current Architecture Analysis](#current-architecture-analysis)
03. [Core System Components](#core-system-components)
04. [Data Models and Type System](#data-models-and-type-system)
05. [Rust Technology Stack](#rust-technology-stack)
06. [Module-by-Module Rewrite Plan](#module-by-module-rewrite-plan)
07. [Implementation Phases](#implementation-phases)
08. [Testing Strategy](#testing-strategy)
09. [Migration Path](#migration-path)
10. [Performance Considerations](#performance-considerations)

---

## Executive Summary

### Project Goals

Rewrite the LLM Werewolf game from Python to Rust to achieve:

1. **Type Safety**: Leverage Rust's strong type system to eliminate runtime errors
2. **Performance**: Dramatically improve execution speed, especially for AI agent coordination
3. **Memory Safety**: Eliminate memory-related bugs through Rust's ownership system
4. **Concurrency**: Better handle concurrent LLM API calls and game state updates
5. **Portability**: Easier distribution as single binary without Python runtime dependency

### Current Project Overview

LLM Werewolf is a social deduction game (Mafia/Werewolf) powered by Large Language Models. The game features:

- **Multi-Agent System**: Supports LLM agents (GPT, Claude, DeepSeek), human players, and demo bots
- **Rich Role System**: 20+ unique roles with special abilities (Werewolf, Seer, Witch, Hunter, etc.)
- **Phase-Based Gameplay**: Structured game loop with night actions, day discussion, and voting
- **Interactive TUI**: Real-time terminal UI using Textual framework
- **Console Mode**: Auto-play mode for batch simulations
- **Event System**: Comprehensive event logging for game replay and analysis
- **Configuration System**: YAML-based player and game configuration

---

## Current Architecture Analysis

### System Architecture Overview

The Python implementation follows a layered architecture:

```
┌─────────────────────────────────────────────────────────────┐
│                    Presentation Layer                        │
│  - TUI (Textual): Interactive terminal interface             │
│  - CLI: Console auto-play mode                               │
│  - Components: PlayerPanel, GamePanel, ChatPanel, DebugPanel │
└─────────────────────────────────────────────────────────────┘
                            ↕
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│  - GameEngine: Main game loop coordinator                    │
│  - EventLogger: Event management and notification            │
│  - VictoryChecker: Win condition evaluation                  │
└─────────────────────────────────────────────────────────────┘
                            ↕
┌─────────────────────────────────────────────────────────────┐
│                      Domain Layer                            │
│  - GameState: Centralized state management                   │
│  - Player: Player entity with role and agent                 │
│  - Roles: 20+ role implementations (Werewolf, Seer, etc.)    │
│  - Actions: Night/day action implementations                 │
└─────────────────────────────────────────────────────────────┘
                            ↕
┌─────────────────────────────────────────────────────────────┐
│                   Infrastructure Layer                       │
│  - Agents: LLMAgent, HumanAgent, DemoAgent                   │
│  - Config: YAML parsing, validation (Pydantic)               │
│  - External APIs: OpenAI-compatible chat completion          │
└─────────────────────────────────────────────────────────────┘
```

### Key Design Patterns

1. **State Machine**: Game phases transition in fixed order (SETUP → NIGHT → DAY_DISCUSSION → DAY_VOTING → NIGHT...)
2. **Strategy Pattern**: Different agent implementations (LLM, Human, Demo) share common interface
3. **Factory Pattern**: Agent and role creation from configuration
4. **Observer Pattern**: Event system notifies UI of game state changes
5. **Command Pattern**: Actions encapsulate game operations with validate/execute methods

### Directory Structure (Python)

```
src/llm_werewolf/
├── ai/
│   ├── agents.py          # Agent implementations (LLM, Human, Demo)
│   └── message.py         # Message formatting utilities
├── config/
│   ├── game_config.py     # Game rules configuration
│   └── role_presets.py    # Predefined role combinations
├── core/
│   ├── actions.py         # Action system (Kill, Save, Vote, etc.)
│   ├── events.py          # Event types and logger
│   ├── game_engine.py     # Main game loop controller
│   ├── game_state.py      # Centralized game state
│   ├── player.py          # Player entity
│   ├── victory.py         # Victory condition checker
│   └── roles/
│       ├── base.py        # Role abstract base class
│       ├── werewolf.py    # Werewolf faction roles
│       ├── villager.py    # Villager faction roles
│       └── neutral.py     # Neutral roles
├── ui/
│   ├── tui_app.py         # Textual TUI application
│   ├── styles.py          # TUI styling
│   └── components/
│       ├── player_panel.py
│       ├── game_panel.py
│       ├── chat_panel.py
│       └── debug_panel.py
├── cli.py                 # Console mode entry point
└── tui.py                 # TUI mode entry point
```

---

## Core System Components

### 1. Game Engine (`game_engine.py`)

**Responsibilities:**

- Orchestrates the complete game loop
- Manages phase transitions (setup → night → day discussion → voting)
- Coordinates action collection and execution
- Triggers victory condition checks
- Emits events for UI updates

**Key Methods:**

```python
class GameEngine:
    def __init__(self, config: GameConfig | None = None)
    def setup_game(self, players: list[tuple], roles: list[Role])
    def run_night_phase() -> list[str]
    def run_day_phase() -> list[str]
    def run_voting_phase() -> list[str]
    def process_actions(self, actions: list[Action]) -> list[str]
    def resolve_deaths() -> list[str]
    def check_victory() -> bool
    def play_game() -> str  # Auto-play full game
    def step() -> list[str]  # Execute single phase (for TUI)
    def on_event(event: Event)  # Callback for event notifications
```

**Critical Flow:**

1. **Setup Phase**:

   - Validate player count matches role count
   - Shuffle roles and assign randomly to players
   - Initialize GameState with player list
   - Emit GAME_STARTED event

2. **Night Phase**:

   - Set phase to NIGHT
   - Query each alive player for night actions via `role.get_night_actions(game_state)`
   - Sort actions by priority (Guard → Werewolf → Witch → Seer)
   - Execute actions in order
   - Resolve deaths with special cases (Witch save, Guard protect, Elder 2 lives)
   - Check for lover heartbreak deaths

3. **Day Discussion Phase**:

   - Announce night deaths
   - Iterate through alive players
   - Build context message for each player's agent
   - Collect speech from `agent.get_response(context)`
   - Log PLAYER_SPEECH events

4. **Voting Phase**:

   - Collect votes from players who can vote
   - Handle vote ties
   - Special case: Idiot survives vote but loses voting rights
   - Emit PLAYER_ELIMINATED event

5. **Victory Check**:

   - Check lover victory (highest priority)
   - Check werewolf victory (werewolves ≥ villagers)
   - Check villager victory (all werewolves dead)

### 2. Game State (`game_state.py`)

**Responsibilities:**

- Central source of truth for all game data
- Tracks phase, round number, alive/dead players
- Records night actions (werewolf target, witch actions, guard protection)
- Manages vote tallies
- Provides query methods for game logic

**Data Structure:**

```python
class GameState:
    # Core state
    players: list[Player]
    player_dict: dict[str, Player]  # player_id -> Player
    phase: GamePhase
    round_number: int

    # Death tracking
    night_deaths: set[str]  # player_ids who died this night
    day_deaths: set[str]  # player_ids who died this day

    # Night action tracking
    werewolf_target: str | None
    witch_save_used: bool
    witch_poison_used: bool
    witch_saved_target: str | None
    witch_poison_target: str | None
    guard_protected: str | None
    seer_checked: dict[int, str]  # round -> player_id

    # Voting
    votes: dict[str, str]  # voter_id -> target_id
    raven_marked: str | None  # Counts as extra vote

    # Victory
    winner: str | None
```

**Key Methods:**

```python
def get_phase() -> GamePhase
def next_phase() -> GamePhase
def get_alive_players(except_ids: list[str] | None) -> list[Player]
def get_players_with_night_actions() -> list[Player]
def get_players_by_camp(camp: str) -> list[Player]
def count_alive_by_camp(camp: str) -> int
def add_vote(voter_id: str, target_id: str)
def get_vote_counts() -> dict[str, int]
def reset_deaths()  # Called at start of new round
```

**Phase Transition Logic:**

```
SETUP → NIGHT (round = 1)
NIGHT → DAY_DISCUSSION
DAY_DISCUSSION → DAY_VOTING
DAY_VOTING → NIGHT (round += 1, clear temporary data)
```

### 3. Role System (`roles/`)

**Base Role Abstract Class:**

```python
class Role(ABC):
    player: Player
    ability_uses: int
    config: RoleConfig

    @abstractmethod
    def get_config() -> RoleConfig

    @abstractmethod
    def get_night_actions(game_state: GameState) -> list[Action]

    def has_night_action(game_state: GameState) -> bool
    def can_act_tonight(player: Player, round_number: int) -> bool
```

**Role Configuration:**

```python
class RoleConfig:
    name: str
    camp: Camp  # WEREWOLF, VILLAGER, NEUTRAL
    description: str
    priority: ActionPriority | None
    can_act_night: bool
    can_act_day: bool
    max_uses: int | None  # Limit ability usage
```

**Action Priority Enum** (higher value = executes earlier):

```python
class ActionPriority(int, Enum):
    CUPID = 100  # Cupid links lovers (night 1 only)
    THIEF = 95  # Thief chooses role (night 1 only)
    GUARD = 90  # Guard protects someone
    WEREWOLF = 80  # Werewolves kill
    WHITE_WOLF = 75  # White wolf kills another wolf
    WITCH = 70  # Witch uses potions
    SEER = 60  # Seer checks someone
    GRAVEYARD_KEEPER = 50
    RAVEN = 40  # Raven marks for extra vote
```

**Role Categories:**

**Werewolf Faction (8 roles):**

- Werewolf: Standard werewolf, collectively kills at night
- AlphaWolf: Can shoot when eliminated
- WhiteWolf: Can kill another werewolf every 2 nights
- WolfBeauty: Charms a player; if WolfBeauty dies, charmed player dies too
- GuardianWolf: Can protect one werewolf from elimination
- HiddenWolf: Appears as villager to Seer
- NightmareWolf: Can block a player's night action
- BloodMoonApostle: Special werewolf with unique mechanics

**Villager Faction (12 roles):**

- Villager: No special abilities, can only vote
- Seer: Check one player's identity each night
- Witch: One save potion, one poison potion (single use each)
- Hunter: When killed, can shoot another player
- Guard: Protect one player from werewolf kill (can't protect same player twice in row)
- Elder: Has 2 lives, survives first werewolf attack
- Idiot: Survives vote elimination but loses voting rights
- Knight: Can challenge someone to duel during day (one time use)
- Cupid: Links two players as lovers on night 1
- Raven: Marks player for double vote count
- Magician: Can swap two players' identities
- GraveyardKeeper: Can check if a dead player was werewolf

**Neutral Faction (2 roles):**

- Thief: Chooses from 2 random roles on night 1
- Lover: Win condition changes to "only lovers survive"

### 4. Action System (`actions.py`)

**Base Action Abstract Class:**

```python
class Action(ABC):
    actor: Player
    game_state: GameState

    @abstractmethod
    def get_action_type() -> ActionType

    @abstractmethod
    def validate() -> bool  # Check if action is legal

    @abstractmethod
    def execute() -> list[str]  # Perform action, return messages
```

**Action Types:**

**Night Actions:**

- WerewolfKillAction: Sets game_state.werewolf_target
- WitchSaveAction: Sets game_state.witch_saved_target, consumes save potion
- WitchPoisonAction: Sets game_state.witch_poison_target, consumes poison potion
- SeerCheckAction: Records check in game_state.seer_checked
- GuardProtectAction: Sets game_state.guard_protected
- CupidLinkAction: Sets lover relationship on both players
- RavenMarkAction: Sets game_state.raven_marked
- WhiteWolfKillAction: Directly kills werewolf
- WolfBeautyCharmAction: Records charmed player

**Day Actions:**

- VoteAction: Records vote in game_state.votes
- HunterShootAction: Directly kills target (triggered on Hunter death)
- KnightDuelAction: Kills werewolf or self based on camp

**Action Execution Order:**
Actions are sorted by priority before execution. The `process_actions()` method in GameEngine uses this priority map:

```python
priority_map = {
    "GuardProtectAction": 0,  # Execute first
    "WerewolfKillAction": 1,
    "WitchSaveAction": 2,
    "WitchPoisonAction": 3,
    "SeerCheckAction": 4,
    # Others default to 100 (execute last)
}
```

### 5. Agent System (`ai/agents.py`)

**Agent Interface:**
All agents implement:

```python
def get_response(message: str) -> str
```

**Agent Types:**

**1. LLMAgent (OpenAI-compatible API):**

```python
class LLMAgent:
    model_name: str
    api_key: str
    base_url: str
    temperature: float = 0.7
    max_tokens: int = 500
    chat_history: list[dict[str, str]]
    client: OpenAI  # Lazy initialized

    def get_response(message: str) -> str:
        # Append message to chat_history
        # Call client.chat.completions.create()
        # Return response content
```

**2. HumanAgent (Console Input):**

```python
class HumanAgent:
    model_name: str = "human"

    def get_response(message: str) -> str:
        # Print message to console
        # Return user input from stdin
```

**3. DemoAgent (Random Responses):**

```python
class DemoAgent:
    model_name: str = "demo"

    def get_response(message: str) -> str:
        # Return random canned response
        return random.choice(
            [
                "I agree.",
                "I'm not sure about that.",
                "Let me think about it.",
                ...,
            ]
        )
```

**Agent Factory:**

```python
def create_agent(config: PlayerConfig) -> LLMAgent | HumanAgent | DemoAgent:
    if config.model == "human":
        return HumanAgent()
    elif config.model == "demo":
        return DemoAgent()
    else:
        # Load API key from environment variable
        api_key = os.getenv(config.api_key_env)
        return LLMAgent(
            model_name=config.model,
            api_key=api_key,
            base_url=config.base_url,
            temperature=config.temperature,
            max_tokens=config.max_tokens,
        )
```

### 6. Player Entity (`player.py`)

**Player Data Structure:**

```python
class Player:
    player_id: str
    name: str
    role: Role  # Role instance
    agent: AgentType | None
    ai_model: str

    # State
    _alive: bool = True
    statuses: set[PlayerStatus]
    lover_partner_id: str | None
    can_vote_flag: bool = True
```

**Player Status Enum:**

```python
class PlayerStatus(str, Enum):
    ALIVE = "alive"
    DEAD = "dead"
    PROTECTED = "protected"  # By Guard
    POISONED = "poisoned"  # By Witch
    SAVED = "saved"  # By Witch
    CHARMED = "charmed"  # By Wolf Beauty
    BLOCKED = "blocked"  # By Nightmare Wolf
    MARKED = "marked"  # By Raven
    REVEALED = "revealed"  # Idiot revealed
    NO_VOTE = "no_vote"  # Lost voting rights
    LOVER = "lover"  # Is in love
```

**Key Methods:**

```python
def is_alive() -> bool
def kill()
def revive()
def can_vote() -> bool
def disable_voting()
def set_lover(partner_id: str)
def is_lover() -> bool
def get_role_name() -> str
def get_camp() -> str
```

### 7. Event System (`events.py`)

**Event Structure:**

```python
class Event:
    event_type: EventType
    timestamp: datetime
    round_number: int
    phase: str
    message: str
    data: dict  # Additional structured data
    visible_to: list[str] | None  # None = visible to all
```

**Event Types:**

```python
class EventType(str, Enum):
    # Game flow
    GAME_STARTED = "game_started"
    GAME_ENDED = "game_ended"
    PHASE_CHANGED = "phase_changed"

    # Player events
    PLAYER_DIED = "player_died"
    PLAYER_ELIMINATED = "player_eliminated"
    PLAYER_SPEECH = "player_speech"

    # Action events
    WEREWOLF_KILLED = "werewolf_killed"
    WITCH_SAVED = "witch_saved"
    WITCH_POISONED = "witch_poisoned"
    SEER_CHECKED = "seer_checked"
    GUARD_PROTECTED = "guard_protected"

    # Voting
    VOTE_CAST = "vote_cast"
    VOTE_RESULT = "vote_result"

    # Special
    LOVERS_LINKED = "lovers_linked"
    HUNTER_REVENGE = "hunter_revenge"
```

**EventLogger:**

```python
class EventLogger:
    events: list[Event] = []

    def create_event(...) -> Event
    def log_event(event: Event)
    def get_events_for_player(player_id: str) -> list[Event]
    def get_events_by_type(event_type: EventType) -> list[Event]
```

### 8. Victory System (`victory.py`)

**Victory Checker:**

```python
class VictoryChecker:
    game_state: GameState

    def check_victory() -> VictoryResult:
        # Priority order:
        # 1. Lover victory (only 2 lovers remain)
        # 2. Werewolf victory (werewolves >= villagers)
        # 3. Villager victory (all werewolves dead)

    def check_werewolf_victory() -> VictoryResult
    def check_villager_victory() -> VictoryResult
    def check_lover_victory() -> VictoryResult
```

**Victory Result:**

```python
class VictoryResult:
    has_winner: bool
    winner_camp: str | None  # "werewolf", "villager", "lover"
    winner_ids: list[str]
    reason: str
```

### 9. Configuration System (`config/`)

**Game Config:**

```python
class GameConfig:
    num_players: int  # 6-20
    role_names: list[str]
    night_timeout: int = 60
    day_timeout: int = 300
    vote_timeout: int = 60
    allow_revote: bool = False
    show_role_on_death: bool = True

    def to_role_list() -> list[Role]:
        # Map role names to Role classes
```

**Player Config:**

```python
class PlayerConfig:
    name: str
    model: str  # "human", "demo", or LLM model name
    base_url: str | None  # Required for LLM models
    api_key_env: str | None  # Environment variable name
    temperature: float = 0.7
    max_tokens: int = 500
```

**Players Config (YAML root):**

```python
class PlayersConfig:
    players: list[PlayerConfig]
    preset: str | None  # "6-players", "9-players", etc.
    show_debug: bool = False
```

**Role Presets:**

```python
ROLE_PRESETS = {
    "6-players": ["Werewolf", "Werewolf", "Seer", "Witch", "Villager", "Villager"],
    "9-players": [
        "Werewolf",
        "Werewolf",
        "Seer",
        "Witch",
        "Hunter",
        "Guard",
        "Villager",
        "Villager",
        "Villager",
    ],
    "12-players": [...],
    "15-players": [...],
    "expert": [...],
    "chaos": [...],
}
```

### 10. TUI System (`ui/`)

**Main TUI App:**

```python
class WerewolfTUI(App):  # Textual App
    game_engine: GameEngine
    show_debug_flag: bool

    # Component references
    player_panel: PlayerPanel
    game_panel: GamePanel
    chat_panel: ChatPanel
    debug_panel: DebugPanel

    def on_mount():
        # Set up event callback
        game_engine.on_event = self.on_game_event

    def on_game_event(event: Event):
        # Update panels based on event
        chat_panel.add_event(event)
        update_game_state()

    def action_next_step():
        # Execute game_engine.step()
        # Update UI
```

**UI Components:**

1. **PlayerPanel**: Shows player list with status indicators

   - Player names
   - Alive/dead status
   - Special statuses (protected, lover, marked)
   - AI model indicator

2. **GamePanel**: Shows current game state

   - Current phase and round
   - Faction counts (werewolves, villagers)
   - Vote tallies during voting phase

3. **ChatPanel**: Scrollable event log

   - Color-coded events
   - Player speeches
   - System messages

4. **DebugPanel**: Debug information (toggleable)

   - Session info
   - Config details
   - Role assignments
   - Night action tracking

**Key Bindings:**

- `q`: Quit game
- `d`: Toggle debug panel
- `n`: Next step (advance one phase)
- Arrow keys: Navigation

---

## Data Models and Type System

### Python Type System Analysis

Current Python implementation uses:

- **Pydantic** for runtime validation and data models
- **Type hints** for static type checking (mypy)
- **Enums** for fixed value sets (GamePhase, EventType, Camp, etc.)
- **Abstract base classes** (Role, Action) for polymorphism
- **Type unions** (Player | None, AgentType = LLMAgent | HumanAgent | DemoAgent)

### Rust Type System Mapping

| Python Type          | Rust Equivalent       | Notes                                                      |
| -------------------- | --------------------- | ---------------------------------------------------------- |
| `str`                | `String` or `&str`    | Use `String` for owned, `&str` for borrowed                |
| `int`                | `i32`, `u32`, `usize` | Choose based on range and sign                             |
| `float`              | `f32` or `f64`        | Use `f64` for better precision                             |
| `bool`               | `bool`                | Direct mapping                                             |
| `list[T]`            | `Vec<T>`              | Dynamic array                                              |
| `set[T]`             | `HashSet<T>`          | Requires `T: Hash + Eq`                                    |
| `dict[K, V]`         | `HashMap<K, V>`       | Requires `K: Hash + Eq`                                    |
| `Enum`               | `enum`                | Rust enums are more powerful                               |
| `Optional[T]`        | `Option<T>`           | Use `None` → `Option::None`, `Some(x)` → `Option::Some(x)` |
| `Union[A, B]`        | `enum`                | Use enum with variants                                     |
| Pydantic model       | `struct` with `serde` | Use serde for serialization                                |
| ABC (abstract class) | `trait`               | Rust traits define interfaces                              |

### Critical Data Structures in Rust

**GamePhase Enum:**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    Setup,
    Night,
    DayDiscussion,
    DayVoting,
    Ended,
}
```

**Camp Enum:**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Camp {
    Werewolf,
    Villager,
    Neutral,
}
```

**ActionPriority Enum:**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ActionPriority {
    Raven = 40,
    GraveyardKeeper = 50,
    Seer = 60,
    Witch = 70,
    WhiteWolf = 75,
    Werewolf = 80,
    Guard = 90,
    Thief = 95,
    Cupid = 100,
}
```

**Player Structure:**

```rust
pub struct Player {
    pub player_id: String,
    pub name: String,
    pub role: Box<dyn Role>,
    pub agent: Option<Box<dyn Agent>>,
    pub ai_model: String,

    alive: bool,
    statuses: HashSet<PlayerStatus>,
    lover_partner_id: Option<String>,
    can_vote_flag: bool,
}
```

**GameState Structure:**

```rust
pub struct GameState {
    pub players: Vec<Player>,
    player_map: HashMap<String, usize>,  // player_id -> index
    pub phase: GamePhase,
    pub round_number: u32,

    pub night_deaths: HashSet<String>,
    pub day_deaths: HashSet<String>,

    pub werewolf_target: Option<String>,
    pub witch_saved_target: Option<String>,
    pub witch_poison_target: Option<String>,
    pub guard_protected: Option<String>,
    pub seer_checked: HashMap<u32, String>,

    pub votes: HashMap<String, String>,
    pub raven_marked: Option<String>,

    pub winner: Option<String>,
}
```

---

## Rust Technology Stack

### Core Dependencies

**Essential Crates:**

1. **serde** (1.0) + **serde_json** (1.0) + **serde_yaml** (0.9)

   - Purpose: Serialization/deserialization for configs and data models
   - Usage: YAML config parsing, JSON API communication

2. **tokio** (1.35) with full features

   - Purpose: Async runtime for concurrent LLM API calls
   - Usage: Parallel agent queries, non-blocking I/O

3. **reqwest** (0.11) with json feature

   - Purpose: HTTP client for LLM API calls
   - Usage: OpenAI-compatible API requests

4. **anyhow** (1.0) + **thiserror** (1.0)

   - Purpose: Error handling
   - Usage: anyhow for application errors, thiserror for library errors

5. **tracing** (0.1) + **tracing-subscriber** (0.3)

   - Purpose: Structured logging and diagnostics
   - Usage: Debug logging, event tracing

6. **ratatui** (0.25)

   - Purpose: Terminal User Interface (TUI) framework
   - Usage: Replace Python's Textual framework
   - Components: PlayerPanel, GamePanel, ChatPanel, DebugPanel

7. **crossterm** (0.27)

   - Purpose: Terminal manipulation (works with ratatui)
   - Usage: Input handling, terminal setup

**Additional Crates:**

08. **clap** (4.4) with derive feature

    - Purpose: Command-line argument parsing
    - Usage: CLI entry points for TUI/console modes

09. **config** (0.13) or **figment** (0.10)

    - Purpose: Configuration management
    - Usage: Load and validate YAML configs

10. **dashmap** (5.5)

    - Purpose: Concurrent HashMap
    - Usage: Shared game state in async context

11. **derive_more** (0.99)

    - Purpose: Derive macro utilities
    - Usage: Implement common traits easily

12. **strum** (0.25) + **strum_macros** (0.25)

    - Purpose: Enum utilities (string conversion, iteration)
    - Usage: Serialize enums, iterate over variants

**Development Dependencies:**

13. **proptest** (1.4)

    - Purpose: Property-based testing
    - Usage: Test game logic invariants

14. **criterion** (0.5)

    - Purpose: Benchmarking
    - Usage: Performance regression testing

15. **mockall** (0.12)

    - Purpose: Mocking framework
    - Usage: Mock agents and API responses in tests

### Project Structure

```
werewolf-rust/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── configs/
│   ├── demo.yaml
│   └── players.yaml
├── src/
│   ├── main.rs                 # Entry point
│   ├── lib.rs                  # Library root
│   │
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── console.rs          # Console mode
│   │   └── tui.rs              # TUI mode
│   │
│   ├── agents/
│   │   ├── mod.rs
│   │   ├── traits.rs           # Agent trait
│   │   ├── llm.rs              # LLMAgent
│   │   ├── human.rs            # HumanAgent
│   │   └── demo.rs             # DemoAgent
│   │
│   ├── config/
│   │   ├── mod.rs
│   │   ├── game.rs             # GameConfig
│   │   ├── players.rs          # PlayerConfig, PlayersConfig
│   │   └── presets.rs          # Role presets
│   │
│   ├── core/
│   │   ├── mod.rs
│   │   ├── game_engine.rs      # GameEngine
│   │   ├── game_state.rs       # GameState
│   │   ├── player.rs           # Player
│   │   ├── events.rs           # Event system
│   │   ├── victory.rs          # VictoryChecker
│   │   │
│   │   ├── actions/
│   │   │   ├── mod.rs
│   │   │   ├── traits.rs       # Action trait
│   │   │   ├── night.rs        # Night actions
│   │   │   ├── day.rs          # Day actions
│   │   │   └── vote.rs         # Voting actions
│   │   │
│   │   └── roles/
│   │       ├── mod.rs
│   │       ├── traits.rs       # Role trait
│   │       ├── base.rs         # RoleConfig, Camp, Priority
│   │       ├── werewolf.rs     # Werewolf roles
│   │       ├── villager.rs     # Villager roles
│   │       └── neutral.rs      # Neutral roles
│   │
│   └── ui/
│       ├── mod.rs
│       ├── app.rs              # Main TUI app
│       ├── styles.rs           # Color schemes
│       └── components/
│           ├── mod.rs
│           ├── player_panel.rs
│           ├── game_panel.rs
│           ├── chat_panel.rs
│           └── debug_panel.rs
│
└── tests/
    ├── integration/
    │   ├── game_flow.rs
    │   └── victory.rs
    └── unit/
        ├── actions.rs
        ├── roles.rs
        └── state.rs
```

---

## Module-by-Module Rewrite Plan

### Phase 1: Foundation Layer (Weeks 1-2)

#### 1.1 Core Data Structures

**File:** `src/core/game_state.rs`

**Tasks:**

1. Define enums: `GamePhase`, `Camp`, `PlayerStatus`, `ActionPriority`
2. Implement `GameState` struct with all fields
3. Implement state query methods:
   - `get_alive_players(&self) -> Vec<&Player>`
   - `get_players_by_camp(&self, camp: Camp) -> Vec<&Player>`
   - `count_alive_by_camp(&self, camp: Camp) -> usize`
   - `add_vote(&mut self, voter_id: String, target_id: String)`
   - `get_vote_counts(&self) -> HashMap<String, usize>`
4. Implement phase transition:
   - `next_phase(&mut self) -> GamePhase`
   - Reset temporary data when moving to new night phase
5. Add unit tests for all methods

**Python → Rust Translation Notes:**

- Replace `player_dict` with `HashMap<String, usize>` storing index into `players` Vec
- Use `Option<String>` instead of `str | None`
- Use `HashSet<String>` instead of Python `set[str]`
- Consider using `Rc<RefCell<Player>>` if needed for shared ownership

**File:** `src/core/player.rs`

**Tasks:**

1. Define `Player` struct
2. Implement methods:
   - `is_alive(&self) -> bool`
   - `kill(&mut self)`
   - `revive(&mut self)`
   - `can_vote(&self) -> bool`
   - `set_lover(&mut self, partner_id: String)`
   - `get_role_name(&self) -> &str`
   - `get_camp(&self) -> Camp`
3. Handle player status management
4. Add builder pattern for player creation

**Challenges:**

- Role field requires trait object: `Box<dyn Role>`
- Agent field: `Option<Box<dyn Agent>>`
- Need to handle ownership carefully when passing Player to actions

#### 1.2 Configuration System

**File:** `src/config/game.rs`

**Tasks:**

1. Define `GameConfig` struct with serde derives
2. Implement validation logic (min players, werewolf requirement)
3. Implement `to_role_list()` method
4. Add role name → Role type mapping

**File:** `src/config/players.rs`

**Tasks:**

1. Define `PlayerConfig` and `PlayersConfig` structs
2. Implement YAML deserialization with serde_yaml
3. Validate unique player names
4. Validate model-specific fields (base_url for LLMs)

**File:** `src/config/presets.rs`

**Tasks:**

1. Define role preset constants
2. Implement preset lookup function
3. Add preset validation (count matches num_players)

#### 1.3 Event System

**File:** `src/core/events.rs`

**Tasks:**

1. Define `EventType` enum (30+ variants)
2. Define `Event` struct with chrono::DateTime
3. Implement `EventLogger` struct
4. Methods:
   - `create_event(...) -> Event`
   - `log_event(&mut self, event: Event)`
   - `get_events_for_player(&self, player_id: &str) -> Vec<&Event>`
   - `get_events_by_type(&self, event_type: EventType) -> Vec<&Event>`

**Considerations:**

- Use `chrono` crate for timestamps
- Consider using channels for event notification
- Make EventLogger thread-safe with `Arc<Mutex<EventLogger>>`

### Phase 2: Game Logic Layer (Weeks 3-4)

#### 2.1 Role System

**File:** `src/core/roles/traits.rs`

**Tasks:**

1. Define `Role` trait:

```rust
pub trait Role: Send + Sync {
    fn get_config(&self) -> &RoleConfig;
    fn get_night_actions(&self, game_state: &GameState) -> Vec<Box<dyn Action>>;
    fn has_night_action(&self, game_state: &GameState) -> bool;
    fn get_name(&self) -> &str { &self.get_config().name }
    fn get_camp(&self) -> Camp { self.get_config().camp }
}
```

2. Define `RoleConfig` struct:

```rust
pub struct RoleConfig {
    pub name: String,
    pub camp: Camp,
    pub description: String,
    pub priority: Option<ActionPriority>,
    pub can_act_night: bool,
    pub can_act_day: bool,
    pub max_uses: Option<u32>,
}
```

**File:** `src/core/roles/werewolf.rs`

**Tasks:**

1. Implement Werewolf role
2. Implement AlphaWolf role
3. Implement WhiteWolf role (kills werewolf every 2 nights)
4. Implement WolfBeauty role (charm mechanic)
5. Implement other werewolf variants
6. Each role struct implements `Role` trait

**Example:**

```rust
pub struct Werewolf {
    config: RoleConfig,
}

impl Role for Werewolf {
    fn get_config(&self) -> &RoleConfig {
        &self.config
    }

    fn get_night_actions(&self, game_state: &GameState) -> Vec<Box<dyn Action>> {
        // Werewolf kill logic
        // Return WerewolfKillAction
    }
}
```

**File:** `src/core/roles/villager.rs`

**Tasks:**

1. Implement all villager roles (Seer, Witch, Hunter, Guard, etc.)
2. Handle stateful roles (Witch potions, Guard's last_protected)
3. Implement role-specific data fields

**Example (Witch with state):**

```rust
pub struct Witch {
    config: RoleConfig,
    pub has_save_potion: bool,
    pub has_poison_potion: bool,
}
```

#### 2.2 Action System

**File:** `src/core/actions/traits.rs`

**Tasks:**

1. Define `Action` trait:

```rust
pub trait Action: Send + Sync {
    fn get_action_type(&self) -> ActionType;
    fn validate(&self, game_state: &GameState) -> bool;
    fn execute(&mut self, game_state: &mut GameState) -> Vec<String>;
    fn get_priority(&self) -> i32;
}
```

2. Define `ActionType` enum (all action variants)

**File:** `src/core/actions/night.rs`

**Tasks:**

1. Implement night actions:
   - `WerewolfKillAction`
   - `WitchSaveAction`
   - `WitchPoisonAction`
   - `SeerCheckAction`
   - `GuardProtectAction`
   - `CupidLinkAction`
   - Others

**Example:**

```rust
pub struct WerewolfKillAction {
    actor_id: String,
    target_id: String,
}

impl Action for WerewolfKillAction {
    fn validate(&self, game_state: &GameState) -> bool {
        // Check actor and target are alive
    }

    fn execute(&mut self, game_state: &mut GameState) -> Vec<String> {
        game_state.werewolf_target = Some(self.target_id.clone());
        vec![format!("Werewolves target {}", self.target_id)]
    }

    fn get_priority(&self) -> i32 {
        ActionPriority::Werewolf as i32
    }
}
```

**File:** `src/core/actions/day.rs` and `src/core/actions/vote.rs`

Implement day actions and voting similarly.

#### 2.3 Victory System

**File:** `src/core/victory.rs`

**Tasks:**

1. Define `VictoryResult` struct
2. Implement `VictoryChecker`:
   - `check_victory(&self, game_state: &GameState) -> VictoryResult`
   - `check_werewolf_victory(&self, game_state: &GameState) -> VictoryResult`
   - `check_villager_victory(&self, game_state: &GameState) -> VictoryResult`
   - `check_lover_victory(&self, game_state: &GameState) -> VictoryResult`

**Victory Logic:**

```rust
impl VictoryChecker {
    pub fn check_victory(&self, game_state: &GameState) -> VictoryResult {
        // Priority: Lover → Werewolf → Villager
        if let Some(result) = self.check_lover_victory(game_state) {
            return result;
        }
        // ... check other conditions
    }
}
```

#### 2.4 Game Engine

**File:** `src/core/game_engine.rs`

**Tasks:**

1. Define `GameEngine` struct with GameState and EventLogger
2. Implement `setup_game()`: assign roles, initialize state
3. Implement phase methods:
   - `run_night_phase(&mut self) -> Vec<String>`
   - `run_day_phase(&mut self) -> Vec<String>`
   - `run_voting_phase(&mut self) -> Vec<String>`
4. Implement `process_actions(&mut self, actions: Vec<Box<dyn Action>>) -> Vec<String>`
   - Sort actions by priority
   - Execute in order
5. Implement `resolve_deaths(&mut self) -> Vec<String>`
   - Handle Witch save
   - Handle Guard protection
   - Handle Elder 2 lives
   - Handle lover heartbreak
6. Implement `check_victory(&self) -> bool`
7. Implement `step(&mut self) -> Vec<String>` for TUI
8. Implement `play_game(&mut self) -> String` for console mode

**Death Resolution Logic:**

```rust
fn resolve_deaths(&mut self) -> Vec<String> {
    let mut messages = Vec::new();

    // Handle werewolf target
    if let Some(target_id) = &self.game_state.werewolf_target {
        // Check witch save
        if self.game_state.witch_saved_target.as_ref() == Some(target_id) {
            messages.push(format!("{} was saved by witch", target_id));
        }
        // Check guard protection
        else if self.game_state.guard_protected.as_ref() == Some(target_id) {
            messages.push(format!("{} was protected by guard", target_id));
        }
        // Check Elder
        else if let Some(player) = self.get_player_mut(target_id) {
            // Handle Elder 2 lives logic
            // Kill player
            // Check lover heartbreak
        }
    }

    // Handle witch poison
    // ...

    messages
}
```

**Challenges:**

- Manage mutable borrow of GameState across methods
- Handle async agent calls (need tokio)
- Event callback system for UI updates

### Phase 3: Agent Layer (Week 5)

#### 3.1 Agent System

**File:** `src/agents/traits.rs`

**Tasks:**

1. Define `Agent` trait:

```rust
#[async_trait]
pub trait Agent: Send + Sync {
    async fn get_response(&mut self, message: &str) -> Result<String, AgentError>;
    fn get_model_name(&self) -> &str;
}
```

2. Define `AgentError` enum with thiserror

**File:** `src/agents/llm.rs`

**Tasks:**

1. Define `LLMAgent` struct:

```rust
pub struct LLMAgent {
    model_name: String,
    api_key: String,
    base_url: String,
    temperature: f32,
    max_tokens: u32,
    chat_history: Vec<ChatMessage>,
    client: reqwest::Client,
}
```

2. Implement OpenAI-compatible API calls:

```rust
async fn get_response(&mut self, message: &str) -> Result<String, AgentError> {
    self.chat_history.push(ChatMessage {
        role: "user".to_string(),
        content: message.to_string(),
    });

    let request_body = ChatCompletionRequest {
        model: self.model_name.clone(),
        messages: self.chat_history.clone(),
        temperature: self.temperature,
        max_tokens: self.max_tokens,
    };

    let response = self.client
        .post(&format!("{}/chat/completions", self.base_url))
        .header("Authorization", format!("Bearer {}", self.api_key))
        .json(&request_body)
        .send()
        .await?
        .json::<ChatCompletionResponse>()
        .await?;

    let content = response.choices[0].message.content.clone();
    self.chat_history.push(ChatMessage {
        role: "assistant".to_string(),
        content: content.clone(),
    });

    Ok(content)
}
```

**File:** `src/agents/human.rs`

**Tasks:**

1. Implement console input agent
2. Handle stdin reading (blocking)

**File:** `src/agents/demo.rs`

**Tasks:**

1. Implement random response agent
2. Use rand crate for selection

#### 3.2 Agent Factory

**File:** `src/agents/mod.rs`

**Tasks:**

1. Implement agent creation from PlayerConfig:

```rust
pub fn create_agent(config: &PlayerConfig) -> Result<Box<dyn Agent>, AgentError> {
    match config.model.as_str() {
        "human" => Ok(Box::new(HumanAgent::new())),
        "demo" => Ok(Box::new(DemoAgent::new())),
        _ => {
            let api_key = std::env::var(&config.api_key_env?)
                .map_err(|_| AgentError::MissingApiKey(config.api_key_env.clone()))?;

            Ok(Box::new(LLMAgent::new(
                config.model.clone(),
                api_key,
                config.base_url.clone().ok_or(AgentError::MissingBaseUrl)?,
                config.temperature,
                config.max_tokens,
            )))
        }
    }
}
```

### Phase 4: User Interface Layer (Weeks 6-7)

#### 4.1 Console Mode

**File:** `src/cli/console.rs`

**Tasks:**

1. Implement auto-play game loop
2. Print events to stdout using Rich-style formatting
3. Command-line argument parsing with clap

**Example:**

```rust
pub async fn run_console_mode(config_path: &str) -> Result<()> {
    let players_config = load_players_config(config_path)?;
    let game_config = load_game_config(&players_config.preset)?;

    let mut engine = GameEngine::new(game_config);
    engine.setup_game(/* players and roles */)?;

    while !engine.check_victory() {
        engine.run_night_phase().await?;
        if engine.check_victory() { break; }

        engine.run_day_phase().await?;
        engine.run_voting_phase().await?;
    }

    println!("{}", engine.get_victory_message());
    Ok(())
}
```

#### 4.2 TUI Application

**File:** `src/ui/app.rs`

**Tasks:**

1. Define main TUI app struct:

```rust
pub struct WerewolfTUI {
    game_engine: GameEngine,
    show_debug: bool,
    event_receiver: mpsc::Receiver<Event>,

    // Component state
    player_panel_state: PlayerPanelState,
    chat_panel_state: ChatPanelState,
    // ...
}
```

2. Implement ratatui App trait
3. Handle input events (key presses)
4. Render UI components

**File:** `src/ui/components/player_panel.rs`

**Tasks:**

1. Render player list with status indicators
2. Use ratatui widgets (List, Table)
3. Color-code based on alive/dead status

**File:** `src/ui/components/game_panel.rs`

**Tasks:**

1. Display current phase and round
2. Show faction counts
3. Display vote tallies

**File:** `src/ui/components/chat_panel.rs`

**Tasks:**

1. Implement scrollable event log
2. Color-code events by type
3. Handle text wrapping

**File:** `src/ui/components/debug_panel.rs`

**Tasks:**

1. Show role assignments
2. Display night action state
3. Show configuration details

**Key Bindings Implementation:**

```rust
fn handle_input(&mut self, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('q') => self.should_quit = true,
        KeyCode::Char('d') => self.toggle_debug(),
        KeyCode::Char('n') => self.next_step().await?,
        _ => {}
    }
    Ok(())
}

async fn next_step(&mut self) -> Result<()> {
    let messages = self.game_engine.step().await?;
    for msg in messages {
        self.add_system_message(msg);
    }
    Ok(())
}
```

### Phase 5: Integration and Polish (Week 8)

#### 5.1 Main Entry Points

**File:** `src/main.rs`

**Tasks:**

1. Parse command-line arguments with clap
2. Route to TUI or console mode
3. Load configuration
4. Initialize logging with tracing

**Example:**

```rust
#[derive(Parser)]
#[command(name = "werewolf")]
#[command(about = "LLM-powered Werewolf game")]
struct Cli {
    /// Path to config file
    config: PathBuf,

    /// Run in console mode (auto-play)
    #[arg(long)]
    console: bool,

    /// Show debug panel in TUI mode
    #[arg(long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt::init();

    if cli.console {
        run_console_mode(&cli.config).await?;
    } else {
        run_tui_mode(&cli.config, cli.debug).await?;
    }

    Ok(())
}
```

#### 5.2 Error Handling

**File:** `src/error.rs`

**Tasks:**

1. Define comprehensive error types with thiserror
2. Implement error conversion from dependencies
3. Add context with anyhow where appropriate

**Example:**

```rust
#[derive(Error, Debug)]
pub enum WerewolfError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Agent error: {0}")]
    Agent(#[from] AgentError),

    #[error("Game state error: {0}")]
    GameState(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

#### 5.3 Logging and Observability

**Tasks:**

1. Add tracing spans to all major functions
2. Log important events at appropriate levels
3. Add metrics for performance monitoring

**Example:**

```rust
#[tracing::instrument(skip(self))]
async fn run_night_phase(&mut self) -> Result<Vec<String>> {
    tracing::info!("Starting night phase for round {}", self.game_state.round_number);

    let actions = self.collect_night_actions().await?;
    tracing::debug!("Collected {} night actions", actions.len());

    let messages = self.process_actions(actions)?;
    Ok(messages)
}
```

---

## Implementation Phases

### Phase 1: Foundation (Weeks 1-2)

**Goal:** Core data structures and configuration system

**Deliverables:**

- ✅ GameState with all fields and methods
- ✅ Player entity with status management
- ✅ Configuration structs with YAML parsing
- ✅ Event system with EventLogger
- ✅ Enums: GamePhase, Camp, PlayerStatus, ActionPriority, EventType
- ✅ Unit tests for all components (>80% coverage)

**Success Criteria:**

- Can load YAML config file
- Can create GameState with players
- Can transition between game phases
- Events can be created and logged

### Phase 2: Game Logic (Weeks 3-4)

**Goal:** Implement all game rules and mechanics

**Deliverables:**

- ✅ Role trait and all 20+ role implementations
- ✅ Action trait and all action types
- ✅ VictoryChecker with all victory conditions
- ✅ GameEngine with complete game loop
- ✅ Death resolution logic (Witch, Guard, Elder, Lover)
- ✅ Integration tests for game scenarios

**Success Criteria:**

- Can play a complete game programmatically
- All roles have correct night actions
- Victory conditions trigger correctly
- Death mechanics work properly (protection, potions, lovers)

### Phase 3: Agent System (Week 5)

**Goal:** LLM and human agent integration

**Deliverables:**

- ✅ Agent trait (async)
- ✅ LLMAgent with OpenAI-compatible API calls
- ✅ HumanAgent with console input
- ✅ DemoAgent with random responses
- ✅ Async agent querying in GameEngine
- ✅ Mock agents for testing

**Success Criteria:**

- LLMAgent successfully calls GPT/Claude APIs
- Chat history persists across turns
- Agents can be created from PlayerConfig
- Concurrent agent calls work (tokio)

### Phase 4: User Interface (Weeks 6-7)

**Goal:** TUI and console interfaces

**Deliverables:**

- ✅ Console mode (auto-play)
- ✅ ratatui-based TUI application
- ✅ All UI components (PlayerPanel, GamePanel, ChatPanel, DebugPanel)
- ✅ Event-driven UI updates
- ✅ Keyboard input handling
- ✅ Color schemes and styling

**Success Criteria:**

- TUI displays game state correctly
- Can advance game step-by-step with 'n' key
- Events appear in chat panel
- Debug panel shows internal state
- Console mode runs full games automatically

### Phase 5: Polish & Release (Week 8)

**Goal:** Production-ready release

**Deliverables:**

- ✅ Comprehensive documentation (README, API docs)
- ✅ Example configurations
- ✅ CI/CD pipeline (GitHub Actions)
- ✅ Binary releases for Linux, macOS, Windows
- ✅ Performance benchmarks
- ✅ Migration guide from Python version

**Success Criteria:**

- Binary runs on all platforms
- Documentation is complete and accurate
- Test coverage >80%
- Benchmarks show >10x speedup over Python
- No memory leaks or panics

---

## Testing Strategy

### Unit Tests

**Coverage Target:** >80% line coverage

**Test Structure:**

```
tests/unit/
├── actions.rs          # Test each action type
├── roles.rs            # Test each role implementation
├── game_state.rs       # Test state transitions
├── player.rs           # Test player methods
├── events.rs           # Test event creation
├── victory.rs          # Test victory conditions
└── config.rs           # Test config parsing
```

**Example Test:**

```rust
#[test]
fn test_witch_save_action() {
    let mut game_state = create_test_game_state();
    let witch = create_test_player("witch", Box::new(Witch::new()));
    let target = create_test_player("target", Box::new(Villager::new()));

    let mut action = WitchSaveAction::new(witch.player_id, target.player_id);
    assert!(action.validate(&game_state));

    let messages = action.execute(&mut game_state);
    assert_eq!(game_state.witch_saved_target, Some(target.player_id));
    assert!(!witch.role.has_save_potion);
}
```

### Integration Tests

**Test Scenarios:**

1. **Full Game Flow**: Play complete 6-player game, verify victory
2. **Role Interactions**: Test Witch save + Guard protect, lover heartbreak
3. **Vote Mechanics**: Test ties, Idiot survival, Raven marks
4. **Special Abilities**: Hunter revenge, Knight duel, Cupid linking

**Example Integration Test:**

```rust
#[tokio::test]
async fn test_full_game_villager_victory() {
    let config = create_test_config("6-players");
    let mut engine = GameEngine::new(config);

    // Set up with 2 werewolves, 4 villagers
    engine.setup_game(/* ... */)?;

    // Play until werewolves eliminated
    while !engine.check_victory() {
        engine.run_night_phase().await?;
        engine.run_day_phase().await?;
        engine.run_voting_phase().await?;
    }

    let result = engine.get_victory_result();
    assert!(result.has_winner);
    assert_eq!(result.winner_camp, "villager");
}
```

### Property-Based Tests

Use **proptest** for invariant testing:

```rust
proptest! {
    #[test]
    fn prop_alive_count_never_exceeds_total(
        num_players in 6usize..=20,
        deaths in prop::collection::vec(0usize..20, 0..10)
    ) {
        let mut game_state = create_game_state_with_players(num_players);

        for death_idx in deaths {
            if death_idx < num_players {
                game_state.players[death_idx].kill();
            }
        }

        let alive_count = game_state.get_alive_players().len();
        assert!(alive_count <= num_players);
    }
}
```

### Performance Benchmarks

Use **criterion** for performance regression testing:

```rust
fn bench_game_engine_step(c: &mut Criterion) {
    c.bench_function("game_engine_step", |b| {
        let mut engine = create_test_engine();
        b.iter(|| {
            black_box(engine.step());
        });
    });
}
```

**Benchmark Targets:**

- GameEngine.step(): \<10ms per phase
- Action processing: \<1ms for 20 actions
- Victory check: \<100µs
- Event logging: \<50µs per event

### Mock Testing

Use **mockall** for mocking agents:

```rust
mock! {
    pub Agent {}

    #[async_trait]
    impl Agent for Agent {
        async fn get_response(&mut self, message: &str) -> Result<String, AgentError>;
        fn get_model_name(&self) -> &str;
    }
}

#[tokio::test]
async fn test_day_phase_with_mock_agent() {
    let mut mock_agent = MockAgent::new();
    mock_agent
        .expect_get_response()
        .times(1)
        .returning(|_| Ok("I suspect player 2.".to_string()));

    // Use mock_agent in game engine
    // Verify behavior
}
```

---

## Migration Path

### Compatibility Considerations

**Config File Compatibility:**

- ✅ Keep YAML format identical to Python version
- ✅ Same field names and structure
- ✅ Support same role presets
- ❌ No breaking changes to user configs

**API Compatibility:**

- ✅ Same OpenAI-compatible API format
- ✅ Same environment variable names for API keys
- ✅ Same chat completion request/response format

### Data Migration

**No migration needed** - this is a clean rewrite, not a data migration. The game doesn't persist state between runs.

### Deployment Strategy

**Binary Distribution:**

1. Build release binaries for each platform:

   ```bash
   # Linux
   cargo build --release --target x86_64-unknown-linux-gnu

   # macOS (Intel)
   cargo build --release --target x86_64-apple-darwin

   # macOS (Apple Silicon)
   cargo build --release --target aarch64-apple-darwin

   # Windows
   cargo build --release --target x86_64-pc-windows-msvc
   ```

2. Package binaries with example configs

3. Distribute via GitHub Releases

**Docker Image:**

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/werewolf /usr/local/bin/
COPY configs/ /configs/
CMD ["werewolf", "/configs/demo.yaml"]
```

### Feature Parity Checklist

**Core Features:**

- ✅ All 20+ roles implemented
- ✅ All action types (night/day/vote)
- ✅ Victory conditions (werewolf/villager/lover)
- ✅ Death mechanics (protection, potions, lovers, Elder)
- ✅ Event system with full logging
- ✅ LLM agent integration
- ✅ Human player support
- ✅ Demo agent for testing

**UI Features:**

- ✅ TUI mode with ratatui
- ✅ Console mode (auto-play)
- ✅ All panels (Player, Game, Chat, Debug)
- ✅ Keyboard controls (q, d, n)
- ✅ Color-coded events

**Configuration:**

- ✅ YAML-based config
- ✅ Role presets (6, 9, 12, 15 players, expert, chaos)
- ✅ Player config with agent selection
- ✅ Game config with timeouts and rules

### Deprecation Plan

**Python Version:**

- Keep Python version maintained until Rust version reaches v1.0
- Add deprecation notice to Python README
- Redirect new users to Rust version
- Eventually archive Python repository

---

## Performance Considerations

### Expected Performance Improvements

**Metrics to Track:**

1. **Startup Time**

   - Python: ~500ms (import overhead)
   - Rust: \<50ms (compiled binary)
   - **Target:** 10x faster

2. **Game Step Execution**

   - Python: ~20-50ms per phase
   - Rust: \<5ms per phase
   - **Target:** 5-10x faster

3. **LLM API Call Latency**

   - Dominated by network/API, minimal difference
   - Rust may have slight edge in async handling

4. **Memory Usage**

   - Python: ~50-100MB (runtime + dependencies)
   - Rust: \<10MB (no runtime, minimal allocations)
   - **Target:** 5-10x less memory

5. **Binary Size**

   - Python: N/A (requires interpreter)
   - Rust: ~5-10MB (release, stripped)
   - **Target:** Single binary distribution

### Optimization Opportunities

**1. Concurrent Agent Queries:**

```rust
async fn collect_night_actions(&mut self) -> Result<Vec<Box<dyn Action>>> {
    let players = self.game_state.get_players_with_night_actions();

    // Query all agents concurrently
    let futures: Vec<_> = players
        .into_iter()
        .map(|player| async move {
            let context = self.build_night_context(player);
            player.agent.get_response(&context).await
        })
        .collect();

    let responses = join_all(futures).await;

    // Parse responses into actions
    // ...
}
```

**2. Zero-Copy String Handling:**
Use `&str` instead of `String` where possible to avoid allocations.

**3. Pre-allocated Collections:**

```rust
// Pre-allocate vectors with known capacity
let mut actions = Vec::with_capacity(num_players);
let mut messages = Vec::with_capacity(10);
```

**4. Enum Dispatch Instead of Trait Objects:**
For actions with known types, use enum dispatch:

```rust
pub enum ActionEnum {
    WerewolfKill(WerewolfKillAction),
    WitchSave(WitchSaveAction),
    // ...
}

impl ActionEnum {
    fn execute(&mut self, game_state: &mut GameState) -> Vec<String> {
        match self {
            ActionEnum::WerewolfKill(a) => a.execute(game_state),
            ActionEnum::WitchSave(a) => a.execute(game_state),
            // ...
        }
    }
}
```

**5. Smart Pointer Optimization:**

- Use `Rc` instead of `Arc` in single-threaded contexts
- Use `&` references instead of cloning when possible
- Consider using `Cow<str>` for strings that may or may not be owned

### Profiling Strategy

**Tools:**

1. **cargo-flamegraph**: CPU profiling
2. **heaptrack**: Memory profiling
3. **criterion**: Benchmark regression testing
4. **tokio-console**: Async task monitoring

**Profiling Workflow:**

```bash
# CPU profiling
cargo flamegraph --bin werewolf -- configs/demo.yaml --console

# Memory profiling
heaptrack target/release/werewolf configs/demo.yaml --console

# Benchmarking
cargo bench --bench game_engine
```

---

## Appendix A: Critical Python Code Snippets

### GameEngine Night Phase

```python
def run_night_phase(self) -> list[str]:
    messages = []
    self.game_state.set_phase(GamePhase.NIGHT)

    # Get all players with night actions
    players_with_night_actions = self.game_state.get_players_with_night_actions()

    # Collect actions from each player
    night_actions: list[Action] = []
    for player in players_with_night_actions:
        action = player.role.get_night_actions(self.game_state)
        if action:
            night_actions.extend(action)

    # Process actions in priority order
    action_messages = self.process_actions(night_actions)
    messages.extend(action_messages)

    # Resolve deaths
    death_messages = self.resolve_deaths()
    messages.extend(death_messages)

    return messages
```

### Death Resolution with Special Cases

```python
def _handle_werewolf_kill(self, target: Player) -> list[str]:
    messages = []

    # Check witch save
    if self.game_state.witch_saved_target == target.player_id:
        messages.append(f"{target.name} was saved by the witch!")
    # Check guard protection
    elif self.game_state.guard_protected == target.player_id:
        messages.append(f"{target.name} was protected by the guard!")
    else:
        # Check Elder with 2 lives
        if isinstance(target.role, Elder) and target.role.lives > 1:
            target.role.lives -= 1
            messages.append(f"{target.name} survived (Elder)!")
        else:
            target.kill()
            self.game_state.night_deaths.add(target.player_id)

            # Check lover heartbreak
            if target.is_lover() and target.lover_partner_id:
                partner = self.game_state.get_player(target.lover_partner_id)
                if partner and partner.is_alive():
                    partner.kill()
                    self.game_state.night_deaths.add(partner.player_id)
                    messages.append(f"{partner.name} died of heartbreak!")

    return messages
```

### Action Priority Sorting

```python
def process_actions(self, actions: list[Action]) -> list[str]:
    def get_action_priority(action: Action) -> int:
        priority_map = {
            "GuardProtectAction": 0,
            "WerewolfKillAction": 1,
            "WitchSaveAction": 2,
            "WitchPoisonAction": 3,
            "SeerCheckAction": 4,
        }
        return priority_map.get(action.__class__.__name__, 100)

    sorted_actions = sorted(actions, key=get_action_priority)

    messages = []
    for action in sorted_actions:
        if action.validate():
            result_messages = action.execute()
            messages.extend(result_messages)

    return messages
```

---

## Appendix B: Rust Type Signatures Reference

### Core Traits

```rust
// Role trait
pub trait Role: Send + Sync {
    fn get_config(&self) -> &RoleConfig;
    fn get_night_actions(&self, game_state: &GameState) -> Vec<Box<dyn Action>>;
    fn has_night_action(&self, game_state: &GameState) -> bool;
}

// Action trait
pub trait Action: Send + Sync {
    fn get_action_type(&self) -> ActionType;
    fn validate(&self, game_state: &GameState) -> bool;
    fn execute(&mut self, game_state: &mut GameState) -> Vec<String>;
    fn get_priority(&self) -> i32;
}

// Agent trait
#[async_trait]
pub trait Agent: Send + Sync {
    async fn get_response(&mut self, message: &str) -> Result<String, AgentError>;
    fn get_model_name(&self) -> &str;
}
```

### Key Structures

```rust
pub struct GameState {
    pub players: Vec<Player>,
    player_map: HashMap<String, usize>,
    pub phase: GamePhase,
    pub round_number: u32,
    pub night_deaths: HashSet<String>,
    pub day_deaths: HashSet<String>,
    pub werewolf_target: Option<String>,
    pub witch_saved_target: Option<String>,
    pub witch_poison_target: Option<String>,
    pub guard_protected: Option<String>,
    pub seer_checked: HashMap<u32, String>,
    pub votes: HashMap<String, String>,
    pub raven_marked: Option<String>,
    pub winner: Option<String>,
}

pub struct Player {
    pub player_id: String,
    pub name: String,
    pub role: Box<dyn Role>,
    pub agent: Option<Box<dyn Agent>>,
    pub ai_model: String,
    alive: bool,
    statuses: HashSet<PlayerStatus>,
    lover_partner_id: Option<String>,
    can_vote_flag: bool,
}

pub struct GameEngine {
    config: GameConfig,
    game_state: Option<GameState>,
    event_logger: EventLogger,
    victory_checker: Option<VictoryChecker>,
}
```

---

## Appendix C: Dependencies Summary

```toml
[dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }
async-trait = "0.1"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"

# HTTP client
reqwest = { version = "0.11", features = ["json"] }

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# TUI
ratatui = "0.25"
crossterm = "0.27"

# CLI
clap = { version = "4.4", features = ["derive"] }

# Config management
config = "0.13"

# Utilities
derive_more = "0.99"
strum = { version = "0.25", features = ["derive"] }
strum_macros = "0.25"
dashmap = "5.5"
chrono = { version = "0.4", features = ["serde"] }
rand = "0.8"

[dev-dependencies]
proptest = "1.4"
criterion = "0.5"
mockall = "0.12"
tokio-test = "0.4"
```

---

## Conclusion

This specification provides a comprehensive roadmap for rewriting the LLM Werewolf game from Python to Rust. The rewrite will deliver significant improvements in performance, memory safety, type safety, and deployment simplicity while maintaining full feature parity with the original implementation.

**Key Success Metrics:**

- ✅ 100% feature parity with Python version
- ✅ 10x performance improvement
- ✅ 80%+ test coverage
- ✅ Single binary distribution
- ✅ Zero memory safety issues
- ✅ Production-ready TUI and console modes

**Timeline:** 8 weeks from start to production release

**Next Steps:**

1. Set up Rust project structure
2. Begin Phase 1: Foundation layer implementation
3. Establish CI/CD pipeline for continuous testing
4. Iterative development following phase plan
5. Performance benchmarking throughout
6. Beta testing with real LLM agents
7. Documentation and release preparation
