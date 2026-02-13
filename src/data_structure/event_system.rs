use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::sync::{Arc, Mutex};
use super::ObjectId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    SelectionChanged,
    EntityAdded,
    EntityRemoved,
    EntityModified,
    LayerAdded,
    LayerRemoved,
    LayerModified,
    BlockAdded,
    BlockRemoved,
    BlockModified,
    DocumentModified,
    ViewportChanged,
    CursorMoved,
    KeyPressed,
    MouseClicked,
    MouseDragged,
    ZoomChanged,
    GridSnapped,
    ToolChanged,
    LayerVisibilityChanged,
    LayerLockChanged,
    SelectionHighlighted,
    ObjectHovered,
    TransactionStarted,
    TransactionEnded,
    UndoPerformed,
    RedoPerformed,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct Event {
    event_type: EventType,
    timestamp: std::time::SystemTime,
    source: Option<ObjectId>,
    data: Option<Box<dyn Any>>,
    propagated: bool,
}

impl Event {
    #[inline]
    pub fn new(event_type: EventType) -> Self {
        Self {
            event_type,
            timestamp: std::time::SystemTime::now(),
            source: None,
            data: None,
            propagated: false,
        }
    }

    #[inline]
    pub fn with_source(mut self, source: ObjectId) -> Self {
        self.source = Some(source);
        self
    }

    #[inline]
    pub fn with_data<T: Any>(mut self, data: T) -> Self {
        self.data = Some(Box::new(data));
        self
    }

    #[inline]
    pub fn event_type(&self) -> &EventType {
        &self.event_type
    }

    #[inline]
    pub fn timestamp(&self) -> &std::time::SystemTime {
        &self.timestamp
    }

    #[inline]
    pub fn source(&self) -> Option<&ObjectId> {
        self.source.as_ref()
    }

    #[inline]
    pub fn take_data<T: Any>(&mut self) -> Option<T> {
        self.data.take().and_then(|boxed| boxed.downcast().ok().map(|boxed| *boxed))
    }

    #[inline]
    pub fn has_data<T: Any>(&self) -> bool {
        self.data.as_ref().and_then(|d| d.downcast_ref::<T>()).is_some()
    }

    #[inline]
    pub fn is_propagation_stopped(&self) -> bool {
        self.propagated
    }

    #[inline]
    pub fn stop_propagation(&mut self) {
        self.propagated = true;
    }
}

pub type EventHandler = Arc<dyn Fn(&mut Event) + Send + Sync>;

pub struct EventHandlerRegistration {
    handler: EventHandler,
    priority: i32,
    once: bool,
}

impl EventHandlerRegistration {
    #[inline]
    pub fn new(handler: EventHandler, priority: i32, once: bool) -> Self {
        Self {
            handler,
            priority,
            once,
        }
    }

    #[inline]
    pub fn handler(&self) -> &EventHandler {
        &self.handler
    }

    #[inline]
    pub fn priority(&self) -> i32 {
        self.priority
    }

    #[inline]
    pub fn is_once(&self) -> bool {
        self.once
    }
}

pub struct EventEmitter {
    handlers: Vec<EventHandlerRegistration>,
    event_type: EventType,
}

impl EventEmitter {
    #[inline]
    pub fn new(event_type: EventType) -> Self {
        Self {
            handlers: Vec::new(),
            event_type,
        }
    }

    #[inline]
    pub fn on<F>(&mut self, priority: i32, handler: F)
    where
        F: Fn(&mut Event) + Send + Sync + 'static,
    {
        self.handlers.push(EventHandlerRegistration::new(
            Arc::new(handler),
            priority,
            false,
        ));
        self.handlers.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    #[inline]
    pub fn once<F>(&mut self, priority: i32, handler: F)
    where
        F: Fn(&mut Event) + Send + Sync + 'static,
    {
        self.handlers.push(EventHandlerRegistration::new(
            Arc::new(handler),
            priority,
            true,
        ));
        self.handlers.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    #[inline]
    pub fn off(&mut self, handler: &EventHandler) {
        self.handlers.retain(|h| &h.handler != handler);
    }

    #[inline]
    pub fn emit(&mut self, event: &mut Event) {
        self.handlers.retain_mut(|registration| {
            let mut should_retain = true;
            if registration.priority >= 0 {
                (registration.handler)(event);
                should_retain = !registration.once && !event.is_propagation_stopped();
            }
            should_retain
        });
    }

    #[inline]
    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.handlers.clear();
    }
}

pub struct EventBus {
    emitters: HashMap<EventType, EventEmitter>,
    global_handlers: Vec<EventHandlerRegistration>,
}

impl EventBus {
    #[inline]
    pub fn new() -> Self {
        Self {
            emitters: HashMap::new(),
            global_handlers: Vec::new(),
        }
    }

