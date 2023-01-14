struct LiquidityPool {
    token0: f32,
    token1: f32,
    product: f32,
    ratio: f32
}

impl LiquidityPool {
    fn new(token0: f32, token1: f32) -> Self {
        let product = token0 * token1;
        let ratio = token0 / token1;
        LiquidityPool { token0: token0, token1: token1, product: product, ratio: ratio }
    }

    fn deposit(&mut self, token0: f32, token1: f32) -> Result<(f32, f32), ExchangeError> {
        let deposit_a = token0;
        let deposit_b = token1 * self.get_price_of_b();

        if deposit_a != deposit_b {
            let err_msg = format!("Deposit should be in the ratio of {} : {}", self.ratio * 100.0, 100);
            return Err(ExchangeError::NotEqualError(err_msg));
        }
        else {
            self.token0 += token0;
            self.token1 += token1;
            self.product = self.token0 * self.token1;
            self.update_price();
            Ok((deposit_a, deposit_b))
        }
    }

    fn withdraw(&mut self, token0: f32, token1: f32) -> Result<(), ExchangeError> {
        let withdraw_a = token0;
        let withdraw_b = token1 * self.get_price_of_b();

        if withdraw_b != withdraw_a {
            let err_msg = format!("Deposit should be in the ratio of {} : {}!", self.ratio * 100.0, 100);
            return Err(ExchangeError::NotEqualError(err_msg));
        }
        else if withdraw_a > self.token0 || withdraw_b > self.token1 {
                return  Err(ExchangeError::InadequateDepositError("Your withdrawal exceeds the deposit in pool!".to_string()));
        }
        else {
            self.token0 -= token0;
            self.token1 -= token1;
            self.product = self.token0 * self.token1;
            self.update_price();

            Ok(())
        }
    }

    fn update_price(&mut self) -> (f32, f32) {
        let price_of_a = self.token1 / self.token0;
        let price_of_b = self.token0 / self.token1;

        (price_of_a, price_of_b)
    }

    fn get_price_of_a(&mut self) -> f32 {
        let price = self.update_price();
        price.0
    }

    fn get_price_of_b(&mut self) -> f32 {
        let price = self.update_price();
        price.1
    }

    fn exchange_a(&mut self, token1: f32) -> f32 {
        let token1_bef_exch = self.token1;
        self.token1 += token1;
        let exch_amt = (self.product / self.token1) / token1_bef_exch;
        exch_amt 
    }

    fn exchange_b(&mut self, token0: f32)  -> f32 {
        let token0_before_exch = self.token0;
        self.token0 += token0;
        let exch_amt = (self.product / self.token0) / token0_before_exch;
        exch_amt 
    }
}


#[derive(Debug)]
enum ExchangeError {
    NotEqualError(String),
    InadequateDepositError(String)
}

impl std::fmt::Display for ExchangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEqualError(s) => {
                write!(f, "{}", s)
            }
            Self::InadequateDepositError(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

impl std::error::Error for ExchangeError {}
fn main() -> Result<(), ExchangeError> {
    let mut eth_dai_pool = LiquidityPool::new(10.0, 1000.0);
    let deposit = eth_dai_pool.deposit(100.0, 10000.0)?;
    /*match deposit {
        Ok(s) => println!("{:?}", s),
        Err(ExchangeError::NotEqualError(e)) => println!("{:?}", e),
        Err(ExchangeError::InadequateDepositError(e)) => println!("{:?}", e)
    }*/
    assert_eq!(eth_dai_pool.exchange_a(20.0), 0.009981851);
    Ok(())
}
