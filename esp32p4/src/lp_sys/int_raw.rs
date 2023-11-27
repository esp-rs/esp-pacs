#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `LP_ADDRHOLE_INT_RAW` reader - the raw interrupt status of lp addrhole(for lp peri and lp ram tee apm, and lp matrix default slave)"]
pub type LP_ADDRHOLE_INT_RAW_R = crate::BitReader;
#[doc = "Field `IDBUS_ADDRHOLE_INT_RAW` reader - the raw interrupt status of idbus addrhole(only for lp cpu ibus and dbus)"]
pub type IDBUS_ADDRHOLE_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_CORE_AHB_TIMEOUT_INT_RAW` reader - the raw interrupt status of lp core ahb bus timeout"]
pub type LP_CORE_AHB_TIMEOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_CORE_IBUS_TIMEOUT_INT_RAW` reader - the raw interrupt status of lp core ibus timeout"]
pub type LP_CORE_IBUS_TIMEOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_CORE_DBUS_TIMEOUT_INT_RAW` reader - the raw interrupt status of lp core dbus timeout"]
pub type LP_CORE_DBUS_TIMEOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `ETM_TASK_ULP_INT_RAW` reader - the raw interrupt status of etm task ulp"]
pub type ETM_TASK_ULP_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLOW_CLK_TICK_INT_RAW` reader - the raw interrupt status of slow_clk_tick"]
pub type SLOW_CLK_TICK_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - the raw interrupt status of lp addrhole(for lp peri and lp ram tee apm, and lp matrix default slave)"]
    #[inline(always)]
    pub fn lp_addrhole_int_raw(&self) -> LP_ADDRHOLE_INT_RAW_R {
        LP_ADDRHOLE_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the raw interrupt status of idbus addrhole(only for lp cpu ibus and dbus)"]
    #[inline(always)]
    pub fn idbus_addrhole_int_raw(&self) -> IDBUS_ADDRHOLE_INT_RAW_R {
        IDBUS_ADDRHOLE_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the raw interrupt status of lp core ahb bus timeout"]
    #[inline(always)]
    pub fn lp_core_ahb_timeout_int_raw(&self) -> LP_CORE_AHB_TIMEOUT_INT_RAW_R {
        LP_CORE_AHB_TIMEOUT_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the raw interrupt status of lp core ibus timeout"]
    #[inline(always)]
    pub fn lp_core_ibus_timeout_int_raw(&self) -> LP_CORE_IBUS_TIMEOUT_INT_RAW_R {
        LP_CORE_IBUS_TIMEOUT_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the raw interrupt status of lp core dbus timeout"]
    #[inline(always)]
    pub fn lp_core_dbus_timeout_int_raw(&self) -> LP_CORE_DBUS_TIMEOUT_INT_RAW_R {
        LP_CORE_DBUS_TIMEOUT_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the raw interrupt status of etm task ulp"]
    #[inline(always)]
    pub fn etm_task_ulp_int_raw(&self) -> ETM_TASK_ULP_INT_RAW_R {
        ETM_TASK_ULP_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - the raw interrupt status of slow_clk_tick"]
    #[inline(always)]
    pub fn slow_clk_tick_int_raw(&self) -> SLOW_CLK_TICK_INT_RAW_R {
        SLOW_CLK_TICK_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "lp_addrhole_int_raw",
                &format_args!("{}", self.lp_addrhole_int_raw().bit()),
            )
            .field(
                "idbus_addrhole_int_raw",
                &format_args!("{}", self.idbus_addrhole_int_raw().bit()),
            )
            .field(
                "lp_core_ahb_timeout_int_raw",
                &format_args!("{}", self.lp_core_ahb_timeout_int_raw().bit()),
            )
            .field(
                "lp_core_ibus_timeout_int_raw",
                &format_args!("{}", self.lp_core_ibus_timeout_int_raw().bit()),
            )
            .field(
                "lp_core_dbus_timeout_int_raw",
                &format_args!("{}", self.lp_core_dbus_timeout_int_raw().bit()),
            )
            .field(
                "etm_task_ulp_int_raw",
                &format_args!("{}", self.etm_task_ulp_int_raw().bit()),
            )
            .field(
                "slow_clk_tick_int_raw",
                &format_args!("{}", self.slow_clk_tick_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "raw interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
