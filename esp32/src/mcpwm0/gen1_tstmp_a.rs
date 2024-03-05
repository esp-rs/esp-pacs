#[doc = "Register `GEN1_TSTMP_A` reader"]
pub type R = crate::R<GEN1_TSTMP_A_SPEC>;
#[doc = "Register `GEN1_TSTMP_A` writer"]
pub type W = crate::W<GEN1_TSTMP_A_SPEC>;
#[doc = "Field `GEN1_A` reader - "]
pub type GEN1_A_R = crate::FieldReader<u16>;
#[doc = "Field `GEN1_A` writer - "]
pub type GEN1_A_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gen1_a(&self) -> GEN1_A_R {
        GEN1_A_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN1_TSTMP_A")
            .field("gen1_a", &format_args!("{}", self.gen1_a().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GEN1_TSTMP_A_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn gen1_a(&mut self) -> GEN1_A_W<GEN1_TSTMP_A_SPEC> {
        GEN1_A_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen1_tstmp_a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen1_tstmp_a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN1_TSTMP_A_SPEC;
impl crate::RegisterSpec for GEN1_TSTMP_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen1_tstmp_a::R`](R) reader structure"]
impl crate::Readable for GEN1_TSTMP_A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen1_tstmp_a::W`](W) writer structure"]
impl crate::Writable for GEN1_TSTMP_A_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEN1_TSTMP_A to value 0"]
impl crate::Resettable for GEN1_TSTMP_A_SPEC {
    const RESET_VALUE: u32 = 0;
}
