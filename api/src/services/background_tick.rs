use anyhow::Result;

pub trait RequiresBackgroundTick {
    fn background_tick(&self) -> Result<()>;
}