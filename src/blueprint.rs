use core::num::NonZeroUsize;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Object with keys starting from 1
pub type ConnectionWrapper = HashMap<NonZeroUsize, Connection>;
/// Direction of an entity (gives no further explanation, todo, after decoding simple BPs infer directions)
pub type Direction = usize;
/// No further explanation given.
pub type GraphicsVariation = u8;
/// The number of items in a given stack.
pub type ItemCountType = u32;
/// 1 or more instances of key/value pairs. Key is the name of the item, string. Value is the amount of items to be requested
pub type ItemRequest = HashMap<String, ItemCountType>;
/// The index of an item stack in a container.
pub type ItemStackIndex = u16;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A book full of many blueprints.
pub struct BlueprintBook {
    /// The name of the item that was saved ("blueprint-book" in vanilla).
    pub item: String,
    /// The name of the blueprint set by the user.
    pub label: String,
    /// The color of the label of this blueprint.
    pub label_color: Color,
    /// The actual content of the blueprint book.
    pub blueprints: Vec<BookBpWrapper>,
    /// Index of the currently selected blueprint
    pub active_index: usize,
    /// The map version of the map the blueprint [book] was created in.
    pub version: Version,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "u64")]
/// A Factorio game version
pub struct Version {
    major: u16,
    minor: u16,
    patch: u16,
    developer: u16,
}

