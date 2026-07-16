#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `LP_CORE_IBUS_TIMEOUT_INT_ST` reader - "]
pub type LP_CORE_IBUS_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_CORE_DBUS_TIMEOUT_INT_ST` reader - "]
pub type LP_CORE_DBUS_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `ETM_TASK_ULP_INT_ST` reader - "]
pub type ETM_TASK_ULP_INT_ST_R = crate::BitReader;
#[doc = "Field `SLOW_CLK_TICK_INT_ST` reader - "]
pub type SLOW_CLK_TICK_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lp_core_ibus_timeout_int_st(&self) -> LP_CORE_IBUS_TIMEOUT_INT_ST_R {
        LP_CORE_IBUS_TIMEOUT_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lp_core_dbus_timeout_int_st(&self) -> LP_CORE_DBUS_TIMEOUT_INT_ST_R {
        LP_CORE_DBUS_TIMEOUT_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn etm_task_ulp_int_st(&self) -> ETM_TASK_ULP_INT_ST_R {
        ETM_TASK_ULP_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
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
                "lp_core_ibus_timeout_int_st",
                &self.lp_core_ibus_timeout_int_st(),
            )
            .field(
                "lp_core_dbus_timeout_int_st",
                &self.lp_core_dbus_timeout_int_st(),
            )
            .field("etm_task_ulp_int_st", &self.etm_task_ulp_int_st())
            .field("slow_clk_tick_int_st", &self.slow_clk_tick_int_st())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
