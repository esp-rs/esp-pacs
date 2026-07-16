#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `LP_CORE_IBUS_TIMEOUT_INT_RAW` reader - "]
pub type LP_CORE_IBUS_TIMEOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_CORE_IBUS_TIMEOUT_INT_RAW` writer - "]
pub type LP_CORE_IBUS_TIMEOUT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_CORE_DBUS_TIMEOUT_INT_RAW` reader - "]
pub type LP_CORE_DBUS_TIMEOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_CORE_DBUS_TIMEOUT_INT_RAW` writer - "]
pub type LP_CORE_DBUS_TIMEOUT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_ULP_INT_RAW` reader - "]
pub type ETM_TASK_ULP_INT_RAW_R = crate::BitReader;
#[doc = "Field `ETM_TASK_ULP_INT_RAW` writer - "]
pub type ETM_TASK_ULP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW_CLK_TICK_INT_RAW` reader - "]
pub type SLOW_CLK_TICK_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLOW_CLK_TICK_INT_RAW` writer - "]
pub type SLOW_CLK_TICK_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lp_core_ibus_timeout_int_raw(&self) -> LP_CORE_IBUS_TIMEOUT_INT_RAW_R {
        LP_CORE_IBUS_TIMEOUT_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lp_core_dbus_timeout_int_raw(&self) -> LP_CORE_DBUS_TIMEOUT_INT_RAW_R {
        LP_CORE_DBUS_TIMEOUT_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn etm_task_ulp_int_raw(&self) -> ETM_TASK_ULP_INT_RAW_R {
        ETM_TASK_ULP_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
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
                "lp_core_ibus_timeout_int_raw",
                &self.lp_core_ibus_timeout_int_raw(),
            )
            .field(
                "lp_core_dbus_timeout_int_raw",
                &self.lp_core_dbus_timeout_int_raw(),
            )
            .field("etm_task_ulp_int_raw", &self.etm_task_ulp_int_raw())
            .field("slow_clk_tick_int_raw", &self.slow_clk_tick_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lp_core_ibus_timeout_int_raw(
        &mut self,
    ) -> LP_CORE_IBUS_TIMEOUT_INT_RAW_W<'_, INT_RAW_SPEC> {
        LP_CORE_IBUS_TIMEOUT_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lp_core_dbus_timeout_int_raw(
        &mut self,
    ) -> LP_CORE_DBUS_TIMEOUT_INT_RAW_W<'_, INT_RAW_SPEC> {
        LP_CORE_DBUS_TIMEOUT_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn etm_task_ulp_int_raw(&mut self) -> ETM_TASK_ULP_INT_RAW_W<'_, INT_RAW_SPEC> {
        ETM_TASK_ULP_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slow_clk_tick_int_raw(&mut self) -> SLOW_CLK_TICK_INT_RAW_W<'_, INT_RAW_SPEC> {
        SLOW_CLK_TICK_INT_RAW_W::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
