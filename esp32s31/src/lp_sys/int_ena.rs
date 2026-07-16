#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `LP_CORE_IBUS_TIMEOUT_INT_ENA` reader - "]
pub type LP_CORE_IBUS_TIMEOUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_CORE_IBUS_TIMEOUT_INT_ENA` writer - "]
pub type LP_CORE_IBUS_TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_CORE_DBUS_TIMEOUT_INT_ENA` reader - "]
pub type LP_CORE_DBUS_TIMEOUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_CORE_DBUS_TIMEOUT_INT_ENA` writer - "]
pub type LP_CORE_DBUS_TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_TASK_ULP_INT_ENA` reader - "]
pub type ETM_TASK_ULP_INT_ENA_R = crate::BitReader;
#[doc = "Field `ETM_TASK_ULP_INT_ENA` writer - "]
pub type ETM_TASK_ULP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW_CLK_TICK_INT_ENA` reader - "]
pub type SLOW_CLK_TICK_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLOW_CLK_TICK_INT_ENA` writer - "]
pub type SLOW_CLK_TICK_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lp_core_ibus_timeout_int_ena(&self) -> LP_CORE_IBUS_TIMEOUT_INT_ENA_R {
        LP_CORE_IBUS_TIMEOUT_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lp_core_dbus_timeout_int_ena(&self) -> LP_CORE_DBUS_TIMEOUT_INT_ENA_R {
        LP_CORE_DBUS_TIMEOUT_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn etm_task_ulp_int_ena(&self) -> ETM_TASK_ULP_INT_ENA_R {
        ETM_TASK_ULP_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slow_clk_tick_int_ena(&self) -> SLOW_CLK_TICK_INT_ENA_R {
        SLOW_CLK_TICK_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "lp_core_ibus_timeout_int_ena",
                &self.lp_core_ibus_timeout_int_ena(),
            )
            .field(
                "lp_core_dbus_timeout_int_ena",
                &self.lp_core_dbus_timeout_int_ena(),
            )
            .field("etm_task_ulp_int_ena", &self.etm_task_ulp_int_ena())
            .field("slow_clk_tick_int_ena", &self.slow_clk_tick_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lp_core_ibus_timeout_int_ena(
        &mut self,
    ) -> LP_CORE_IBUS_TIMEOUT_INT_ENA_W<'_, INT_ENA_SPEC> {
        LP_CORE_IBUS_TIMEOUT_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lp_core_dbus_timeout_int_ena(
        &mut self,
    ) -> LP_CORE_DBUS_TIMEOUT_INT_ENA_W<'_, INT_ENA_SPEC> {
        LP_CORE_DBUS_TIMEOUT_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn etm_task_ulp_int_ena(&mut self) -> ETM_TASK_ULP_INT_ENA_W<'_, INT_ENA_SPEC> {
        ETM_TASK_ULP_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slow_clk_tick_int_ena(&mut self) -> SLOW_CLK_TICK_INT_ENA_W<'_, INT_ENA_SPEC> {
        SLOW_CLK_TICK_INT_ENA_W::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
