//! 하네스에 등록할 hook event와 그 정책 종류의 단일 catalog.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Policy {
    SubagentStop,
    RoundStop,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EventSpec {
    pub name: &'static str,
    pub policy: Policy,
}

pub const EVENTS: &[EventSpec] = &[
    EventSpec {
        name: "SubagentStop",
        policy: Policy::SubagentStop,
    },
    EventSpec {
        name: "Stop",
        policy: Policy::RoundStop,
    },
];

#[must_use]
pub fn find(event: &str) -> Option<EventSpec> {
    EVENTS.iter().copied().find(|spec| spec.name == event)
}
