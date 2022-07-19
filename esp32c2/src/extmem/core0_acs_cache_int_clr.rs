#[doc = "Register `CORE0_ACS_CACHE_INT_CLR` writer"]
pub struct W(crate::W<CORE0_ACS_CACHE_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE0_ACS_CACHE_INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CORE0_ACS_CACHE_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE0_ACS_CACHE_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE0_IBUS_ACS_MSK_IC_INT_CLR` writer - The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access."]
pub type CORE0_IBUS_ACS_MSK_IC_INT_CLR_W<'a> =
    crate::BitWriter<'a, u32, CORE0_ACS_CACHE_INT_CLR_SPEC, bool, 0>;
#[doc = "Field `CORE0_IBUS_WR_IC_INT_CLR` writer - The bit is used to clear interrupt by ibus trying to write icache"]
pub type CORE0_IBUS_WR_IC_INT_CLR_W<'a> =
    crate::BitWriter<'a, u32, CORE0_ACS_CACHE_INT_CLR_SPEC, bool, 1>;
#[doc = "Field `CORE0_IBUS_REJECT_INT_CLR` writer - The bit is used to clear interrupt by authentication fail."]
pub type CORE0_IBUS_REJECT_INT_CLR_W<'a> =
    crate::BitWriter<'a, u32, CORE0_ACS_CACHE_INT_CLR_SPEC, bool, 2>;
#[doc = "Field `CORE0_DBUS_ACS_MSK_IC_INT_CLR` writer - The bit is used to clear interrupt by cpu access icache while the corresponding dbus is disabled or icache is disabled which include speculative access."]
pub type CORE0_DBUS_ACS_MSK_IC_INT_CLR_W<'a> =
    crate::BitWriter<'a, u32, CORE0_ACS_CACHE_INT_CLR_SPEC, bool, 3>;
#[doc = "Field `CORE0_DBUS_REJECT_INT_CLR` writer - The bit is used to clear interrupt by authentication fail."]
pub type CORE0_DBUS_REJECT_INT_CLR_W<'a> =
    crate::BitWriter<'a, u32, CORE0_ACS_CACHE_INT_CLR_SPEC, bool, 4>;
#[doc = "Field `CORE0_DBUS_WR_IC_INT_CLR` writer - The bit is used to clear interrupt by dbus trying to write icache"]
pub type CORE0_DBUS_WR_IC_INT_CLR_W<'a> =
    crate::BitWriter<'a, u32, CORE0_ACS_CACHE_INT_CLR_SPEC, bool, 5>;
impl W {
    #[doc = "Bit 0 - The bit is used to clear interrupt by cpu access icache while the corresponding ibus is disabled or icache is disabled which include speculative access."]
    #[inline(always)]
    pub fn core0_ibus_acs_msk_ic_int_clr(&mut self) -> CORE0_IBUS_ACS_MSK_IC_INT_CLR_W {
        CORE0_IBUS_ACS_MSK_IC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to clear interrupt by ibus trying to write icache"]
    #[inline(always)]
    pub fn core0_ibus_wr_ic_int_clr(&mut self) -> CORE0_IBUS_WR_IC_INT_CLR_W {
        CORE0_IBUS_WR_IC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to clear interrupt by authentication fail."]
    #[inline(always)]
    pub fn core0_ibus_reject_int_clr(&mut self) -> CORE0_IBUS_REJECT_INT_CLR_W {
        CORE0_IBUS_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - The bit is used to clear interrupt by cpu access icache while the corresponding dbus is disabled or icache is disabled which include speculative access."]
    #[inline(always)]
    pub fn core0_dbus_acs_msk_ic_int_clr(&mut self) -> CORE0_DBUS_ACS_MSK_IC_INT_CLR_W {
        CORE0_DBUS_ACS_MSK_IC_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - The bit is used to clear interrupt by authentication fail."]
    #[inline(always)]
    pub fn core0_dbus_reject_int_clr(&mut self) -> CORE0_DBUS_REJECT_INT_CLR_W {
        CORE0_DBUS_REJECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - The bit is used to clear interrupt by dbus trying to write icache"]
    #[inline(always)]
    pub fn core0_dbus_wr_ic_int_clr(&mut self) -> CORE0_DBUS_WR_IC_INT_CLR_W {
        CORE0_DBUS_WR_IC_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core0_acs_cache_int_clr](index.html) module"]
pub struct CORE0_ACS_CACHE_INT_CLR_SPEC;
impl crate::RegisterSpec for CORE0_ACS_CACHE_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [core0_acs_cache_int_clr::W](W) writer structure"]
impl crate::Writable for CORE0_ACS_CACHE_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE0_ACS_CACHE_INT_CLR to value 0"]
impl crate::Resettable for CORE0_ACS_CACHE_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
