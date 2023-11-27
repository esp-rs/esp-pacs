#[doc = "Register `HP_SLEEP_HP_REGULATOR1` reader"]
pub type R = crate::R<HP_SLEEP_HP_REGULATOR1_SPEC>;
#[doc = "Register `HP_SLEEP_HP_REGULATOR1` writer"]
pub type W = crate::W<HP_SLEEP_HP_REGULATOR1_SPEC>;
#[doc = "Field `HP_SLEEP_HP_REGULATOR_DRV_B` reader - need_des"]
pub type HP_SLEEP_HP_REGULATOR_DRV_B_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP_HP_REGULATOR_DRV_B` writer - need_des"]
pub type HP_SLEEP_HP_REGULATOR_DRV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 26:31 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_hp_regulator_drv_b(&self) -> HP_SLEEP_HP_REGULATOR_DRV_B_R {
        HP_SLEEP_HP_REGULATOR_DRV_B_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_SLEEP_HP_REGULATOR1")
            .field(
                "hp_sleep_hp_regulator_drv_b",
                &format_args!("{}", self.hp_sleep_hp_regulator_drv_b().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_SLEEP_HP_REGULATOR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 26:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_hp_regulator_drv_b(
        &mut self,
    ) -> HP_SLEEP_HP_REGULATOR_DRV_B_W<HP_SLEEP_HP_REGULATOR1_SPEC> {
        HP_SLEEP_HP_REGULATOR_DRV_B_W::new(self, 26)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_hp_regulator1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_hp_regulator1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_SLEEP_HP_REGULATOR1_SPEC;
impl crate::RegisterSpec for HP_SLEEP_HP_REGULATOR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_sleep_hp_regulator1::R`](R) reader structure"]
impl crate::Readable for HP_SLEEP_HP_REGULATOR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_sleep_hp_regulator1::W`](W) writer structure"]
impl crate::Writable for HP_SLEEP_HP_REGULATOR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_SLEEP_HP_REGULATOR1 to value 0"]
impl crate::Resettable for HP_SLEEP_HP_REGULATOR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
