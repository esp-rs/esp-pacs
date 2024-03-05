#[doc = "Register `ECO_HIGH` reader"]
pub type R = crate::R<ECO_HIGH_SPEC>;
#[doc = "Register `ECO_HIGH` writer"]
pub type W = crate::W<ECO_HIGH_SPEC>;
#[doc = "Field `RDN_ECO_HIGH` reader - logic high eco registers"]
pub type RDN_ECO_HIGH_R = crate::FieldReader<u32>;
#[doc = "Field `RDN_ECO_HIGH` writer - logic high eco registers"]
pub type RDN_ECO_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - logic high eco registers"]
    #[inline(always)]
    pub fn rdn_eco_high(&self) -> RDN_ECO_HIGH_R {
        RDN_ECO_HIGH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECO_HIGH")
            .field(
                "rdn_eco_high",
                &format_args!("{}", self.rdn_eco_high().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ECO_HIGH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - logic high eco registers"]
    #[inline(always)]
    #[must_use]
    pub fn rdn_eco_high(&mut self) -> RDN_ECO_HIGH_W<ECO_HIGH_SPEC> {
        RDN_ECO_HIGH_W::new(self, 0)
    }
}
#[doc = "I2S ECO register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eco_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eco_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECO_HIGH_SPEC;
impl crate::RegisterSpec for ECO_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_high::R`](R) reader structure"]
impl crate::Readable for ECO_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eco_high::W`](W) writer structure"]
impl crate::Writable for ECO_HIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECO_HIGH to value 0xffff_ffff"]
impl crate::Resettable for ECO_HIGH_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
