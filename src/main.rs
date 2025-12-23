extern crate ash;
extern crate sdl2 as sdl;
extern crate smallvec;
extern crate vulkano;

mod vw_engine;
mod vw_window;

fn main() -> anyhow::Result<()> {
    if cfg!(debug_assertions) {
        println!("Running in debug mode.");
    }

    let _engine = vw_engine::VkWizardEngine::new()?;

    Ok(())
}
