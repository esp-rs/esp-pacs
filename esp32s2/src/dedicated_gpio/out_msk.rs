#[doc = "Register `OUT_MSK` writer"]
pub type W = crate::W<OUT_MSK_SPEC>;
#[doc = "Field `OUT_VALUE` writer - This register is used to configure updated output value of 8-channel dedicated GPIO."]
pub type OUT_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OUT_MSK` writer - This register is used to configure channels which would be updated. 1: corresponding channel's output would be updated."]
pub type OUT_MSK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_MSK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure updated output value of 8-channel dedicated GPIO."]
    #[inline(always)]
    #[must_use]
    pub fn out_value(&mut self) -> OUT_VALUE_W<OUT_MSK_SPEC> {
        OUT_VALUE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - This register is used to configure channels which would be updated. 1: corresponding channel's output would be updated."]
    #[inline(always)]
    #[must_use]
    pub fn out_msk(&mut self) -> OUT_MSK_W<OUT_MSK_SPEC> {
        OUT_MSK_W::new(self, 8)
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
#[doc = "Dedicated GPIO mask output register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_msk::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_MSK_SPEC;
impl crate::RegisterSpec for OUT_MSK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_msk::W`](W) writer structure"]
impl crate::Writable for OUT_MSK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_MSK to value 0"]
impl crate::Resettable for OUT_MSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
