#[doc = "Register `OUT_IDV` writer"]
pub type W = crate::W<OUT_IDV_SPEC>;
#[doc = "Field `CH0` writer - Configure channel 0 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
pub type CH0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1` writer - Configure channel 1 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
pub type CH1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH2` writer - Configure channel 2 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
pub type CH2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH3` writer - Configure channel 3 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
pub type CH3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH4` writer - Configure channel 4 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
pub type CH4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH5` writer - Configure channel 5 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
pub type CH5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH6` writer - Configure channel 6 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
pub type CH6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH7` writer - Configure channel 7 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
pub type CH7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_IDV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure channel 0 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<OUT_IDV_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Configure channel 1 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<OUT_IDV_SPEC> {
        CH1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Configure channel 2 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<OUT_IDV_SPEC> {
        CH2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Configure channel 3 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<OUT_IDV_SPEC> {
        CH3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Configure channel 4 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<OUT_IDV_SPEC> {
        CH4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Configure channel 5 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<OUT_IDV_SPEC> {
        CH5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Configure channel 6 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<OUT_IDV_SPEC> {
        CH6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Configure channel 7 output value. 0: hold output value. 1: set output value. 2: clear output value. 3: inverse output value."]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<OUT_IDV_SPEC> {
        CH7_W::new(self, 14)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Dedicated GPIO individual output register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_idv::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_IDV_SPEC;
impl crate::RegisterSpec for OUT_IDV_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_idv::W`](W) writer structure"]
impl crate::Writable for OUT_IDV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_IDV to value 0"]
impl crate::Resettable for OUT_IDV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
