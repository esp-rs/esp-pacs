#[doc = "Register `GEN0_TSTMP_A` reader"]
pub type R = crate::R<GEN0_TSTMP_A_SPEC>;
#[doc = "Register `GEN0_TSTMP_A` writer"]
pub type W = crate::W<GEN0_TSTMP_A_SPEC>;
#[doc = "Field `CMPR0_A` reader - PWM generator 0 time stamp A's shadow register"]
pub type CMPR0_A_R = crate::FieldReader<u16>;
#[doc = "Field `CMPR0_A` writer - PWM generator 0 time stamp A's shadow register"]
pub type CMPR0_A_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PWM generator 0 time stamp A's shadow register"]
    #[inline(always)]
    pub fn cmpr0_a(&self) -> CMPR0_A_R {
        CMPR0_A_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN0_TSTMP_A")
            .field("cmpr0_a", &self.cmpr0_a())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 0 time stamp A's shadow register"]
    #[inline(always)]
    pub fn cmpr0_a(&mut self) -> CMPR0_A_W<'_, GEN0_TSTMP_A_SPEC> {
        CMPR0_A_W::new(self, 0)
    }
}
#[doc = "Shadow register for register A.\n\nYou can [`read`](crate::Reg::read) this register and get [`gen0_tstmp_a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen0_tstmp_a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN0_TSTMP_A_SPEC;
impl crate::RegisterSpec for GEN0_TSTMP_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen0_tstmp_a::R`](R) reader structure"]
impl crate::Readable for GEN0_TSTMP_A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen0_tstmp_a::W`](W) writer structure"]
impl crate::Writable for GEN0_TSTMP_A_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GEN0_TSTMP_A to value 0"]
impl crate::Resettable for GEN0_TSTMP_A_SPEC {}
