use crate::{cpu::Cpu, mem::Mem};

use std::thread::sleep;
use std::time::{Duration, Instant};
use log::{debug};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{EventLoop},
    window::{WindowBuilder},
};

use pixels::{PixelsBuilder,SurfaceTexture};

const WIDTH: u32 = 32;
const HEIGHT: u32 = 32;
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
    pub fn loadeasy(&mut self, bin: Vec<u8>) {
        self.mem.copy_from_slice(0x0600, &bin);
        self.cpu.pc = 0x0600;
    }
    pub fn render_easy(&self, f: &mut [u8]) {
        for (i, c) in self.mem.get_frame_easy().iter().enumerate() {
            let offset = i * 4;
            let color = match c {
                0 => [0x00, 0x00, 0x00, 0xff],
                1 => [0xff, 0xff, 0xff, 0xff],
                2 => [0x00, 0x00, 0xff, 0xff],
                3 => [0xff, 0x00, 0x00, 0xff],
                4 => [0x00, 0xff, 0x00, 0xff],
                _ => [0xff, 0x00, 0xff, 0xff],
            };
            f[offset..offset + 4].copy_from_slice(&color);
        }
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

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(
                window_size.width,
                window_size.height,
                &window,
            );
            PixelsBuilder::new(WIDTH, HEIGHT, surface_texture)
                .surface_texture_format(pixels::wgpu::TextureFormat::Bgra8UnormSrgb)
                .build()
                .unwrap()
        };

        debug!("frame len {}", pixels.frame().len());

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
                Emu::run_cpu_until(&mut self, 128);
                self.render_easy(pixels.frame_mut());
                pixels.render().expect("render failed");
                sleep(Duration::from_millis(10));
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