impl From<u64> for Version {
    fn from(value: u64) -> Self {
        // The factorio version string "consists of four consecutive 2-byte unsigned little-endian values".
        let bytes = value.to_be_bytes();

        let mut version_numbers: [u16; 4] = [0; 4];
        for i in 0..4 {
            version_numbers[i] = u16::from_le_bytes([bytes[i * 2], bytes[i * 2 + 1]])
        }

        Self {
            major: version_numbers[0],
            minor: version_numbers[1],
            patch: version_numbers[2],
            developer: version_numbers[3],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A wrapper around a blueprint for use in blueprint books so its index in the book can be easily referenced back to.
pub struct BookBpWrapper {
    /// The index of the blueprint in its book
    pub index: usize,
    /// The actual blueprint
    pub blueprint: Blueprint,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A factorio blueprint.
pub struct Blueprint {
    /// The name of the item that was saved ("blueprint" in vanilla).
    pub item: String,
    /// The name of the blueprint set by the user.
    pub label: String,
    /// The color of the label of this blueprint.
    pub label_color: Color,
    /// The actual content of the blueprint
    pub entities: Vec<Entity>,
    /// The tiles included in the blueprint.
    pub tiles: Vec<Tile>,
    /// The icons of the blueprint set by the user.
    pub icons: Vec<Icon>,
    /// The schedules for trains in this blueprint.
    pub schedules: Vec<Schedule>,
    /// The map version of the map the blueprint was created in.
    pub version: Version,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// An icon displayed in an inventory
pub struct Icon {
    /// Index of the icon.
    pub index: NonZeroUsize,
    /// The icon that is displayed.
    pub signal: SignalId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A circuit signal ID.
pub struct SignalId {
    /// The name of the signal prototype this signal is set to.
    pub name: String,
    #[serde(rename = "type")]
    /// Type of the signal.
    pub signal_type: SignalType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// The type of a circuit signal
pub enum SignalType {
    /// Represents an item (e.g. iron plate)
    Item,
    /// Represents a fluid (e.g. crude oil)
    Fluid,
    /// Represents a virtual value (e.g. "Signal 0")
    Virtual,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A placed structure in the blueprint (e.g. Assembling Machine 3)
pub struct Entity {
    /// Index of the entity.
    pub entity_number: NonZeroUsize,
    /// Prototype name of the entity.
    pub name: String,
    /// Position of the entity within the blueprint.
    pub position: Position,
    /// Direction of the entity.
    pub direction: Option<Direction>,
    /// Orientation of cargo wagon or locomotive, value 0 to 1.
    pub orientation: Option<f64>,
    /// Circuit connection.
    pub connections: Option<ConnectionWrapper>,
    #[serde(rename = "neighbours")]
    /// Copper wire connections
    pub neighbors: Option<Vec<NonZeroUsize>>,
    /// Item requests by this entity; this is what defines the item-request-proxy when the blueprint is placed.
    pub items: ItemRequest,
    /// Name of the recipe prototype this assembling machine is set to.
    pub recipe: String,
    /// Used by (Prototype/Container)[https://wiki.factorio.com/Prototype/Container]. The index of the first inaccessible item slot due to limiting with the red "bar".
    pub bar: ItemStackIndex,
    /// Cargo wagon inventory configuration.
    pub inventory: Inventory,
    /// Used by (Prototype/InfinityContainer)[https://wiki.factorio.com/Prototype/InfinityContainer].
    pub infinity_settings: InfinitySettings,
    #[serde(rename = "type")]
    /// Type of the underground belt or loader.
    pub io_type: IoType,
    /// Input priority of the splitter.
    pub input_priority: Option<IoPriority>,
    /// Output priority of the splitter.
    pub output_priority: Option<IoPriority>,
    /// Filter of the splitter. Name of the item prototype the filter is set to.
    pub filter: Option<String>,
    /// Filters of the filter inserter or loader.
    pub filters: Option<Vec<ItemFilter>>,
    /// Filter mode of the filter inserter.
    pub filter_mode: FilterMode,
    /// The stack size the inserter is set to.
    pub override_stack_size: Option<u8>,
    /// The drop position the inserter is set to.
    pub drop_position: Position,
    /// The pickup potition the inserter is set to.
    pub pickup_position: Position,
    /// Used by (Prototype/LogisticContainer)[https://wiki.factorio.com/Prototype/LogisticContainer].
    pub request_filters: Option<LogisticFilter>,
    /// Whether this requester chest can request from buffer chests
    pub request_from_buffers: bool,
    /// Used by (Programmable speaker)[https://wiki.factorio.com/Programmable_speaker],
    pub parameters: SpeakerParameter,
    /// Used by (Programmable speaker)[https://wiki.factorio.com/Programmable_speaker],
    pub alert_parameters: SpeakerAlertParameter,
    /// Used by the rocket silo. Whether auto launch is enabled.
    pub auto_launch: bool,
    /// Used by (Prototype/SimpleEntityWithForce)[https://wiki.factorio.com/Prototype/SimpleEntityWithForce] or (Prototype/SimpleEntityWithOwner)[https://wiki.factorio.com/Prototype/SimpleEntityWithOwner]
    pub variation: GraphicsVariation,
    /// Color of the (Prototype/SimpleEntityWithForce)[https://wiki.factorio.com/Prototype/SimpleEntityWithForce], (Prototype/SimpleEntityWithOwner)[https://wiki.factorio.com/Prototype/SimpleEntityWithOwner], or train station
    pub color: Color,
    /// The name of the train station,
    pub station: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// The type of an underground belt or loader
pub enum IoType {
    /// Inserts into an inventory (down for underground)
    Input,
    /// Removes from an inventory (up for underground)
    Output,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// The input/output priority for a splitter.
pub enum IoPriority {
    /// Priority to/from the right.
    Right,
    /// Priority to/from the left.
    Left,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// The mode for a filter inserter.
pub enum FilterMode {
    /// Only allow the selected item(s).
    Whitelist,
    /// Allow everything except for the selected item(s).
    Blacklist,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// An inventory of a non-logistics container.
pub struct Inventory {
    /// Array of item filters
    pub filters: Vec<ItemFilter>,
    /// The index of the first inaccessible item slot due to limiting with the red "bar".
    pub bar: ItemStackIndex,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A train's automation schedule.
pub struct Schedule {
    /// Array of schedule records.
    pub schedule: Vec<ScheduleRecord>,
    /// Array of entity numbers of locomotives using this schedule.
    pub locomotives: Vec<NonZeroUsize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A single item in a train's automation schedule.
pub struct ScheduleRecord {
    /// The name of the stop for this schedule record.
    pub station: String,
    /// Array of wait conditions.
    pub wait_conditions: Vec<WaitCondition>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A wait condition for train automation.
pub struct WaitCondition {
    #[serde(rename = "type")]
    /// The wait condition type.
    pub condition_type: ConditionType,
    /// Tells how this condition is to be compared with the preceeding conditions in the corresponding wait_conditions array.
    pub compare_type: CompareType,
    /// Number of ticks to wait or of inactivity. Only present when type is "time" or "inactivity".
    pub ticks: Option<usize>,
    #[serde(skip)]
    #[serde(default)]
    /// CircuitCondition Object, only present when type is "item_count", "circuit" or "fluid_count".
    pub condition: Option<CircuitCondition>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The circuit condition object is not expanded on the Factorio Wiki.
/// It's likely to be
/// - an item name
/// - a count
/// - <, >, >=, <=, =, =/=
/// based on the train item count conditions, but it could be more.
pub struct CircuitCondition {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// The types of wait conditions that can be used in a train schedule
pub enum ConditionType {
    /// Wait until an amount of time has passed.
    Time,
    /// Wait until the train has been inactive for a period of time.
    Inactivity,
    /// Wait until the train cargo is full.
    Full,
    /// Wait until the train cargo is empty.
    Empty,
    /// Wait until the train cargo reaches an item condition.
    ItemCount,
    /// Wait until the circuit network reaches a condition.
    Circuit,
    /// Unsure. This is not an option for wait conditions in vanilla.
    RobotsInactive,
    /// Wait until the train cargo reaches a fluid condition.
    FluidCount,
    /// Wait until a passenger is present.
    PassengerPresent,
    /// Wait until a passenger is not present.
    PassengerNotPresent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// Determines how multiple comparisons in a wait condition combine
pub enum CompareType {
    /// The left AND right (or top AND bottom) side must be true.
    And,
    /// Only one of the left OR right (or top OR bottom) sides must be true.
    Or,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Floor tiles in a blueprint, i.e. landfill, concrete, stone bricks
pub struct Tile {
    /// Prototype name of the tile (e.g. "concrete").
    pub name: String,
    /// Position of the entity within the blueprint.
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A position within a blueprint
pub struct Position {
    /// X position within the blueprint, 0 is the center.
    pub x: f64,
    /// Y position within the blueprint, 0 is the center.
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Object containing information about the connections to other entities formed by red or green wires.
pub struct Connection {
    #[serde(rename = "1")]
    /// First connection point. The default for everything that doesn't have multiple connection points.
    pub first: ConnectionPoint,
    #[serde(rename = "2")]
    /// Second connection point. For example, the "output" part of an arithmetic combinator.
    pub second: ConnectionPoint,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The actual point where a wire is connected to. Contains information about where it is connected to.
pub struct ConnectionPoint {
    /// An array containing all the connections from this point created by red wire.
    pub red: Vec<ConnectionData>,
    /// An array containing all the connections from this point created by green wire.
    pub green: Vec<ConnectionData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Information about a single connection between two connection points.
pub struct ConnectionData {
    /// ID of the entity this connection is connected with.
    pub entity_id: NonZeroUsize,
    /// The circuit connector id of the entity this connection is connected to
    pub circuit_id: CircuitConnectorId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(C)]
/// The type of entity a circuit is connected to.
pub enum CircuitConnectorId {
    /// An accumulator
    Accumulator,
    /// A constant combinator
    ConstantCombinator,
    /// A container (e.g. steel chest)
    Container,
    /// Any container linked to a logistics network (e.g. passive provider chest)
    LinkedContainer,
    /// A programmable speaker
    ProgrammableSpeaker,
    /// A rail signal
    RailSignal,
    /// A rail chain signal
    RailChainSignal,
    /// A roboport
    Roboport,
    /// A fluid storage tank
    StorageTank,
    /// A wall or gate
    Wall,
    /// Any electric pole (e.g. medium power pole)
    ElectricPole,
    /// Any inserter (e.g. fast inserter)
    Inserter,
    /// A lamp
    Lamp,
    /// Any combinator's (e.g. constant combinator) input
    CombinatorInput,
    /// Any combinator's (e.g. constant combinator) output
    CombinatorOutput,
    /// An offshore pump
    OffshorePump,
    /// A pump
    Pump,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// An item filter in a non-logistics container.
pub struct ItemFilter {
    /// Name of the item prototype this filter is set to.
    pub name: String,
    /// Index of the filter.
    pub index: NonZeroUsize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The settings on an "infinite" container
pub struct InfinitySettings {
    /// Whether the "remove unfiltered items" checkbox is checked.
    pub remove_unfiltered_items: bool,
    /// Filters of the infinity container.
    pub filters: Option<Vec<InfinityFilter>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A filter for items/fluids in an "infinite" container.
pub struct InfinityFilter {
    /// Name of the item prototype the filter is set to.
    pub name: String,
    /// Number the filter is set to.
    pub count: ItemCountType,
    /// Mode of the filter.
    pub mode: InfinityFilterMode,
    /// Index of the filter.
    pub index: NonZeroUsize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// The mode for filters in an "infinite" container.
pub enum InfinityFilterMode {
    /// Ensure there are "at-least" X of the item in the container.
    AtLeast,
    /// Ensure there are "at-most" X of the item in the container.
    AtMost,
    /// Ensure there are "exactly" X of the item in the container.
    Exactly,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A logistics filter/request in a logistics container.
pub struct LogisticFilter {
    /// Name of the item prototype this filter is set to.
    pub name: String,
    /// Index of the filter.
    pub index: NonZeroUsize,
    /// Number the filter is set to. Is 0 for storage chests.
    pub count: ItemCountType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Parameters for a speaker.
pub struct SpeakerParameter {
    /// Volume of the speaker.
    pub playback_volume: f64,
    /// Whether global playback is enabled.
    pub playback_globally: bool,
    /// Whether polyphony is allowed.
    pub allow_polyphony: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Parameters for a speaker alert.
pub struct SpeakerAlertParameter {
    /// Whether an alert is shown.
    pub show_alert: bool,
    /// Whether an alert icon is shown on the map.
    pub show_on_map: bool,
    /// The icon that is displayed with the alert.
    pub icon_signal_id: SignalId,
    /// Message of the alert.
    pub alert_message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Determines the color of an object.
pub struct Color {
    /// Red, 0 to 1.
    pub r: f64,
    /// Green, 0 to 1.
    pub g: f64,
    /// Blue, 0 to 1.
    pub b: f64,
    /// Transparency, 0 to 1.
    pub a: f64,
}
