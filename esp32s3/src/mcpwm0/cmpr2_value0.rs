#[doc = "Register `CMPR2_VALUE0` reader"]
pub type R = crate::R<CMPR2_VALUE0_SPEC>;
#[doc = "Register `CMPR2_VALUE0` writer"]
pub type W = crate::W<CMPR2_VALUE0_SPEC>;
#[doc = "Field `CMPR2_A` reader - PWM generator 2 time stamp A's shadow register"]
pub type CMPR2_A_R = crate::FieldReader<u16>;
#[doc = "Field `CMPR2_A` writer - PWM generator 2 time stamp A's shadow register"]
pub type CMPR2_A_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PWM generator 2 time stamp A's shadow register"]
    #[inline(always)]
    pub fn cmpr2_a(&self) -> CMPR2_A_R {
        CMPR2_A_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPR2_VALUE0")
            .field("cmpr2_a", &format_args!("{}", self.cmpr2_a().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMPR2_VALUE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 2 time stamp A's shadow register"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr2_a(&mut self) -> CMPR2_A_W<CMPR2_VALUE0_SPEC, 0> {
        CMPR2_A_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Shadow register for register A.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpr2_value0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpr2_value0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPR2_VALUE0_SPEC;
impl crate::RegisterSpec for CMPR2_VALUE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpr2_value0::R`](R) reader structure"]
impl crate::Readable for CMPR2_VALUE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpr2_value0::W`](W) writer structure"]
impl crate::Writable for CMPR2_VALUE0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPR2_VALUE0 to value 0"]
impl crate::Resettable for CMPR2_VALUE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
