#[doc = "Register `REGISTER93_CHANNEL1LOCREDITREGISTER` reader"]
pub type R = crate::R<REGISTER93_CHANNEL1LOCREDITREGISTER_SPEC>;
#[doc = "Register `REGISTER93_CHANNEL1LOCREDITREGISTER` writer"]
pub type W = crate::W<REGISTER93_CHANNEL1LOCREDITREGISTER_SPEC>;
#[doc = "Field `CH1_LC` reader - loCredit This field contains the loCredit value required for the creditbased shaper algorithm for Channel 1 This is the minimum value that can be accumulated in the credit parameter This is specified in bits scaled by 1,024 The maximum value is maxInterferenceSize, that is, besteffort maximum frame size which is 16,384 bytes or 131,072 bits The value to be specified is 131,072 * 1,024 = 134,217,728 or 0x0800_0000 The programmed value is 2's complement _negative number_, that is, 0xF800_0000"]
pub type CH1_LC_R = crate::FieldReader<u32>;
#[doc = "Field `CH1_LC` writer - loCredit This field contains the loCredit value required for the creditbased shaper algorithm for Channel 1 This is the minimum value that can be accumulated in the credit parameter This is specified in bits scaled by 1,024 The maximum value is maxInterferenceSize, that is, besteffort maximum frame size which is 16,384 bytes or 131,072 bits The value to be specified is 131,072 * 1,024 = 134,217,728 or 0x0800_0000 The programmed value is 2's complement _negative number_, that is, 0xF800_0000"]
pub type CH1_LC_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - loCredit This field contains the loCredit value required for the creditbased shaper algorithm for Channel 1 This is the minimum value that can be accumulated in the credit parameter This is specified in bits scaled by 1,024 The maximum value is maxInterferenceSize, that is, besteffort maximum frame size which is 16,384 bytes or 131,072 bits The value to be specified is 131,072 * 1,024 = 134,217,728 or 0x0800_0000 The programmed value is 2's complement _negative number_, that is, 0xF800_0000"]
    #[inline(always)]
    pub fn ch1_lc(&self) -> CH1_LC_R {
        CH1_LC_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER93_CHANNEL1LOCREDITREGISTER")
            .field("ch1_lc", &self.ch1_lc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:28 - loCredit This field contains the loCredit value required for the creditbased shaper algorithm for Channel 1 This is the minimum value that can be accumulated in the credit parameter This is specified in bits scaled by 1,024 The maximum value is maxInterferenceSize, that is, besteffort maximum frame size which is 16,384 bytes or 131,072 bits The value to be specified is 131,072 * 1,024 = 134,217,728 or 0x0800_0000 The programmed value is 2's complement _negative number_, that is, 0xF800_0000"]
    #[inline(always)]
    pub fn ch1_lc(&mut self) -> CH1_LC_W<'_, REGISTER93_CHANNEL1LOCREDITREGISTER_SPEC> {
        CH1_LC_W::new(self, 0)
    }
}
#[doc = "Contains the loCredit value required for the creditbased shaper algorithm for Channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`register93_channel1locreditregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register93_channel1locreditregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER93_CHANNEL1LOCREDITREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER93_CHANNEL1LOCREDITREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register93_channel1locreditregister::R`](R) reader structure"]
impl crate::Readable for REGISTER93_CHANNEL1LOCREDITREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register93_channel1locreditregister::W`](W) writer structure"]
impl crate::Writable for REGISTER93_CHANNEL1LOCREDITREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER93_CHANNEL1LOCREDITREGISTER to value 0x1fff_ffff"]
impl crate::Resettable for REGISTER93_CHANNEL1LOCREDITREGISTER_SPEC {
    const RESET_VALUE: u32 = 0x1fff_ffff;
}
