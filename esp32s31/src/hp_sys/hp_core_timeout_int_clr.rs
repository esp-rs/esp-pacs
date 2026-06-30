#[doc = "Register `HP_CORE_TIMEOUT_INT_CLR` writer"]
pub type W = crate::W<HP_CORE_TIMEOUT_INT_CLR_SPEC>;
#[doc = "Field `HP_CORE0_IBUS_TIMEOUT_INT_CLR` writer - Write 1 to clear hp_core0_ibus_timeout int"]
pub type HP_CORE0_IBUS_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CORE1_IBUS_TIMEOUT_INT_CLR` writer - Write 1 to clear hp_core1_ibus_timeout int"]
pub type HP_CORE1_IBUS_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CORE0_DBUS_TIMEOUT_INT_CLR` writer - Write 1 to clear hp_core0_dbus_timeout int"]
pub type HP_CORE0_DBUS_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CORE1_DBUS_TIMEOUT_INT_CLR` writer - Write 1 to clear hp_core1_dbus_timeout int"]
pub type HP_CORE1_DBUS_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_CORE_TIMEOUT_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 2 - Write 1 to clear hp_core0_ibus_timeout int"]
    #[inline(always)]
    pub fn hp_core0_ibus_timeout_int_clr(
        &mut self,
    ) -> HP_CORE0_IBUS_TIMEOUT_INT_CLR_W<'_, HP_CORE_TIMEOUT_INT_CLR_SPEC> {
        HP_CORE0_IBUS_TIMEOUT_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear hp_core1_ibus_timeout int"]
    #[inline(always)]
    pub fn hp_core1_ibus_timeout_int_clr(
        &mut self,
    ) -> HP_CORE1_IBUS_TIMEOUT_INT_CLR_W<'_, HP_CORE_TIMEOUT_INT_CLR_SPEC> {
        HP_CORE1_IBUS_TIMEOUT_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear hp_core0_dbus_timeout int"]
    #[inline(always)]
    pub fn hp_core0_dbus_timeout_int_clr(
        &mut self,
    ) -> HP_CORE0_DBUS_TIMEOUT_INT_CLR_W<'_, HP_CORE_TIMEOUT_INT_CLR_SPEC> {
        HP_CORE0_DBUS_TIMEOUT_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear hp_core1_dbus_timeout int"]
    #[inline(always)]
    pub fn hp_core1_dbus_timeout_int_clr(
        &mut self,
    ) -> HP_CORE1_DBUS_TIMEOUT_INT_CLR_W<'_, HP_CORE_TIMEOUT_INT_CLR_SPEC> {
        HP_CORE1_DBUS_TIMEOUT_INT_CLR_W::new(self, 5)
    }
}
#[doc = "interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_core_timeout_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CORE_TIMEOUT_INT_CLR_SPEC;
impl crate::RegisterSpec for HP_CORE_TIMEOUT_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_core_timeout_int_clr::W`](W) writer structure"]
impl crate::Writable for HP_CORE_TIMEOUT_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_CORE_TIMEOUT_INT_CLR to value 0"]
impl crate::Resettable for HP_CORE_TIMEOUT_INT_CLR_SPEC {}
