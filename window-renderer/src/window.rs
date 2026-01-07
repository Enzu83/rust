use std::time::{Duration, Instant};

use crate::color::{BLACK, Color};

use anyhow::{Result, bail};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder}
};

const DEFAULT_WINDOW_TITLE: &'static str = "Default Window";
const DEFAULT_WINDOW_WIDTH: u32 = 640;
const DEFAULT_WINDOW_HEIGHT: u32 = 480;
const DEFAULT_PIXELS_WIDTH: u32 = 320;
const DEFAULT_PIXELS_HEIGHT: u32 = 240;
const DEFAULT_WINDOW_REFRESH_RATE: f32 = 60f32;

pub struct WindowRenderer {
    window: Window,
    pixels: Pixels,
    last_draw: Instant,
    refresh_time: Duration,
}

impl WindowRenderer {
    pub fn create_default(event_loop: &EventLoop<()>) -> Result<Self> {
        let window = WindowBuilder::new()
            .with_title(DEFAULT_WINDOW_TITLE)
            .with_inner_size(LogicalSize::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT))
            .build(event_loop)?;

        let surface = SurfaceTexture::new(DEFAULT_PIXELS_WIDTH, DEFAULT_PIXELS_HEIGHT, &window);
        let pixels = Pixels::new(DEFAULT_PIXELS_WIDTH, DEFAULT_PIXELS_HEIGHT, surface)?;

        Ok(Self {
            window: window,
            pixels,
            last_draw: Instant::now(),
            refresh_time: Duration::from_secs_f32(1.0 / DEFAULT_WINDOW_REFRESH_RATE),
        })
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: &Color) -> Result<()> {
        let surface_width = self.pixels.texture().width();
        let surface_height = self.pixels.texture().height();

        if width > surface_width || height > surface_height {
            bail!("Tried to draw {:?} outside the surface texture {:?}", (width, height), (surface_width, surface_height))
        }
        
        let frame = self.pixels.frame_mut();
        for dy in 0..height {
            for dx in 0..width {
                let idx = (((y + dy) * surface_width + (x + dx)) * 4) as usize;
                frame[idx] = color.r;
                frame[idx+1] = color.g;
                frame[idx+2] = color.b;
                frame[idx+3] = color.a;
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, x: u32, y: u32, color: &Color) -> Result<()> {
        self.draw_rect(x, y, 1, 1, color)
    }

    pub fn draw_hline(&mut self, y: u32, color: &Color) -> Result<()> {
        self.draw_rect(0, y, self.pixels.texture().width(), 1, color)
    }

    pub fn draw_vline(&mut self, x: u32, color: &Color) -> Result<()> {
        self.draw_rect(x, 0, 1, self.pixels.texture().height(), color)
    }

    pub fn fill(&mut self, color: &Color) -> Result<()> {
        self.draw_rect(0, 0, self.pixels.texture().width(), self.pixels.texture().height(), color)
    }

    pub fn clear(&mut self) -> Result<()> {
        self.fill(BLACK)
    }

    pub fn handle_event(mut self, event: Event<'_, ()>, control_flow: &mut ControlFlow) {
        match event {
            Event::MainEventsCleared => {
                if self.last_draw.elapsed() > self.refresh_time {
                    self.last_draw = Instant::now();
                    self.window.request_redraw();
                }
            },
            Event::RedrawRequested(_) => {
                if let Err(e) = self.pixels.render() {
                    eprintln!("pixels error: {e}");
                    *control_flow = ControlFlow::Exit;
                }
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                },
                _ => {},
            }
            _ => {},
        }
    }
}
