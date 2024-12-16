#[doc = "Register `L2_CACHE_ACCESS_ATTR_CTRL` reader"]
pub type R = crate::R<L2_CACHE_ACCESS_ATTR_CTRL_SPEC>;
#[doc = "Register `L2_CACHE_ACCESS_ATTR_CTRL` writer"]
pub type W = crate::W<L2_CACHE_ACCESS_ATTR_CTRL_SPEC>;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_CC` reader - Set this bit to force the request to l2 cache with cacheable attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of cacheable and non-cacheable."]
pub type L2_CACHE_ACCESS_FORCE_CC_R = crate::BitReader;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_CC` writer - Set this bit to force the request to l2 cache with cacheable attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of cacheable and non-cacheable."]
pub type L2_CACHE_ACCESS_FORCE_CC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_WB` reader - Set this bit to force the request to l2 cache with write-back attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-back and write-through."]
pub type L2_CACHE_ACCESS_FORCE_WB_R = crate::BitReader;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_WB` writer - Set this bit to force the request to l2 cache with write-back attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-back and write-through."]
pub type L2_CACHE_ACCESS_FORCE_WB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_WMA` reader - Set this bit to force the request to l2 cache with write-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-miss-allocate and write-miss-no-allocate."]
pub type L2_CACHE_ACCESS_FORCE_WMA_R = crate::BitReader;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_WMA` writer - Set this bit to force the request to l2 cache with write-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-miss-allocate and write-miss-no-allocate."]
pub type L2_CACHE_ACCESS_FORCE_WMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_RMA` reader - Set this bit to force the request to l2 cache with read-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of read-miss-allocate and read-miss-no-allocate."]
pub type L2_CACHE_ACCESS_FORCE_RMA_R = crate::BitReader;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_RMA` writer - Set this bit to force the request to l2 cache with read-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of read-miss-allocate and read-miss-no-allocate."]
pub type L2_CACHE_ACCESS_FORCE_RMA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to force the request to l2 cache with cacheable attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of cacheable and non-cacheable."]
    #[inline(always)]
    pub fn l2_cache_access_force_cc(&self) -> L2_CACHE_ACCESS_FORCE_CC_R {
        L2_CACHE_ACCESS_FORCE_CC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force the request to l2 cache with write-back attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-back and write-through."]
    #[inline(always)]
    pub fn l2_cache_access_force_wb(&self) -> L2_CACHE_ACCESS_FORCE_WB_R {
        L2_CACHE_ACCESS_FORCE_WB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force the request to l2 cache with write-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-miss-allocate and write-miss-no-allocate."]
    #[inline(always)]
    pub fn l2_cache_access_force_wma(&self) -> L2_CACHE_ACCESS_FORCE_WMA_R {
        L2_CACHE_ACCESS_FORCE_WMA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force the request to l2 cache with read-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of read-miss-allocate and read-miss-no-allocate."]
    #[inline(always)]
    pub fn l2_cache_access_force_rma(&self) -> L2_CACHE_ACCESS_FORCE_RMA_R {
        L2_CACHE_ACCESS_FORCE_RMA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_ACCESS_ATTR_CTRL")
            .field("l2_cache_access_force_cc", &self.l2_cache_access_force_cc())
            .field("l2_cache_access_force_wb", &self.l2_cache_access_force_wb())
            .field(
                "l2_cache_access_force_wma",
                &self.l2_cache_access_force_wma(),
            )
            .field(
                "l2_cache_access_force_rma",
                &self.l2_cache_access_force_rma(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to force the request to l2 cache with cacheable attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of cacheable and non-cacheable."]
    #[inline(always)]
    pub fn l2_cache_access_force_cc(
        &mut self,
    ) -> L2_CACHE_ACCESS_FORCE_CC_W<L2_CACHE_ACCESS_ATTR_CTRL_SPEC> {
        L2_CACHE_ACCESS_FORCE_CC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to force the request to l2 cache with write-back attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-back and write-through."]
    #[inline(always)]
    pub fn l2_cache_access_force_wb(
        &mut self,
    ) -> L2_CACHE_ACCESS_FORCE_WB_W<L2_CACHE_ACCESS_ATTR_CTRL_SPEC> {
        L2_CACHE_ACCESS_FORCE_WB_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force the request to l2 cache with write-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-miss-allocate and write-miss-no-allocate."]
    #[inline(always)]
    pub fn l2_cache_access_force_wma(
        &mut self,
    ) -> L2_CACHE_ACCESS_FORCE_WMA_W<L2_CACHE_ACCESS_ATTR_CTRL_SPEC> {
        L2_CACHE_ACCESS_FORCE_WMA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to force the request to l2 cache with read-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of read-miss-allocate and read-miss-no-allocate."]
    #[inline(always)]
    pub fn l2_cache_access_force_rma(
        &mut self,
    ) -> L2_CACHE_ACCESS_FORCE_RMA_W<L2_CACHE_ACCESS_ATTR_CTRL_SPEC> {
        L2_CACHE_ACCESS_FORCE_RMA_W::new(self, 3)
    }
}
#[doc = "L2 cache access attribute control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_access_attr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_access_attr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_ACCESS_ATTR_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACCESS_ATTR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_access_attr_ctrl::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_ACCESS_ATTR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_access_attr_ctrl::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_ACCESS_ATTR_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_CACHE_ACCESS_ATTR_CTRL to value 0x0f"]
impl crate::Resettable for L2_CACHE_ACCESS_ATTR_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
