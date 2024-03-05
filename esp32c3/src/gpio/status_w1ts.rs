#[doc = "Register `STATUS_W1TS` writer"]
pub type W = crate::W<STATUS_W1TS_SPEC>;
#[doc = "Field `STATUS_W1TS` writer - GPIO interrupt status set register for GPIO0-25"]
pub type STATUS_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:25 - GPIO interrupt status set register for GPIO0-25"]
    #[inline(always)]
    #[must_use]
    pub fn status_w1ts(&mut self) -> STATUS_W1TS_W<STATUS_W1TS_SPEC> {
        STATUS_W1TS_W::new(self, 0)
    }
}
#[doc = "GPIO interrupt status set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_W1TS_SPEC;
impl crate::RegisterSpec for STATUS_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`status_w1ts::W`](W) writer structure"]
impl crate::Writable for STATUS_W1TS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS_W1TS to value 0"]
impl crate::Resettable for STATUS_W1TS_SPEC {
    const RESET_VALUE: u32 = 0;
}
