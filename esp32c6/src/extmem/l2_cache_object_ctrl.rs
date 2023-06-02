#[doc = "Register `L2_CACHE_OBJECT_CTRL` reader"]
pub struct R(crate::R<L2_CACHE_OBJECT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_OBJECT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_OBJECT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_OBJECT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_TAG_OBJECT` reader - Set this bit to set L2-Cache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type L2_CACHE_TAG_OBJECT_R = crate::BitReader;
#[doc = "Field `L2_CACHE_MEM_OBJECT` reader - Set this bit to set L2-Cache data memory as object. This bit should be onehot with the others fields inside this register."]
pub type L2_CACHE_MEM_OBJECT_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - Set this bit to set L2-Cache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l2_cache_tag_object(&self) -> L2_CACHE_TAG_OBJECT_R {
        L2_CACHE_TAG_OBJECT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to set L2-Cache data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l2_cache_mem_object(&self) -> L2_CACHE_MEM_OBJECT_R {
        L2_CACHE_MEM_OBJECT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_OBJECT_CTRL")
            .field(
                "l2_cache_tag_object",
                &format_args!("{}", self.l2_cache_tag_object().bit()),
            )
            .field(
                "l2_cache_mem_object",
                &format_args!("{}", self.l2_cache_mem_object().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_OBJECT_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Cache Tag and Data memory Object control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_object_ctrl](index.html) module"]
pub struct L2_CACHE_OBJECT_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_OBJECT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_object_ctrl::R](R) reader structure"]
impl crate::Readable for L2_CACHE_OBJECT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_OBJECT_CTRL to value 0"]
impl crate::Resettable for L2_CACHE_OBJECT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
