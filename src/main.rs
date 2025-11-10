extern crate ash;
extern crate sdl2 as sdl;
extern crate vulkano;

mod vk;
mod vw_engine;
mod vw_window;

fn main() -> anyhow::Result<()> {
    let engine = vw_engine::VkWizardEngine::new()?;
    if cfg!(debug_assertions) {
        println!("Running in debug mode.");
    }
    engine.run();

    return Ok(());

    // let vk_surface = window.create_vk_surface(device.vk_instance().handle().as_raw() as _);
}
