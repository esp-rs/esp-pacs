#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `OVERFLOW` reader - Triggered when counter register of RTC main timer overflow."]
pub type OVERFLOW_R = crate::BitReader;
#[doc = "Field `OVERFLOW` writer - Triggered when counter register of RTC main timer overflow."]
pub type OVERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC_WAKEUP` reader - Triggered when RTC main timer reach the target value."]
pub type SOC_WAKEUP_R = crate::BitReader;
#[doc = "Field `SOC_WAKEUP` writer - Triggered when RTC main timer reach the target value."]
pub type SOC_WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Triggered when counter register of RTC main timer overflow."]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Triggered when RTC main timer reach the target value."]
    #[inline(always)]
    pub fn soc_wakeup(&self) -> SOC_WAKEUP_R {
        SOC_WAKEUP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("overflow", &self.overflow())
            .field("soc_wakeup", &self.soc_wakeup())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - Triggered when counter register of RTC main timer overflow."]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W<'_, INT_RAW_SPEC> {
        OVERFLOW_W::new(self, 30)
    }
    #[doc = "Bit 31 - Triggered when RTC main timer reach the target value."]
    #[inline(always)]
    pub fn soc_wakeup(&mut self) -> SOC_WAKEUP_W<'_, INT_RAW_SPEC> {
        SOC_WAKEUP_W::new(self, 31)
    }
}
#[doc = "RTC timer interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
