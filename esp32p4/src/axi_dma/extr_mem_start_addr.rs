#[doc = "Register `EXTR_MEM_START_ADDR` reader"]
pub type R = crate::R<EXTR_MEM_START_ADDR_SPEC>;
#[doc = "Register `EXTR_MEM_START_ADDR` writer"]
pub type W = crate::W<EXTR_MEM_START_ADDR_SPEC>;
#[doc = "Field `ACCESS_EXTR_MEM_START_ADDR` reader - The start address of accessible address space."]
pub type ACCESS_EXTR_MEM_START_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ACCESS_EXTR_MEM_START_ADDR` writer - The start address of accessible address space."]
pub type ACCESS_EXTR_MEM_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The start address of accessible address space."]
    #[inline(always)]
    pub fn access_extr_mem_start_addr(&self) -> ACCESS_EXTR_MEM_START_ADDR_R {
        ACCESS_EXTR_MEM_START_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTR_MEM_START_ADDR")
            .field(
                "access_extr_mem_start_addr",
                &self.access_extr_mem_start_addr(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The start address of accessible address space."]
    #[inline(always)]
    pub fn access_extr_mem_start_addr(
        &mut self,
    ) -> ACCESS_EXTR_MEM_START_ADDR_W<EXTR_MEM_START_ADDR_SPEC> {
        ACCESS_EXTR_MEM_START_ADDR_W::new(self, 0)
    }
}
#[doc = "The start address of accessible address space.\n\nYou can [`read`](crate::Reg::read) this register and get [`extr_mem_start_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extr_mem_start_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTR_MEM_START_ADDR_SPEC;
impl crate::RegisterSpec for EXTR_MEM_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extr_mem_start_addr::R`](R) reader structure"]
impl crate::Readable for EXTR_MEM_START_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extr_mem_start_addr::W`](W) writer structure"]
impl crate::Writable for EXTR_MEM_START_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTR_MEM_START_ADDR to value 0x3010_0000"]
impl crate::Resettable for EXTR_MEM_START_ADDR_SPEC {
    const RESET_VALUE: u32 = 0x3010_0000;
}
