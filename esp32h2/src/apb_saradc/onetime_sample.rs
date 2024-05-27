///Register `ONETIME_SAMPLE` reader
pub type R = crate::R<ONETIME_SAMPLE_SPEC>;
///Register `ONETIME_SAMPLE` writer
pub type W = crate::W<ONETIME_SAMPLE_SPEC>;
///Field `ONETIME_ATTEN` reader - configure onetime atten
pub type ONETIME_ATTEN_R = crate::FieldReader;
///Field `ONETIME_ATTEN` writer - configure onetime atten
pub type ONETIME_ATTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ONETIME_CHANNEL` reader - configure onetime channel
pub type ONETIME_CHANNEL_R = crate::FieldReader;
///Field `ONETIME_CHANNEL` writer - configure onetime channel
pub type ONETIME_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ONETIME_START` reader - trigger adc onetime sample
pub type ONETIME_START_R = crate::BitReader;
///Field `ONETIME_START` writer - trigger adc onetime sample
pub type ONETIME_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SARADC2_ONETIME_SAMPLE` reader - enable adc2 onetime sample
pub type SARADC2_ONETIME_SAMPLE_R = crate::BitReader;
///Field `SARADC2_ONETIME_SAMPLE` writer - enable adc2 onetime sample
pub type SARADC2_ONETIME_SAMPLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SARADC1_ONETIME_SAMPLE` reader - enable adc1 onetime sample
pub type SARADC1_ONETIME_SAMPLE_R = crate::BitReader;
///Field `SARADC1_ONETIME_SAMPLE` writer - enable adc1 onetime sample
pub type SARADC1_ONETIME_SAMPLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 23:24 - configure onetime atten
    #[inline(always)]
    pub fn onetime_atten(&self) -> ONETIME_ATTEN_R {
        ONETIME_ATTEN_R::new(((self.bits >> 23) & 3) as u8)
    }
    ///Bits 25:28 - configure onetime channel
    #[inline(always)]
    pub fn onetime_channel(&self) -> ONETIME_CHANNEL_R {
        ONETIME_CHANNEL_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    ///Bit 29 - trigger adc onetime sample
    #[inline(always)]
    pub fn onetime_start(&self) -> ONETIME_START_R {
        ONETIME_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - enable adc2 onetime sample
    #[inline(always)]
    pub fn saradc2_onetime_sample(&self) -> SARADC2_ONETIME_SAMPLE_R {
        SARADC2_ONETIME_SAMPLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - enable adc1 onetime sample
    #[inline(always)]
    pub fn saradc1_onetime_sample(&self) -> SARADC1_ONETIME_SAMPLE_R {
        SARADC1_ONETIME_SAMPLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ONETIME_SAMPLE")
            .field("onetime_atten", &self.onetime_atten())
            .field("onetime_channel", &self.onetime_channel())
            .field("onetime_start", &self.onetime_start())
            .field("saradc2_onetime_sample", &self.saradc2_onetime_sample())
            .field("saradc1_onetime_sample", &self.saradc1_onetime_sample())
            .finish()
    }
}
impl W {
    ///Bits 23:24 - configure onetime atten
    #[inline(always)]
    #[must_use]
    pub fn onetime_atten(&mut self) -> ONETIME_ATTEN_W<ONETIME_SAMPLE_SPEC> {
        ONETIME_ATTEN_W::new(self, 23)
    }
    ///Bits 25:28 - configure onetime channel
    #[inline(always)]
    #[must_use]
    pub fn onetime_channel(&mut self) -> ONETIME_CHANNEL_W<ONETIME_SAMPLE_SPEC> {
        ONETIME_CHANNEL_W::new(self, 25)
    }
    ///Bit 29 - trigger adc onetime sample
    #[inline(always)]
    #[must_use]
    pub fn onetime_start(&mut self) -> ONETIME_START_W<ONETIME_SAMPLE_SPEC> {
        ONETIME_START_W::new(self, 29)
    }
    ///Bit 30 - enable adc2 onetime sample
    #[inline(always)]
    #[must_use]
    pub fn saradc2_onetime_sample(&mut self) -> SARADC2_ONETIME_SAMPLE_W<ONETIME_SAMPLE_SPEC> {
        SARADC2_ONETIME_SAMPLE_W::new(self, 30)
    }
    ///Bit 31 - enable adc1 onetime sample
    #[inline(always)]
    #[must_use]
    pub fn saradc1_onetime_sample(&mut self) -> SARADC1_ONETIME_SAMPLE_W<ONETIME_SAMPLE_SPEC> {
        SARADC1_ONETIME_SAMPLE_W::new(self, 31)
    }
}
/**digital saradc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`onetime_sample::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`onetime_sample::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ONETIME_SAMPLE_SPEC;
impl crate::RegisterSpec for ONETIME_SAMPLE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`onetime_sample::R`](R) reader structure
impl crate::Readable for ONETIME_SAMPLE_SPEC {}
///`write(|w| ..)` method takes [`onetime_sample::W`](W) writer structure
impl crate::Writable for ONETIME_SAMPLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ONETIME_SAMPLE to value 0x1a00_0000
impl crate::Resettable for ONETIME_SAMPLE_SPEC {
    const RESET_VALUE: u32 = 0x1a00_0000;
}
