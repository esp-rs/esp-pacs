#[doc = "Register `GEN1_TSTMP_A` reader"]
pub type R = crate::R<GEN1_TSTMP_A_SPEC>;
#[doc = "Register `GEN1_TSTMP_A` writer"]
pub type W = crate::W<GEN1_TSTMP_A_SPEC>;
#[doc = "Field `CMPR1_A` reader - PWM generator 1 time stamp A's shadow register"]
pub type CMPR1_A_R = crate::FieldReader<u16>;
#[doc = "Field `CMPR1_A` writer - PWM generator 1 time stamp A's shadow register"]
pub type CMPR1_A_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
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
            .field("cmpr1_a", &format_args!("{}", self.cmpr1_a().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GEN1_TSTMP_A_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 1 time stamp A's shadow register"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr1_a(&mut self) -> CMPR1_A_W<GEN1_TSTMP_A_SPEC, 0> {
        CMPR1_A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Shadow register for register A.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen1_tstmp_a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen1_tstmp_a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN1_TSTMP_A_SPEC;
impl crate::RegisterSpec for GEN1_TSTMP_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen1_tstmp_a::R`](R) reader structure"]
impl crate::Readable for GEN1_TSTMP_A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen1_tstmp_a::W`](W) writer structure"]
impl crate::Writable for GEN1_TSTMP_A_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GEN1_TSTMP_A to value 0"]
impl crate::Resettable for GEN1_TSTMP_A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
