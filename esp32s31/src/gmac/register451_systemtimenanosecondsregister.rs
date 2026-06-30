#[doc = "Register `REGISTER451_SYSTEMTIMENANOSECONDSREGISTER` reader"]
pub type R = crate::R<REGISTER451_SYSTEMTIMENANOSECONDSREGISTER_SPEC>;
#[doc = "Field `TSSS_RO` reader - Timestamp Sub Seconds The value in this field has the sub second representation of time, with an accuracy of 046 ns When Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_, each bit represents 1 ns and the maximum value is 0x3B9A_C9FF, after which it rollsover to zero"]
pub type TSSS_RO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - Timestamp Sub Seconds The value in this field has the sub second representation of time, with an accuracy of 046 ns When Bit 9 _TSCTRLSSR_ is set in Register 448 _Timestamp Control Register_, each bit represents 1 ns and the maximum value is 0x3B9A_C9FF, after which it rollsover to zero"]
    #[inline(always)]
    pub fn tsss_ro(&self) -> TSSS_RO_R {
        TSSS_RO_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER451_SYSTEMTIMENANOSECONDSREGISTER")
            .field("tsss_ro", &self.tsss_ro())
            .finish()
    }
}
#[doc = "Contains 32 bits of the nanoseconds field of the system time This register is only present when IEEE1588 timestamping is enabled without an external timestamp input\n\nYou can [`read`](crate::Reg::read) this register and get [`register451_systemtimenanosecondsregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER451_SYSTEMTIMENANOSECONDSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER451_SYSTEMTIMENANOSECONDSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register451_systemtimenanosecondsregister::R`](R) reader structure"]
impl crate::Readable for REGISTER451_SYSTEMTIMENANOSECONDSREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER451_SYSTEMTIMENANOSECONDSREGISTER to value 0"]
impl crate::Resettable for REGISTER451_SYSTEMTIMENANOSECONDSREGISTER_SPEC {}
