#[doc = "Register `CORE_TIMEOUT_INT_CLR` writer"]
pub type W = crate::W<CORE_TIMEOUT_INT_CLR_SPEC>;
#[doc = "Field `CORE0_AHB_TIMEOUT_INT_CLR` writer - Write 1 to clear hp_core0_ahb_timeout int"]
pub type CORE0_AHB_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_AHB_TIMEOUT_INT_CLR` writer - Write 1 to clear hp_core1_ahb_timeout int"]
pub type CORE1_AHB_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0_IBUS_TIMEOUT_INT_CLR` writer - Write 1 to clear hp_core0_ibus_timeout int"]
pub type CORE0_IBUS_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_IBUS_TIMEOUT_INT_CLR` writer - Write 1 to clear hp_core1_ibus_timeout int"]
pub type CORE1_IBUS_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE0_DBUS_TIMEOUT_INT_CLR` writer - Write 1 to clear hp_core0_dbus_timeout int"]
pub type CORE0_DBUS_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE1_DBUS_TIMEOUT_INT_CLR` writer - Write 1 to clear hp_core1_dbus_timeout int"]
pub type CORE1_DBUS_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_TIMEOUT_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear hp_core0_ahb_timeout int"]
    #[inline(always)]
    #[must_use]
    pub fn core0_ahb_timeout_int_clr(
        &mut self,
    ) -> CORE0_AHB_TIMEOUT_INT_CLR_W<CORE_TIMEOUT_INT_CLR_SPEC> {
        CORE0_AHB_TIMEOUT_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear hp_core1_ahb_timeout int"]
    #[inline(always)]
    #[must_use]
    pub fn core1_ahb_timeout_int_clr(
        &mut self,
    ) -> CORE1_AHB_TIMEOUT_INT_CLR_W<CORE_TIMEOUT_INT_CLR_SPEC> {
        CORE1_AHB_TIMEOUT_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear hp_core0_ibus_timeout int"]
    #[inline(always)]
    #[must_use]
    pub fn core0_ibus_timeout_int_clr(
        &mut self,
    ) -> CORE0_IBUS_TIMEOUT_INT_CLR_W<CORE_TIMEOUT_INT_CLR_SPEC> {
        CORE0_IBUS_TIMEOUT_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear hp_core1_ibus_timeout int"]
    #[inline(always)]
    #[must_use]
    pub fn core1_ibus_timeout_int_clr(
        &mut self,
    ) -> CORE1_IBUS_TIMEOUT_INT_CLR_W<CORE_TIMEOUT_INT_CLR_SPEC> {
        CORE1_IBUS_TIMEOUT_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear hp_core0_dbus_timeout int"]
    #[inline(always)]
    #[must_use]
    pub fn core0_dbus_timeout_int_clr(
        &mut self,
    ) -> CORE0_DBUS_TIMEOUT_INT_CLR_W<CORE_TIMEOUT_INT_CLR_SPEC> {
        CORE0_DBUS_TIMEOUT_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear hp_core1_dbus_timeout int"]
    #[inline(always)]
    #[must_use]
    pub fn core1_dbus_timeout_int_clr(
        &mut self,
    ) -> CORE1_DBUS_TIMEOUT_INT_CLR_W<CORE_TIMEOUT_INT_CLR_SPEC> {
        CORE1_DBUS_TIMEOUT_INT_CLR_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_timeout_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_TIMEOUT_INT_CLR_SPEC;
impl crate::RegisterSpec for CORE_TIMEOUT_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`core_timeout_int_clr::W`](W) writer structure"]
impl crate::Writable for CORE_TIMEOUT_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_TIMEOUT_INT_CLR to value 0"]
impl crate::Resettable for CORE_TIMEOUT_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
