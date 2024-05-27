///Register `RO_STATUS` reader
pub type R = crate::R<RO_STATUS_SPEC>;
///Field `OUTFIFO_RO_CNT` reader - The register stores the 8byte number of the data in reorder Tx FIFO for channel 0.
pub type OUTFIFO_RO_CNT_R = crate::FieldReader;
///Field `OUT_RO_WR_STATE` reader - The register stores the state of read ram of reorder
pub type OUT_RO_WR_STATE_R = crate::FieldReader;
///Field `OUT_RO_RD_STATE` reader - The register stores the state of write ram of reorder
pub type OUT_RO_RD_STATE_R = crate::FieldReader;
///Field `OUT_PIXEL_BYTE` reader - the number of bytes contained in a pixel at TX channel 0: 1byte 1: 1.5bytes 2 : 2bytes 3: 2.5bytes 4: 3bytes 5: 4bytes
pub type OUT_PIXEL_BYTE_R = crate::FieldReader;
///Field `OUT_BURST_BLOCK_NUM` reader - the number of macro blocks contained in a burst of data at TX channel
pub type OUT_BURST_BLOCK_NUM_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - The register stores the 8byte number of the data in reorder Tx FIFO for channel 0.
    #[inline(always)]
    pub fn outfifo_ro_cnt(&self) -> OUTFIFO_RO_CNT_R {
        OUTFIFO_RO_CNT_R::new((self.bits & 3) as u8)
    }
    ///Bits 6:7 - The register stores the state of read ram of reorder
    #[inline(always)]
    pub fn out_ro_wr_state(&self) -> OUT_RO_WR_STATE_R {
        OUT_RO_WR_STATE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - The register stores the state of write ram of reorder
    #[inline(always)]
    pub fn out_ro_rd_state(&self) -> OUT_RO_RD_STATE_R {
        OUT_RO_RD_STATE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:13 - the number of bytes contained in a pixel at TX channel 0: 1byte 1: 1.5bytes 2 : 2bytes 3: 2.5bytes 4: 3bytes 5: 4bytes
    #[inline(always)]
    pub fn out_pixel_byte(&self) -> OUT_PIXEL_BYTE_R {
        OUT_PIXEL_BYTE_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bits 14:17 - the number of macro blocks contained in a burst of data at TX channel
    #[inline(always)]
    pub fn out_burst_block_num(&self) -> OUT_BURST_BLOCK_NUM_R {
        OUT_BURST_BLOCK_NUM_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RO_STATUS")
            .field("outfifo_ro_cnt", &self.outfifo_ro_cnt())
            .field("out_ro_wr_state", &self.out_ro_wr_state())
            .field("out_ro_rd_state", &self.out_ro_rd_state())
            .field("out_pixel_byte", &self.out_pixel_byte())
            .field("out_burst_block_num", &self.out_burst_block_num())
            .finish()
    }
}
/**TX CHx reorder status register. Available on CH0

You can [`read`](crate::generic::Reg::read) this register and get [`ro_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RO_STATUS_SPEC;
impl crate::RegisterSpec for RO_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ro_status::R`](R) reader structure
impl crate::Readable for RO_STATUS_SPEC {}
///`reset()` method sets RO_STATUS to value 0x0800
impl crate::Resettable for RO_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x0800;
}
