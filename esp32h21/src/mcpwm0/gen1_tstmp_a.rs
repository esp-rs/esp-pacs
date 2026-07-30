#[doc = "Register `GEN1_TSTMP_A` reader"]
pub type R = crate::R<GEN1_TSTMP_A_SPEC>;
#[doc = "Register `GEN1_TSTMP_A` writer"]
pub type W = crate::W<GEN1_TSTMP_A_SPEC>;
#[doc = "Field `CMPR1_A` reader - PWM generator 1 time stamp A's shadow register"]
pub type CMPR1_A_R = crate::FieldReader<u16>;
#[doc = "Field `CMPR1_A` writer - PWM generator 1 time stamp A's shadow register"]
pub type CMPR1_A_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PWM generator 1 time stamp A's shadow register"]
    #[inline(always)]
    pub fn cmpr1_a(&self) -> CMPR1_A_R {
        CMPR1_A_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN1_TSTMP_A")
            .field("cmpr1_a", &self.cmpr1_a())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 1 time stamp A's shadow register"]
    #[inline(always)]
    pub fn cmpr1_a(&mut self) -> CMPR1_A_W<'_, GEN1_TSTMP_A_SPEC> {
        CMPR1_A_W::new(self, 0)
    }
}
#[doc = "Shadow register for register A.\n\nYou can [`read`](crate::Reg::read) this register and get [`gen1_tstmp_a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen1_tstmp_a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN1_TSTMP_A_SPEC;
impl crate::RegisterSpec for GEN1_TSTMP_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen1_tstmp_a::R`](R) reader structure"]
impl crate::Readable for GEN1_TSTMP_A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen1_tstmp_a::W`](W) writer structure"]
impl crate::Writable for GEN1_TSTMP_A_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GEN1_TSTMP_A to value 0"]
impl crate::Resettable for GEN1_TSTMP_A_SPEC {}
