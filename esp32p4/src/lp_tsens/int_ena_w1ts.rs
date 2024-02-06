#[doc = "Register `INT_ENA_W1TS` writer"]
pub type W = crate::W<INT_ENA_W1TS_SPEC>;
#[doc = "Field `COCPU_TSENS_WAKE_INT_ENA_W1TS` writer - Write 1 to this field to assert interrupt enable."]
pub type COCPU_TSENS_WAKE_INT_ENA_W1TS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to this field to assert interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_tsens_wake_int_ena_w1ts(
        &mut self,
    ) -> COCPU_TSENS_WAKE_INT_ENA_W1TS_W<INT_ENA_W1TS_SPEC> {
        COCPU_TSENS_WAKE_INT_ENA_W1TS_W::new(self, 0)
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
#[doc = "Tsens wakeup interrupt enable assert.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_W1TS_SPEC;
impl crate::RegisterSpec for INT_ENA_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_ena_w1ts::W`](W) writer structure"]
impl crate::Writable for INT_ENA_W1TS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA_W1TS to value 0"]
impl crate::Resettable for INT_ENA_W1TS_SPEC {
    const RESET_VALUE: u32 = 0;
}
