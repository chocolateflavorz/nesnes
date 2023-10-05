use crate::{cpu::Cpu, mem::Mem};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window}, dpi::LogicalSize,
};  

const WIDTH: u32  = 24;
const HEIGHT: u32 = 24;
const SCALE: f64  = 30.0;
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
    pub fn run(&mut self) {
        let event_loop = EventLoop::new();
        let window = {
            let size = LogicalSize::new( WIDTH as f64, HEIGHT as f64);
            let scaled_size = LogicalSize::new( WIDTH as f64 * SCALE, HEIGHT as f64 * SCALE);
            WindowBuilder::new()
                .with_title("EASY2A03")
                .with_inner_size(size)
                .with_min_inner_size(scaled_size)
                .build(&event_loop)
                .unwrap()
        };
        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();
            control_flow.set_wait();
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    control_flow.set_exit();
                },
                _ => (),
            }
        });
    }
}
