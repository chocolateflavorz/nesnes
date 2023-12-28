use crate::{emu::Emu, mem::Mem};
use device_query::{DeviceQuery, DeviceState, Keycode};

use log::debug;
use std::fs::File;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use pixels::{PixelsBuilder, SurfaceTexture};
use rand::prelude::*;

impl Mem {
    #[inline]
    pub fn get_frame_easy(&self) -> &[u8] {
        &self.mem[0x200..0x600]
    }
}

impl Emu {
    const WIDTH_EASY: u32 = 32;
    const HEIGHT_EASY: u32 = 32;
    const SCALE_EASY: f64 = 30.0;

    pub fn load_easy(&mut self, bin: Vec<u8>) {
        self.mem.copy_from_slice(0x0600, &bin);
        self.cpu.pc = 0x0600;
    }
    pub fn render_easy(&self, f: &mut [u8]) {
        for (i, c) in self.mem.get_frame_easy().iter().enumerate() {
            let offset = i * 4;
            let color = match c {
                0 => [0x00, 0x00, 0x00, 0xff],
                1 => [0xff, 0xff, 0xff, 0xff],
                2 | 6 | 10 | 14 => [0x00, 0x00, 0xff, 0xff],
                3 | 7 | 11 | 15 => [0xff, 0x00, 0x00, 0xff],
                4 | 8 | 12 => [0x00, 0xff, 0x00, 0xff],
                5 | 9 | 13 => [0xcc, 0xcc, 0x20, 0xff],
                _ => [0xff, 0x00, 0xff, 0xff],
            };
            f[offset..offset + 4].copy_from_slice(&color);
        }
    }
    pub fn run_easy(mut self) {
        let event_loop = EventLoop::new();
        let window = {
            let size = LogicalSize::new(Self::WIDTH_EASY as f64, Self::HEIGHT_EASY as f64);
            let scaled_size = LogicalSize::new(
                Self::WIDTH_EASY as f64 * Self::SCALE_EASY,
                Self::HEIGHT_EASY as f64 * Self::SCALE_EASY,
            );
            WindowBuilder::new()
                .with_title("EASY2A03")
                .with_inner_size(size)
                .with_min_inner_size(scaled_size)
                .build(&event_loop)
                .unwrap()
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            PixelsBuilder::new(Self::WIDTH_EASY, Self::HEIGHT_EASY, surface_texture)
                .surface_texture_format(pixels::wgpu::TextureFormat::Bgra8UnormSrgb)
                .build()
                .unwrap()
        };

        debug!("frame len {}", pixels.frame().len());

        let mut rng = SmallRng::from_entropy();
        let device_state = DeviceState::new();
        let mut last_time = Instant::now();
        let mut time_acc = 0.0f32;
        let mut breaking = false;

        const FRAME_TIME: f32 = 1.0 / 90.0;

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();
            let elapsed = last_time.elapsed().as_secs_f32();
            time_acc += elapsed;
            last_time = Instant::now();
            if time_acc >= FRAME_TIME {
                time_acc -= FRAME_TIME;
                self.stat.frame_counter += 1;
                self.mem.store_u8(0x00fe, rng.gen_range(0..=0xff));
                let keys: Vec<Keycode> = device_state.get_keys();
                keys.contains(&Keycode::A)
                    .then(|| self.mem.store_u8(0x00ff, 0x61));
                keys.contains(&Keycode::S)
                    .then(|| self.mem.store_u8(0x00ff, 0x73));
                keys.contains(&Keycode::D)
                    .then(|| self.mem.store_u8(0x00ff, 0x64));
                keys.contains(&Keycode::W)
                    .then(|| self.mem.store_u8(0x00ff, 0x77));
                keys.contains(&Keycode::Escape)
                    .then(|| breaking = true);
                if (!breaking) {
                    self.run_cpu_clocks(256);
                    self.render_easy(pixels.frame_mut());
                    pixels.render().expect("render failed");
                    sleep(Duration::from_millis(10));
                } else {
                    use pretty_hex::*;
                    //Emu::cpuinfo(&self);
                    let mut file = std::fs::File::create("foo.txt").unwrap();
                    file.write(format!("{:?}", self.mem.mem[0x000..0x600].hex_dump()).to_string().as_bytes()).expect("FUCKWRITER");
                    control_flow.set_exit();
                }
            }
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    control_flow.set_exit();
                }
                /*
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput {input, .. },
                    ..
                 } => {
                    if let Some(keycode) = input.virtual_keycode {
                        match keycode {
                            winit::event::VirtualKeyCode::W => {
                                self.mem.store_u8(0xff, 0x77);
                                debug!("WW PRESS");
                            },
                            winit::event::VirtualKeyCode::A => {
                                self.mem.store_u8(0xff, 0x73);
                            },
                            winit::event::VirtualKeyCode::S => {
                                self.mem.store_u8(0xff, 0x61);
                            },
                            winit::event::VirtualKeyCode::D => {
                                self.mem.store_u8(0xff, 0x64);
                            },
                            _ => (),
                        }
                    }
                 }
                */
                _ => (),
            }
        });
    }
}
