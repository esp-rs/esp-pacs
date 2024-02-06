#[doc = "Register `RESULT` reader"]
pub type R = crate::R<RESULT_SPEC>;
#[doc = "Field `OPERATION_RESULT` reader - The operation result bit of ECDSA Accelerator, only valid when ECDSA calculation is done."]
pub type OPERATION_RESULT_R = crate::BitReader;
#[doc = "Field `K_VALUE_WARNING` reader - The k value warning bit of ECDSA Accelerator, valid when k value is bigger than the curve order, then actually taken k = k mod n."]
pub type K_VALUE_WARNING_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The operation result bit of ECDSA Accelerator, only valid when ECDSA calculation is done."]
    #[inline(always)]
    pub fn operation_result(&self) -> OPERATION_RESULT_R {
        OPERATION_RESULT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The k value warning bit of ECDSA Accelerator, valid when k value is bigger than the curve order, then actually taken k = k mod n."]
    #[inline(always)]
    pub fn k_value_warning(&self) -> K_VALUE_WARNING_R {
        K_VALUE_WARNING_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESULT")
            .field(
                "operation_result",
                &format_args!("{}", self.operation_result().bit()),
            )
            .field(
                "k_value_warning",
                &format_args!("{}", self.k_value_warning().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RESULT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "ECDSA result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESULT_SPEC;
impl crate::RegisterSpec for RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result::R`](R) reader structure"]
impl crate::Readable for RESULT_SPEC {}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for RESULT_SPEC {
    const RESET_VALUE: u32 = 0;
}
