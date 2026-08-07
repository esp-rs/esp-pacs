#[doc = "Register `HP_PERI_PMS_CONF` writer"]
pub type W = crate::W<HP_PERI_PMS_CONF_SPEC>;
#[doc = "Field `HP_PERI_PMS_EXCEPTION_CLR` writer - Configures whether or not to clear hp peri_pms_record_reg.\\\\ 0: No clear\\\\ 1: Clear peri_pms_record_reg\\\\"]
pub type HP_PERI_PMS_EXCEPTION_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_PERI_PMS_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear hp peri_pms_record_reg.\\\\ 0: No clear\\\\ 1: Clear peri_pms_record_reg\\\\"]
    #[inline(always)]
    pub fn hp_peri_pms_exception_clr(
        &mut self,
    ) -> HP_PERI_PMS_EXCEPTION_CLR_W<'_, HP_PERI_PMS_CONF_SPEC> {
        HP_PERI_PMS_EXCEPTION_CLR_W::new(self, 0)
    }
}
#[doc = "HP Peripherals PMS configuration register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_peri_pms_conf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_PERI_PMS_CONF_SPEC;
impl crate::RegisterSpec for HP_PERI_PMS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_peri_pms_conf::W`](W) writer structure"]
impl crate::Writable for HP_PERI_PMS_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_PERI_PMS_CONF to value 0"]
impl crate::Resettable for HP_PERI_PMS_CONF_SPEC {}
