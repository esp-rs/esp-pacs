#[doc = "Register `REGISTER450_SYSTEMTIMESECONDSREGISTER` reader"]
pub type R = crate::R<REGISTER450_SYSTEMTIMESECONDSREGISTER_SPEC>;
#[doc = "Field `TSS_RO` reader - Timestamp Second The value in this field indicates the current value in seconds of the System Time maintained by the MAC"]
pub type TSS_RO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Second The value in this field indicates the current value in seconds of the System Time maintained by the MAC"]
    #[inline(always)]
    pub fn tss_ro(&self) -> TSS_RO_R {
        TSS_RO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER450_SYSTEMTIMESECONDSREGISTER")
            .field("tss_ro", &self.tss_ro())
            .finish()
    }
}
#[doc = "Contains the lower 32 bits of the seconds field of the system time This register is present only when IEEE1588 timestamping is enabled without an external timestamp input\n\nYou can [`read`](crate::Reg::read) this register and get [`register450_systemtimesecondsregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER450_SYSTEMTIMESECONDSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER450_SYSTEMTIMESECONDSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register450_systemtimesecondsregister::R`](R) reader structure"]
impl crate::Readable for REGISTER450_SYSTEMTIMESECONDSREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER450_SYSTEMTIMESECONDSREGISTER to value 0"]
impl crate::Resettable for REGISTER450_SYSTEMTIMESECONDSREGISTER_SPEC {}
