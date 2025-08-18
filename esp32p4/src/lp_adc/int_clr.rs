#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `COCPU_SARADC1_INT_CLR` writer - ADC1 Conversion is done, int clear."]
pub type COCPU_SARADC1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC2_INT_CLR` writer - ADC2 Conversion is done, int clear."]
pub type COCPU_SARADC2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC1_ERROR_INT_CLR` writer - An errro occurs from ADC1, int clear."]
pub type COCPU_SARADC1_ERROR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC2_ERROR_INT_CLR` writer - An errro occurs from ADC2, int clear."]
pub type COCPU_SARADC2_ERROR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC1_WAKE_INT_CLR` writer - A wakeup event is triggered from ADC1, int clear."]
pub type COCPU_SARADC1_WAKE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC2_WAKE_INT_CLR` writer - A wakeup event is triggered from ADC2, int clear."]
pub type COCPU_SARADC2_WAKE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - ADC1 Conversion is done, int clear."]
    #[inline(always)]
    pub fn cocpu_saradc1_int_clr(&mut self) -> COCPU_SARADC1_INT_CLR_W<'_, INT_CLR_SPEC> {
        COCPU_SARADC1_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC2 Conversion is done, int clear."]
    #[inline(always)]
    pub fn cocpu_saradc2_int_clr(&mut self) -> COCPU_SARADC2_INT_CLR_W<'_, INT_CLR_SPEC> {
        COCPU_SARADC2_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - An errro occurs from ADC1, int clear."]
    #[inline(always)]
    pub fn cocpu_saradc1_error_int_clr(
        &mut self,
    ) -> COCPU_SARADC1_ERROR_INT_CLR_W<'_, INT_CLR_SPEC> {
        COCPU_SARADC1_ERROR_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - An errro occurs from ADC2, int clear."]
    #[inline(always)]
    pub fn cocpu_saradc2_error_int_clr(
        &mut self,
    ) -> COCPU_SARADC2_ERROR_INT_CLR_W<'_, INT_CLR_SPEC> {
        COCPU_SARADC2_ERROR_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - A wakeup event is triggered from ADC1, int clear."]
    #[inline(always)]
    pub fn cocpu_saradc1_wake_int_clr(&mut self) -> COCPU_SARADC1_WAKE_INT_CLR_W<'_, INT_CLR_SPEC> {
        COCPU_SARADC1_WAKE_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - A wakeup event is triggered from ADC2, int clear."]
    #[inline(always)]
    pub fn cocpu_saradc2_wake_int_clr(&mut self) -> COCPU_SARADC2_WAKE_INT_CLR_W<'_, INT_CLR_SPEC> {
        COCPU_SARADC2_WAKE_INT_CLR_W::new(self, 5)
    }
}
#[doc = "Interrupt clear registers.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
