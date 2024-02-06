#[doc = "Register `T1QNR` reader"]
pub type R = crate::R<T1QNR_SPEC>;
#[doc = "Field `CHROMINANCE_QNR_VAL` reader - write this reg to configure 64 quantization coefficient in t1 table"]
pub type CHROMINANCE_QNR_VAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - write this reg to configure 64 quantization coefficient in t1 table"]
    #[inline(always)]
    pub fn chrominance_qnr_val(&self) -> CHROMINANCE_QNR_VAL_R {
        CHROMINANCE_QNR_VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T1QNR")
            .field(
                "chrominance_qnr_val",
                &format_args!("{}", self.chrominance_qnr_val().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T1QNR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1qnr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T1QNR_SPEC;
impl crate::RegisterSpec for T1QNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t1qnr::R`](R) reader structure"]
impl crate::Readable for T1QNR_SPEC {}
#[doc = "`reset()` method sets T1QNR to value 0"]
impl crate::Resettable for T1QNR_SPEC {
    const RESET_VALUE: u32 = 0;
}
