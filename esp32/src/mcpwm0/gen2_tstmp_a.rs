#[doc = "Register `GEN2_TSTMP_A` reader"]
pub type R = crate::R<GEN2_TSTMP_A_SPEC>;
#[doc = "Register `GEN2_TSTMP_A` writer"]
pub type W = crate::W<GEN2_TSTMP_A_SPEC>;
#[doc = "Field `GEN2_A` reader - "]
pub type GEN2_A_R = crate::FieldReader<u16>;
#[doc = "Field `GEN2_A` writer - "]
pub type GEN2_A_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gen2_a(&self) -> GEN2_A_R {
        GEN2_A_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN2_TSTMP_A")
            .field("gen2_a", &format_args!("{}", self.gen2_a().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GEN2_TSTMP_A_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn gen2_a(&mut self) -> GEN2_A_W<GEN2_TSTMP_A_SPEC, 0> {
        GEN2_A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen2_tstmp_a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen2_tstmp_a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN2_TSTMP_A_SPEC;
impl crate::RegisterSpec for GEN2_TSTMP_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen2_tstmp_a::R`](R) reader structure"]
impl crate::Readable for GEN2_TSTMP_A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen2_tstmp_a::W`](W) writer structure"]
impl crate::Writable for GEN2_TSTMP_A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GEN2_TSTMP_A to value 0"]
impl crate::Resettable for GEN2_TSTMP_A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
