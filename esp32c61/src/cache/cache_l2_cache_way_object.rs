#[doc = "Register `CACHE_L2_CACHE_WAY_OBJECT` reader"]
pub type R = crate::R<CACHE_L2_CACHE_WAY_OBJECT_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_WAY_OBJECT` reader - Set this bits to select which way of the tag-object will be accessed. 0: way0, 1: way1, 2: way2, 3: way3, ?, 7: way7."]
pub type CACHE_L2_CACHE_WAY_OBJECT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Set this bits to select which way of the tag-object will be accessed. 0: way0, 1: way1, 2: way2, 3: way3, ?, 7: way7."]
    #[inline(always)]
    pub fn cache_l2_cache_way_object(&self) -> CACHE_L2_CACHE_WAY_OBJECT_R {
        CACHE_L2_CACHE_WAY_OBJECT_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_WAY_OBJECT")
            .field(
                "cache_l2_cache_way_object",
                &self.cache_l2_cache_way_object(),
            )
            .finish()
    }
}
#[doc = "Cache Tag and Data memory way register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_way_object::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_WAY_OBJECT_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_WAY_OBJECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_way_object::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_WAY_OBJECT_SPEC {}
#[doc = "`reset()` method sets CACHE_L2_CACHE_WAY_OBJECT to value 0"]
impl crate::Resettable for CACHE_L2_CACHE_WAY_OBJECT_SPEC {}
