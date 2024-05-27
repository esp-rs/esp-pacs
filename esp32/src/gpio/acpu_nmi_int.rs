#[doc = "Register `ACPU_NMI_INT` reader"]
pub type R = crate::R<ACPU_NMI_INT_SPEC>;
#[doc = "Field `APPCPU_NMI_INT` reader - GPIO0~31 APP CPU non-maskable interrupt status"]
pub type APPCPU_NMI_INT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 APP CPU non-maskable interrupt status"]
    #[inline(always)]
    pub fn appcpu_nmi_int(&self) -> APPCPU_NMI_INT_R {
        APPCPU_NMI_INT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACPU_NMI_INT")
            .field("appcpu_nmi_int", &self.appcpu_nmi_int())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acpu_nmi_int::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACPU_NMI_INT_SPEC;
impl crate::RegisterSpec for ACPU_NMI_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acpu_nmi_int::R`](R) reader structure"]
impl crate::Readable for ACPU_NMI_INT_SPEC {}
#[doc = "`reset()` method sets ACPU_NMI_INT to value 0"]
impl crate::Resettable for ACPU_NMI_INT_SPEC {
    const RESET_VALUE: u32 = 0;
}
