#[doc = "Register `INTR_MEM_START_ADDR` reader"]
pub type R = crate::R<INTR_MEM_START_ADDR_SPEC>;
#[doc = "Register `INTR_MEM_START_ADDR` writer"]
pub type W = crate::W<INTR_MEM_START_ADDR_SPEC>;
#[doc = "Field `ACCESS_INTR_MEM_START_ADDR` reader - The start address of accessible address space."]
pub type ACCESS_INTR_MEM_START_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ACCESS_INTR_MEM_START_ADDR` writer - The start address of accessible address space."]
pub type ACCESS_INTR_MEM_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The start address of accessible address space."]
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
                &format_args!("{}", self.access_intr_mem_start_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR_MEM_START_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The start address of accessible address space."]
    #[inline(always)]
    #[must_use]
    pub fn access_intr_mem_start_addr(
        &mut self,
    ) -> ACCESS_INTR_MEM_START_ADDR_W<INTR_MEM_START_ADDR_SPEC> {
        ACCESS_INTR_MEM_START_ADDR_W::new(self, 0)
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
#[doc = "The start address of accessible address space.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mem_start_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mem_start_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_MEM_START_ADDR_SPEC;
impl crate::RegisterSpec for INTR_MEM_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mem_start_addr::R`](R) reader structure"]
impl crate::Readable for INTR_MEM_START_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_mem_start_addr::W`](W) writer structure"]
impl crate::Writable for INTR_MEM_START_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_MEM_START_ADDR to value 0"]
impl crate::Resettable for INTR_MEM_START_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
