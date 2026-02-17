use std::time::Instant;

pub struct DeltaTime {
    current: f32,
    previous: Instant,
}

impl DeltaTime {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            current: 0.0,
            previous: now,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.current = now.duration_since(self.previous).as_secs_f32();
        self.previous = now;
    }

    pub fn delta(&self) -> f32 {
        self.current
    }

    pub fn seconds(&self) -> f32 {
        self.current
    }

    pub fn milliseconds(&self) -> f32 {
        self.current * 1000.0
    }
}

impl Default for DeltaTime {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FixedTimestep {
    accumulator: f32,
    timestep: f32,
    interpolation: f32,
}

impl FixedTimestep {
    pub fn new(fixed_delta_time: f32) -> Self {
        Self {
            accumulator: 0.0,
            timestep: fixed_delta_time,
            interpolation: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.accumulator += delta_time;
        self.interpolation = self.accumulator / self.timestep;
    }

    pub fn should_update(&self) -> bool {
        self.accumulator >= self.timestep
    }

    pub fn consume_tick(&mut self) {
        if self.accumulator >= self.timestep {
            self.accumulator -= self.timestep;
        }
    }

    pub fn interpolation(&self) -> f32 {
        self.interpolation.min(1.0)
    }

    pub fn timestep(&self) -> f32 {
        self.timestep
    }

    pub fn accumulator(&self) -> f32 {
        self.accumulator
    }
}

pub struct FpsCounter {
    frame_times: Vec<f32>,
    max_samples: usize,
    current_fps: f32,
    frame_count: u32,
    last_update: Instant,
    update_interval: f32,
}

impl FpsCounter {
    pub fn new(max_samples: usize, update_interval: f32) -> Self {
        Self {
            frame_times: Vec::with_capacity(max_samples),
            max_samples,
            current_fps: 0.0,
            frame_count: 0,
            last_update: Instant::now(),
            update_interval,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.frame_times.push(delta_time);
        if self.frame_times.len() > self.max_samples {
            self.frame_times.remove(0);
        }

        self.frame_count += 1;
        let elapsed = self.last_update.elapsed().as_secs_f32();
        if elapsed >= self.update_interval {
            let avg_frame_time = if !self.frame_times.is_empty() {
                self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32
            } else {
                delta_time
            };
            self.current_fps = if avg_frame_time > 0.0 {
                1.0 / avg_frame_time
            } else {
                0.0
            };
            self.last_update = Instant::now();
        }
    }

    pub fn fps(&self) -> f32 {
        self.current_fps
    }

    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }

    pub fn average_frame_time(&self) -> f32 {
        if !self.frame_times.is_empty() {
            self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32
        } else {
            0.0
        }
    }

    pub fn reset(&mut self) {
        self.frame_times.clear();
        self.current_fps = 0.0;
        self.frame_count = 0;
        self.last_update = Instant::now();
    }
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self::new(60, 0.5)
    }
}
