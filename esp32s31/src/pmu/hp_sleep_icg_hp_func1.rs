#[doc = "Register `HP_SLEEP_ICG_HP_FUNC1` reader"]
pub type R = crate::R<HP_SLEEP_ICG_HP_FUNC1_SPEC>;
#[doc = "Register `HP_SLEEP_ICG_HP_FUNC1` writer"]
pub type W = crate::W<HP_SLEEP_ICG_HP_FUNC1_SPEC>;
#[doc = "Field `HP_SLEEP_DIG_ICG_FUNC1_EN` reader - need_des"]
pub type HP_SLEEP_DIG_ICG_FUNC1_EN_R = crate::FieldReader<u32>;
#[doc = "Field `HP_SLEEP_DIG_ICG_FUNC1_EN` writer - need_des"]
pub type HP_SLEEP_DIG_ICG_FUNC1_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dig_icg_func1_en(&self) -> HP_SLEEP_DIG_ICG_FUNC1_EN_R {
        HP_SLEEP_DIG_ICG_FUNC1_EN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_SLEEP_ICG_HP_FUNC1")
            .field(
                "hp_sleep_dig_icg_func1_en",
                &self.hp_sleep_dig_icg_func1_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dig_icg_func1_en(
        &mut self,
    ) -> HP_SLEEP_DIG_ICG_FUNC1_EN_W<'_, HP_SLEEP_ICG_HP_FUNC1_SPEC> {
        HP_SLEEP_DIG_ICG_FUNC1_EN_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_icg_hp_func1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_icg_hp_func1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_SLEEP_ICG_HP_FUNC1_SPEC;
impl crate::RegisterSpec for HP_SLEEP_ICG_HP_FUNC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_sleep_icg_hp_func1::R`](R) reader structure"]
impl crate::Readable for HP_SLEEP_ICG_HP_FUNC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_sleep_icg_hp_func1::W`](W) writer structure"]
impl crate::Writable for HP_SLEEP_ICG_HP_FUNC1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_SLEEP_ICG_HP_FUNC1 to value 0xffff_ffff"]
impl crate::Resettable for HP_SLEEP_ICG_HP_FUNC1_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
