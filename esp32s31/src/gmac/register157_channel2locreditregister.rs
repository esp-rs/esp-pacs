#[doc = "Register `REGISTER157_CHANNEL2LOCREDITREGISTER` reader"]
pub type R = crate::R<REGISTER157_CHANNEL2LOCREDITREGISTER_SPEC>;
#[doc = "Register `REGISTER157_CHANNEL2LOCREDITREGISTER` writer"]
pub type W = crate::W<REGISTER157_CHANNEL2LOCREDITREGISTER_SPEC>;
#[doc = "Field `CH2_LC` reader - loCredit This field contains the loCredit value required for the creditbased shaper algorithm for Channel 2 This is the minimum value that can be accumulated in the credit parameter This is specified in bits scaled by 1,024 The maximum value is maxInterferenceSize, that is, besteffort maximum frame size which is 16,384 bytes or 131,072 bits The value to be specified is 131,072 * 1,024 = 134,217,728 or 0x0800_0000 The programmed value is 2's complement _negative number_, that is, 0xF800_0000"]
pub type CH2_LC_R = crate::FieldReader<u32>;
#[doc = "Field `CH2_LC` writer - loCredit This field contains the loCredit value required for the creditbased shaper algorithm for Channel 2 This is the minimum value that can be accumulated in the credit parameter This is specified in bits scaled by 1,024 The maximum value is maxInterferenceSize, that is, besteffort maximum frame size which is 16,384 bytes or 131,072 bits The value to be specified is 131,072 * 1,024 = 134,217,728 or 0x0800_0000 The programmed value is 2's complement _negative number_, that is, 0xF800_0000"]
pub type CH2_LC_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - loCredit This field contains the loCredit value required for the creditbased shaper algorithm for Channel 2 This is the minimum value that can be accumulated in the credit parameter This is specified in bits scaled by 1,024 The maximum value is maxInterferenceSize, that is, besteffort maximum frame size which is 16,384 bytes or 131,072 bits The value to be specified is 131,072 * 1,024 = 134,217,728 or 0x0800_0000 The programmed value is 2's complement _negative number_, that is, 0xF800_0000"]
    #[inline(always)]
    pub fn ch2_lc(&self) -> CH2_LC_R {
        CH2_LC_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER157_CHANNEL2LOCREDITREGISTER")
            .field("ch2_lc", &self.ch2_lc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:28 - loCredit This field contains the loCredit value required for the creditbased shaper algorithm for Channel 2 This is the minimum value that can be accumulated in the credit parameter This is specified in bits scaled by 1,024 The maximum value is maxInterferenceSize, that is, besteffort maximum frame size which is 16,384 bytes or 131,072 bits The value to be specified is 131,072 * 1,024 = 134,217,728 or 0x0800_0000 The programmed value is 2's complement _negative number_, that is, 0xF800_0000"]
    #[inline(always)]
    pub fn ch2_lc(&mut self) -> CH2_LC_W<'_, REGISTER157_CHANNEL2LOCREDITREGISTER_SPEC> {
        CH2_LC_W::new(self, 0)
    }
}
#[doc = "Contains the loCredit value required for the creditbased shaper algorithm for Channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`register157_channel2locreditregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register157_channel2locreditregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER157_CHANNEL2LOCREDITREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER157_CHANNEL2LOCREDITREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register157_channel2locreditregister::R`](R) reader structure"]
impl crate::Readable for REGISTER157_CHANNEL2LOCREDITREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register157_channel2locreditregister::W`](W) writer structure"]
impl crate::Writable for REGISTER157_CHANNEL2LOCREDITREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER157_CHANNEL2LOCREDITREGISTER to value 0x1fff_ffff"]
impl crate::Resettable for REGISTER157_CHANNEL2LOCREDITREGISTER_SPEC {
    const RESET_VALUE: u32 = 0x1fff_ffff;
}
