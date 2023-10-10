use crate::{cpu::Cpu, mem::Mem};

use std::thread::sleep;
use std::time::{Duration, Instant};

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

const WIDTH: u32 = 24;
const HEIGHT: u32 = 24;
const SCALE: f64 = 30.0;

pub struct Emu {
    pub stat: Stat,
    pub cpu: Cpu,
    pub mem: Mem,
}

pub struct Stat {
    pub cycle_counter: u32,
    pub frame_counter: u32,
}

impl Default for Emu {
    fn default() -> Self {
        Emu {
            stat: Stat::new(),
            cpu: Cpu::default(),
            mem: Mem::default(),
        }
    }
}

impl Stat {
    pub fn new() -> Self {
        Stat {
            cycle_counter: 0,
            frame_counter: 0,
        }
    }
}

impl Emu {
    pub fn load(&mut self, bin: Vec<u8>) {
        self.mem.copy_from_slice(0x8000, &bin);
    }
    pub fn render(&mut self) {
        println!("Gekki form render");
    }
    pub fn run(mut self) {
        let event_loop = EventLoop::new();
        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
            let scaled_size = LogicalSize::new(WIDTH as f64 * SCALE, HEIGHT as f64 * SCALE);
            WindowBuilder::new()
                .with_title("EASY2A03")
                .with_inner_size(size)
                .with_min_inner_size(scaled_size)
                .build(&event_loop)
                .unwrap()
        };

        let wakeup = Instant::now();
        let mut last_time = Instant::now();
        let mut time_acc = 0.0f32;
        const FRAME_TIME: f32 = 1.0 / 24.0;

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();
            let elapsed = last_time.elapsed().as_secs_f32();
            time_acc += elapsed;
            last_time = Instant::now();
            if time_acc >= FRAME_TIME {
                time_acc -= FRAME_TIME;
                self.stat.frame_counter += 1;
                self.render();
                sleep(Duration::from_millis(1));
            }
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    control_flow.set_exit();
                }
                _ => (),
            }
        });
    }
}
