#[doc = "Register `REGISTER156_CHANNEL2HICREDITREGISTER` reader"]
pub type R = crate::R<REGISTER156_CHANNEL2HICREDITREGISTER_SPEC>;
#[doc = "Register `REGISTER156_CHANNEL2HICREDITREGISTER` writer"]
pub type W = crate::W<REGISTER156_CHANNEL2HICREDITREGISTER_SPEC>;
#[doc = "Field `CH2_HC` reader - hiCredit This field contains the hiCredit value required for the creditbased shaper algorithm for Channel 2 This is the maximum value that can be accumulated in the credit parameter This is specified in bits scaled by 1,024 The maximum value is maxInterferenceSize, that is, besteffort maximum frame size which is 16,384 bytes or 131,072 bits The value to be specified is 131,072 * 1,024 = 134,217,728 or 0x0800_0000"]
pub type CH2_HC_R = crate::FieldReader<u32>;
#[doc = "Field `CH2_HC` writer - hiCredit This field contains the hiCredit value required for the creditbased shaper algorithm for Channel 2 This is the maximum value that can be accumulated in the credit parameter This is specified in bits scaled by 1,024 The maximum value is maxInterferenceSize, that is, besteffort maximum frame size which is 16,384 bytes or 131,072 bits The value to be specified is 131,072 * 1,024 = 134,217,728 or 0x0800_0000"]
pub type CH2_HC_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - hiCredit This field contains the hiCredit value required for the creditbased shaper algorithm for Channel 2 This is the maximum value that can be accumulated in the credit parameter This is specified in bits scaled by 1,024 The maximum value is maxInterferenceSize, that is, besteffort maximum frame size which is 16,384 bytes or 131,072 bits The value to be specified is 131,072 * 1,024 = 134,217,728 or 0x0800_0000"]
    #[inline(always)]
    pub fn ch2_hc(&self) -> CH2_HC_R {
        CH2_HC_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER156_CHANNEL2HICREDITREGISTER")
            .field("ch2_hc", &self.ch2_hc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:28 - hiCredit This field contains the hiCredit value required for the creditbased shaper algorithm for Channel 2 This is the maximum value that can be accumulated in the credit parameter This is specified in bits scaled by 1,024 The maximum value is maxInterferenceSize, that is, besteffort maximum frame size which is 16,384 bytes or 131,072 bits The value to be specified is 131,072 * 1,024 = 134,217,728 or 0x0800_0000"]
    #[inline(always)]
    pub fn ch2_hc(&mut self) -> CH2_HC_W<'_, REGISTER156_CHANNEL2HICREDITREGISTER_SPEC> {
        CH2_HC_W::new(self, 0)
    }
}
#[doc = "Contains the hiCredit value required for the creditbased shaper algorithm for Channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`register156_channel2hicreditregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register156_channel2hicreditregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER156_CHANNEL2HICREDITREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER156_CHANNEL2HICREDITREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register156_channel2hicreditregister::R`](R) reader structure"]
impl crate::Readable for REGISTER156_CHANNEL2HICREDITREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register156_channel2hicreditregister::W`](W) writer structure"]
impl crate::Writable for REGISTER156_CHANNEL2HICREDITREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER156_CHANNEL2HICREDITREGISTER to value 0"]
impl crate::Resettable for REGISTER156_CHANNEL2HICREDITREGISTER_SPEC {}
