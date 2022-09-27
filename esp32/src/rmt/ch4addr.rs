#[doc = "Register `CH4ADDR` reader"]
pub struct R(crate::R<CH4ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH4ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH4ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH4ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APB_MEM_ADDR` reader - The ram relative address in channel4 by apb fifo access"]
pub type APB_MEM_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The ram relative address in channel4 by apb fifo access"]
    #[inline(always)]
    pub fn apb_mem_addr(&self) -> APB_MEM_ADDR_R {
        APB_MEM_ADDR_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4addr](index.html) module"]
pub struct CH4ADDR_SPEC;
impl crate::RegisterSpec for CH4ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch4addr::R](R) reader structure"]
impl crate::Readable for CH4ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH4ADDR to value 0"]
impl crate::Resettable for CH4ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
