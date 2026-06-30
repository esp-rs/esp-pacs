#[doc = "Register `OUT_RO_STATUS_CH%s` reader"]
pub type R = crate::R<OUT_RO_STATUS_CH_SPEC>;
#[doc = "Field `OUTFIFO_RO_CNT_CH` reader - The register stores the byte number of the data in color convert Tx FIFO for channel 0."]
pub type OUTFIFO_RO_CNT_CH_R = crate::FieldReader;
#[doc = "Field `OUT_RO_WR_STATE_CH` reader - The register stores the state of read ram of reorder"]
pub type OUT_RO_WR_STATE_CH_R = crate::FieldReader;
#[doc = "Field `OUT_RO_RD_STATE_CH` reader - The register stores the state of write ram of reorder"]
pub type OUT_RO_RD_STATE_CH_R = crate::FieldReader;
#[doc = "Field `OUT_PIXEL_BYTE_CH` reader - the number of bytes contained in a pixel at TX channel 0: 1byte 1: 1.5bytes 2 : 2bytes 3: 2.5bytes 4: 3bytes 5: 4bytes"]
pub type OUT_PIXEL_BYTE_CH_R = crate::FieldReader;
#[doc = "Field `OUT_BURST_BLOCK_NUM_CH` reader - the number of macro blocks contained in a burst of data at TX channel"]
pub type OUT_BURST_BLOCK_NUM_CH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - The register stores the byte number of the data in color convert Tx FIFO for channel 0."]
    #[inline(always)]
    pub fn outfifo_ro_cnt_ch(&self) -> OUTFIFO_RO_CNT_CH_R {
        OUTFIFO_RO_CNT_CH_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - The register stores the state of read ram of reorder"]
    #[inline(always)]
    pub fn out_ro_wr_state_ch(&self) -> OUT_RO_WR_STATE_CH_R {
        OUT_RO_WR_STATE_CH_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - The register stores the state of write ram of reorder"]
    #[inline(always)]
    pub fn out_ro_rd_state_ch(&self) -> OUT_RO_RD_STATE_CH_R {
        OUT_RO_RD_STATE_CH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:13 - the number of bytes contained in a pixel at TX channel 0: 1byte 1: 1.5bytes 2 : 2bytes 3: 2.5bytes 4: 3bytes 5: 4bytes"]
    #[inline(always)]
    pub fn out_pixel_byte_ch(&self) -> OUT_PIXEL_BYTE_CH_R {
        OUT_PIXEL_BYTE_CH_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:17 - the number of macro blocks contained in a burst of data at TX channel"]
    #[inline(always)]
    pub fn out_burst_block_num_ch(&self) -> OUT_BURST_BLOCK_NUM_CH_R {
        OUT_BURST_BLOCK_NUM_CH_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_RO_STATUS_CH")
            .field("outfifo_ro_cnt_ch", &self.outfifo_ro_cnt_ch())
            .field("out_ro_wr_state_ch", &self.out_ro_wr_state_ch())
            .field("out_ro_rd_state_ch", &self.out_ro_rd_state_ch())
            .field("out_pixel_byte_ch", &self.out_pixel_byte_ch())
            .field("out_burst_block_num_ch", &self.out_burst_block_num_ch())
            .finish()
    }
}
#[doc = "Represents the status of the tx reorder module of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_ro_status_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_RO_STATUS_CH_SPEC;
impl crate::RegisterSpec for OUT_RO_STATUS_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_ro_status_ch::R`](R) reader structure"]
impl crate::Readable for OUT_RO_STATUS_CH_SPEC {}
#[doc = "`reset()` method sets OUT_RO_STATUS_CH%s to value 0"]
impl crate::Resettable for OUT_RO_STATUS_CH_SPEC {}
