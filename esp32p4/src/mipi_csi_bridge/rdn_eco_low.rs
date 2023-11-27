#[doc = "Register `RDN_ECO_LOW` reader"]
pub type R = crate::R<RDN_ECO_LOW_SPEC>;
#[doc = "Register `RDN_ECO_LOW` writer"]
pub type W = crate::W<RDN_ECO_LOW_SPEC>;
#[doc = "Field `RDN_ECO_LOW` reader - N/A"]
pub type RDN_ECO_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `RDN_ECO_LOW` writer - N/A"]
pub type RDN_ECO_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn rdn_eco_low(&self) -> RDN_ECO_LOW_R {
        RDN_ECO_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDN_ECO_LOW")
            .field(
                "rdn_eco_low",
                &format_args!("{}", self.rdn_eco_low().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RDN_ECO_LOW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rdn_eco_low(&mut self) -> RDN_ECO_LOW_W<RDN_ECO_LOW_SPEC> {
        RDN_ECO_LOW_W::new(self, 0)
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
#[doc = "N/A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDN_ECO_LOW_SPEC;
impl crate::RegisterSpec for RDN_ECO_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdn_eco_low::R`](R) reader structure"]
impl crate::Readable for RDN_ECO_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rdn_eco_low::W`](W) writer structure"]
impl crate::Writable for RDN_ECO_LOW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RDN_ECO_LOW to value 0"]
impl crate::Resettable for RDN_ECO_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
