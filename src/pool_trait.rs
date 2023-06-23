use crate::ExchangeError;
pub trait Pool {
    fn deposit(&mut self, token0: f32, token1: f32) -> Result<(f32, f32), ExchangeError>;

    fn withdraw(&mut self, token0: f32, token1: f32) -> Result<(), ExchangeError>;

    fn exchange_a(&mut self, token1: f32) -> f32;

    fn exchange_b(&mut self, token0: f32) -> f32;
}