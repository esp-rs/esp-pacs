#[doc = "Register `L1_CACHE_VADDR` reader"]
pub type R = crate::R<L1_CACHE_VADDR_SPEC>;
#[doc = "Register `L1_CACHE_VADDR` writer"]
pub type W = crate::W<L1_CACHE_VADDR_SPEC>;
#[doc = "Field `L1_CACHE_VADDR` reader - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
pub type L1_CACHE_VADDR_R = crate::FieldReader<u32>;
#[doc = "Field `L1_CACHE_VADDR` writer - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
pub type L1_CACHE_VADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
    #[inline(always)]
    pub fn l1_cache_vaddr(&self) -> L1_CACHE_VADDR_R {
        L1_CACHE_VADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_VADDR")
            .field(
                "l1_cache_vaddr",
                &format_args!("{}", self.l1_cache_vaddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_VADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits stores the virtual address which will decide where inside the specified tag memory object will be accessed."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_vaddr(&mut self) -> L1_CACHE_VADDR_W<L1_CACHE_VADDR_SPEC> {
        L1_CACHE_VADDR_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cache Vaddr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_vaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_vaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_VADDR_SPEC;
impl crate::RegisterSpec for L1_CACHE_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_vaddr::R`](R) reader structure"]
impl crate::Readable for L1_CACHE_VADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_vaddr::W`](W) writer structure"]
impl crate::Writable for L1_CACHE_VADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1_CACHE_VADDR to value 0x4000_0000"]
impl crate::Resettable for L1_CACHE_VADDR_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
