#[doc = "Register `OUT_IDV` writer"]
pub type W = crate::W<OUT_IDV_SPEC>;
#[doc = "Field `CH(0-7)` writer - Configure channel %s output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
pub type CH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_IDV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Configure channel (0-7) output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn ch(&mut self, n: u8) -> CH_W<OUT_IDV_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - Configure channel 0 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH_W<OUT_IDV_SPEC> {
        CH_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Configure channel 1 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH_W<OUT_IDV_SPEC> {
        CH_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Configure channel 2 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH_W<OUT_IDV_SPEC> {
        CH_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Configure channel 3 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH_W<OUT_IDV_SPEC> {
        CH_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Configure channel 4 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH_W<OUT_IDV_SPEC> {
        CH_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Configure channel 5 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH_W<OUT_IDV_SPEC> {
        CH_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Configure channel 6 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH_W<OUT_IDV_SPEC> {
        CH_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Configure channel 7 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH_W<OUT_IDV_SPEC> {
        CH_W::new(self, 14)
    }
}
#[doc = "Dedicated GPIO individual output register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_idv::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_IDV_SPEC;
impl crate::RegisterSpec for OUT_IDV_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_idv::W`](W) writer structure"]
impl crate::Writable for OUT_IDV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_IDV to value 0"]
impl crate::Resettable for OUT_IDV_SPEC {
    const RESET_VALUE: u32 = 0;
}
