#[doc = "Register `CACHE_L2_CACHE_OBJECT_CTRL` reader"]
pub type R = crate::R<CACHE_L2_CACHE_OBJECT_CTRL_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_TAG_OBJECT` reader - Set this bit to set L2-Cache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type CACHE_L2_CACHE_TAG_OBJECT_R = crate::BitReader;
#[doc = "Field `CACHE_L2_CACHE_MEM_OBJECT` reader - Set this bit to set L2-Cache data memory as object. This bit should be onehot with the others fields inside this register."]
pub type CACHE_L2_CACHE_MEM_OBJECT_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - Set this bit to set L2-Cache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn cache_l2_cache_tag_object(&self) -> CACHE_L2_CACHE_TAG_OBJECT_R {
        CACHE_L2_CACHE_TAG_OBJECT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to set L2-Cache data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn cache_l2_cache_mem_object(&self) -> CACHE_L2_CACHE_MEM_OBJECT_R {
        CACHE_L2_CACHE_MEM_OBJECT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_OBJECT_CTRL")
            .field(
                "cache_l2_cache_tag_object",
                &self.cache_l2_cache_tag_object(),
            )
            .field(
                "cache_l2_cache_mem_object",
                &self.cache_l2_cache_mem_object(),
            )
            .finish()
    }
}
#[doc = "Cache Tag and Data memory Object control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_object_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_OBJECT_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_OBJECT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_object_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_OBJECT_CTRL_SPEC {}
#[doc = "`reset()` method sets CACHE_L2_CACHE_OBJECT_CTRL to value 0"]
impl crate::Resettable for CACHE_L2_CACHE_OBJECT_CTRL_SPEC {}
