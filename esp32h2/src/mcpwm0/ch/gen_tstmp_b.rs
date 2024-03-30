#[doc = "Register `GEN_TSTMP_B` reader"]
pub type R = crate::R<GEN_TSTMP_B_SPEC>;
#[doc = "Register `GEN_TSTMP_B` writer"]
pub type W = crate::W<GEN_TSTMP_B_SPEC>;
#[doc = "Field `CMPR0_B` reader - PWM generator 0 time stamp B's shadow register"]
pub type CMPR0_B_R = crate::FieldReader<u16>;
#[doc = "Field `CMPR0_B` writer - PWM generator 0 time stamp B's shadow register"]
pub type CMPR0_B_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PWM generator 0 time stamp B's shadow register"]
    #[inline(always)]
    pub fn cmpr0_b(&self) -> CMPR0_B_R {
        CMPR0_B_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN_TSTMP_B")
            .field("cmpr0_b", &format_args!("{}", self.cmpr0_b().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GEN_TSTMP_B_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 0 time stamp B's shadow register"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr0_b(&mut self) -> CMPR0_B_W<GEN_TSTMP_B_SPEC> {
        CMPR0_B_W::new(self, 0)
    }
}
#[doc = "Shadow register for register B.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_tstmp_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_tstmp_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN_TSTMP_B_SPEC;
impl crate::RegisterSpec for GEN_TSTMP_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen_tstmp_b::R`](R) reader structure"]
impl crate::Readable for GEN_TSTMP_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen_tstmp_b::W`](W) writer structure"]
impl crate::Writable for GEN_TSTMP_B_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEN_TSTMP_B to value 0"]
impl crate::Resettable for GEN_TSTMP_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
