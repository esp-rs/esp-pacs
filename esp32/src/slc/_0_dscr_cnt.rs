///Register `_0_DSCR_CNT` reader
pub type R = crate::R<_0_DSCR_CNT_SPEC>;
///Field `SLC0_RX_DSCR_CNT_LAT` reader -
pub type SLC0_RX_DSCR_CNT_LAT_R = crate::FieldReader<u16>;
///Field `SLC0_RX_GET_EOF_OCC` reader -
pub type SLC0_RX_GET_EOF_OCC_R = crate::BitReader;
impl R {
    ///Bits 0:9
    #[inline(always)]
    pub fn slc0_rx_dscr_cnt_lat(&self) -> SLC0_RX_DSCR_CNT_LAT_R {
        SLC0_RX_DSCR_CNT_LAT_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 16
    #[inline(always)]
    pub fn slc0_rx_get_eof_occ(&self) -> SLC0_RX_GET_EOF_OCC_R {
        SLC0_RX_GET_EOF_OCC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_DSCR_CNT")
            .field("slc0_rx_dscr_cnt_lat", &self.slc0_rx_dscr_cnt_lat())
            .field("slc0_rx_get_eof_occ", &self.slc0_rx_get_eof_occ())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`_0_dscr_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct _0_DSCR_CNT_SPEC;
impl crate::RegisterSpec for _0_DSCR_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`_0_dscr_cnt::R`](R) reader structure
impl crate::Readable for _0_DSCR_CNT_SPEC {}
///`reset()` method sets _0_DSCR_CNT to value 0
impl crate::Resettable for _0_DSCR_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
