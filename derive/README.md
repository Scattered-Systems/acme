# acme-derive

## Overview

## Examples
    
    extern crate acme_derive;
    use acme_derive::SampleFunction;
    
    #[derive(SampleFunction)]
    pub struct Tmp;
    
    pub fn main {
        let res: u16 = sample();
        assert_eq!(18, res.clone())
    }