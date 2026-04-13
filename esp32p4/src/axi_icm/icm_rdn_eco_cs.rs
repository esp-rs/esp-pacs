#[doc = "Register `ICM_RDN_ECO_CS` reader"]
pub type R = crate::R<ICM_RDN_ECO_CS_SPEC>;
#[doc = "Register `ICM_RDN_ECO_CS` writer"]
pub type W = crate::W<ICM_RDN_ECO_CS_SPEC>;
#[doc = "Field `ICM_REG_RDN_ECO_EN` reader - "]
pub type ICM_REG_RDN_ECO_EN_R = crate::BitReader;
#[doc = "Field `ICM_REG_RDN_ECO_EN` writer - "]
pub type ICM_REG_RDN_ECO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICM_REG_RDN_ECO_RESULT` reader - "]
pub type ICM_REG_RDN_ECO_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn icm_reg_rdn_eco_en(&self) -> ICM_REG_RDN_ECO_EN_R {
        ICM_REG_RDN_ECO_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icm_reg_rdn_eco_result(&self) -> ICM_REG_RDN_ECO_RESULT_R {
        ICM_REG_RDN_ECO_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_RDN_ECO_CS")
            .field("icm_reg_rdn_eco_en", &self.icm_reg_rdn_eco_en())
            .field("icm_reg_rdn_eco_result", &self.icm_reg_rdn_eco_result())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn icm_reg_rdn_eco_en(&mut self) -> ICM_REG_RDN_ECO_EN_W<'_, ICM_RDN_ECO_CS_SPEC> {
        ICM_REG_RDN_ECO_EN_W::new(self, 0)
    }
}
#[doc = "RDN ECO control/status\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_rdn_eco_cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_rdn_eco_cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_RDN_ECO_CS_SPEC;
impl crate::RegisterSpec for ICM_RDN_ECO_CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_rdn_eco_cs::R`](R) reader structure"]
impl crate::Readable for ICM_RDN_ECO_CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icm_rdn_eco_cs::W`](W) writer structure"]
impl crate::Writable for ICM_RDN_ECO_CS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_RDN_ECO_CS to value 0"]
impl crate::Resettable for ICM_RDN_ECO_CS_SPEC {}