    #[inline]
    pub fn on<F>(&mut self, event_type: EventType, priority: i32, handler: F)
    where
        F: Fn(&mut Event) + Send + Sync + 'static,
    {
        self.emitters
            .entry(event_type)
            .or_insert_with(|| EventEmitter::new(event_type))
            .on(priority, handler);
    }

    #[inline]
    pub fn once<F>(&mut self, event_type: EventType, priority: i32, handler: F)
    where
        F: Fn(&mut Event) + Send + Sync + 'static,
    {
        self.emitters
            .entry(event_type)
            .or_insert_with(|| EventEmitter::new(event_type))
            .once(priority, handler);
    }

    #[inline]
    pub fn on_global<F>(&mut self, priority: i32, handler: F)
    where
        F: Fn(&mut Event) + Send + Sync + 'static,
    {
        self.global_handlers.push(EventHandlerRegistration::new(
            Arc::new(handler),
            priority,
            false,
        ));
        self.global_handlers.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    #[inline]
    pub fn emit(&mut self, event: &mut Event) {
        if let Some(emitter) = self.emitters.get_mut(event.event_type()) {
            emitter.emit(event);
        }

        for registration in &mut self.global_handlers {
            if registration.priority >= 0 {
                (registration.handler)(event);
                if event.is_propagation_stopped() {
                    break;
                }
            }
        }
    }

    #[inline]
    pub fn emit_event(&mut self, event_type: EventType) {
        let mut event = Event::new(event_type);
        self.emit(&mut event);
    }

    #[inline]
    pub fn emit_with_data<T: Any>(&mut self, event_type: EventType, data: T) {
        let mut event = Event::new(event_type).with_data(data);
        self.emit(&mut event);
    }

    #[inline]
    pub fn emit_selection_changed(&mut self, selection: &[ObjectId]) {
        let mut event = Event::new(EventType::SelectionChanged).with_data(selection.to_vec());
        self.emit(&mut event);
    }

    #[inline]
    pub fn emit_entity_added(&mut self, entity_id: &ObjectId) {
        let mut event = Event::new(EventType::EntityAdded).with_source(entity_id.clone());
        self.emit(&mut event);
    }

    #[inline]
    pub fn emit_entity_removed(&mut self, entity_id: &ObjectId) {
        let mut event = Event::new(EventType::EntityRemoved).with_source(entity_id.clone());
        self.emit(&mut event);
    }

    #[inline]
    pub fn emit_entity_modified(&mut self, entity_id: &ObjectId) {
        let mut event = Event::new(EventType::EntityModified).with_source(entity_id.clone());
        self.emit(&mut event);
    }

    #[inline]
    pub fn emit_transaction_started(&mut self, name: &str) {
        let mut event = Event::new(EventType::TransactionStarted).with_data(name.to_string());
        self.emit(&mut event);
    }

    #[inline]
    pub fn emit_transaction_ended(&mut self) {
        let mut event = Event::new(EventType::TransactionEnded);
        self.emit(&mut event);
    }

    #[inline]
    pub fn off(&mut self, event_type: EventType, handler: &EventHandler) {
        if let Some(emitter) = self.emitters.get_mut(&event_type) {
            emitter.off(handler);
        }
    }

    #[inline]
    pub fn off_global(&mut self, handler: &EventHandler) {
        self.global_handlers.retain(|h| &h.handler != handler);
    }

    #[inline]
    pub fn clear_event(&mut self, event_type: EventType) {
        if let Some(emitter) = self.emitters.get_mut(&event_type) {
            emitter.clear();
        }
    }

    #[inline]
    pub fn clear_all(&mut self) {
        for emitter in self.emitters.values_mut() {
            emitter.clear();
        }
        self.global_handlers.clear();
    }

    #[inline]
    pub fn handler_count(&self, event_type: EventType) -> usize {
        self.emitters.get(&event_type).map(|e| e.handler_count()).unwrap_or(0)
    }

    #[inline]
    pub fn total_handler_count(&self) -> usize {
        let emitter_count: usize = self.emitters.values().map(|e| e.handler_count()).sum();
        emitter_count + self.global_handlers.len()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct SelectionChangeEvent {
    pub added: Vec<ObjectId>,
    pub removed: Vec<ObjectId>,
    pub current: Vec<ObjectId>,
}

#[derive(Debug, Clone)]
pub struct EntityChangeEvent {
    pub entity_id: ObjectId,
    pub old_data: Option<Box<dyn Any>>,
    pub new_data: Option<Box<dyn Any>>,
}

#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub x: f64,
    pub y: f64,
    pub button: MouseButton,
    pub modifiers: ModifierKeys,
    pub double_click: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModifierKeys {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl Default for ModifierKeys {
    fn default() -> Self {
        Self {
            shift: false,
            ctrl: false,
            alt: false,
            meta: false,
        }
    }
}

impl ModifierKeys {
    #[inline]
    pub fn new(shift: bool, ctrl: bool, alt: bool, meta: bool) -> Self {
        Self {
            shift,
            ctrl,
            alt,
            meta,
        }
    }

    #[inline]
    pub fn none() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key_code: u32,
    pub key: String,
    pub modifiers: ModifierKeys,
    pub repeat: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let event = Event::new(EventType::SelectionChanged);
        assert_eq!(event.event_type(), &EventType::SelectionChanged);
        assert!(event.source().is_none());
        assert!(!event.is_propagation_stopped());
    }

    #[test]
    fn test_event_with_source() {
        let id = ObjectId::new();
        let event = Event::new(EventType::EntityAdded).with_source(id.clone());
        assert_eq!(event.source(), Some(&id));
    }

    #[test]
    fn test_event_with_data() {
        let event = Event::new(EventType::MouseClicked).with_data(42i32);
        assert!(event.has_data::<i32>());
        assert!(!event.has_data::<String>());
    }

    #[test]
    fn test_event_bus_creation() {
        let bus = EventBus::new();
        assert_eq!(bus.total_handler_count(), 0);
    }

    #[test]
    fn test_event_handler_registration() {
        let mut bus = EventBus::new();
        let mut call_count = 0;
        
        bus.on(EventType::SelectionChanged, 0, |_| {
            call_count += 1;
        });
        
        assert_eq!(bus.handler_count(EventType::SelectionChanged), 1);
        
        let mut event = Event::new(EventType::SelectionChanged);
        bus.emit(&mut event);
        
        assert_eq!(call_count, 1);
    }

    #[test]
    fn test_event_propagation() {
        let mut bus = EventBus::new();
        let mut call_count = 0;
        let mut propagation_stopped = false;
        
        bus.on(EventType::SelectionChanged, 0, |e| {
            call_count += 1;
            e.stop_propagation();
        });
        
        bus.on(EventType::SelectionChanged, 0, |_| {
            call_count += 1;
        });
        
        let mut event = Event::new(EventType::SelectionChanged);
        bus.emit(&mut event);
        
        assert_eq!(call_count, 1);
    }

    #[test]
    fn test_global_event_handlers() {
        let mut bus = EventBus::new();
        let mut global_call_count = 0;
        let mut specific_call_count = 0;
        
        bus.on_global(0, |_| {
            global_call_count += 1;
        });
        
        bus.on(EventType::SelectionChanged, 0, |_| {
            specific_call_count += 1;
        });
        
        let mut event1 = Event::new(EventType::SelectionChanged);
        bus.emit(&mut event1);
        
        let mut event2 = Event::new(EventType::EntityAdded);
        bus.emit(&mut event2);
        
        assert_eq!(specific_call_count, 1);
        assert_eq!(global_call_count, 2);
    }

    #[test]
    fn test_once_handler() {
        let mut bus = EventBus::new();
        let mut call_count = 0;
        
        bus.once(EventType::SelectionChanged, 0, |_| {
            call_count += 1;
        });
        
        assert_eq!(bus.handler_count(EventType::SelectionChanged), 1);
        
        let mut event = Event::new(EventType::SelectionChanged);
        bus.emit(&mut event);
        
        assert_eq!(call_count, 1);
        assert_eq!(bus.handler_count(EventType::SelectionChanged), 0);
    }

    #[test]
    fn test_event_clear() {
        let mut bus = EventBus::new();
        
        bus.on(EventType::SelectionChanged, 0, |_| {});
        bus.on(EventType::EntityAdded, 0, |_| {});
        bus.on_global(0, |_| {});
        
        assert_eq!(bus.total_handler_count(), 3);
        
        bus.clear_all();
        
        assert_eq!(bus.total_handler_count(), 0);
    }

    #[test]
    fn test_selection_change_event() {
        let added = vec![ObjectId::new(), ObjectId::new()];
        let removed = vec![ObjectId::new()];
        let current = vec![added[0].clone()];
        
        let event = SelectionChangeEvent {
            added: added.clone(),
            removed: removed.clone(),
            current: current.clone(),
        };
        
        assert_eq!(event.added.len(), 2);
        assert_eq!(event.removed.len(), 1);
        assert_eq!(event.current.len(), 1);
    }

    #[test]
    fn test_mouse_event() {
        let mouse_event = MouseEvent {
            x: 100.5,
            y: 200.75,
            button: MouseButton::Left,
            modifiers: ModifierKeys::new(true, false, false, false),
            double_click: false,
        };
        
        assert_eq!(mouse_event.x, 100.5);
        assert_eq!(mouse_event.button, MouseButton::Left);
        assert!(mouse_event.modifiers.shift);
        assert!(!mouse_event.modifiers.ctrl);
    }

    #[test]
    fn test_key_event() {
        let key_event = KeyEvent {
            key_code: 65,
            key: "A".to_string(),
            modifiers: ModifierKeys::none(),
            repeat: false,
        };
        
        assert_eq!(key_event.key_code, 65);
        assert_eq!(key_event.key, "A");
        assert!(!key_event.repeat);
    }
}
