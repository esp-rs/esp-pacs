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
    pub fn cocpu_tsens_wake_int_ena_w1ts(
        &mut self,
    ) -> COCPU_TSENS_WAKE_INT_ENA_W1TS_W<'_, INT_ENA_W1TS_SPEC> {
        COCPU_TSENS_WAKE_INT_ENA_W1TS_W::new(self, 0)
    }
}
#[doc = "Tsens wakeup interrupt enable assert.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_W1TS_SPEC;
impl crate::RegisterSpec for INT_ENA_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_ena_w1ts::W`](W) writer structure"]
impl crate::Writable for INT_ENA_W1TS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA_W1TS to value 0"]
impl crate::Resettable for INT_ENA_W1TS_SPEC {}
