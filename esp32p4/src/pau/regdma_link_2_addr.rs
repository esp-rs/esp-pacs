#[doc = "Register `REGDMA_LINK_2_ADDR` reader"]
pub type R = crate::R<REGDMA_LINK_2_ADDR_SPEC>;
#[doc = "Register `REGDMA_LINK_2_ADDR` writer"]
pub type W = crate::W<REGDMA_LINK_2_ADDR_SPEC>;
#[doc = "Field `LINK_ADDR_2` reader - Link_2_addr reg"]
pub type LINK_ADDR_2_R = crate::FieldReader<u32>;
#[doc = "Field `LINK_ADDR_2` writer - Link_2_addr reg"]
pub type LINK_ADDR_2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Link_2_addr reg"]
    #[inline(always)]
    pub fn link_addr_2(&self) -> LINK_ADDR_2_R {
        LINK_ADDR_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_LINK_2_ADDR")
            .field("link_addr_2", &self.link_addr_2().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGDMA_LINK_2_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Link_2_addr reg"]
    #[inline(always)]
    #[must_use]
    pub fn link_addr_2(&mut self) -> LINK_ADDR_2_W<REGDMA_LINK_2_ADDR_SPEC> {
        LINK_ADDR_2_W::new(self, 0)
    }
}
#[doc = "Link_2_addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regdma_link_2_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_link_2_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGDMA_LINK_2_ADDR_SPEC;
impl crate::RegisterSpec for REGDMA_LINK_2_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regdma_link_2_addr::R`](R) reader structure"]
impl crate::Readable for REGDMA_LINK_2_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`regdma_link_2_addr::W`](W) writer structure"]
impl crate::Writable for REGDMA_LINK_2_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGDMA_LINK_2_ADDR to value 0"]
impl crate::Resettable for REGDMA_LINK_2_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
