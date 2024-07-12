#[doc = "Register `CORE1_ACS_CACHE_INT_CLR` writer"]
pub type W = crate::W<CORE1_ACS_CACHE_INT_CLR_SPEC>;
#[doc = "Field `CORE1_IBUS_ACS_MSK_IC_INT_CLR` writer - The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access."]
pub type CORE1_IBUS_ACS_MSK_IC_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_IBUS_WR_IC_INT_CLR` writer - The bit is used to clear interrupt by ibus trying to write icache"]
pub type CORE1_IBUS_WR_IC_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_IBUS_REJECT_INT_CLR` writer - The bit is used to clear interrupt by authentication fail."]
pub type CORE1_IBUS_REJECT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_DBUS_ACS_MSK_DC_INT_CLR` writer - The bit is used to clear interrupt by cpu access dcache while the corresponding dbus is disabled or dcache is disabled which include speculative access."]
pub type CORE1_DBUS_ACS_MSK_DC_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_DBUS_REJECT_INT_CLR` writer - The bit is used to clear interrupt by authentication fail."]
pub type CORE1_DBUS_REJECT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE1_ACS_CACHE_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access."]
    #[inline(always)]
    #[must_use]
    pub fn core1_ibus_acs_msk_ic_int_clr(
        &mut self,
    ) -> CORE1_IBUS_ACS_MSK_IC_INT_CLR_W<CORE1_ACS_CACHE_INT_CLR_SPEC> {
        CORE1_IBUS_ACS_MSK_IC_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to clear interrupt by ibus trying to write icache"]
    #[inline(always)]
    #[must_use]
    pub fn core1_ibus_wr_ic_int_clr(
        &mut self,
    ) -> CORE1_IBUS_WR_IC_INT_CLR_W<CORE1_ACS_CACHE_INT_CLR_SPEC> {
        CORE1_IBUS_WR_IC_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to clear interrupt by authentication fail."]
    #[inline(always)]
    #[must_use]
    pub fn core1_ibus_reject_int_clr(
        &mut self,
    ) -> CORE1_IBUS_REJECT_INT_CLR_W<CORE1_ACS_CACHE_INT_CLR_SPEC> {
        CORE1_IBUS_REJECT_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - The bit is used to clear interrupt by cpu access dcache while the corresponding dbus is disabled or dcache is disabled which include speculative access."]
    #[inline(always)]
    #[must_use]
    pub fn core1_dbus_acs_msk_dc_int_clr(
        &mut self,
    ) -> CORE1_DBUS_ACS_MSK_DC_INT_CLR_W<CORE1_ACS_CACHE_INT_CLR_SPEC> {
        CORE1_DBUS_ACS_MSK_DC_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - The bit is used to clear interrupt by authentication fail."]
    #[inline(always)]
    #[must_use]
    pub fn core1_dbus_reject_int_clr(
        &mut self,
    ) -> CORE1_DBUS_REJECT_INT_CLR_W<CORE1_ACS_CACHE_INT_CLR_SPEC> {
        CORE1_DBUS_REJECT_INT_CLR_W::new(self, 4)
    }
}
#[doc = "******* Description ***********\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_acs_cache_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE1_ACS_CACHE_INT_CLR_SPEC;
impl crate::RegisterSpec for CORE1_ACS_CACHE_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`core1_acs_cache_int_clr::W`](W) writer structure"]
impl crate::Writable for CORE1_ACS_CACHE_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE1_ACS_CACHE_INT_CLR to value 0"]
impl crate::Resettable for CORE1_ACS_CACHE_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
