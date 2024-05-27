///Register `VAD_OB0` reader
pub type R = crate::R<VAD_OB0_SPEC>;
///Field `SPEECH_COUNT_OB` reader - Reg silent count observe
pub type SPEECH_COUNT_OB_R = crate::FieldReader;
///Field `SILENT_COUNT_OB` reader - Reg speech count observe
pub type SILENT_COUNT_OB_R = crate::FieldReader;
///Field `MAX_SIGNAL0_OB` reader - Reg max signal0 observe
pub type MAX_SIGNAL0_OB_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:7 - Reg silent count observe
    #[inline(always)]
    pub fn speech_count_ob(&self) -> SPEECH_COUNT_OB_R {
        SPEECH_COUNT_OB_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Reg speech count observe
    #[inline(always)]
    pub fn silent_count_ob(&self) -> SILENT_COUNT_OB_R {
        SILENT_COUNT_OB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:31 - Reg max signal0 observe
    #[inline(always)]
    pub fn max_signal0_ob(&self) -> MAX_SIGNAL0_OB_R {
        MAX_SIGNAL0_OB_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_OB0")
            .field("speech_count_ob", &self.speech_count_ob())
            .field("silent_count_ob", &self.silent_count_ob())
            .field("max_signal0_ob", &self.max_signal0_ob())
            .finish()
    }
}
/**I2S VAD Observe register

You can [`read`](crate::generic::Reg::read) this register and get [`vad_ob0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VAD_OB0_SPEC;
impl crate::RegisterSpec for VAD_OB0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`vad_ob0::R`](R) reader structure
impl crate::Readable for VAD_OB0_SPEC {}
///`reset()` method sets VAD_OB0 to value 0
impl crate::Resettable for VAD_OB0_SPEC {
    const RESET_VALUE: u32 = 0;
}
