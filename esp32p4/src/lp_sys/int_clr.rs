#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `LP_ADDRHOLE_INT_CLR` writer - write 1 to clear lp addrhole int"]
pub type LP_ADDRHOLE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDBUS_ADDRHOLE_INT_CLR` writer - write 1 to clear idbus addrhole int"]
pub type IDBUS_ADDRHOLE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_CORE_AHB_TIMEOUT_INT_CLR` writer - Write 1 to clear lp_core_ahb_timeout int"]
pub type LP_CORE_AHB_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_CORE_IBUS_TIMEOUT_INT_CLR` writer - Write 1 to clear lp_core_ibus_timeout int"]
pub type LP_CORE_IBUS_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_CORE_DBUS_TIMEOUT_INT_CLR` writer - Write 1 to clear lp_core_dbus_timeout int"]
pub type LP_CORE_DBUS_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_ULP_INT_CLR` writer - Write 1 to clear etm tasl ulp int"]
pub type ETM_TASK_ULP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW_CLK_TICK_INT_CLR` writer - Write 1 to clear slow_clk_tick int"]
pub type SLOW_CLK_TICK_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to clear lp addrhole int"]
    #[inline(always)]
    pub fn lp_addrhole_int_clr(&mut self) -> LP_ADDRHOLE_INT_CLR_W<INT_CLR_SPEC> {
        LP_ADDRHOLE_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - write 1 to clear idbus addrhole int"]
    #[inline(always)]
    pub fn idbus_addrhole_int_clr(&mut self) -> IDBUS_ADDRHOLE_INT_CLR_W<INT_CLR_SPEC> {
        IDBUS_ADDRHOLE_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear lp_core_ahb_timeout int"]
    #[inline(always)]
    pub fn lp_core_ahb_timeout_int_clr(&mut self) -> LP_CORE_AHB_TIMEOUT_INT_CLR_W<INT_CLR_SPEC> {
        LP_CORE_AHB_TIMEOUT_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear lp_core_ibus_timeout int"]
    #[inline(always)]
    pub fn lp_core_ibus_timeout_int_clr(&mut self) -> LP_CORE_IBUS_TIMEOUT_INT_CLR_W<INT_CLR_SPEC> {
        LP_CORE_IBUS_TIMEOUT_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear lp_core_dbus_timeout int"]
    #[inline(always)]
    pub fn lp_core_dbus_timeout_int_clr(&mut self) -> LP_CORE_DBUS_TIMEOUT_INT_CLR_W<INT_CLR_SPEC> {
        LP_CORE_DBUS_TIMEOUT_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear etm tasl ulp int"]
    #[inline(always)]
    pub fn etm_task_ulp_int_clr(&mut self) -> ETM_TASK_ULP_INT_CLR_W<INT_CLR_SPEC> {
        ETM_TASK_ULP_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to clear slow_clk_tick int"]
    #[inline(always)]
    pub fn slow_clk_tick_int_clr(&mut self) -> SLOW_CLK_TICK_INT_CLR_W<INT_CLR_SPEC> {
        SLOW_CLK_TICK_INT_CLR_W::new(self, 6)
    }
}
#[doc = "interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
