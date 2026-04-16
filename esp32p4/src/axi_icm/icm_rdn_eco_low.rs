#[doc = "Register `ICM_RDN_ECO_LOW` reader"]
pub type R = crate::R<ICM_RDN_ECO_LOW_SPEC>;
#[doc = "Register `ICM_RDN_ECO_LOW` writer"]
pub type W = crate::W<ICM_RDN_ECO_LOW_SPEC>;
#[doc = "Field `RDN_ECO_LOW` reader - "]
pub type RDN_ECO_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `RDN_ECO_LOW` writer - "]
pub type RDN_ECO_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rdn_eco_low(&self) -> RDN_ECO_LOW_R {
        RDN_ECO_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_RDN_ECO_LOW")
            .field("rdn_eco_low", &self.rdn_eco_low())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rdn_eco_low(&mut self) -> RDN_ECO_LOW_W<'_, ICM_RDN_ECO_LOW_SPEC> {
        RDN_ECO_LOW_W::new(self, 0)
    }
}
#[doc = "RDN ECO low\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_rdn_eco_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_rdn_eco_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_RDN_ECO_LOW_SPEC;
impl crate::RegisterSpec for ICM_RDN_ECO_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_rdn_eco_low::R`](R) reader structure"]
impl crate::Readable for ICM_RDN_ECO_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icm_rdn_eco_low::W`](W) writer structure"]
impl crate::Writable for ICM_RDN_ECO_LOW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_RDN_ECO_LOW to value 0"]
impl crate::Resettable for ICM_RDN_ECO_LOW_SPEC {}
