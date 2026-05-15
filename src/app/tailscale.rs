pub struct TailscaleTotalState {
    base_bytes: Option<u64>,
    carry_bytes: u64,
    last_bytes: Option<u64>,
}

pub struct TailscaleUpdate {
    pub delta_bytes: Option<u64>,
    pub latest_bytes: Option<u64>,
}

impl TailscaleTotalState {
    pub fn new() -> Self {
        Self {
            base_bytes: None,
            carry_bytes: 0,
            last_bytes: None,
        }
    }

    pub fn from_absolute(value: Option<u64>) -> Self {
        Self {
            base_bytes: value,
            carry_bytes: 0,
            last_bytes: value,
        }
    }

    fn total(&self) -> Option<u64> {
        let base = self.base_bytes?;
        let last = self.last_bytes?;
        Some(self.carry_bytes.saturating_add(last.saturating_sub(base)))
    }

    pub fn update(&mut self, absolute_bytes: Option<u64>) -> TailscaleUpdate {
        let Some(absolute) = absolute_bytes else {
            return TailscaleUpdate {
                delta_bytes: None,
                latest_bytes: None,
            };
        };

        let previous_total = self.total();
        if self.base_bytes.is_none() {
            self.base_bytes = Some(absolute);
            self.last_bytes = Some(absolute);
            return TailscaleUpdate {
                delta_bytes: Some(0),
                latest_bytes: Some(0),
            };
        }

        if let (Some(last), Some(base)) = (self.last_bytes, self.base_bytes) {
            if absolute < last {
                self.carry_bytes = self.carry_bytes.saturating_add(last.saturating_sub(base));
                self.base_bytes = Some(absolute);
            }
        }

        self.last_bytes = Some(absolute);
        let current_total = self.total();
        let delta = match (previous_total, current_total) {
            (Some(previous), Some(current)) => Some(current.saturating_sub(previous)),
            _ => Some(0),
        };
        TailscaleUpdate {
            delta_bytes: delta,
            latest_bytes: current_total,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_counter_reset() {
        let mut state = TailscaleTotalState::new();
        assert_eq!(state.update(Some(100)).latest_bytes, Some(0));
        assert_eq!(state.update(Some(150)).delta_bytes, Some(50));
        assert_eq!(state.update(Some(20)).latest_bytes, Some(50));
        assert_eq!(state.update(Some(30)).delta_bytes, Some(10));
    }
}
