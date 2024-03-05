#[doc = "Register `CACHE_ACS_CNT_CLR` writer"]
pub type W = crate::W<CACHE_ACS_CNT_CLR_SPEC>;
#[doc = "Field `IBUS_ACS_CNT_CLR` writer - The bit is used to clear ibus counter."]
pub type IBUS_ACS_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUS_ACS_CNT_CLR` writer - The bit is used to clear dbus counter."]
pub type DBUS_ACS_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_ACS_CNT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to clear ibus counter."]
    #[inline(always)]
    #[must_use]
    pub fn ibus_acs_cnt_clr(&mut self) -> IBUS_ACS_CNT_CLR_W<CACHE_ACS_CNT_CLR_SPEC> {
        IBUS_ACS_CNT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to clear dbus counter."]
    #[inline(always)]
    #[must_use]
    pub fn dbus_acs_cnt_clr(&mut self) -> DBUS_ACS_CNT_CLR_W<CACHE_ACS_CNT_CLR_SPEC> {
        DBUS_ACS_CNT_CLR_W::new(self, 1)
    }
}
#[doc = "This description will be updated in the near future.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_acs_cnt_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ACS_CNT_CLR_SPEC;
impl crate::RegisterSpec for CACHE_ACS_CNT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cache_acs_cnt_clr::W`](W) writer structure"]
impl crate::Writable for CACHE_ACS_CNT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHE_ACS_CNT_CLR to value 0"]
impl crate::Resettable for CACHE_ACS_CNT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
