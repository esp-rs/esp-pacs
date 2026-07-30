#[doc = "Register `GEN1_TSTMP_B` reader"]
pub type R = crate::R<GEN1_TSTMP_B_SPEC>;
#[doc = "Register `GEN1_TSTMP_B` writer"]
pub type W = crate::W<GEN1_TSTMP_B_SPEC>;
#[doc = "Field `CMPR1_B` reader - PWM generator 1 time stamp B's shadow register"]
pub type CMPR1_B_R = crate::FieldReader<u16>;
#[doc = "Field `CMPR1_B` writer - PWM generator 1 time stamp B's shadow register"]
pub type CMPR1_B_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PWM generator 1 time stamp B's shadow register"]
    #[inline(always)]
    pub fn cmpr1_b(&self) -> CMPR1_B_R {
        CMPR1_B_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN1_TSTMP_B")
            .field("cmpr1_b", &self.cmpr1_b())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 1 time stamp B's shadow register"]
    #[inline(always)]
    pub fn cmpr1_b(&mut self) -> CMPR1_B_W<'_, GEN1_TSTMP_B_SPEC> {
        CMPR1_B_W::new(self, 0)
    }
}
#[doc = "Shadow register for register B.\n\nYou can [`read`](crate::Reg::read) this register and get [`gen1_tstmp_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen1_tstmp_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN1_TSTMP_B_SPEC;
impl crate::RegisterSpec for GEN1_TSTMP_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen1_tstmp_b::R`](R) reader structure"]
impl crate::Readable for GEN1_TSTMP_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen1_tstmp_b::W`](W) writer structure"]
impl crate::Writable for GEN1_TSTMP_B_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GEN1_TSTMP_B to value 0"]
impl crate::Resettable for GEN1_TSTMP_B_SPEC {}
