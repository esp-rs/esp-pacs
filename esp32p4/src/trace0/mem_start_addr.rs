#[doc = "Register `MEM_START_ADDR` reader"]
pub type R = crate::R<MEM_START_ADDR_SPEC>;
#[doc = "Register `MEM_START_ADDR` writer"]
pub type W = crate::W<MEM_START_ADDR_SPEC>;
#[doc = "Field `MEM_START_ADDR` reader - The start address of trace memory"]
pub type MEM_START_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `MEM_START_ADDR` writer - The start address of trace memory"]
pub type MEM_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The start address of trace memory"]
    #[inline(always)]
    pub fn mem_start_addr(&self) -> MEM_START_ADDR_R {
        MEM_START_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_START_ADDR")
            .field("mem_start_addr", &self.mem_start_addr().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_START_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The start address of trace memory"]
    #[inline(always)]
    #[must_use]
    pub fn mem_start_addr(&mut self) -> MEM_START_ADDR_W<MEM_START_ADDR_SPEC> {
        MEM_START_ADDR_W::new(self, 0)
    }
}
#[doc = "mem start addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_start_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_start_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_START_ADDR_SPEC;
impl crate::RegisterSpec for MEM_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_start_addr::R`](R) reader structure"]
impl crate::Readable for MEM_START_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_start_addr::W`](W) writer structure"]
impl crate::Writable for MEM_START_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_START_ADDR to value 0"]
impl crate::Resettable for MEM_START_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
