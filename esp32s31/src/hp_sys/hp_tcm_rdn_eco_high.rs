#[doc = "Register `HP_TCM_RDN_ECO_HIGH` reader"]
pub type R = crate::R<HP_TCM_RDN_ECO_HIGH_SPEC>;
#[doc = "Register `HP_TCM_RDN_ECO_HIGH` writer"]
pub type W = crate::W<HP_TCM_RDN_ECO_HIGH_SPEC>;
#[doc = "Field `HP_REG_HP_TCM_RDN_ECO_HIGH` reader - NA"]
pub type HP_REG_HP_TCM_RDN_ECO_HIGH_R = crate::FieldReader<u32>;
#[doc = "Field `HP_REG_HP_TCM_RDN_ECO_HIGH` writer - NA"]
pub type HP_REG_HP_TCM_RDN_ECO_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn hp_reg_hp_tcm_rdn_eco_high(&self) -> HP_REG_HP_TCM_RDN_ECO_HIGH_R {
        HP_REG_HP_TCM_RDN_ECO_HIGH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_TCM_RDN_ECO_HIGH")
            .field(
                "hp_reg_hp_tcm_rdn_eco_high",
                &self.hp_reg_hp_tcm_rdn_eco_high(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn hp_reg_hp_tcm_rdn_eco_high(
        &mut self,
    ) -> HP_REG_HP_TCM_RDN_ECO_HIGH_W<'_, HP_TCM_RDN_ECO_HIGH_SPEC> {
        HP_REG_HP_TCM_RDN_ECO_HIGH_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_tcm_rdn_eco_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_tcm_rdn_eco_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_TCM_RDN_ECO_HIGH_SPEC;
impl crate::RegisterSpec for HP_TCM_RDN_ECO_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_tcm_rdn_eco_high::R`](R) reader structure"]
impl crate::Readable for HP_TCM_RDN_ECO_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_tcm_rdn_eco_high::W`](W) writer structure"]
impl crate::Writable for HP_TCM_RDN_ECO_HIGH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_TCM_RDN_ECO_HIGH to value 0xffff_ffff"]
impl crate::Resettable for HP_TCM_RDN_ECO_HIGH_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
