#[doc = "Register `CMPR1_VALUE1` reader"]
pub type R = crate::R<CMPR1_VALUE1_SPEC>;
#[doc = "Register `CMPR1_VALUE1` writer"]
pub type W = crate::W<CMPR1_VALUE1_SPEC>;
#[doc = "Field `CMPR1_B` reader - PWM generator 1 time stamp B's shadow register"]
pub type CMPR1_B_R = crate::FieldReader<u16>;
#[doc = "Field `CMPR1_B` writer - PWM generator 1 time stamp B's shadow register"]
pub type CMPR1_B_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
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
        f.debug_struct("CMPR1_VALUE1")
            .field("cmpr1_b", &format_args!("{}", self.cmpr1_b().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMPR1_VALUE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 1 time stamp B's shadow register"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr1_b(&mut self) -> CMPR1_B_W<CMPR1_VALUE1_SPEC, 0> {
        CMPR1_B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Shadow register for register B.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpr1_value1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpr1_value1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPR1_VALUE1_SPEC;
impl crate::RegisterSpec for CMPR1_VALUE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpr1_value1::R`](R) reader structure"]
impl crate::Readable for CMPR1_VALUE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpr1_value1::W`](W) writer structure"]
impl crate::Writable for CMPR1_VALUE1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPR1_VALUE1 to value 0"]
impl crate::Resettable for CMPR1_VALUE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
