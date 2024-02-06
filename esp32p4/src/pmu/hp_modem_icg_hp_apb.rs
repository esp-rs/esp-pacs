#[doc = "Register `HP_MODEM_ICG_HP_APB` writer"]
pub type W = crate::W<HP_MODEM_ICG_HP_APB_SPEC>;
#[doc = "Field `HP_MODEM_DIG_ICG_APB_EN` writer - need_des"]
pub type HP_MODEM_DIG_ICG_APB_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_ICG_HP_APB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_dig_icg_apb_en(
        &mut self,
    ) -> HP_MODEM_DIG_ICG_APB_EN_W<HP_MODEM_ICG_HP_APB_SPEC> {
        HP_MODEM_DIG_ICG_APB_EN_W::new(self, 0)
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
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_icg_hp_apb::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_ICG_HP_APB_SPEC;
impl crate::RegisterSpec for HP_MODEM_ICG_HP_APB_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_icg_hp_apb::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_ICG_HP_APB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HP_MODEM_ICG_HP_APB to value 0xffff_ffff"]
impl crate::Resettable for HP_MODEM_ICG_HP_APB_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
