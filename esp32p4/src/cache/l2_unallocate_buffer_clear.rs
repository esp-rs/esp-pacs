#[doc = "Register `L2_UNALLOCATE_BUFFER_CLEAR` reader"]
pub type R = crate::R<L2_UNALLOCATE_BUFFER_CLEAR_SPEC>;
#[doc = "Register `L2_UNALLOCATE_BUFFER_CLEAR` writer"]
pub type W = crate::W<L2_UNALLOCATE_BUFFER_CLEAR_SPEC>;
#[doc = "Field `L2_CACHE_UNALLOC_CLR` reader - The bit is used to clear the unallocate request buffer of l2 icache where the unallocate request is responsed but not completed."]
pub type L2_CACHE_UNALLOC_CLR_R = crate::BitReader;
#[doc = "Field `L2_CACHE_UNALLOC_CLR` writer - The bit is used to clear the unallocate request buffer of l2 icache where the unallocate request is responsed but not completed."]
pub type L2_CACHE_UNALLOC_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("l2_cache_unalloc_clr", &self.l2_cache_unalloc_clr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - The bit is used to clear the unallocate request buffer of l2 icache where the unallocate request is responsed but not completed."]
    #[inline(always)]
    pub fn l2_cache_unalloc_clr(
        &mut self,
    ) -> L2_CACHE_UNALLOC_CLR_W<'_, L2_UNALLOCATE_BUFFER_CLEAR_SPEC> {
        L2_CACHE_UNALLOC_CLR_W::new(self, 5)
    }
}
#[doc = "Unallocate request buffer clear registers\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_unallocate_buffer_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_unallocate_buffer_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_UNALLOCATE_BUFFER_CLEAR_SPEC;
impl crate::RegisterSpec for L2_UNALLOCATE_BUFFER_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_unallocate_buffer_clear::R`](R) reader structure"]
impl crate::Readable for L2_UNALLOCATE_BUFFER_CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_unallocate_buffer_clear::W`](W) writer structure"]
impl crate::Writable for L2_UNALLOCATE_BUFFER_CLEAR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_UNALLOCATE_BUFFER_CLEAR to value 0"]
impl crate::Resettable for L2_UNALLOCATE_BUFFER_CLEAR_SPEC {}
