#[doc = "Register `T0QNR` reader"]
pub type R = crate::R<T0QNR_SPEC>;
#[doc = "Field `T0_QNR_VAL` reader - write this reg to configure 64 quantization coefficient in t0 table"]
pub type T0_QNR_VAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - write this reg to configure 64 quantization coefficient in t0 table"]
    #[inline(always)]
    pub fn t0_qnr_val(&self) -> T0_QNR_VAL_R {
        T0_QNR_VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0QNR")
            .field("t0_qnr_val", &format_args!("{}", self.t0_qnr_val().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0QNR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0qnr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0QNR_SPEC;
impl crate::RegisterSpec for T0QNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0qnr::R`](R) reader structure"]
impl crate::Readable for T0QNR_SPEC {}
#[doc = "`reset()` method sets T0QNR to value 0"]
impl crate::Resettable for T0QNR_SPEC {
    const RESET_VALUE: u32 = 0;
}
