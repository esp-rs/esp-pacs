#[doc = "Register `L1_UNALLOCATE_BUFFER_CLEAR` reader"]
pub type R = crate::R<L1_UNALLOCATE_BUFFER_CLEAR_SPEC>;
#[doc = "Register `L1_UNALLOCATE_BUFFER_CLEAR` writer"]
pub type W = crate::W<L1_UNALLOCATE_BUFFER_CLEAR_SPEC>;
#[doc = "Field `L1_ICACHE0_UNALLOC_CLR` reader - The bit is used to clear the unallocate request buffer of l1 icache0 where the unallocate request is responsed but not completed."]
pub type L1_ICACHE0_UNALLOC_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_UNALLOC_CLR` writer - The bit is used to clear the unallocate request buffer of l1 icache0 where the unallocate request is responsed but not completed."]
pub type L1_ICACHE0_UNALLOC_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_UNALLOC_CLR` reader - The bit is used to clear the unallocate request buffer of l1 icache1 where the unallocate request is responsed but not completed."]
pub type L1_ICACHE1_UNALLOC_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_UNALLOC_CLR` writer - The bit is used to clear the unallocate request buffer of l1 icache1 where the unallocate request is responsed but not completed."]
pub type L1_ICACHE1_UNALLOC_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_UNALLOC_CLR` reader - Reserved"]
pub type L1_ICACHE2_UNALLOC_CLR_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_UNALLOC_CLR` reader - Reserved"]
pub type L1_ICACHE3_UNALLOC_CLR_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_UNALLOC_CLR` reader - The bit is used to clear the unallocate request buffer of l1 dcache where the unallocate request is responsed but not completed."]
pub type L1_DCACHE_UNALLOC_CLR_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_UNALLOC_CLR` writer - The bit is used to clear the unallocate request buffer of l1 dcache where the unallocate request is responsed but not completed."]
pub type L1_DCACHE_UNALLOC_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to clear the unallocate request buffer of l1 icache0 where the unallocate request is responsed but not completed."]
    #[inline(always)]
    pub fn l1_icache0_unalloc_clr(&self) -> L1_ICACHE0_UNALLOC_CLR_R {
        L1_ICACHE0_UNALLOC_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to clear the unallocate request buffer of l1 icache1 where the unallocate request is responsed but not completed."]
    #[inline(always)]
    pub fn l1_icache1_unalloc_clr(&self) -> L1_ICACHE1_UNALLOC_CLR_R {
        L1_ICACHE1_UNALLOC_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_unalloc_clr(&self) -> L1_ICACHE2_UNALLOC_CLR_R {
        L1_ICACHE2_UNALLOC_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_unalloc_clr(&self) -> L1_ICACHE3_UNALLOC_CLR_R {
        L1_ICACHE3_UNALLOC_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to clear the unallocate request buffer of l1 dcache where the unallocate request is responsed but not completed."]
    #[inline(always)]
    pub fn l1_dcache_unalloc_clr(&self) -> L1_DCACHE_UNALLOC_CLR_R {
        L1_DCACHE_UNALLOC_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_UNALLOCATE_BUFFER_CLEAR")
            .field("l1_icache0_unalloc_clr", &self.l1_icache0_unalloc_clr())
            .field("l1_icache1_unalloc_clr", &self.l1_icache1_unalloc_clr())
            .field("l1_icache2_unalloc_clr", &self.l1_icache2_unalloc_clr())
            .field("l1_icache3_unalloc_clr", &self.l1_icache3_unalloc_clr())
            .field("l1_dcache_unalloc_clr", &self.l1_dcache_unalloc_clr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to clear the unallocate request buffer of l1 icache0 where the unallocate request is responsed but not completed."]
    #[inline(always)]
    pub fn l1_icache0_unalloc_clr(
        &mut self,
    ) -> L1_ICACHE0_UNALLOC_CLR_W<L1_UNALLOCATE_BUFFER_CLEAR_SPEC> {
        L1_ICACHE0_UNALLOC_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to clear the unallocate request buffer of l1 icache1 where the unallocate request is responsed but not completed."]
    #[inline(always)]
    pub fn l1_icache1_unalloc_clr(
        &mut self,
    ) -> L1_ICACHE1_UNALLOC_CLR_W<L1_UNALLOCATE_BUFFER_CLEAR_SPEC> {
        L1_ICACHE1_UNALLOC_CLR_W::new(self, 1)
    }
    #[doc = "Bit 4 - The bit is used to clear the unallocate request buffer of l1 dcache where the unallocate request is responsed but not completed."]
    #[inline(always)]
    pub fn l1_dcache_unalloc_clr(
        &mut self,
    ) -> L1_DCACHE_UNALLOC_CLR_W<L1_UNALLOCATE_BUFFER_CLEAR_SPEC> {
        L1_DCACHE_UNALLOC_CLR_W::new(self, 4)
    }
}
#[doc = "Unallocate request buffer clear registers\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_unallocate_buffer_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_unallocate_buffer_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_UNALLOCATE_BUFFER_CLEAR_SPEC;
impl crate::RegisterSpec for L1_UNALLOCATE_BUFFER_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_unallocate_buffer_clear::R`](R) reader structure"]
impl crate::Readable for L1_UNALLOCATE_BUFFER_CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_unallocate_buffer_clear::W`](W) writer structure"]
impl crate::Writable for L1_UNALLOCATE_BUFFER_CLEAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1_UNALLOCATE_BUFFER_CLEAR to value 0"]
impl crate::Resettable for L1_UNALLOCATE_BUFFER_CLEAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
