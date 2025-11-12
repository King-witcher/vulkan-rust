extern crate ash;
extern crate sdl2 as sdl;
extern crate smallvec;
extern crate vulkano;

mod vk;
mod vw_engine;
mod vw_window;

fn main() -> anyhow::Result<()> {
    if cfg!(debug_assertions) {
        println!("Running in debug mode.");
    }

    let engine = vw_engine::VkWizardEngine::new()?;
    engine.run();

    Ok(())
}
