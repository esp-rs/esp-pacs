#[doc = "Register `LP_PERI_PMS_CONF` writer"]
pub type W = crate::W<LP_PERI_PMS_CONF_SPEC>;
#[doc = "Field `LP_PERI_PMS_EXCEPTION_CLR` writer - Configures whether or not to clear lp peri_pms_record_reg.\\\\ 0: No clear\\\\ 1: Clear peri_pms_record_reg\\\\"]
pub type LP_PERI_PMS_EXCEPTION_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_PERI_PMS_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear lp peri_pms_record_reg.\\\\ 0: No clear\\\\ 1: Clear peri_pms_record_reg\\\\"]
    #[inline(always)]
    pub fn lp_peri_pms_exception_clr(
        &mut self,
    ) -> LP_PERI_PMS_EXCEPTION_CLR_W<'_, LP_PERI_PMS_CONF_SPEC> {
        LP_PERI_PMS_EXCEPTION_CLR_W::new(self, 0)
    }
}
#[doc = "LP Peripherals PMS configuration register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_peri_pms_conf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_PERI_PMS_CONF_SPEC;
impl crate::RegisterSpec for LP_PERI_PMS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lp_peri_pms_conf::W`](W) writer structure"]
impl crate::Writable for LP_PERI_PMS_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_PERI_PMS_CONF to value 0"]
impl crate::Resettable for LP_PERI_PMS_CONF_SPEC {}
