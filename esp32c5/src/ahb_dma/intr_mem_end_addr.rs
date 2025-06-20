#[doc = "Register `INTR_MEM_END_ADDR` reader"]
pub type R = crate::R<INTR_MEM_END_ADDR_SPEC>;
#[doc = "Register `INTR_MEM_END_ADDR` writer"]
pub type W = crate::W<INTR_MEM_END_ADDR_SPEC>;
#[doc = "Field `ACCESS_INTR_MEM_END_ADDR` reader - Configures the end address of accessible address space."]
pub type ACCESS_INTR_MEM_END_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ACCESS_INTR_MEM_END_ADDR` writer - Configures the end address of accessible address space."]
pub type ACCESS_INTR_MEM_END_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the end address of accessible address space."]
    #[inline(always)]
    pub fn access_intr_mem_end_addr(&self) -> ACCESS_INTR_MEM_END_ADDR_R {
        ACCESS_INTR_MEM_END_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_MEM_END_ADDR")
            .field("access_intr_mem_end_addr", &self.access_intr_mem_end_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the end address of accessible address space."]
    #[inline(always)]
    pub fn access_intr_mem_end_addr(
        &mut self,
    ) -> ACCESS_INTR_MEM_END_ADDR_W<INTR_MEM_END_ADDR_SPEC> {
        ACCESS_INTR_MEM_END_ADDR_W::new(self, 0)
    }
}
#[doc = "Accessible address space end address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_end_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_end_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_MEM_END_ADDR_SPEC;
impl crate::RegisterSpec for INTR_MEM_END_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mem_end_addr::R`](R) reader structure"]
impl crate::Readable for INTR_MEM_END_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_mem_end_addr::W`](W) writer structure"]
impl crate::Writable for INTR_MEM_END_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_MEM_END_ADDR to value 0xffff_ffff"]
impl crate::Resettable for INTR_MEM_END_ADDR_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
