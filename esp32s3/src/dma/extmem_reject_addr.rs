#[doc = "Register `EXTMEM_REJECT_ADDR` reader"]
pub struct R(crate::R<EXTMEM_REJECT_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTMEM_REJECT_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTMEM_REJECT_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTMEM_REJECT_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXTMEM_REJECT_ADDR` reader - This register store the first address rejected by permission control when accessing external RAM."]
pub type EXTMEM_REJECT_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register store the first address rejected by permission control when accessing external RAM."]
    #[inline(always)]
    pub fn extmem_reject_addr(&self) -> EXTMEM_REJECT_ADDR_R {
        EXTMEM_REJECT_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTMEM_REJECT_ADDR")
            .field(
                "extmem_reject_addr",
                &format_args!("{}", self.extmem_reject_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXTMEM_REJECT_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Reject address accessing external RAM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmem_reject_addr](index.html) module"]
pub struct EXTMEM_REJECT_ADDR_SPEC;
impl crate::RegisterSpec for EXTMEM_REJECT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extmem_reject_addr::R](R) reader structure"]
impl crate::Readable for EXTMEM_REJECT_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXTMEM_REJECT_ADDR to value 0"]
impl crate::Resettable for EXTMEM_REJECT_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
