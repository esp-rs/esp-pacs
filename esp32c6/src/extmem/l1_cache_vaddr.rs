///Register `L1_CACHE_VADDR` reader
pub type R = crate::R<L1_CACHE_VADDR_SPEC>;
///Register `L1_CACHE_VADDR` writer
pub type W = crate::W<L1_CACHE_VADDR_SPEC>;
///Field `L1_CACHE_VADDR` reader - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed.
pub type L1_CACHE_VADDR_R = crate::FieldReader<u32>;
///Field `L1_CACHE_VADDR` writer - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed.
pub type L1_CACHE_VADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed.
    #[inline(always)]
    pub fn l1_cache_vaddr(&self) -> L1_CACHE_VADDR_R {
        L1_CACHE_VADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_VADDR")
            .field("l1_cache_vaddr", &self.l1_cache_vaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed.
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_vaddr(&mut self) -> L1_CACHE_VADDR_W<L1_CACHE_VADDR_SPEC> {
        L1_CACHE_VADDR_W::new(self, 0)
    }
}
/**Cache Vaddr register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_vaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_vaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_CACHE_VADDR_SPEC;
impl crate::RegisterSpec for L1_CACHE_VADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_cache_vaddr::R`](R) reader structure
impl crate::Readable for L1_CACHE_VADDR_SPEC {}
///`write(|w| ..)` method takes [`l1_cache_vaddr::W`](W) writer structure
impl crate::Writable for L1_CACHE_VADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L1_CACHE_VADDR to value 0x4000_0000
impl crate::Resettable for L1_CACHE_VADDR_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
