#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `COCPU_SARADC1_INT_ENA` reader - ADC1 Conversion is done, int enable."]
pub type COCPU_SARADC1_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC1_INT_ENA` writer - ADC1 Conversion is done, int enable."]
pub type COCPU_SARADC1_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC2_INT_ENA` reader - ADC2 Conversion is done, int enable."]
pub type COCPU_SARADC2_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC2_INT_ENA` writer - ADC2 Conversion is done, int enable."]
pub type COCPU_SARADC2_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC1_ERROR_INT_ENA` reader - An errro occurs from ADC1, int enable."]
pub type COCPU_SARADC1_ERROR_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC1_ERROR_INT_ENA` writer - An errro occurs from ADC1, int enable."]
pub type COCPU_SARADC1_ERROR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC2_ERROR_INT_ENA` reader - An errro occurs from ADC2, int enable."]
pub type COCPU_SARADC2_ERROR_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC2_ERROR_INT_ENA` writer - An errro occurs from ADC2, int enable."]
pub type COCPU_SARADC2_ERROR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC1_WAKE_INT_ENA` reader - A wakeup event is triggered from ADC1, int enable."]
pub type COCPU_SARADC1_WAKE_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC1_WAKE_INT_ENA` writer - A wakeup event is triggered from ADC1, int enable."]
pub type COCPU_SARADC1_WAKE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC2_WAKE_INT_ENA` reader - A wakeup event is triggered from ADC2, int enable."]
pub type COCPU_SARADC2_WAKE_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC2_WAKE_INT_ENA` writer - A wakeup event is triggered from ADC2, int enable."]
pub type COCPU_SARADC2_WAKE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC1 Conversion is done, int enable."]
    #[inline(always)]
    pub fn cocpu_saradc1_int_ena(&self) -> COCPU_SARADC1_INT_ENA_R {
        COCPU_SARADC1_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC2 Conversion is done, int enable."]
    #[inline(always)]
    pub fn cocpu_saradc2_int_ena(&self) -> COCPU_SARADC2_INT_ENA_R {
        COCPU_SARADC2_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - An errro occurs from ADC1, int enable."]
    #[inline(always)]
    pub fn cocpu_saradc1_error_int_ena(&self) -> COCPU_SARADC1_ERROR_INT_ENA_R {
        COCPU_SARADC1_ERROR_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - An errro occurs from ADC2, int enable."]
    #[inline(always)]
    pub fn cocpu_saradc2_error_int_ena(&self) -> COCPU_SARADC2_ERROR_INT_ENA_R {
        COCPU_SARADC2_ERROR_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A wakeup event is triggered from ADC1, int enable."]
    #[inline(always)]
    pub fn cocpu_saradc1_wake_int_ena(&self) -> COCPU_SARADC1_WAKE_INT_ENA_R {
        COCPU_SARADC1_WAKE_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A wakeup event is triggered from ADC2, int enable."]
    #[inline(always)]
    pub fn cocpu_saradc2_wake_int_ena(&self) -> COCPU_SARADC2_WAKE_INT_ENA_R {
        COCPU_SARADC2_WAKE_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("cocpu_saradc1_int_ena", &self.cocpu_saradc1_int_ena())
            .field("cocpu_saradc2_int_ena", &self.cocpu_saradc2_int_ena())
            .field(
                "cocpu_saradc1_error_int_ena",
                &self.cocpu_saradc1_error_int_ena(),
            )
            .field(
                "cocpu_saradc2_error_int_ena",
                &self.cocpu_saradc2_error_int_ena(),
            )
            .field(
                "cocpu_saradc1_wake_int_ena",
                &self.cocpu_saradc1_wake_int_ena(),
            )
            .field(
                "cocpu_saradc2_wake_int_ena",
                &self.cocpu_saradc2_wake_int_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ADC1 Conversion is done, int enable."]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_saradc1_int_ena(&mut self) -> COCPU_SARADC1_INT_ENA_W<INT_ENA_SPEC> {
        COCPU_SARADC1_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC2 Conversion is done, int enable."]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_saradc2_int_ena(&mut self) -> COCPU_SARADC2_INT_ENA_W<INT_ENA_SPEC> {
        COCPU_SARADC2_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - An errro occurs from ADC1, int enable."]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_saradc1_error_int_ena(&mut self) -> COCPU_SARADC1_ERROR_INT_ENA_W<INT_ENA_SPEC> {
        COCPU_SARADC1_ERROR_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - An errro occurs from ADC2, int enable."]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_saradc2_error_int_ena(&mut self) -> COCPU_SARADC2_ERROR_INT_ENA_W<INT_ENA_SPEC> {
        COCPU_SARADC2_ERROR_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - A wakeup event is triggered from ADC1, int enable."]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_saradc1_wake_int_ena(&mut self) -> COCPU_SARADC1_WAKE_INT_ENA_W<INT_ENA_SPEC> {
        COCPU_SARADC1_WAKE_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - A wakeup event is triggered from ADC2, int enable."]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_saradc2_wake_int_ena(&mut self) -> COCPU_SARADC2_WAKE_INT_ENA_W<INT_ENA_SPEC> {
        COCPU_SARADC2_WAKE_INT_ENA_W::new(self, 5)
    }
}
#[doc = "Interrupt enable registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
