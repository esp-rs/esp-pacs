#[doc = "Register `HP_MODEM_ICG_MODEM` reader"]
pub type R = crate::R<HP_MODEM_ICG_MODEM_SPEC>;
#[doc = "Register `HP_MODEM_ICG_MODEM` writer"]
pub type W = crate::W<HP_MODEM_ICG_MODEM_SPEC>;
#[doc = "Field `HP_MODEM_DIG_ICG_MODEM_CODE` reader - need_des"]
pub type HP_MODEM_DIG_ICG_MODEM_CODE_R = crate::FieldReader;
#[doc = "Field `HP_MODEM_DIG_ICG_MODEM_CODE` writer - need_des"]
pub type HP_MODEM_DIG_ICG_MODEM_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dig_icg_modem_code(&self) -> HP_MODEM_DIG_ICG_MODEM_CODE_R {
        HP_MODEM_DIG_ICG_MODEM_CODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_MODEM_ICG_MODEM")
            .field(
                "hp_modem_dig_icg_modem_code",
                &format_args!("{}", self.hp_modem_dig_icg_modem_code().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_ICG_MODEM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_dig_icg_modem_code(
        &mut self,
    ) -> HP_MODEM_DIG_ICG_MODEM_CODE_W<HP_MODEM_ICG_MODEM_SPEC> {
        HP_MODEM_DIG_ICG_MODEM_CODE_W::new(self, 30)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_modem_icg_modem::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_icg_modem::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_ICG_MODEM_SPEC;
impl crate::RegisterSpec for HP_MODEM_ICG_MODEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_modem_icg_modem::R`](R) reader structure"]
impl crate::Readable for HP_MODEM_ICG_MODEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_modem_icg_modem::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_ICG_MODEM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HP_MODEM_ICG_MODEM to value 0"]
impl crate::Resettable for HP_MODEM_ICG_MODEM_SPEC {
    const RESET_VALUE: u32 = 0;
}
