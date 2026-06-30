#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `DONE_INT_ST` reader - This status interrupt bit turns to high level when JPEG finishes encoding a picture.."]
pub type DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `RLE_PARALLEL_ERR_INT_ST` reader - The status interrupt bit to sign that rle parallel error when decoding."]
pub type RLE_PARALLEL_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `CID_ERR_INT_ST` reader - The status interrupt bit to sign that scan id check with component fails when decoding."]
pub type CID_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `C_DHT_DC_ID_ERR_INT_ST` reader - The status interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
pub type C_DHT_DC_ID_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `C_DHT_AC_ID_ERR_INT_ST` reader - The status interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
pub type C_DHT_AC_ID_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `C_DQT_ID_ERR_INT_ST` reader - The status interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
pub type C_DQT_ID_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `RST_UXP_ERR_INT_ST` reader - The status interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
pub type RST_UXP_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `RST_CHECK_NONE_ERR_INT_ST` reader - The status interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
pub type RST_CHECK_NONE_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `RST_CHECK_POS_ERR_INT_ST` reader - The status interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
pub type RST_CHECK_POS_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `OUT_EOF_INT_ST` reader - The status interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
pub type OUT_EOF_INT_ST_R = crate::BitReader;
#[doc = "Field `SR_COLOR_MODE_ERR_INT_ST` reader - The status interrupt bit to sign that the selected source color mode is not supported."]
pub type SR_COLOR_MODE_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `DCT_DONE_INT_ST` reader - The status interrupt bit to sign that one dct calculation is finished."]
pub type DCT_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `BS_LAST_BLOCK_EOF_INT_ST` reader - The status interrupt bit to sign that the coding process for last block is finished."]
pub type BS_LAST_BLOCK_EOF_INT_ST_R = crate::BitReader;
#[doc = "Field `SCAN_CHECK_NONE_ERR_INT_ST` reader - The status interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
pub type SCAN_CHECK_NONE_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SCAN_CHECK_POS_ERR_INT_ST` reader - The status interrupt bit to sign that SOS header marker position wrong when decoding."]
pub type SCAN_CHECK_POS_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `UXP_DET_INT_ST` reader - The status interrupt bit to sign that unsupported header marker is detected when decoding."]
pub type UXP_DET_INT_ST_R = crate::BitReader;
#[doc = "Field `EN_FRAME_EOF_ERR_INT_ST` reader - The status interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
pub type EN_FRAME_EOF_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `EN_FRAME_EOF_LACK_INT_ST` reader - The status interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
pub type EN_FRAME_EOF_LACK_INT_ST_R = crate::BitReader;
#[doc = "Field `DE_FRAME_EOF_ERR_INT_ST` reader - The status interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
pub type DE_FRAME_EOF_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `DE_FRAME_EOF_LACK_INT_ST` reader - The status interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
pub type DE_FRAME_EOF_LACK_INT_ST_R = crate::BitReader;
#[doc = "Field `SOS_UNMATCH_ERR_INT_ST` reader - The status interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
pub type SOS_UNMATCH_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `MARKER_ERR_FST_SCAN_INT_ST` reader - The status interrupt bit to sign that the first scan has header marker error when decoding."]
pub type MARKER_ERR_FST_SCAN_INT_ST_R = crate::BitReader;
#[doc = "Field `MARKER_ERR_OTHER_SCAN_INT_ST` reader - The status interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
pub type MARKER_ERR_OTHER_SCAN_INT_ST_R = crate::BitReader;
#[doc = "Field `UNDET_INT_ST` reader - The status interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
pub type UNDET_INT_ST_R = crate::BitReader;
#[doc = "Field `DECODE_TIMEOUT_INT_ST` reader - The status interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
pub type DECODE_TIMEOUT_INT_ST_R = crate::BitReader;
#[doc = "Field `ENCODE_TIMEOUT_INT_ST` reader - The status interrupt bit to sign that encode can not output bitstream time is longer than the setting encode timeout time when encoding."]
pub type ENCODE_TIMEOUT_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This status interrupt bit turns to high level when JPEG finishes encoding a picture.."]
    #[inline(always)]
    pub fn done_int_st(&self) -> DONE_INT_ST_R {
        DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The status interrupt bit to sign that rle parallel error when decoding."]
    #[inline(always)]
    pub fn rle_parallel_err_int_st(&self) -> RLE_PARALLEL_ERR_INT_ST_R {
        RLE_PARALLEL_ERR_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The status interrupt bit to sign that scan id check with component fails when decoding."]
    #[inline(always)]
    pub fn cid_err_int_st(&self) -> CID_ERR_INT_ST_R {
        CID_ERR_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The status interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dht_dc_id_err_int_st(&self) -> C_DHT_DC_ID_ERR_INT_ST_R {
        C_DHT_DC_ID_ERR_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dht_ac_id_err_int_st(&self) -> C_DHT_AC_ID_ERR_INT_ST_R {
        C_DHT_AC_ID_ERR_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The status interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dqt_id_err_int_st(&self) -> C_DQT_ID_ERR_INT_ST_R {
        C_DQT_ID_ERR_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The status interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
    #[inline(always)]
    pub fn rst_uxp_err_int_st(&self) -> RST_UXP_ERR_INT_ST_R {
        RST_UXP_ERR_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The status interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
    #[inline(always)]
    pub fn rst_check_none_err_int_st(&self) -> RST_CHECK_NONE_ERR_INT_ST_R {
        RST_CHECK_NONE_ERR_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The status interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
    #[inline(always)]
    pub fn rst_check_pos_err_int_st(&self) -> RST_CHECK_POS_ERR_INT_ST_R {
        RST_CHECK_POS_ERR_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The status interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
    #[inline(always)]
    pub fn out_eof_int_st(&self) -> OUT_EOF_INT_ST_R {
        OUT_EOF_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The status interrupt bit to sign that the selected source color mode is not supported."]
    #[inline(always)]
    pub fn sr_color_mode_err_int_st(&self) -> SR_COLOR_MODE_ERR_INT_ST_R {
        SR_COLOR_MODE_ERR_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The status interrupt bit to sign that one dct calculation is finished."]
    #[inline(always)]
    pub fn dct_done_int_st(&self) -> DCT_DONE_INT_ST_R {
        DCT_DONE_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The status interrupt bit to sign that the coding process for last block is finished."]
    #[inline(always)]
    pub fn bs_last_block_eof_int_st(&self) -> BS_LAST_BLOCK_EOF_INT_ST_R {
        BS_LAST_BLOCK_EOF_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The status interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
    #[inline(always)]
    pub fn scan_check_none_err_int_st(&self) -> SCAN_CHECK_NONE_ERR_INT_ST_R {
        SCAN_CHECK_NONE_ERR_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The status interrupt bit to sign that SOS header marker position wrong when decoding."]
    #[inline(always)]
    pub fn scan_check_pos_err_int_st(&self) -> SCAN_CHECK_POS_ERR_INT_ST_R {
        SCAN_CHECK_POS_ERR_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The status interrupt bit to sign that unsupported header marker is detected when decoding."]
    #[inline(always)]
    pub fn uxp_det_int_st(&self) -> UXP_DET_INT_ST_R {
        UXP_DET_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The status interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
    #[inline(always)]
    pub fn en_frame_eof_err_int_st(&self) -> EN_FRAME_EOF_ERR_INT_ST_R {
        EN_FRAME_EOF_ERR_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The status interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
    #[inline(always)]
    pub fn en_frame_eof_lack_int_st(&self) -> EN_FRAME_EOF_LACK_INT_ST_R {
        EN_FRAME_EOF_LACK_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The status interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
    #[inline(always)]
    pub fn de_frame_eof_err_int_st(&self) -> DE_FRAME_EOF_ERR_INT_ST_R {
        DE_FRAME_EOF_ERR_INT_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The status interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
    #[inline(always)]
    pub fn de_frame_eof_lack_int_st(&self) -> DE_FRAME_EOF_LACK_INT_ST_R {
        DE_FRAME_EOF_LACK_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The status interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
    #[inline(always)]
    pub fn sos_unmatch_err_int_st(&self) -> SOS_UNMATCH_ERR_INT_ST_R {
        SOS_UNMATCH_ERR_INT_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The status interrupt bit to sign that the first scan has header marker error when decoding."]
    #[inline(always)]
    pub fn marker_err_fst_scan_int_st(&self) -> MARKER_ERR_FST_SCAN_INT_ST_R {
        MARKER_ERR_FST_SCAN_INT_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The status interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
    #[inline(always)]
    pub fn marker_err_other_scan_int_st(&self) -> MARKER_ERR_OTHER_SCAN_INT_ST_R {
        MARKER_ERR_OTHER_SCAN_INT_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The status interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
    #[inline(always)]
    pub fn undet_int_st(&self) -> UNDET_INT_ST_R {
        UNDET_INT_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The status interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
    #[inline(always)]
    pub fn decode_timeout_int_st(&self) -> DECODE_TIMEOUT_INT_ST_R {
        DECODE_TIMEOUT_INT_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The status interrupt bit to sign that encode can not output bitstream time is longer than the setting encode timeout time when encoding."]
    #[inline(always)]
    pub fn encode_timeout_int_st(&self) -> ENCODE_TIMEOUT_INT_ST_R {
        ENCODE_TIMEOUT_INT_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("done_int_st", &self.done_int_st())
            .field("rle_parallel_err_int_st", &self.rle_parallel_err_int_st())
            .field("cid_err_int_st", &self.cid_err_int_st())
            .field("c_dht_dc_id_err_int_st", &self.c_dht_dc_id_err_int_st())
            .field("c_dht_ac_id_err_int_st", &self.c_dht_ac_id_err_int_st())
            .field("c_dqt_id_err_int_st", &self.c_dqt_id_err_int_st())
            .field("rst_uxp_err_int_st", &self.rst_uxp_err_int_st())
            .field(
                "rst_check_none_err_int_st",
                &self.rst_check_none_err_int_st(),
            )
            .field("rst_check_pos_err_int_st", &self.rst_check_pos_err_int_st())
            .field("out_eof_int_st", &self.out_eof_int_st())
            .field("sr_color_mode_err_int_st", &self.sr_color_mode_err_int_st())
            .field("dct_done_int_st", &self.dct_done_int_st())
            .field("bs_last_block_eof_int_st", &self.bs_last_block_eof_int_st())
            .field(
                "scan_check_none_err_int_st",
                &self.scan_check_none_err_int_st(),
            )
            .field(
                "scan_check_pos_err_int_st",
                &self.scan_check_pos_err_int_st(),
            )
            .field("uxp_det_int_st", &self.uxp_det_int_st())
            .field("en_frame_eof_err_int_st", &self.en_frame_eof_err_int_st())
            .field("en_frame_eof_lack_int_st", &self.en_frame_eof_lack_int_st())
            .field("de_frame_eof_err_int_st", &self.de_frame_eof_err_int_st())
            .field("de_frame_eof_lack_int_st", &self.de_frame_eof_lack_int_st())
            .field("sos_unmatch_err_int_st", &self.sos_unmatch_err_int_st())
            .field(
                "marker_err_fst_scan_int_st",
                &self.marker_err_fst_scan_int_st(),
            )
            .field(
                "marker_err_other_scan_int_st",
                &self.marker_err_other_scan_int_st(),
            )
            .field("undet_int_st", &self.undet_int_st())
            .field("decode_timeout_int_st", &self.decode_timeout_int_st())
            .field("encode_timeout_int_st", &self.encode_timeout_int_st())
            .finish()
    }
}
#[doc = "Interrupt status registers\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
