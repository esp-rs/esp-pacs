#[doc = "Register `ONETIME_SAMPLE` reader"]
pub type R = crate::R<ONETIME_SAMPLE_SPEC>;
#[doc = "Register `ONETIME_SAMPLE` writer"]
pub type W = crate::W<ONETIME_SAMPLE_SPEC>;
#[doc = "Field `SARADC_ONETIME_ATTEN` reader - configure onetime atten"]
pub type SARADC_ONETIME_ATTEN_R = crate::FieldReader;
#[doc = "Field `SARADC_ONETIME_ATTEN` writer - configure onetime atten"]
pub type SARADC_ONETIME_ATTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SARADC_ONETIME_CHANNEL` reader - configure onetime channel"]
pub type SARADC_ONETIME_CHANNEL_R = crate::FieldReader;
#[doc = "Field `SARADC_ONETIME_CHANNEL` writer - configure onetime channel"]
pub type SARADC_ONETIME_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SARADC_ONETIME_START` reader - trigger adc onetime sample"]
pub type SARADC_ONETIME_START_R = crate::BitReader;
#[doc = "Field `SARADC_ONETIME_START` writer - trigger adc onetime sample"]
pub type SARADC_ONETIME_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC2_ONETIME_SAMPLE` reader - enable adc2 onetime sample"]
pub type SARADC2_ONETIME_SAMPLE_R = crate::BitReader;
#[doc = "Field `SARADC2_ONETIME_SAMPLE` writer - enable adc2 onetime sample"]
pub type SARADC2_ONETIME_SAMPLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC1_ONETIME_SAMPLE` reader - enable adc1 onetime sample"]
pub type SARADC1_ONETIME_SAMPLE_R = crate::BitReader;
#[doc = "Field `SARADC1_ONETIME_SAMPLE` writer - enable adc1 onetime sample"]
pub type SARADC1_ONETIME_SAMPLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 23:24 - configure onetime atten"]
    #[inline(always)]
    pub fn saradc_onetime_atten(&self) -> SARADC_ONETIME_ATTEN_R {
        SARADC_ONETIME_ATTEN_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:28 - configure onetime channel"]
    #[inline(always)]
    pub fn saradc_onetime_channel(&self) -> SARADC_ONETIME_CHANNEL_R {
        SARADC_ONETIME_CHANNEL_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - trigger adc onetime sample"]
    #[inline(always)]
    pub fn saradc_onetime_start(&self) -> SARADC_ONETIME_START_R {
        SARADC_ONETIME_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - enable adc2 onetime sample"]
    #[inline(always)]
    pub fn saradc2_onetime_sample(&self) -> SARADC2_ONETIME_SAMPLE_R {
        SARADC2_ONETIME_SAMPLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable adc1 onetime sample"]
    #[inline(always)]
    pub fn saradc1_onetime_sample(&self) -> SARADC1_ONETIME_SAMPLE_R {
        SARADC1_ONETIME_SAMPLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ONETIME_SAMPLE")
            .field(
                "saradc_onetime_atten",
                &format_args!("{}", self.saradc_onetime_atten().bits()),
            )
            .field(
                "saradc_onetime_channel",
                &format_args!("{}", self.saradc_onetime_channel().bits()),
            )
            .field(
                "saradc_onetime_start",
                &format_args!("{}", self.saradc_onetime_start().bit()),
            )
            .field(
                "saradc2_onetime_sample",
                &format_args!("{}", self.saradc2_onetime_sample().bit()),
            )
            .field(
                "saradc1_onetime_sample",
                &format_args!("{}", self.saradc1_onetime_sample().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ONETIME_SAMPLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 23:24 - configure onetime atten"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_onetime_atten(&mut self) -> SARADC_ONETIME_ATTEN_W<ONETIME_SAMPLE_SPEC> {
        SARADC_ONETIME_ATTEN_W::new(self, 23)
    }
    #[doc = "Bits 25:28 - configure onetime channel"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_onetime_channel(&mut self) -> SARADC_ONETIME_CHANNEL_W<ONETIME_SAMPLE_SPEC> {
        SARADC_ONETIME_CHANNEL_W::new(self, 25)
    }
    #[doc = "Bit 29 - trigger adc onetime sample"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_onetime_start(&mut self) -> SARADC_ONETIME_START_W<ONETIME_SAMPLE_SPEC> {
        SARADC_ONETIME_START_W::new(self, 29)
    }
    #[doc = "Bit 30 - enable adc2 onetime sample"]
    #[inline(always)]
    #[must_use]
    pub fn saradc2_onetime_sample(&mut self) -> SARADC2_ONETIME_SAMPLE_W<ONETIME_SAMPLE_SPEC> {
        SARADC2_ONETIME_SAMPLE_W::new(self, 30)
    }
    #[doc = "Bit 31 - enable adc1 onetime sample"]
    #[inline(always)]
    #[must_use]
    pub fn saradc1_onetime_sample(&mut self) -> SARADC1_ONETIME_SAMPLE_W<ONETIME_SAMPLE_SPEC> {
        SARADC1_ONETIME_SAMPLE_W::new(self, 31)
    }
}
#[doc = "digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`onetime_sample::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`onetime_sample::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ONETIME_SAMPLE_SPEC;
impl crate::RegisterSpec for ONETIME_SAMPLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onetime_sample::R`](R) reader structure"]
impl crate::Readable for ONETIME_SAMPLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`onetime_sample::W`](W) writer structure"]
impl crate::Writable for ONETIME_SAMPLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONETIME_SAMPLE to value 0x1a00_0000"]
impl crate::Resettable for ONETIME_SAMPLE_SPEC {
    const RESET_VALUE: u32 = 0x1a00_0000;
}
