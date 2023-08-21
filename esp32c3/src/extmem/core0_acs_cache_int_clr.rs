#[doc = "Register `CORE0_ACS_CACHE_INT_CLR` writer"]
pub type W = crate::W<CORE0_ACS_CACHE_INT_CLR_SPEC>;
#[doc = "Field `CORE0_IBUS_ACS_MSK_IC_INT_CLR` writer - The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access."]
pub type CORE0_IBUS_ACS_MSK_IC_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CORE0_IBUS_WR_IC_INT_CLR` writer - The bit is used to clear interrupt by ibus trying to write icache"]
pub type CORE0_IBUS_WR_IC_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CORE0_IBUS_REJECT_INT_CLR` writer - The bit is used to clear interrupt by authentication fail."]
pub type CORE0_IBUS_REJECT_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CORE0_DBUS_ACS_MSK_IC_INT_CLR` writer - The bit is used to clear interrupt by cpu access icache while the corresponding dbus is disabled or icache is disabled which include speculative access."]
pub type CORE0_DBUS_ACS_MSK_IC_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CORE0_DBUS_REJECT_INT_CLR` writer - The bit is used to clear interrupt by authentication fail."]
pub type CORE0_DBUS_REJECT_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CORE0_DBUS_WR_IC_INT_CLR` writer - The bit is used to clear interrupt by dbus trying to write icache"]
pub type CORE0_DBUS_WR_IC_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE0_ACS_CACHE_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access."]
    #[inline(always)]
    #[must_use]
    pub fn core0_ibus_acs_msk_ic_int_clr(
        &mut self,
    ) -> CORE0_IBUS_ACS_MSK_IC_INT_CLR_W<CORE0_ACS_CACHE_INT_CLR_SPEC, 0> {
        CORE0_IBUS_ACS_MSK_IC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to clear interrupt by ibus trying to write icache"]
    #[inline(always)]
    #[must_use]
    pub fn core0_ibus_wr_ic_int_clr(
        &mut self,
    ) -> CORE0_IBUS_WR_IC_INT_CLR_W<CORE0_ACS_CACHE_INT_CLR_SPEC, 1> {
        CORE0_IBUS_WR_IC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to clear interrupt by authentication fail."]
    #[inline(always)]
    #[must_use]
    pub fn core0_ibus_reject_int_clr(
        &mut self,
    ) -> CORE0_IBUS_REJECT_INT_CLR_W<CORE0_ACS_CACHE_INT_CLR_SPEC, 2> {
        CORE0_IBUS_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - The bit is used to clear interrupt by cpu access icache while the corresponding dbus is disabled or icache is disabled which include speculative access."]
    #[inline(always)]
    #[must_use]
    pub fn core0_dbus_acs_msk_ic_int_clr(
        &mut self,
    ) -> CORE0_DBUS_ACS_MSK_IC_INT_CLR_W<CORE0_ACS_CACHE_INT_CLR_SPEC, 3> {
        CORE0_DBUS_ACS_MSK_IC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - The bit is used to clear interrupt by authentication fail."]
    #[inline(always)]
    #[must_use]
    pub fn core0_dbus_reject_int_clr(
        &mut self,
    ) -> CORE0_DBUS_REJECT_INT_CLR_W<CORE0_ACS_CACHE_INT_CLR_SPEC, 4> {
        CORE0_DBUS_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - The bit is used to clear interrupt by dbus trying to write icache"]
    #[inline(always)]
    #[must_use]
    pub fn core0_dbus_wr_ic_int_clr(
        &mut self,
    ) -> CORE0_DBUS_WR_IC_INT_CLR_W<CORE0_ACS_CACHE_INT_CLR_SPEC, 5> {
        CORE0_DBUS_WR_IC_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core0_acs_cache_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE0_ACS_CACHE_INT_CLR_SPEC;
impl crate::RegisterSpec for CORE0_ACS_CACHE_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`core0_acs_cache_int_clr::W`](W) writer structure"]
impl crate::Writable for CORE0_ACS_CACHE_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE0_ACS_CACHE_INT_CLR to value 0"]
impl crate::Resettable for CORE0_ACS_CACHE_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
