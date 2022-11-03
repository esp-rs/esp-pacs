#[doc = "Register `CORE0_CPU_INT_EIP_STATUS` reader"]
pub struct R(crate::R<CORE0_CPU_INT_EIP_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE0_CPU_INT_EIP_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE0_CPU_INT_EIP_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE0_CPU_INT_EIP_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE0_CPU_INT_EIP_STATUS` reader - Need add description"]
pub type CORE0_CPU_INT_EIP_STATUS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Need add description"]
    #[inline(always)]
    pub fn core0_cpu_int_eip_status(&self) -> CORE0_CPU_INT_EIP_STATUS_R {
        CORE0_CPU_INT_EIP_STATUS_R::new(self.bits)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core0_cpu_int_eip_status](index.html) module"]
pub struct CORE0_CPU_INT_EIP_STATUS_SPEC;
impl crate::RegisterSpec for CORE0_CPU_INT_EIP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core0_cpu_int_eip_status::R](R) reader structure"]
impl crate::Readable for CORE0_CPU_INT_EIP_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE0_CPU_INT_EIP_STATUS to value 0"]
impl crate::Resettable for CORE0_CPU_INT_EIP_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
