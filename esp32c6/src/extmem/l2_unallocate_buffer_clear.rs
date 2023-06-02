#[doc = "Register `L2_UNALLOCATE_BUFFER_CLEAR` reader"]
pub struct R(crate::R<L2_UNALLOCATE_BUFFER_CLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_UNALLOCATE_BUFFER_CLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_UNALLOCATE_BUFFER_CLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_UNALLOCATE_BUFFER_CLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_UNALLOC_CLR` reader - The bit is used to clear the unallocate request buffer of l2 icache where the unallocate request is responsed but not completed."]
pub type L2_CACHE_UNALLOC_CLR_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - The bit is used to clear the unallocate request buffer of l2 icache where the unallocate request is responsed but not completed."]
    #[inline(always)]
    pub fn l2_cache_unalloc_clr(&self) -> L2_CACHE_UNALLOC_CLR_R {
        L2_CACHE_UNALLOC_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_UNALLOCATE_BUFFER_CLEAR")
            .field(
                "l2_cache_unalloc_clr",
                &format_args!("{}", self.l2_cache_unalloc_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_UNALLOCATE_BUFFER_CLEAR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Unallocate request buffer clear registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_unallocate_buffer_clear](index.html) module"]
pub struct L2_UNALLOCATE_BUFFER_CLEAR_SPEC;
impl crate::RegisterSpec for L2_UNALLOCATE_BUFFER_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_unallocate_buffer_clear::R](R) reader structure"]
impl crate::Readable for L2_UNALLOCATE_BUFFER_CLEAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_UNALLOCATE_BUFFER_CLEAR to value 0"]
impl crate::Resettable for L2_UNALLOCATE_BUFFER_CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
