#[doc = "Register `REGDMA_LINK_MAC_ADDR` reader"]
pub type R = crate::R<REGDMA_LINK_MAC_ADDR_SPEC>;
#[doc = "Register `REGDMA_LINK_MAC_ADDR` writer"]
pub type W = crate::W<REGDMA_LINK_MAC_ADDR_SPEC>;
#[doc = "Field `LINK_ADDR_MAC` reader - Link_mac_addr reg"]
pub type LINK_ADDR_MAC_R = crate::FieldReader<u32>;
#[doc = "Field `LINK_ADDR_MAC` writer - Link_mac_addr reg"]
pub type LINK_ADDR_MAC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Link_mac_addr reg"]
    #[inline(always)]
    pub fn link_addr_mac(&self) -> LINK_ADDR_MAC_R {
        LINK_ADDR_MAC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_LINK_MAC_ADDR")
            .field(
                "link_addr_mac",
                &format_args!("{}", self.link_addr_mac().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGDMA_LINK_MAC_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Link_mac_addr reg"]
    #[inline(always)]
    #[must_use]
    pub fn link_addr_mac(&mut self) -> LINK_ADDR_MAC_W<REGDMA_LINK_MAC_ADDR_SPEC> {
        LINK_ADDR_MAC_W::new(self, 0)
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
#[doc = "Link_mac_addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regdma_link_mac_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_link_mac_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGDMA_LINK_MAC_ADDR_SPEC;
impl crate::RegisterSpec for REGDMA_LINK_MAC_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regdma_link_mac_addr::R`](R) reader structure"]
impl crate::Readable for REGDMA_LINK_MAC_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`regdma_link_mac_addr::W`](W) writer structure"]
impl crate::Writable for REGDMA_LINK_MAC_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGDMA_LINK_MAC_ADDR to value 0"]
impl crate::Resettable for REGDMA_LINK_MAC_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
