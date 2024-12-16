#[doc = "Register `RDN_ECO_LOW` reader"]
pub type R = crate::R<RDN_ECO_LOW_SPEC>;
#[doc = "Register `RDN_ECO_LOW` writer"]
pub type W = crate::W<RDN_ECO_LOW_SPEC>;
#[doc = "Field `RDN_ECO_LOW` reader - The start address of accessible address space."]
pub type RDN_ECO_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `RDN_ECO_LOW` writer - The start address of accessible address space."]
pub type RDN_ECO_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The start address of accessible address space."]
    #[inline(always)]
    pub fn rdn_eco_low(&self) -> RDN_ECO_LOW_R {
        RDN_ECO_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDN_ECO_LOW")
            .field("rdn_eco_low", &self.rdn_eco_low())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The start address of accessible address space."]
    #[inline(always)]
    pub fn rdn_eco_low(&mut self) -> RDN_ECO_LOW_W<RDN_ECO_LOW_SPEC> {
        RDN_ECO_LOW_W::new(self, 0)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_eco_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_eco_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDN_ECO_LOW_SPEC;
impl crate::RegisterSpec for RDN_ECO_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdn_eco_low::R`](R) reader structure"]
impl crate::Readable for RDN_ECO_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rdn_eco_low::W`](W) writer structure"]
impl crate::Writable for RDN_ECO_LOW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RDN_ECO_LOW to value 0"]
impl crate::Resettable for RDN_ECO_LOW_SPEC {
    const RESET_VALUE: u32 = 0;
}
