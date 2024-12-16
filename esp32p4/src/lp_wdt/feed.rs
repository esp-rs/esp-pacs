#[doc = "Register `FEED` writer"]
pub type W = crate::W<FEED_SPEC>;
#[doc = "Field `FEED` writer - need_des"]
pub type FEED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FEED_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn feed(&mut self) -> FEED_W<FEED_SPEC> {
        FEED_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feed::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FEED_SPEC;
impl crate::RegisterSpec for FEED_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`feed::W`](W) writer structure"]
impl crate::Writable for FEED_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEED to value 0"]
impl crate::Resettable for FEED_SPEC {
    const RESET_VALUE: u32 = 0;
}
