#[doc = "Register `TOUCH_CLR` writer"]
pub type W = crate::W<TOUCH_CLR_SPEC>;
#[doc = "Field `TOUCH_CHANNEL_CLR` writer - need_des"]
pub type TOUCH_CHANNEL_CLR_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `TOUCH_STATUS_CLR` writer - need_des"]
pub type TOUCH_STATUS_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:14 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_channel_clr(&mut self) -> TOUCH_CHANNEL_CLR_W<TOUCH_CLR_SPEC> {
        TOUCH_CHANNEL_CLR_W::new(self, 0)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_status_clr(&mut self) -> TOUCH_STATUS_CLR_W<TOUCH_CLR_SPEC> {
        TOUCH_STATUS_CLR_W::new(self, 15)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_CLR_SPEC;
impl crate::RegisterSpec for TOUCH_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`touch_clr::W`](W) writer structure"]
impl crate::Writable for TOUCH_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUCH_CLR to value 0"]
impl crate::Resettable for TOUCH_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
