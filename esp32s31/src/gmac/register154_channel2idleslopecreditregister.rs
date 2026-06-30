#[doc = "Register `REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER` reader"]
pub type R = crate::R<REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER_SPEC>;
#[doc = "Register `REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER` writer"]
pub type W = crate::W<REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER_SPEC>;
#[doc = "Field `CH2_ISC` reader - idleSlopeCredit This field contains the idleSlopeCredit value required for the creditbased shaper algorithm for Channel 2 This is the rate of change of credit in bits per cycle _40ns and 8ns for 100 Mbps and 1000 Mbps respectively_ when the credit is increasing The software should program this field with computed credit in bits per cycle scaled by 1024 The maximum value is portTransmitRate, that is, 0x2000 in 1000 Mbps mode and 0x1000 in 100 Mbps mode"]
pub type CH2_ISC_R = crate::FieldReader<u16>;
#[doc = "Field `CH2_ISC` writer - idleSlopeCredit This field contains the idleSlopeCredit value required for the creditbased shaper algorithm for Channel 2 This is the rate of change of credit in bits per cycle _40ns and 8ns for 100 Mbps and 1000 Mbps respectively_ when the credit is increasing The software should program this field with computed credit in bits per cycle scaled by 1024 The maximum value is portTransmitRate, that is, 0x2000 in 1000 Mbps mode and 0x1000 in 100 Mbps mode"]
pub type CH2_ISC_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - idleSlopeCredit This field contains the idleSlopeCredit value required for the creditbased shaper algorithm for Channel 2 This is the rate of change of credit in bits per cycle _40ns and 8ns for 100 Mbps and 1000 Mbps respectively_ when the credit is increasing The software should program this field with computed credit in bits per cycle scaled by 1024 The maximum value is portTransmitRate, that is, 0x2000 in 1000 Mbps mode and 0x1000 in 100 Mbps mode"]
    #[inline(always)]
    pub fn ch2_isc(&self) -> CH2_ISC_R {
        CH2_ISC_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER")
            .field("ch2_isc", &self.ch2_isc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - idleSlopeCredit This field contains the idleSlopeCredit value required for the creditbased shaper algorithm for Channel 2 This is the rate of change of credit in bits per cycle _40ns and 8ns for 100 Mbps and 1000 Mbps respectively_ when the credit is increasing The software should program this field with computed credit in bits per cycle scaled by 1024 The maximum value is portTransmitRate, that is, 0x2000 in 1000 Mbps mode and 0x1000 in 100 Mbps mode"]
    #[inline(always)]
    pub fn ch2_isc(&mut self) -> CH2_ISC_W<'_, REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER_SPEC> {
        CH2_ISC_W::new(self, 0)
    }
}
#[doc = "Contains the idleSlope credit value required for the creditbased shaper algorithm for Channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`register154_channel2idleslopecreditregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register154_channel2idleslopecreditregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register154_channel2idleslopecreditregister::R`](R) reader structure"]
impl crate::Readable for REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register154_channel2idleslopecreditregister::W`](W) writer structure"]
impl crate::Writable for REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER to value 0"]
impl crate::Resettable for REGISTER154_CHANNEL2IDLESLOPECREDITREGISTER_SPEC {}
