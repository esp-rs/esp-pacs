#[doc = "Register `REGDMA_CURRENT_LINK_ADDR` reader"]
pub struct R(crate::R<REGDMA_CURRENT_LINK_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGDMA_CURRENT_LINK_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGDMA_CURRENT_LINK_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGDMA_CURRENT_LINK_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURRENT_LINK_ADDR` reader - current link addr reg"]
pub type CURRENT_LINK_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - current link addr reg"]
    #[inline(always)]
    pub fn current_link_addr(&self) -> CURRENT_LINK_ADDR_R {
        CURRENT_LINK_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_CURRENT_LINK_ADDR")
            .field(
                "current_link_addr",
                &format_args!("{}", self.current_link_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGDMA_CURRENT_LINK_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "current link addr\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regdma_current_link_addr](index.html) module"]
pub struct REGDMA_CURRENT_LINK_ADDR_SPEC;
impl crate::RegisterSpec for REGDMA_CURRENT_LINK_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regdma_current_link_addr::R](R) reader structure"]
impl crate::Readable for REGDMA_CURRENT_LINK_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REGDMA_CURRENT_LINK_ADDR to value 0"]
impl crate::Resettable for REGDMA_CURRENT_LINK_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
