#[doc = "Register `ECO_LOW` reader"]
pub type R = crate::R<ECO_LOW_SPEC>;
#[doc = "Register `ECO_LOW` writer"]
pub type W = crate::W<ECO_LOW_SPEC>;
#[doc = "Field `RDN_ECO_LOW` reader - redundant registers for jpeg"]
pub type RDN_ECO_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `RDN_ECO_LOW` writer - redundant registers for jpeg"]
pub type RDN_ECO_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - redundant registers for jpeg"]
    #[inline(always)]
    pub fn rdn_eco_low(&self) -> RDN_ECO_LOW_R {
        RDN_ECO_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECO_LOW")
            .field("rdn_eco_low", &self.rdn_eco_low().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ECO_LOW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - redundant registers for jpeg"]
    #[inline(always)]
    #[must_use]
    pub fn rdn_eco_low(&mut self) -> RDN_ECO_LOW_W<ECO_LOW_SPEC> {
        RDN_ECO_LOW_W::new(self, 0)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eco_low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eco_low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECO_LOW_SPEC;
impl crate::RegisterSpec for ECO_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_low::R`](R) reader structure"]
impl crate::Readable for ECO_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eco_low::W`](W) writer structure"]
impl crate::Writable for ECO_LOW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECO_LOW to value 0"]
impl crate::Resettable for ECO_LOW_SPEC {
    const RESET_VALUE: u32 = 0;
}
