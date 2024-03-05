#[doc = "Register `WR_MESSAGE_%s` writer"]
pub type W = crate::W<WR_MESSAGE__SPEC>;
#[doc = "Field `WDATA` writer - Store the %sth 32-bit of message."]
pub type WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WR_MESSAGE__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Store the %sth 32-bit of message."]
    #[inline(always)]
    #[must_use]
    pub fn wdata(&mut self) -> WDATA_W<WR_MESSAGE__SPEC> {
        WDATA_W::new(self, 0)
    }
}
#[doc = "Message register %s\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_message_::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WR_MESSAGE__SPEC;
impl crate::RegisterSpec for WR_MESSAGE__SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wr_message_::W`](W) writer structure"]
impl crate::Writable for WR_MESSAGE__SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WR_MESSAGE_%s to value 0"]
impl crate::Resettable for WR_MESSAGE__SPEC {
    const RESET_VALUE: u32 = 0;
}
