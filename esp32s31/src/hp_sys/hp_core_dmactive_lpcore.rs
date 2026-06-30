#[doc = "Register `HP_CORE_DMACTIVE_LPCORE` reader"]
pub type R = crate::R<HP_CORE_DMACTIVE_LPCORE_SPEC>;
#[doc = "Field `HP_CORE_DMACTIVE_LPCORE` reader - hp core dmactive_lpcore value"]
pub type HP_CORE_DMACTIVE_LPCORE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - hp core dmactive_lpcore value"]
    #[inline(always)]
    pub fn hp_core_dmactive_lpcore(&self) -> HP_CORE_DMACTIVE_LPCORE_R {
        HP_CORE_DMACTIVE_LPCORE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CORE_DMACTIVE_LPCORE")
            .field("hp_core_dmactive_lpcore", &self.hp_core_dmactive_lpcore())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_core_dmactive_lpcore::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CORE_DMACTIVE_LPCORE_SPEC;
impl crate::RegisterSpec for HP_CORE_DMACTIVE_LPCORE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_core_dmactive_lpcore::R`](R) reader structure"]
impl crate::Readable for HP_CORE_DMACTIVE_LPCORE_SPEC {}
#[doc = "`reset()` method sets HP_CORE_DMACTIVE_LPCORE to value 0"]
impl crate::Resettable for HP_CORE_DMACTIVE_LPCORE_SPEC {}
