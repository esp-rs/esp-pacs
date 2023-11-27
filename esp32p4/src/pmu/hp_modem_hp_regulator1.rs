#[doc = "Register `HP_MODEM_HP_REGULATOR1` writer"]
pub type W = crate::W<HP_MODEM_HP_REGULATOR1_SPEC>;
#[doc = "Field `HP_MODEM_HP_REGULATOR_DRV_B` writer - need_des"]
pub type HP_MODEM_HP_REGULATOR_DRV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_HP_REGULATOR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 8:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_hp_regulator_drv_b(
        &mut self,
    ) -> HP_MODEM_HP_REGULATOR_DRV_B_W<HP_MODEM_HP_REGULATOR1_SPEC> {
        HP_MODEM_HP_REGULATOR_DRV_B_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_hp_regulator1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_HP_REGULATOR1_SPEC;
impl crate::RegisterSpec for HP_MODEM_HP_REGULATOR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_hp_regulator1::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_HP_REGULATOR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_MODEM_HP_REGULATOR1 to value 0"]
impl crate::Resettable for HP_MODEM_HP_REGULATOR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
