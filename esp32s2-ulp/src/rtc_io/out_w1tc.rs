#[doc = "Register `OUT_W1TC` writer"]
pub type W = crate::W<OUT_W1TC_SPEC>;
#[doc = "Field `OUT_DATA_W1TC` writer - GPIO0 ~ 21 output clear register. If the value 1 is written to a bit here, the corresponding bit in RTCIO_RTC_GPIO_OUT_REG will be cleared. Recommended operation: use this register to clear RTCIO_RTC_GPIO_OUT_REG."]
pub type OUT_DATA_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 10:31 - GPIO0 ~ 21 output clear register. If the value 1 is written to a bit here, the corresponding bit in RTCIO_RTC_GPIO_OUT_REG will be cleared. Recommended operation: use this register to clear RTCIO_RTC_GPIO_OUT_REG."]
    #[inline(always)]
    #[must_use]
    pub fn out_data_w1tc(&mut self) -> OUT_DATA_W1TC_W<OUT_W1TC_SPEC> {
        OUT_DATA_W1TC_W::new(self, 10)
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
#[doc = "RTC GPIO output bit clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_W1TC_SPEC;
impl crate::RegisterSpec for OUT_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_w1tc::W`](W) writer structure"]
impl crate::Writable for OUT_W1TC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_W1TC to value 0"]
impl crate::Resettable for OUT_W1TC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
