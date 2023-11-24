#[doc = "Register `_0_TXPKT_E_DSCR` reader"]
pub type R = crate::R<_0_TXPKT_E_DSCR_SPEC>;
#[doc = "Register `_0_TXPKT_E_DSCR` writer"]
pub type W = crate::W<_0_TXPKT_E_DSCR_SPEC>;
#[doc = "Field `SLC0_TX_PKT_E_DSCR_ADDR` reader - "]
pub type SLC0_TX_PKT_E_DSCR_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SLC0_TX_PKT_E_DSCR_ADDR` writer - "]
pub type SLC0_TX_PKT_E_DSCR_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc0_tx_pkt_e_dscr_addr(&self) -> SLC0_TX_PKT_E_DSCR_ADDR_R {
        SLC0_TX_PKT_E_DSCR_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_TXPKT_E_DSCR")
            .field(
                "slc0_tx_pkt_e_dscr_addr",
                &format_args!("{}", self.slc0_tx_pkt_e_dscr_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_0_TXPKT_E_DSCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_pkt_e_dscr_addr(&mut self) -> SLC0_TX_PKT_E_DSCR_ADDR_W<_0_TXPKT_E_DSCR_SPEC> {
        SLC0_TX_PKT_E_DSCR_ADDR_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_txpkt_e_dscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_txpkt_e_dscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0_TXPKT_E_DSCR_SPEC;
impl crate::RegisterSpec for _0_TXPKT_E_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0_txpkt_e_dscr::R`](R) reader structure"]
impl crate::Readable for _0_TXPKT_E_DSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_0_txpkt_e_dscr::W`](W) writer structure"]
impl crate::Writable for _0_TXPKT_E_DSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _0_TXPKT_E_DSCR to value 0"]
impl crate::Resettable for _0_TXPKT_E_DSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
