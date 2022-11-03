#[doc = "Register `CPU_PERI_TIMEOUT_ADDR` reader"]
pub struct R(crate::R<CPU_PERI_TIMEOUT_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PERI_TIMEOUT_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PERI_TIMEOUT_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PERI_TIMEOUT_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPU_PERI_TIMEOUT_ADDR` reader - Record the address information of abnormal access"]
pub type CPU_PERI_TIMEOUT_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Record the address information of abnormal access"]
    #[inline(always)]
    pub fn cpu_peri_timeout_addr(&self) -> CPU_PERI_TIMEOUT_ADDR_R {
        CPU_PERI_TIMEOUT_ADDR_R::new(self.bits)
    }
}
#[doc = "CPU_PERI_TIMEOUT_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_peri_timeout_addr](index.html) module"]
pub struct CPU_PERI_TIMEOUT_ADDR_SPEC;
impl crate::RegisterSpec for CPU_PERI_TIMEOUT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_peri_timeout_addr::R](R) reader structure"]
impl crate::Readable for CPU_PERI_TIMEOUT_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPU_PERI_TIMEOUT_ADDR to value 0"]
impl crate::Resettable for CPU_PERI_TIMEOUT_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
