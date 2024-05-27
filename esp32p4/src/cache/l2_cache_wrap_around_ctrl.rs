///Register `L2_CACHE_WRAP_AROUND_CTRL` reader
pub type R = crate::R<L2_CACHE_WRAP_AROUND_CTRL_SPEC>;
///Register `L2_CACHE_WRAP_AROUND_CTRL` writer
pub type W = crate::W<L2_CACHE_WRAP_AROUND_CTRL_SPEC>;
///Field `L2_CACHE_WRAP` reader - Set this bit as 1 to enable L2-Cache wrap around mode.
pub type L2_CACHE_WRAP_R = crate::BitReader;
///Field `L2_CACHE_WRAP` writer - Set this bit as 1 to enable L2-Cache wrap around mode.
pub type L2_CACHE_WRAP_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    ///Bit 5 - Set this bit as 1 to enable L2-Cache wrap around mode.
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_wrap(&mut self) -> L2_CACHE_WRAP_W<L2_CACHE_WRAP_AROUND_CTRL_SPEC> {
        L2_CACHE_WRAP_W::new(self, 5)
    }
}
/**Cache wrap around control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_wrap_around_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_wrap_around_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_CACHE_WRAP_AROUND_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_WRAP_AROUND_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_cache_wrap_around_ctrl::R`](R) reader structure
impl crate::Readable for L2_CACHE_WRAP_AROUND_CTRL_SPEC {}
///`write(|w| ..)` method takes [`l2_cache_wrap_around_ctrl::W`](W) writer structure
impl crate::Writable for L2_CACHE_WRAP_AROUND_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_CACHE_WRAP_AROUND_CTRL to value 0
impl crate::Resettable for L2_CACHE_WRAP_AROUND_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
