#[doc = "Register `REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER` reader"]
pub type R = crate::R<REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER_SPEC>;
#[doc = "Register `REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER` writer"]
pub type W = crate::W<REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER_SPEC>;
#[doc = "Field `CH1_ISC` reader - idleSlopeCredit This field contains the idleSlopeCredit value required for the creditbased shaper algorithm for Channel 1 This is the rate of change of credit in bits per cycle _40ns and 8ns for 100 Mbps and 1000 Mbps respectively_ when the credit is increasing The software should program this field with computed credit in bits per cycle scaled by 1024 The maximum value is portTransmitRate, that is, 0x2000 in 1000 Mbps mode and 0x1000 in 100 Mbps mode"]
pub type CH1_ISC_R = crate::FieldReader<u16>;
#[doc = "Field `CH1_ISC` writer - idleSlopeCredit This field contains the idleSlopeCredit value required for the creditbased shaper algorithm for Channel 1 This is the rate of change of credit in bits per cycle _40ns and 8ns for 100 Mbps and 1000 Mbps respectively_ when the credit is increasing The software should program this field with computed credit in bits per cycle scaled by 1024 The maximum value is portTransmitRate, that is, 0x2000 in 1000 Mbps mode and 0x1000 in 100 Mbps mode"]
pub type CH1_ISC_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - idleSlopeCredit This field contains the idleSlopeCredit value required for the creditbased shaper algorithm for Channel 1 This is the rate of change of credit in bits per cycle _40ns and 8ns for 100 Mbps and 1000 Mbps respectively_ when the credit is increasing The software should program this field with computed credit in bits per cycle scaled by 1024 The maximum value is portTransmitRate, that is, 0x2000 in 1000 Mbps mode and 0x1000 in 100 Mbps mode"]
    #[inline(always)]
    pub fn ch1_isc(&self) -> CH1_ISC_R {
        CH1_ISC_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER")
            .field("ch1_isc", &self.ch1_isc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - idleSlopeCredit This field contains the idleSlopeCredit value required for the creditbased shaper algorithm for Channel 1 This is the rate of change of credit in bits per cycle _40ns and 8ns for 100 Mbps and 1000 Mbps respectively_ when the credit is increasing The software should program this field with computed credit in bits per cycle scaled by 1024 The maximum value is portTransmitRate, that is, 0x2000 in 1000 Mbps mode and 0x1000 in 100 Mbps mode"]
    #[inline(always)]
    pub fn ch1_isc(&mut self) -> CH1_ISC_W<'_, REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER_SPEC> {
        CH1_ISC_W::new(self, 0)
    }
}
#[doc = "Contains the idleSlope credit value required for the creditbased shaper algorithm for Channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`register90_channel1idleslopecreditregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register90_channel1idleslopecreditregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register90_channel1idleslopecreditregister::R`](R) reader structure"]
impl crate::Readable for REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register90_channel1idleslopecreditregister::W`](W) writer structure"]
impl crate::Writable for REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER to value 0"]
impl crate::Resettable for REGISTER90_CHANNEL1IDLESLOPECREDITREGISTER_SPEC {}
