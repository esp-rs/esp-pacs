///Register `_0_TXPKT_E_DSCR` reader
pub type R = crate::R<_0_TXPKT_E_DSCR_SPEC>;
///Register `_0_TXPKT_E_DSCR` writer
pub type W = crate::W<_0_TXPKT_E_DSCR_SPEC>;
///Field `SLC0_TX_PKT_E_DSCR_ADDR` reader -
pub type SLC0_TX_PKT_E_DSCR_ADDR_R = crate::FieldReader<u32>;
///Field `SLC0_TX_PKT_E_DSCR_ADDR` writer -
pub type SLC0_TX_PKT_E_DSCR_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn slc0_tx_pkt_e_dscr_addr(&self) -> SLC0_TX_PKT_E_DSCR_ADDR_R {
        SLC0_TX_PKT_E_DSCR_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_TXPKT_E_DSCR")
            .field("slc0_tx_pkt_e_dscr_addr", &self.slc0_tx_pkt_e_dscr_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    #[must_use]
    pub fn slc0_tx_pkt_e_dscr_addr(&mut self) -> SLC0_TX_PKT_E_DSCR_ADDR_W<_0_TXPKT_E_DSCR_SPEC> {
        SLC0_TX_PKT_E_DSCR_ADDR_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`_0_txpkt_e_dscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_txpkt_e_dscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct _0_TXPKT_E_DSCR_SPEC;
impl crate::RegisterSpec for _0_TXPKT_E_DSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`_0_txpkt_e_dscr::R`](R) reader structure
impl crate::Readable for _0_TXPKT_E_DSCR_SPEC {}
///`write(|w| ..)` method takes [`_0_txpkt_e_dscr::W`](W) writer structure
impl crate::Writable for _0_TXPKT_E_DSCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets _0_TXPKT_E_DSCR to value 0
impl crate::Resettable for _0_TXPKT_E_DSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
