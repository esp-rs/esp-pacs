#[doc = "Register `REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER` reader"]
pub type R = crate::R<REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER_SPEC>;
#[doc = "Register `REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER` writer"]
pub type W = crate::W<REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER_SPEC>;
#[doc = "Field `CH1_SSC` reader - sendSlopeCredit This field contains the sendSlopeCredit value required for creditbased shaper algorithm for Channel 1 This is the rate of change of credit in bits per cycle _40ns and 8ns for 100 Mbps and 1000 Mbps respectively_ when the credit is decreasing The software should program this field with computed credit in bits per cycle scaled by 1024 The maximum value is portTransmitRate, that is, 0x2000 in 1000 Mbps mode and 0x1000 in 100 Mbps mode This field should be programmed with absolute sendSlopeCredit value The creditbased shaper logic subtracts it from the accumulated credit when Channel 1 is selected for transmission"]
pub type CH1_SSC_R = crate::FieldReader<u16>;
#[doc = "Field `CH1_SSC` writer - sendSlopeCredit This field contains the sendSlopeCredit value required for creditbased shaper algorithm for Channel 1 This is the rate of change of credit in bits per cycle _40ns and 8ns for 100 Mbps and 1000 Mbps respectively_ when the credit is decreasing The software should program this field with computed credit in bits per cycle scaled by 1024 The maximum value is portTransmitRate, that is, 0x2000 in 1000 Mbps mode and 0x1000 in 100 Mbps mode This field should be programmed with absolute sendSlopeCredit value The creditbased shaper logic subtracts it from the accumulated credit when Channel 1 is selected for transmission"]
pub type CH1_SSC_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - sendSlopeCredit This field contains the sendSlopeCredit value required for creditbased shaper algorithm for Channel 1 This is the rate of change of credit in bits per cycle _40ns and 8ns for 100 Mbps and 1000 Mbps respectively_ when the credit is decreasing The software should program this field with computed credit in bits per cycle scaled by 1024 The maximum value is portTransmitRate, that is, 0x2000 in 1000 Mbps mode and 0x1000 in 100 Mbps mode This field should be programmed with absolute sendSlopeCredit value The creditbased shaper logic subtracts it from the accumulated credit when Channel 1 is selected for transmission"]
    #[inline(always)]
    pub fn ch1_ssc(&self) -> CH1_SSC_R {
        CH1_SSC_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER")
            .field("ch1_ssc", &self.ch1_ssc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - sendSlopeCredit This field contains the sendSlopeCredit value required for creditbased shaper algorithm for Channel 1 This is the rate of change of credit in bits per cycle _40ns and 8ns for 100 Mbps and 1000 Mbps respectively_ when the credit is decreasing The software should program this field with computed credit in bits per cycle scaled by 1024 The maximum value is portTransmitRate, that is, 0x2000 in 1000 Mbps mode and 0x1000 in 100 Mbps mode This field should be programmed with absolute sendSlopeCredit value The creditbased shaper logic subtracts it from the accumulated credit when Channel 1 is selected for transmission"]
    #[inline(always)]
    pub fn ch1_ssc(&mut self) -> CH1_SSC_W<'_, REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER_SPEC> {
        CH1_SSC_W::new(self, 0)
    }
}
#[doc = "Contains the sendSlope credit value required for the creditbased shaper algorithm for Channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`register91_channel1sendslopecreditregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register91_channel1sendslopecreditregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register91_channel1sendslopecreditregister::R`](R) reader structure"]
impl crate::Readable for REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register91_channel1sendslopecreditregister::W`](W) writer structure"]
impl crate::Writable for REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER to value 0"]
impl crate::Resettable for REGISTER91_CHANNEL1SENDSLOPECREDITREGISTER_SPEC {}
