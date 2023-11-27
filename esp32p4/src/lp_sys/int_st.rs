#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `LP_ADDRHOLE_INT_ST` reader - the masked interrupt status of lp addrhole (for lp peri and lp ram tee apm, and lp matrix default slave)"]
pub type LP_ADDRHOLE_INT_ST_R = crate::BitReader;
#[doc = "Field `IDBUS_ADDRHOLE_INT_ST` reader - the masked interrupt status of idbus addrhole(only for lp cpu ibus and dbus)"]
pub type IDBUS_ADDRHOLE_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_CORE_AHB_TIMEOUT_INT_ST` reader - the masked interrupt status of lp core ahb bus timeout"]
pub type LP_CORE_AHB_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_CORE_IBUS_TIMEOUT_INT_ST` reader - the masked interrupt status of lp core ibus timeout"]
pub type LP_CORE_IBUS_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_CORE_DBUS_TIMEOUT_INT_ST` reader - the masked interrupt status of lp core dbus timeout"]
pub type LP_CORE_DBUS_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `ETM_TASK_ULP_INT_ST` reader - the masked interrupt status of etm task ulp"]
pub type ETM_TASK_ULP_INT_ST_R = crate::BitReader;
#[doc = "Field `SLOW_CLK_TICK_INT_ST` reader - the masked interrupt status of slow_clk_tick"]
pub type SLOW_CLK_TICK_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - the masked interrupt status of lp addrhole (for lp peri and lp ram tee apm, and lp matrix default slave)"]
    #[inline(always)]
    pub fn lp_addrhole_int_st(&self) -> LP_ADDRHOLE_INT_ST_R {
        LP_ADDRHOLE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the masked interrupt status of idbus addrhole(only for lp cpu ibus and dbus)"]
    #[inline(always)]
    pub fn idbus_addrhole_int_st(&self) -> IDBUS_ADDRHOLE_INT_ST_R {
        IDBUS_ADDRHOLE_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the masked interrupt status of lp core ahb bus timeout"]
    #[inline(always)]
    pub fn lp_core_ahb_timeout_int_st(&self) -> LP_CORE_AHB_TIMEOUT_INT_ST_R {
        LP_CORE_AHB_TIMEOUT_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the masked interrupt status of lp core ibus timeout"]
    #[inline(always)]
    pub fn lp_core_ibus_timeout_int_st(&self) -> LP_CORE_IBUS_TIMEOUT_INT_ST_R {
        LP_CORE_IBUS_TIMEOUT_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the masked interrupt status of lp core dbus timeout"]
    #[inline(always)]
    pub fn lp_core_dbus_timeout_int_st(&self) -> LP_CORE_DBUS_TIMEOUT_INT_ST_R {
        LP_CORE_DBUS_TIMEOUT_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the masked interrupt status of etm task ulp"]
    #[inline(always)]
    pub fn etm_task_ulp_int_st(&self) -> ETM_TASK_ULP_INT_ST_R {
        ETM_TASK_ULP_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - the masked interrupt status of slow_clk_tick"]
    #[inline(always)]
    pub fn slow_clk_tick_int_st(&self) -> SLOW_CLK_TICK_INT_ST_R {
        SLOW_CLK_TICK_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "lp_addrhole_int_st",
                &format_args!("{}", self.lp_addrhole_int_st().bit()),
            )
            .field(
                "idbus_addrhole_int_st",
                &format_args!("{}", self.idbus_addrhole_int_st().bit()),
            )
            .field(
                "lp_core_ahb_timeout_int_st",
                &format_args!("{}", self.lp_core_ahb_timeout_int_st().bit()),
            )
            .field(
                "lp_core_ibus_timeout_int_st",
                &format_args!("{}", self.lp_core_ibus_timeout_int_st().bit()),
            )
            .field(
                "lp_core_dbus_timeout_int_st",
                &format_args!("{}", self.lp_core_dbus_timeout_int_st().bit()),
            )
            .field(
                "etm_task_ulp_int_st",
                &format_args!("{}", self.etm_task_ulp_int_st().bit()),
            )
            .field(
                "slow_clk_tick_int_st",
                &format_args!("{}", self.slow_clk_tick_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "masked interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
