#[doc = "Register `CORE_TIMEOUT_INT_ST` reader"]
pub type R = crate::R<CORE_TIMEOUT_INT_ST_SPEC>;
#[doc = "Field `CORE0_AHB_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core0 ahb timeout"]
pub type CORE0_AHB_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `CORE1_AHB_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core1 ahb timeout"]
pub type CORE1_AHB_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `CORE0_IBUS_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core0 ibus timeout"]
pub type CORE0_IBUS_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `CORE1_IBUS_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core1 ibus timeout"]
pub type CORE1_IBUS_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `CORE0_DBUS_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core0 dbus timeout"]
pub type CORE0_DBUS_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `CORE1_DBUS_TIMEOUT_INT_ST` reader - the masked interrupt status of hp core1 dbus timeout"]
pub type CORE1_DBUS_TIMEOUT_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - the masked interrupt status of hp core0 ahb timeout"]
    #[inline(always)]
    pub fn core0_ahb_timeout_int_st(&self) -> CORE0_AHB_TIMEOUT_INT_ST_R {
        CORE0_AHB_TIMEOUT_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the masked interrupt status of hp core1 ahb timeout"]
    #[inline(always)]
    pub fn core1_ahb_timeout_int_st(&self) -> CORE1_AHB_TIMEOUT_INT_ST_R {
        CORE1_AHB_TIMEOUT_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the masked interrupt status of hp core0 ibus timeout"]
    #[inline(always)]
    pub fn core0_ibus_timeout_int_st(&self) -> CORE0_IBUS_TIMEOUT_INT_ST_R {
        CORE0_IBUS_TIMEOUT_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the masked interrupt status of hp core1 ibus timeout"]
    #[inline(always)]
    pub fn core1_ibus_timeout_int_st(&self) -> CORE1_IBUS_TIMEOUT_INT_ST_R {
        CORE1_IBUS_TIMEOUT_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the masked interrupt status of hp core0 dbus timeout"]
    #[inline(always)]
    pub fn core0_dbus_timeout_int_st(&self) -> CORE0_DBUS_TIMEOUT_INT_ST_R {
        CORE0_DBUS_TIMEOUT_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the masked interrupt status of hp core1 dbus timeout"]
    #[inline(always)]
    pub fn core1_dbus_timeout_int_st(&self) -> CORE1_DBUS_TIMEOUT_INT_ST_R {
        CORE1_DBUS_TIMEOUT_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_TIMEOUT_INT_ST")
            .field(
                "core0_ahb_timeout_int_st",
                &format_args!("{}", self.core0_ahb_timeout_int_st().bit()),
            )
            .field(
                "core1_ahb_timeout_int_st",
                &format_args!("{}", self.core1_ahb_timeout_int_st().bit()),
            )
            .field(
                "core0_ibus_timeout_int_st",
                &format_args!("{}", self.core0_ibus_timeout_int_st().bit()),
            )
            .field(
                "core1_ibus_timeout_int_st",
                &format_args!("{}", self.core1_ibus_timeout_int_st().bit()),
            )
            .field(
                "core0_dbus_timeout_int_st",
                &format_args!("{}", self.core0_dbus_timeout_int_st().bit()),
            )
            .field(
                "core1_dbus_timeout_int_st",
                &format_args!("{}", self.core1_dbus_timeout_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_TIMEOUT_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "masked interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_timeout_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_TIMEOUT_INT_ST_SPEC;
impl crate::RegisterSpec for CORE_TIMEOUT_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_timeout_int_st::R`](R) reader structure"]
impl crate::Readable for CORE_TIMEOUT_INT_ST_SPEC {}
#[doc = "`reset()` method sets CORE_TIMEOUT_INT_ST to value 0"]
impl crate::Resettable for CORE_TIMEOUT_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
