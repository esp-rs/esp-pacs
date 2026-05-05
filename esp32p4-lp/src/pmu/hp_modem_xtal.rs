#[doc = "Register `HP_MODEM_XTAL` writer"]
pub type W = crate::W<HP_MODEM_XTAL_SPEC>;
#[doc = "Field `HP_MODEM_XPD_XTAL` writer - PMU_HP_MODEM_XPD_XTAL"]
pub type HP_MODEM_XPD_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_XTAL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31 - PMU_HP_MODEM_XPD_XTAL"]
    #[inline(always)]
    pub fn hp_modem_xpd_xtal(&mut self) -> HP_MODEM_XPD_XTAL_W<'_, HP_MODEM_XTAL_SPEC> {
        HP_MODEM_XPD_XTAL_W::new(self, 31)
    }
}
#[doc = "PMU_HP_MODEM_XTAL\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_xtal::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_XTAL_SPEC;
impl crate::RegisterSpec for HP_MODEM_XTAL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_xtal::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_XTAL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MODEM_XTAL to value 0"]
impl crate::Resettable for HP_MODEM_XTAL_SPEC {}
