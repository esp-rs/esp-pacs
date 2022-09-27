#[doc = "Register `LOG_MEM_WRITING_ADDR` reader"]
pub struct R(crate::R<LOG_MEM_WRITING_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOG_MEM_WRITING_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOG_MEM_WRITING_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOG_MEM_WRITING_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOG_MEM_WRITING_ADDR` reader - mem current addr, it means next writing addr"]
pub type LOG_MEM_WRITING_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - mem current addr, it means next writing addr"]
    #[inline(always)]
    pub fn log_mem_writing_addr(&self) -> LOG_MEM_WRITING_ADDR_R {
        LOG_MEM_WRITING_ADDR_R::new(self.bits)
    }
}
#[doc = "log mem addr status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [log_mem_writing_addr](index.html) module"]
pub struct LOG_MEM_WRITING_ADDR_SPEC;
impl crate::RegisterSpec for LOG_MEM_WRITING_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [log_mem_writing_addr::R](R) reader structure"]
impl crate::Readable for LOG_MEM_WRITING_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOG_MEM_WRITING_ADDR to value 0"]
impl crate::Resettable for LOG_MEM_WRITING_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
