#[doc = "Register `PRO_DCACHE_DBUG2` reader"]
pub struct R(crate::R<PRO_DCACHE_DBUG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DCACHE_DBUG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DCACHE_DBUG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DCACHE_DBUG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_CACHE_VADDR` reader - "]
pub type PRO_CACHE_VADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:26"]
    #[inline(always)]
    pub fn pro_cache_vaddr(&self) -> PRO_CACHE_VADDR_R {
        PRO_CACHE_VADDR_R::new((self.bits & 0x07ff_ffff) as u32)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dcache_dbug2](index.html) module"]
pub struct PRO_DCACHE_DBUG2_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_DBUG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dcache_dbug2::R](R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_DCACHE_DBUG2 to value 0"]
impl crate::Resettable for PRO_DCACHE_DBUG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
