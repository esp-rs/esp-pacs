#[doc = "Register `HP_CORE_TIMEOUT_INT_ST` reader"]
pub type R = crate::R<HP_CORE_TIMEOUT_INT_ST_SPEC>;
#[doc = "Field `HP_CORE0_AHB_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core0 ahb timeout"]
pub type HP_CORE0_AHB_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_CORE1_AHB_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core1 ahb timeout"]
pub type HP_CORE1_AHB_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_CORE0_IBUS_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core0 ibus timeout"]
pub type HP_CORE0_IBUS_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_CORE1_IBUS_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core1 ibus timeout"]
pub type HP_CORE1_IBUS_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_CORE0_DBUS_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core0 dbus timeout"]
pub type HP_CORE0_DBUS_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_CORE1_DBUS_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core1 dbus timeout"]
pub type HP_CORE1_DBUS_TIMEOUT_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - the masked interrupt status of hp core0 ahb timeout"]
    #[inline(always)]
    pub fn hp_core0_ahb_timeout_int_st(&self) -> HP_CORE0_AHB_TIMEOUT_INT_ST_R {
        HP_CORE0_AHB_TIMEOUT_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the masked interrupt status of hp core1 ahb timeout"]
    #[inline(always)]
    pub fn hp_core1_ahb_timeout_int_st(&self) -> HP_CORE1_AHB_TIMEOUT_INT_ST_R {
        HP_CORE1_AHB_TIMEOUT_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the masked interrupt status of hp core0 ibus timeout"]
    #[inline(always)]
    pub fn hp_core0_ibus_timeout_int_st(&self) -> HP_CORE0_IBUS_TIMEOUT_INT_ST_R {
        HP_CORE0_IBUS_TIMEOUT_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the masked interrupt status of hp core1 ibus timeout"]
    #[inline(always)]
    pub fn hp_core1_ibus_timeout_int_st(&self) -> HP_CORE1_IBUS_TIMEOUT_INT_ST_R {
        HP_CORE1_IBUS_TIMEOUT_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the masked interrupt status of hp core0 dbus timeout"]
    #[inline(always)]
    pub fn hp_core0_dbus_timeout_int_st(&self) -> HP_CORE0_DBUS_TIMEOUT_INT_ST_R {
        HP_CORE0_DBUS_TIMEOUT_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the masked interrupt status of hp core1 dbus timeout"]
    #[inline(always)]
    pub fn hp_core1_dbus_timeout_int_st(&self) -> HP_CORE1_DBUS_TIMEOUT_INT_ST_R {
        HP_CORE1_DBUS_TIMEOUT_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CORE_TIMEOUT_INT_ST")
            .field(
                "hp_core0_ahb_timeout_int_st",
                &format_args!("{}", self.hp_core0_ahb_timeout_int_st().bit()),
            )
            .field(
                "hp_core1_ahb_timeout_int_st",
                &format_args!("{}", self.hp_core1_ahb_timeout_int_st().bit()),
            )
            .field(
                "hp_core0_ibus_timeout_int_st",
                &format_args!("{}", self.hp_core0_ibus_timeout_int_st().bit()),
            )
            .field(
                "hp_core1_ibus_timeout_int_st",
                &format_args!("{}", self.hp_core1_ibus_timeout_int_st().bit()),
            )
            .field(
                "hp_core0_dbus_timeout_int_st",
                &format_args!("{}", self.hp_core0_dbus_timeout_int_st().bit()),
            )
            .field(
                "hp_core1_dbus_timeout_int_st",
                &format_args!("{}", self.hp_core1_dbus_timeout_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_CORE_TIMEOUT_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "masked interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_core_timeout_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CORE_TIMEOUT_INT_ST_SPEC;
impl crate::RegisterSpec for HP_CORE_TIMEOUT_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_core_timeout_int_st::R`](R) reader structure"]
impl crate::Readable for HP_CORE_TIMEOUT_INT_ST_SPEC {}
#[doc = "`reset()` method sets HP_CORE_TIMEOUT_INT_ST to value 0"]
impl crate::Resettable for HP_CORE_TIMEOUT_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
