[![Build Status](https://travis-ci.com/immortalinfidel/ppo-rs.svg?branch=master)](https://travis-ci.com/immortalinfidel/ppo-rs)
# Percentage Price Oscillator (PPO)
```
use ppo_rs::PPO;
use ta_common::traits::Indicator;

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
```