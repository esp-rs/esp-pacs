#[doc = "Register `HP_MODEM_ICG_MODEM` writer"]
pub type W = crate::W<HP_MODEM_ICG_MODEM_SPEC>;
#[doc = "Field `HP_MODEM_DIG_ICG_MODEM_CODE` writer - PMU_HP_MODEM_DIG_ICG_MODEM_CODE"]
pub type HP_MODEM_DIG_ICG_MODEM_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_ICG_MODEM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 30:31 - PMU_HP_MODEM_DIG_ICG_MODEM_CODE"]
    #[inline(always)]
    pub fn hp_modem_dig_icg_modem_code(
        &mut self,
    ) -> HP_MODEM_DIG_ICG_MODEM_CODE_W<'_, HP_MODEM_ICG_MODEM_SPEC> {
        HP_MODEM_DIG_ICG_MODEM_CODE_W::new(self, 30)
    }
}
#[doc = "PMU_HP_MODEM_ICG_MODEM\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_icg_modem::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_ICG_MODEM_SPEC;
impl crate::RegisterSpec for HP_MODEM_ICG_MODEM_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_icg_modem::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_ICG_MODEM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MODEM_ICG_MODEM to value 0"]
impl crate::Resettable for HP_MODEM_ICG_MODEM_SPEC {}
