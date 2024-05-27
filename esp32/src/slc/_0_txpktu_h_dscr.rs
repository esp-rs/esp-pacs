#[doc = "Register `_0_TXPKTU_H_DSCR` reader"]
pub type R = crate::R<_0_TXPKTU_H_DSCR_SPEC>;
#[doc = "Field `SLC0_TX_PKT_START_DSCR_ADDR` reader - "]
pub type SLC0_TX_PKT_START_DSCR_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc0_tx_pkt_start_dscr_addr(&self) -> SLC0_TX_PKT_START_DSCR_ADDR_R {
        SLC0_TX_PKT_START_DSCR_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_TXPKTU_H_DSCR")
            .field(
                "slc0_tx_pkt_start_dscr_addr",
                &self.slc0_tx_pkt_start_dscr_addr(),
            )
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_txpktu_h_dscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0_TXPKTU_H_DSCR_SPEC;
impl crate::RegisterSpec for _0_TXPKTU_H_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0_txpktu_h_dscr::R`](R) reader structure"]
impl crate::Readable for _0_TXPKTU_H_DSCR_SPEC {}
#[doc = "`reset()` method sets _0_TXPKTU_H_DSCR to value 0"]
impl crate::Resettable for _0_TXPKTU_H_DSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
