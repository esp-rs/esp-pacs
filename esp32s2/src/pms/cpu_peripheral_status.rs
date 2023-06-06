#[doc = "Register `CPU_PERIPHERAL_STATUS` reader"]
pub struct R(crate::R<CPU_PERIPHERAL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PERIPHERAL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PERIPHERAL_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PERIPHERAL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPU_PERI_BYTE_ERROR_ADDR` reader - Record the illegitimate address of CPU peripheral."]
pub type CPU_PERI_BYTE_ERROR_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Record the illegitimate address of CPU peripheral."]
    #[inline(always)]
    pub fn cpu_peri_byte_error_addr(&self) -> CPU_PERI_BYTE_ERROR_ADDR_R {
        CPU_PERI_BYTE_ERROR_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PERIPHERAL_STATUS")
            .field(
                "cpu_peri_byte_error_addr",
                &format_args!("{}", self.cpu_peri_byte_error_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_PERIPHERAL_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "PeribBus1 peripheral access status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_peripheral_status](index.html) module"]
pub struct CPU_PERIPHERAL_STATUS_SPEC;
impl crate::RegisterSpec for CPU_PERIPHERAL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_peripheral_status::R](R) reader structure"]
impl crate::Readable for CPU_PERIPHERAL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPU_PERIPHERAL_STATUS to value 0"]
impl crate::Resettable for CPU_PERIPHERAL_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
