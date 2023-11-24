#[doc = "Register `CMPR0_VALUE1` reader"]
pub type R = crate::R<CMPR0_VALUE1_SPEC>;
#[doc = "Register `CMPR0_VALUE1` writer"]
pub type W = crate::W<CMPR0_VALUE1_SPEC>;
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
        f.debug_struct("CMPR0_VALUE1")
            .field("cmpr0_b", &format_args!("{}", self.cmpr0_b().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMPR0_VALUE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM generator 0 time stamp B's shadow register"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr0_b(&mut self) -> CMPR0_B_W<CMPR0_VALUE1_SPEC> {
        CMPR0_B_W::new(self, 0)
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
#[doc = "Shadow register for register B.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpr0_value1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpr0_value1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPR0_VALUE1_SPEC;
impl crate::RegisterSpec for CMPR0_VALUE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpr0_value1::R`](R) reader structure"]
impl crate::Readable for CMPR0_VALUE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpr0_value1::W`](W) writer structure"]
impl crate::Writable for CMPR0_VALUE1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPR0_VALUE1 to value 0"]
impl crate::Resettable for CMPR0_VALUE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
