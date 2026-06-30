#[doc = "Register `IN_RO_STATUS_CH%s` reader"]
pub type R = crate::R<IN_RO_STATUS_CH_SPEC>;
#[doc = "Field `INFIFO_RO_CNT_CH` reader - The register stores the byte number of the data in color convert Rx FIFO for channel 0."]
pub type INFIFO_RO_CNT_CH_R = crate::FieldReader;
#[doc = "Field `IN_RO_WR_STATE_CH` reader - The register stores the state of read ram of reorder"]
pub type IN_RO_WR_STATE_CH_R = crate::FieldReader;
#[doc = "Field `IN_RO_RD_STATE_CH` reader - The register stores the state of write ram of reorder"]
pub type IN_RO_RD_STATE_CH_R = crate::FieldReader;
#[doc = "Field `IN_PIXEL_BYTE_CH` reader - the number of bytes contained in a pixel at RX channel 0: 1byte 1: 1.5bytes 2 : 2bytes 3: 2.5bytes 4: 3bytes 5: 4bytes"]
pub type IN_PIXEL_BYTE_CH_R = crate::FieldReader;
#[doc = "Field `IN_BURST_BLOCK_NUM_CH` reader - the number of macro blocks contained in a burst of data at RX channel"]
pub type IN_BURST_BLOCK_NUM_CH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - The register stores the byte number of the data in color convert Rx FIFO for channel 0."]
    #[inline(always)]
    pub fn infifo_ro_cnt_ch(&self) -> INFIFO_RO_CNT_CH_R {
        INFIFO_RO_CNT_CH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - The register stores the state of read ram of reorder"]
    #[inline(always)]
    pub fn in_ro_wr_state_ch(&self) -> IN_RO_WR_STATE_CH_R {
        IN_RO_WR_STATE_CH_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - The register stores the state of write ram of reorder"]
    #[inline(always)]
    pub fn in_ro_rd_state_ch(&self) -> IN_RO_RD_STATE_CH_R {
        IN_RO_RD_STATE_CH_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12 - the number of bytes contained in a pixel at RX channel 0: 1byte 1: 1.5bytes 2 : 2bytes 3: 2.5bytes 4: 3bytes 5: 4bytes"]
    #[inline(always)]
    pub fn in_pixel_byte_ch(&self) -> IN_PIXEL_BYTE_CH_R {
        IN_PIXEL_BYTE_CH_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - the number of macro blocks contained in a burst of data at RX channel"]
    #[inline(always)]
    pub fn in_burst_block_num_ch(&self) -> IN_BURST_BLOCK_NUM_CH_R {
        IN_BURST_BLOCK_NUM_CH_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_RO_STATUS_CH")
            .field("infifo_ro_cnt_ch", &self.infifo_ro_cnt_ch())
            .field("in_ro_wr_state_ch", &self.in_ro_wr_state_ch())
            .field("in_ro_rd_state_ch", &self.in_ro_rd_state_ch())
            .field("in_pixel_byte_ch", &self.in_pixel_byte_ch())
            .field("in_burst_block_num_ch", &self.in_burst_block_num_ch())
            .finish()
    }
}
#[doc = "Represents the status of the rx reorder module of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_ro_status_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_RO_STATUS_CH_SPEC;
impl crate::RegisterSpec for IN_RO_STATUS_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_ro_status_ch::R`](R) reader structure"]
impl crate::Readable for IN_RO_STATUS_CH_SPEC {}
#[doc = "`reset()` method sets IN_RO_STATUS_CH%s to value 0"]
impl crate::Resettable for IN_RO_STATUS_CH_SPEC {}
