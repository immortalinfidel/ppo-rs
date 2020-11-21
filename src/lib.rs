#![feature(external_doc)]

use ema_rs::EMA;
use ta_common::traits::Indicator;

#[doc(include = "../README.md")]
pub struct PPO {
    short_period: u32,
    long_ema: EMA,
    short_ema: EMA,
    index: u32,
}

impl PPO {
    pub fn new(short_period: u32, long_period: u32) -> PPO {
        Self {
            short_period,
            long_ema: EMA::new(long_period),
            short_ema: EMA::new(short_period),
            index: 0,
        }
    }
}

impl Indicator<f64, Option<f64>> for PPO {
    fn next(&mut self, input: f64) -> Option<f64> {
        let long = self.long_ema.next(input);
        let short = self.short_ema.next(input);
        return if self.index >= self.short_period - 1 {
            let ppo = 100_f64 * (short - long) / long;
            Some(ppo)
        } else {
            self.index = self.index + 1;
            None
        };
    }

    fn reset(&mut self) {
        self.short_ema.reset();
        self.long_ema.reset();
        self.index = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::PPO;
    use ta_common::traits::Indicator;

    #[test]
    fn add_works() {
        let mut ppo = PPO::new(2, 5);
        assert_eq!(ppo.next(81.59), None);
        assert_eq!(ppo.next(81.06), Some(-0.21699967245331922));
        assert_eq!(ppo.next(82.87), Some(0.5209675887612042));
        assert_eq!(ppo.next(83.00), Some(0.6190403299147216));
        assert_eq!(ppo.next(83.61), Some(0.7468846224456406));
        assert_eq!(ppo.next(83.15), Some(0.4239424161439804));
        assert_eq!(ppo.next(82.84), Some(0.13356017757692787));
        assert_eq!(ppo.next(83.99), Some(0.4997247953737079));
        assert_eq!(ppo.next(84.55), Some(0.690806261787222));
        assert_eq!(ppo.next(84.36), Some(0.503265546395477));
        assert_eq!(ppo.next(85.53), Some(0.8097662523725208));
        assert_eq!(ppo.next(86.54), Some(1.0883304662225042));
        assert_eq!(ppo.next(86.89), Some(1.039770979517235));
        assert_eq!(ppo.next(87.77), Some(1.1327352631554244));
        assert_eq!(ppo.next(87.29), Some(0.7158876517890078));
    }
}
