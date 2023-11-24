#[doc = "Register `HSCH%s_DUTY` reader"]
pub type R = crate::R<HSCH_DUTY_SPEC>;
#[doc = "Register `HSCH%s_DUTY` writer"]
pub type W = crate::W<HSCH_DUTY_SPEC>;
#[doc = "Field `DUTY` reader - This register represents the current duty of the output signal for high speed channel0."]
pub type DUTY_R = crate::FieldReader<u32>;
#[doc = "Field `DUTY` writer - This register represents the current duty of the output signal for high speed channel0."]
pub type DUTY_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - This register represents the current duty of the output signal for high speed channel0."]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSCH_DUTY")
            .field("duty", &format_args!("{}", self.duty().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HSCH_DUTY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:24 - This register represents the current duty of the output signal for high speed channel0."]
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DUTY_W<HSCH_DUTY_SPEC> {
        DUTY_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsch_duty::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsch_duty::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSCH_DUTY_SPEC;
impl crate::RegisterSpec for HSCH_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsch_duty::R`](R) reader structure"]
impl crate::Readable for HSCH_DUTY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hsch_duty::W`](W) writer structure"]
impl crate::Writable for HSCH_DUTY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSCH%s_DUTY to value 0"]
impl crate::Resettable for HSCH_DUTY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
