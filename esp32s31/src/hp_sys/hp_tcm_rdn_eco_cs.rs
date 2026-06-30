#[doc = "Register `HP_TCM_RDN_ECO_CS` reader"]
pub type R = crate::R<HP_TCM_RDN_ECO_CS_SPEC>;
#[doc = "Register `HP_TCM_RDN_ECO_CS` writer"]
pub type W = crate::W<HP_TCM_RDN_ECO_CS_SPEC>;
#[doc = "Field `HP_REG_HP_TCM_RDN_ECO_EN` reader - NA"]
pub type HP_REG_HP_TCM_RDN_ECO_EN_R = crate::BitReader;
#[doc = "Field `HP_REG_HP_TCM_RDN_ECO_EN` writer - NA"]
pub type HP_REG_HP_TCM_RDN_ECO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_REG_HP_TCM_RDN_ECO_RESULT` reader - NA"]
pub type HP_REG_HP_TCM_RDN_ECO_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn hp_reg_hp_tcm_rdn_eco_en(&self) -> HP_REG_HP_TCM_RDN_ECO_EN_R {
        HP_REG_HP_TCM_RDN_ECO_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn hp_reg_hp_tcm_rdn_eco_result(&self) -> HP_REG_HP_TCM_RDN_ECO_RESULT_R {
        HP_REG_HP_TCM_RDN_ECO_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_TCM_RDN_ECO_CS")
            .field("hp_reg_hp_tcm_rdn_eco_en", &self.hp_reg_hp_tcm_rdn_eco_en())
            .field(
                "hp_reg_hp_tcm_rdn_eco_result",
                &self.hp_reg_hp_tcm_rdn_eco_result(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn hp_reg_hp_tcm_rdn_eco_en(
        &mut self,
    ) -> HP_REG_HP_TCM_RDN_ECO_EN_W<'_, HP_TCM_RDN_ECO_CS_SPEC> {
        HP_REG_HP_TCM_RDN_ECO_EN_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_tcm_rdn_eco_cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_tcm_rdn_eco_cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_TCM_RDN_ECO_CS_SPEC;
impl crate::RegisterSpec for HP_TCM_RDN_ECO_CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_tcm_rdn_eco_cs::R`](R) reader structure"]
impl crate::Readable for HP_TCM_RDN_ECO_CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_tcm_rdn_eco_cs::W`](W) writer structure"]
impl crate::Writable for HP_TCM_RDN_ECO_CS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_TCM_RDN_ECO_CS to value 0"]
impl crate::Resettable for HP_TCM_RDN_ECO_CS_SPEC {}
