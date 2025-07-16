#[doc = "Register `INTR_MEM_START_ADDR` reader"]
pub type R = crate::R<INTR_MEM_START_ADDR_SPEC>;
#[doc = "Register `INTR_MEM_START_ADDR` writer"]
pub type W = crate::W<INTR_MEM_START_ADDR_SPEC>;
#[doc = "Field `ACCESS_INTR_MEM_START_ADDR` reader - Configures the start address of accessible address space."]
pub type ACCESS_INTR_MEM_START_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ACCESS_INTR_MEM_START_ADDR` writer - Configures the start address of accessible address space."]
pub type ACCESS_INTR_MEM_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the start address of accessible address space."]
    #[inline(always)]
    pub fn access_intr_mem_start_addr(&self) -> ACCESS_INTR_MEM_START_ADDR_R {
        ACCESS_INTR_MEM_START_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_MEM_START_ADDR")
            .field(
                "access_intr_mem_start_addr",
                &self.access_intr_mem_start_addr(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the start address of accessible address space."]
    #[inline(always)]
    pub fn access_intr_mem_start_addr(
        &mut self,
    ) -> ACCESS_INTR_MEM_START_ADDR_W<INTR_MEM_START_ADDR_SPEC> {
        ACCESS_INTR_MEM_START_ADDR_W::new(self, 0)
    }
}
#[doc = "Accessible address space start address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_start_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_start_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_MEM_START_ADDR_SPEC;
impl crate::RegisterSpec for INTR_MEM_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mem_start_addr::R`](R) reader structure"]
impl crate::Readable for INTR_MEM_START_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_mem_start_addr::W`](W) writer structure"]
impl crate::Writable for INTR_MEM_START_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTR_MEM_START_ADDR to value 0"]
impl crate::Resettable for INTR_MEM_START_ADDR_SPEC {}
