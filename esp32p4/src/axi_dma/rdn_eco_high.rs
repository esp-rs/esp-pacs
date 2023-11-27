#[doc = "Register `RDN_ECO_HIGH` reader"]
pub type R = crate::R<RDN_ECO_HIGH_SPEC>;
#[doc = "Register `RDN_ECO_HIGH` writer"]
pub type W = crate::W<RDN_ECO_HIGH_SPEC>;
#[doc = "Field `RDN_ECO_HIGH` reader - The start address of accessible address space."]
pub type RDN_ECO_HIGH_R = crate::FieldReader<u32>;
#[doc = "Field `RDN_ECO_HIGH` writer - The start address of accessible address space."]
pub type RDN_ECO_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The start address of accessible address space."]
    #[inline(always)]
    pub fn rdn_eco_high(&self) -> RDN_ECO_HIGH_R {
        RDN_ECO_HIGH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDN_ECO_HIGH")
            .field(
                "rdn_eco_high",
                &format_args!("{}", self.rdn_eco_high().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RDN_ECO_HIGH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The start address of accessible address space."]
    #[inline(always)]
    #[must_use]
    pub fn rdn_eco_high(&mut self) -> RDN_ECO_HIGH_W<RDN_ECO_HIGH_SPEC> {
        RDN_ECO_HIGH_W::new(self, 0)
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
#[doc = "reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDN_ECO_HIGH_SPEC;
impl crate::RegisterSpec for RDN_ECO_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdn_eco_high::R`](R) reader structure"]
impl crate::Readable for RDN_ECO_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rdn_eco_high::W`](W) writer structure"]
impl crate::Writable for RDN_ECO_HIGH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RDN_ECO_HIGH to value 0xffff_ffff"]
impl crate::Resettable for RDN_ECO_HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
