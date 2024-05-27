///Register `L2_CACHE_OBJECT_CTRL` reader
pub type R = crate::R<L2_CACHE_OBJECT_CTRL_SPEC>;
///Field `L2_CACHE_TAG_OBJECT` reader - Set this bit to set L2-Cache tag memory as object. This bit should be onehot with the others fields inside this register.
pub type L2_CACHE_TAG_OBJECT_R = crate::BitReader;
///Field `L2_CACHE_MEM_OBJECT` reader - Set this bit to set L2-Cache data memory as object. This bit should be onehot with the others fields inside this register.
pub type L2_CACHE_MEM_OBJECT_R = crate::BitReader;
impl R {
    ///Bit 5 - Set this bit to set L2-Cache tag memory as object. This bit should be onehot with the others fields inside this register.
    #[inline(always)]
    pub fn l2_cache_tag_object(&self) -> L2_CACHE_TAG_OBJECT_R {
        L2_CACHE_TAG_OBJECT_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 11 - Set this bit to set L2-Cache data memory as object. This bit should be onehot with the others fields inside this register.
    #[inline(always)]
    pub fn l2_cache_mem_object(&self) -> L2_CACHE_MEM_OBJECT_R {
        L2_CACHE_MEM_OBJECT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_OBJECT_CTRL")
            .field("l2_cache_tag_object", &self.l2_cache_tag_object())
            .field("l2_cache_mem_object", &self.l2_cache_mem_object())
            .finish()
    }
}
/**Cache Tag and Data memory Object control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_object_ctrl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_CACHE_OBJECT_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_OBJECT_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_cache_object_ctrl::R`](R) reader structure
impl crate::Readable for L2_CACHE_OBJECT_CTRL_SPEC {}
///`reset()` method sets L2_CACHE_OBJECT_CTRL to value 0
impl crate::Resettable for L2_CACHE_OBJECT_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
