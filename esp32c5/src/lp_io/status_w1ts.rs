#[doc = "Register `STATUS_W1TS` writer"]
pub type W = crate::W<STATUS_W1TS_SPEC>;
#[doc = "Field `STATUS_W1TS` writer - Configures whether or not to set the interrupt status register LP_IO_STATUS_INT of GPIO0 ~ GPIO7.\\\\ - Bit0 is corresponding to GPIO0, bit1 is corresponding to GPIO1, and etc. - If the value 1 is written to a bit here, the corresponding bit in LP_IO_STATUS_INT will be set to 1. - Recommended operation: use this register to set LP_IO_STATUS_INT."]
pub type STATUS_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures whether or not to set the interrupt status register LP_IO_STATUS_INT of GPIO0 ~ GPIO7.\\\\ - Bit0 is corresponding to GPIO0, bit1 is corresponding to GPIO1, and etc. - If the value 1 is written to a bit here, the corresponding bit in LP_IO_STATUS_INT will be set to 1. - Recommended operation: use this register to set LP_IO_STATUS_INT."]
    #[inline(always)]
    pub fn status_w1ts(&mut self) -> STATUS_W1TS_W<'_, STATUS_W1TS_SPEC> {
        STATUS_W1TS_W::new(self, 0)
    }
}
#[doc = "LP GPIO interrupt status set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_W1TS_SPEC;
impl crate::RegisterSpec for STATUS_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`status_w1ts::W`](W) writer structure"]
impl crate::Writable for STATUS_W1TS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS_W1TS to value 0"]
impl crate::Resettable for STATUS_W1TS_SPEC {}
