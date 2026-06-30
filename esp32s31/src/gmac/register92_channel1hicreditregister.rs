#[doc = "Register `REGISTER92_CHANNEL1HICREDITREGISTER` reader"]
pub type R = crate::R<REGISTER92_CHANNEL1HICREDITREGISTER_SPEC>;
#[doc = "Register `REGISTER92_CHANNEL1HICREDITREGISTER` writer"]
pub type W = crate::W<REGISTER92_CHANNEL1HICREDITREGISTER_SPEC>;
#[doc = "Field `CH1_HC` reader - hiCredit This field contains the hiCredit value required for the creditbased shaper algorithm for Channel 1 This is the maximum value that can be accumulated in the credit parameter This is specified in bits scaled by 1,024 The maximum value is maxInterferenceSize, that is, besteffort maximum frame size which is 16,384 bytes or 131,072 bits The value to be specified is 131,072 * 1,024 = 134,217,728 or 0x0800_0000"]
pub type CH1_HC_R = crate::FieldReader<u32>;
#[doc = "Field `CH1_HC` writer - hiCredit This field contains the hiCredit value required for the creditbased shaper algorithm for Channel 1 This is the maximum value that can be accumulated in the credit parameter This is specified in bits scaled by 1,024 The maximum value is maxInterferenceSize, that is, besteffort maximum frame size which is 16,384 bytes or 131,072 bits The value to be specified is 131,072 * 1,024 = 134,217,728 or 0x0800_0000"]
pub type CH1_HC_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - hiCredit This field contains the hiCredit value required for the creditbased shaper algorithm for Channel 1 This is the maximum value that can be accumulated in the credit parameter This is specified in bits scaled by 1,024 The maximum value is maxInterferenceSize, that is, besteffort maximum frame size which is 16,384 bytes or 131,072 bits The value to be specified is 131,072 * 1,024 = 134,217,728 or 0x0800_0000"]
    #[inline(always)]
    pub fn ch1_hc(&self) -> CH1_HC_R {
        CH1_HC_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER92_CHANNEL1HICREDITREGISTER")
            .field("ch1_hc", &self.ch1_hc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:28 - hiCredit This field contains the hiCredit value required for the creditbased shaper algorithm for Channel 1 This is the maximum value that can be accumulated in the credit parameter This is specified in bits scaled by 1,024 The maximum value is maxInterferenceSize, that is, besteffort maximum frame size which is 16,384 bytes or 131,072 bits The value to be specified is 131,072 * 1,024 = 134,217,728 or 0x0800_0000"]
    #[inline(always)]
    pub fn ch1_hc(&mut self) -> CH1_HC_W<'_, REGISTER92_CHANNEL1HICREDITREGISTER_SPEC> {
        CH1_HC_W::new(self, 0)
    }
}
#[doc = "Contains the hiCredit value required for the creditbased shaper algorithm for Channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`register92_channel1hicreditregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register92_channel1hicreditregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER92_CHANNEL1HICREDITREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER92_CHANNEL1HICREDITREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register92_channel1hicreditregister::R`](R) reader structure"]
impl crate::Readable for REGISTER92_CHANNEL1HICREDITREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register92_channel1hicreditregister::W`](W) writer structure"]
impl crate::Writable for REGISTER92_CHANNEL1HICREDITREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER92_CHANNEL1HICREDITREGISTER to value 0"]
impl crate::Resettable for REGISTER92_CHANNEL1HICREDITREGISTER_SPEC {}
