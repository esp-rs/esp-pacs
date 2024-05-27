///Register `L2_CACHE_WRAP_AROUND_CTRL` reader
pub type R = crate::R<L2_CACHE_WRAP_AROUND_CTRL_SPEC>;
///Field `L2_CACHE_WRAP` reader - Set this bit as 1 to enable L2-Cache wrap around mode.
pub type L2_CACHE_WRAP_R = crate::BitReader;
impl R {
    ///Bit 5 - Set this bit as 1 to enable L2-Cache wrap around mode.
    #[inline(always)]
    pub fn l2_cache_wrap(&self) -> L2_CACHE_WRAP_R {
        L2_CACHE_WRAP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_WRAP_AROUND_CTRL")
            .field("l2_cache_wrap", &self.l2_cache_wrap())
            .finish()
    }
}
/**Cache wrap around control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_wrap_around_ctrl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_CACHE_WRAP_AROUND_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_WRAP_AROUND_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_cache_wrap_around_ctrl::R`](R) reader structure
impl crate::Readable for L2_CACHE_WRAP_AROUND_CTRL_SPEC {}
///`reset()` method sets L2_CACHE_WRAP_AROUND_CTRL to value 0
impl crate::Resettable for L2_CACHE_WRAP_AROUND_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
