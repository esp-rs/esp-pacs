#[doc = "Register `L2_CACHE_MISS_ACCESS_CTRL` reader"]
pub type R = crate::R<L2_CACHE_MISS_ACCESS_CTRL_SPEC>;
#[doc = "Field `L2_CACHE_MISS_DISABLE_ACCESS` reader - Set this bit as 1 to disable early restart of L2-Cache"]
pub type L2_CACHE_MISS_DISABLE_ACCESS_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - Set this bit as 1 to disable early restart of L2-Cache"]
    #[inline(always)]
    pub fn l2_cache_miss_disable_access(&self) -> L2_CACHE_MISS_DISABLE_ACCESS_R {
        L2_CACHE_MISS_DISABLE_ACCESS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_MISS_ACCESS_CTRL")
            .field(
                "l2_cache_miss_disable_access",
                &self.l2_cache_miss_disable_access(),
            )
            .finish()
    }
}
#[doc = "Cache wrap around control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_miss_access_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_MISS_ACCESS_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_MISS_ACCESS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_miss_access_ctrl::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_MISS_ACCESS_CTRL_SPEC {}
#[doc = "`reset()` method sets L2_CACHE_MISS_ACCESS_CTRL to value 0"]
impl crate::Resettable for L2_CACHE_MISS_ACCESS_CTRL_SPEC {}
