#[doc = "Register `L2_CACHE_WAY_OBJECT` reader"]
pub struct R(crate::R<L2_CACHE_WAY_OBJECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_WAY_OBJECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_WAY_OBJECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_WAY_OBJECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_WAY_OBJECT` reader - Set this bits to select which way of the tag-object will be accessed. 0: way0, 1: way1, 2: way2, 3: way3, ?, 7: way7."]
pub type L2_CACHE_WAY_OBJECT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Set this bits to select which way of the tag-object will be accessed. 0: way0, 1: way1, 2: way2, 3: way3, ?, 7: way7."]
    #[inline(always)]
    pub fn l2_cache_way_object(&self) -> L2_CACHE_WAY_OBJECT_R {
        L2_CACHE_WAY_OBJECT_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_WAY_OBJECT")
            .field(
                "l2_cache_way_object",
                &format_args!("{}", self.l2_cache_way_object().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_WAY_OBJECT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Cache Tag and Data memory way register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_way_object](index.html) module"]
pub struct L2_CACHE_WAY_OBJECT_SPEC;
impl crate::RegisterSpec for L2_CACHE_WAY_OBJECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_way_object::R](R) reader structure"]
impl crate::Readable for L2_CACHE_WAY_OBJECT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_WAY_OBJECT to value 0"]
impl crate::Resettable for L2_CACHE_WAY_OBJECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
