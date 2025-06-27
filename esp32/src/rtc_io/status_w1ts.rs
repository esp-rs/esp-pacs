#[doc = "Register `STATUS_W1TS` writer"]
pub type W = crate::W<STATUS_W1TS_SPEC>;
#[doc = "Field `STATUS_INT_W1TS` writer - GPIO0~17 interrupt status write 1 to set"]
pub type STATUS_INT_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 14:31 - GPIO0~17 interrupt status write 1 to set"]
    #[inline(always)]
    pub fn status_int_w1ts(&mut self) -> STATUS_INT_W1TS_W<STATUS_W1TS_SPEC> {
        STATUS_INT_W1TS_W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
