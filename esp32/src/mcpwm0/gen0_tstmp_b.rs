#[doc = "Register `GEN0_TSTMP_B` reader"]
pub type R = crate::R<GEN0_TSTMP_B_SPEC>;
#[doc = "Register `GEN0_TSTMP_B` writer"]
pub type W = crate::W<GEN0_TSTMP_B_SPEC>;
#[doc = "Field `GEN0_B` reader - "]
pub type GEN0_B_R = crate::FieldReader<u16>;
#[doc = "Field `GEN0_B` writer - "]
pub type GEN0_B_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gen0_b(&self) -> GEN0_B_R {
        GEN0_B_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN0_TSTMP_B")
            .field("gen0_b", &format_args!("{}", self.gen0_b().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GEN0_TSTMP_B_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn gen0_b(&mut self) -> GEN0_B_W<GEN0_TSTMP_B_SPEC> {
        GEN0_B_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen0_tstmp_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen0_tstmp_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN0_TSTMP_B_SPEC;
impl crate::RegisterSpec for GEN0_TSTMP_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen0_tstmp_b::R`](R) reader structure"]
impl crate::Readable for GEN0_TSTMP_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen0_tstmp_b::W`](W) writer structure"]
impl crate::Writable for GEN0_TSTMP_B_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEN0_TSTMP_B to value 0"]
impl crate::Resettable for GEN0_TSTMP_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
