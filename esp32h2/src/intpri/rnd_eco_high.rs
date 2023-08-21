#[doc = "Register `RND_ECO_HIGH` reader"]
pub type R = crate::R<RND_ECO_HIGH_SPEC>;
#[doc = "Register `RND_ECO_HIGH` writer"]
pub type W = crate::W<RND_ECO_HIGH_SPEC>;
#[doc = "Field `REDCY_HIGH` reader - Only reserved for ECO."]
pub type REDCY_HIGH_R = crate::FieldReader<u32>;
#[doc = "Field `REDCY_HIGH` writer - Only reserved for ECO."]
pub type REDCY_HIGH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Only reserved for ECO."]
    #[inline(always)]
    pub fn redcy_high(&self) -> REDCY_HIGH_R {
        REDCY_HIGH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RND_ECO_HIGH")
            .field("redcy_high", &format_args!("{}", self.redcy_high().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RND_ECO_HIGH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Only reserved for ECO."]
    #[inline(always)]
    #[must_use]
    pub fn redcy_high(&mut self) -> REDCY_HIGH_W<RND_ECO_HIGH_SPEC, 0> {
        REDCY_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "redcy eco high register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rnd_eco_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rnd_eco_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RND_ECO_HIGH_SPEC;
impl crate::RegisterSpec for RND_ECO_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rnd_eco_high::R`](R) reader structure"]
impl crate::Readable for RND_ECO_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rnd_eco_high::W`](W) writer structure"]
impl crate::Writable for RND_ECO_HIGH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RND_ECO_HIGH to value 0xffff_ffff"]
impl crate::Resettable for RND_ECO_HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
