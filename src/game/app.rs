use std::{collections::HashSet, num::NonZeroU32};
use glutin::{
    config::ConfigTemplateBuilder, 
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version}, 
    display::Display, 
    prelude::*, 
    surface::{Surface, SurfaceAttributesBuilder, WindowSurface}
};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId}
};
use crate::{
    game::{self, state::State}, 
    util
};

#[derive(Default)]
pub struct App {
    pub window: Option<Window>,
    pub display: Option<Display>,
    pub surface: Option<Surface<WindowSurface>>,
    pub context: Option<PossiblyCurrentContext>,

    pub keys_pressed: HashSet<KeyCode>,
    pub player: Option<game::player::Player>,
    pub enemy: Option<game::enemy::Enemy>,
    pub ball: Option<game::ball::Ball>,

    pub current_state: Box<dyn State>
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        const DEFAULT_WIDTH: u32 = 800;
        const DEFAULT_HEIGHT: u32 = 600;

        let window_attributes = Window::default_attributes()
            .with_title("Pong!!!")
            .with_transparent(true)
            .with_visible(false)
            .with_inner_size(LogicalSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT));
        let window = event_loop.create_window(window_attributes).expect("Unable to create window");

        let raw_window_handle = window.window_handle().unwrap().as_raw();
        let raw_display_handle = window.display_handle().unwrap().as_raw();
        let display = unsafe {
            Display::new(raw_display_handle, util::display::DISPLAY_PREF(raw_window_handle)).unwrap()
        };

        let template = ConfigTemplateBuilder::new().build();
        let config = unsafe { display
            .find_configs(template).unwrap()
            .next().expect("No GL configs found")
        };

        let window_size = window.inner_size();
        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(window_size.width).unwrap_or_else(|| {NonZeroU32::new(DEFAULT_WIDTH).unwrap() }),
            NonZeroU32::new(window_size.height).unwrap_or_else(|| {NonZeroU32::new(DEFAULT_HEIGHT).unwrap() })
        );
        let surface = unsafe { display.create_window_surface(&config, &surface_attributes).unwrap() };

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(Some(raw_window_handle));
        let not_current = unsafe { display.create_context(&config, &context_attributes).unwrap() };
        let context = not_current.make_current(&surface).unwrap();

        gl::load_with(|sym: &str| {
            let c_str = std::ffi::CString::new(sym).unwrap();
            display.get_proc_address(&c_str)
        });

        self.player = Some(game::player::bind_player());
        self.enemy = Some(game::enemy::bind_enemy());
        self.ball = Some(game::ball::bind_ball());
        self.window = Some(window);
        self.display = Some(display);
        self.surface = Some(surface);
        self.context = Some(context);

        let (window, surface, context) = (
            self.window.as_ref().unwrap(),
            self.surface.as_ref().unwrap(),
            self.context.as_ref().unwrap()
        );

        unsafe {
            gl::Viewport(0, 0, window_size.width as i32, window_size.height as i32);
            gl::ClearColor((0x18 as f32)/255.0, (0x18 as f32)/255.0, (0x18 as f32)/255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(0);
        }

        surface.swap_buffers(context).unwrap();
        window.request_redraw();
        window.set_visible(true);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.window.as_ref() else { return; };
        let Some(surface) = self.surface.as_ref() else { return; };
        let Some(context) = self.context.as_ref() else { return; };
        let Some(player) = self.player.as_mut() else { return; };
        let Some(enemy) = self.enemy.as_mut() else { return; };
        let Some(ball) = self.ball.as_mut() else { return; };

        if window.id() != window_id { return; }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                unsafe {
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    self.current_state.update(player, enemy, ball, &self.keys_pressed);
                    self.current_state.draw(player, enemy, ball);
                    
                }
                surface.swap_buffers(&context).unwrap();
                window.request_redraw();
            }
            WindowEvent::KeyboardInput{event, ..} => {
                match (event.state, event.physical_key) {
                    (ElementState::Pressed, PhysicalKey::Code(code)) => { self.keys_pressed.insert(code); }
                    (ElementState::Released, PhysicalKey::Code(code)) => { self.keys_pressed.remove(&code); }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}