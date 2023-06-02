#[doc = "Register `L2_CACHE_VADDR` reader"]
pub struct R(crate::R<L2_CACHE_VADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_VADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_VADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_VADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_VADDR` reader - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
pub type L2_CACHE_VADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
    #[inline(always)]
    pub fn l2_cache_vaddr(&self) -> L2_CACHE_VADDR_R {
        L2_CACHE_VADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_VADDR")
            .field(
                "l2_cache_vaddr",
                &format_args!("{}", self.l2_cache_vaddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_VADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Cache Vaddr register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_vaddr](index.html) module"]
pub struct L2_CACHE_VADDR_SPEC;
impl crate::RegisterSpec for L2_CACHE_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_vaddr::R](R) reader structure"]
impl crate::Readable for L2_CACHE_VADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_VADDR to value 0x4000_0000"]
impl crate::Resettable for L2_CACHE_VADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
