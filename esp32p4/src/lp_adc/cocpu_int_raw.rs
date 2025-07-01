#[doc = "Register `COCPU_INT_RAW` reader"]
pub type R = crate::R<COCPU_INT_RAW_SPEC>;
#[doc = "Register `COCPU_INT_RAW` writer"]
pub type W = crate::W<COCPU_INT_RAW_SPEC>;
#[doc = "Field `COCPU_SARADC1_INT_RAW` reader - ADC1 Conversion is done, int raw."]
pub type COCPU_SARADC1_INT_RAW_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC1_INT_RAW` writer - ADC1 Conversion is done, int raw."]
pub type COCPU_SARADC1_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC2_INT_RAW` reader - ADC2 Conversion is done, int raw."]
pub type COCPU_SARADC2_INT_RAW_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC2_INT_RAW` writer - ADC2 Conversion is done, int raw."]
pub type COCPU_SARADC2_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC1_ERROR_INT_RAW` reader - An errro occurs from ADC1, int raw."]
pub type COCPU_SARADC1_ERROR_INT_RAW_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC1_ERROR_INT_RAW` writer - An errro occurs from ADC1, int raw."]
pub type COCPU_SARADC1_ERROR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC2_ERROR_INT_RAW` reader - An errro occurs from ADC2, int raw."]
pub type COCPU_SARADC2_ERROR_INT_RAW_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC2_ERROR_INT_RAW` writer - An errro occurs from ADC2, int raw."]
pub type COCPU_SARADC2_ERROR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC1_WAKE_INT_RAW` reader - A wakeup event is triggered from ADC1, int raw."]
pub type COCPU_SARADC1_WAKE_INT_RAW_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC1_WAKE_INT_RAW` writer - A wakeup event is triggered from ADC1, int raw."]
pub type COCPU_SARADC1_WAKE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COCPU_SARADC2_WAKE_INT_RAW` reader - A wakeup event is triggered from ADC2, int raw."]
pub type COCPU_SARADC2_WAKE_INT_RAW_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC2_WAKE_INT_RAW` writer - A wakeup event is triggered from ADC2, int raw."]
pub type COCPU_SARADC2_WAKE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC1 Conversion is done, int raw."]
    #[inline(always)]
    pub fn cocpu_saradc1_int_raw(&self) -> COCPU_SARADC1_INT_RAW_R {
        COCPU_SARADC1_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC2 Conversion is done, int raw."]
    #[inline(always)]
    pub fn cocpu_saradc2_int_raw(&self) -> COCPU_SARADC2_INT_RAW_R {
        COCPU_SARADC2_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - An errro occurs from ADC1, int raw."]
    #[inline(always)]
    pub fn cocpu_saradc1_error_int_raw(&self) -> COCPU_SARADC1_ERROR_INT_RAW_R {
        COCPU_SARADC1_ERROR_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - An errro occurs from ADC2, int raw."]
    #[inline(always)]
    pub fn cocpu_saradc2_error_int_raw(&self) -> COCPU_SARADC2_ERROR_INT_RAW_R {
        COCPU_SARADC2_ERROR_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A wakeup event is triggered from ADC1, int raw."]
    #[inline(always)]
    pub fn cocpu_saradc1_wake_int_raw(&self) -> COCPU_SARADC1_WAKE_INT_RAW_R {
        COCPU_SARADC1_WAKE_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A wakeup event is triggered from ADC2, int raw."]
    #[inline(always)]
    pub fn cocpu_saradc2_wake_int_raw(&self) -> COCPU_SARADC2_WAKE_INT_RAW_R {
        COCPU_SARADC2_WAKE_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COCPU_INT_RAW")
            .field("cocpu_saradc1_int_raw", &self.cocpu_saradc1_int_raw())
            .field("cocpu_saradc2_int_raw", &self.cocpu_saradc2_int_raw())
            .field(
                "cocpu_saradc1_error_int_raw",
                &self.cocpu_saradc1_error_int_raw(),
            )
            .field(
                "cocpu_saradc2_error_int_raw",
                &self.cocpu_saradc2_error_int_raw(),
            )
            .field(
                "cocpu_saradc1_wake_int_raw",
                &self.cocpu_saradc1_wake_int_raw(),
            )
            .field(
                "cocpu_saradc2_wake_int_raw",
                &self.cocpu_saradc2_wake_int_raw(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ADC1 Conversion is done, int raw."]
    #[inline(always)]
    pub fn cocpu_saradc1_int_raw(&mut self) -> COCPU_SARADC1_INT_RAW_W<COCPU_INT_RAW_SPEC> {
        COCPU_SARADC1_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC2 Conversion is done, int raw."]
    #[inline(always)]
    pub fn cocpu_saradc2_int_raw(&mut self) -> COCPU_SARADC2_INT_RAW_W<COCPU_INT_RAW_SPEC> {
        COCPU_SARADC2_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - An errro occurs from ADC1, int raw."]
    #[inline(always)]
    pub fn cocpu_saradc1_error_int_raw(
        &mut self,
    ) -> COCPU_SARADC1_ERROR_INT_RAW_W<COCPU_INT_RAW_SPEC> {
        COCPU_SARADC1_ERROR_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - An errro occurs from ADC2, int raw."]
    #[inline(always)]
    pub fn cocpu_saradc2_error_int_raw(
        &mut self,
    ) -> COCPU_SARADC2_ERROR_INT_RAW_W<COCPU_INT_RAW_SPEC> {
        COCPU_SARADC2_ERROR_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - A wakeup event is triggered from ADC1, int raw."]
    #[inline(always)]
    pub fn cocpu_saradc1_wake_int_raw(
        &mut self,
    ) -> COCPU_SARADC1_WAKE_INT_RAW_W<COCPU_INT_RAW_SPEC> {
        COCPU_SARADC1_WAKE_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - A wakeup event is triggered from ADC2, int raw."]
    #[inline(always)]
    pub fn cocpu_saradc2_wake_int_raw(
        &mut self,
    ) -> COCPU_SARADC2_WAKE_INT_RAW_W<COCPU_INT_RAW_SPEC> {
        COCPU_SARADC2_WAKE_INT_RAW_W::new(self, 5)
    }
}
#[doc = "Interrupt raw registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`cocpu_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cocpu_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COCPU_INT_RAW_SPEC;
impl crate::RegisterSpec for COCPU_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cocpu_int_raw::R`](R) reader structure"]
impl crate::Readable for COCPU_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cocpu_int_raw::W`](W) writer structure"]
impl crate::Writable for COCPU_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COCPU_INT_RAW to value 0"]
impl crate::Resettable for COCPU_INT_RAW_SPEC {}
