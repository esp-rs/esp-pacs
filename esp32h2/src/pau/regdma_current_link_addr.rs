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
