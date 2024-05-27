#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `DONE` reader - This status interrupt bit turns to high level when JPEG finishes encoding a picture.."]
pub type DONE_R = crate::BitReader;
#[doc = "Field `RLE_PARALLEL_ERR` reader - The status interrupt bit to sign that rle parallel error when decoding."]
pub type RLE_PARALLEL_ERR_R = crate::BitReader;
#[doc = "Field `CID_ERR` reader - The status interrupt bit to sign that scan id check with component fails when decoding."]
pub type CID_ERR_R = crate::BitReader;
#[doc = "Field `C_DHT_DC_ID_ERR` reader - The status interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
pub type C_DHT_DC_ID_ERR_R = crate::BitReader;
#[doc = "Field `C_DHT_AC_ID_ERR` reader - The status interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
pub type C_DHT_AC_ID_ERR_R = crate::BitReader;
#[doc = "Field `C_DQT_ID_ERR` reader - The status interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
pub type C_DQT_ID_ERR_R = crate::BitReader;
#[doc = "Field `RST_UXP_ERR` reader - The status interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
pub type RST_UXP_ERR_R = crate::BitReader;
#[doc = "Field `RST_CHECK_NONE_ERR` reader - The status interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
pub type RST_CHECK_NONE_ERR_R = crate::BitReader;
#[doc = "Field `RST_CHECK_POS_ERR` reader - The status interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
pub type RST_CHECK_POS_ERR_R = crate::BitReader;
#[doc = "Field `OUT_EOF` reader - The status interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
pub type OUT_EOF_R = crate::BitReader;
#[doc = "Field `SR_COLOR_MODE_ERR` reader - The status interrupt bit to sign that the selected source color mode is not supported."]
pub type SR_COLOR_MODE_ERR_R = crate::BitReader;
#[doc = "Field `DCT_DONE` reader - The status interrupt bit to sign that one dct calculation is finished."]
pub type DCT_DONE_R = crate::BitReader;
#[doc = "Field `BS_LAST_BLOCK_EOF` reader - The status interrupt bit to sign that the coding process for last block is finished."]
pub type BS_LAST_BLOCK_EOF_R = crate::BitReader;
#[doc = "Field `SCAN_CHECK_NONE_ERR` reader - The status interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
pub type SCAN_CHECK_NONE_ERR_R = crate::BitReader;
#[doc = "Field `SCAN_CHECK_POS_ERR` reader - The status interrupt bit to sign that SOS header marker position wrong when decoding."]
pub type SCAN_CHECK_POS_ERR_R = crate::BitReader;
#[doc = "Field `UXP_DET` reader - The status interrupt bit to sign that unsupported header marker is detected when decoding."]
pub type UXP_DET_R = crate::BitReader;
#[doc = "Field `EN_FRAME_EOF_ERR` reader - The status interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
pub type EN_FRAME_EOF_ERR_R = crate::BitReader;
#[doc = "Field `EN_FRAME_EOF_LACK` reader - The status interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
pub type EN_FRAME_EOF_LACK_R = crate::BitReader;
#[doc = "Field `DE_FRAME_EOF_ERR` reader - The status interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
pub type DE_FRAME_EOF_ERR_R = crate::BitReader;
#[doc = "Field `DE_FRAME_EOF_LACK` reader - The status interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
pub type DE_FRAME_EOF_LACK_R = crate::BitReader;
#[doc = "Field `SOS_UNMATCH_ERR` reader - The status interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
pub type SOS_UNMATCH_ERR_R = crate::BitReader;
#[doc = "Field `MARKER_ERR_FST_SCAN` reader - The status interrupt bit to sign that the first scan has header marker error when decoding."]
pub type MARKER_ERR_FST_SCAN_R = crate::BitReader;
#[doc = "Field `MARKER_ERR_OTHER_SCAN` reader - The status interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
pub type MARKER_ERR_OTHER_SCAN_R = crate::BitReader;
#[doc = "Field `UNDET` reader - The status interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
pub type UNDET_R = crate::BitReader;
#[doc = "Field `DECODE_TIMEOUT` reader - The status interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
pub type DECODE_TIMEOUT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This status interrupt bit turns to high level when JPEG finishes encoding a picture.."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The status interrupt bit to sign that rle parallel error when decoding."]
    #[inline(always)]
    pub fn rle_parallel_err(&self) -> RLE_PARALLEL_ERR_R {
        RLE_PARALLEL_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The status interrupt bit to sign that scan id check with component fails when decoding."]
    #[inline(always)]
    pub fn cid_err(&self) -> CID_ERR_R {
        CID_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The status interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dht_dc_id_err(&self) -> C_DHT_DC_ID_ERR_R {
        C_DHT_DC_ID_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dht_ac_id_err(&self) -> C_DHT_AC_ID_ERR_R {
        C_DHT_AC_ID_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The status interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dqt_id_err(&self) -> C_DQT_ID_ERR_R {
        C_DQT_ID_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The status interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
    #[inline(always)]
    pub fn rst_uxp_err(&self) -> RST_UXP_ERR_R {
        RST_UXP_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The status interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
    #[inline(always)]
    pub fn rst_check_none_err(&self) -> RST_CHECK_NONE_ERR_R {
        RST_CHECK_NONE_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The status interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
    #[inline(always)]
    pub fn rst_check_pos_err(&self) -> RST_CHECK_POS_ERR_R {
        RST_CHECK_POS_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The status interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The status interrupt bit to sign that the selected source color mode is not supported."]
    #[inline(always)]
    pub fn sr_color_mode_err(&self) -> SR_COLOR_MODE_ERR_R {
        SR_COLOR_MODE_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The status interrupt bit to sign that one dct calculation is finished."]
    #[inline(always)]
    pub fn dct_done(&self) -> DCT_DONE_R {
        DCT_DONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The status interrupt bit to sign that the coding process for last block is finished."]
    #[inline(always)]
    pub fn bs_last_block_eof(&self) -> BS_LAST_BLOCK_EOF_R {
        BS_LAST_BLOCK_EOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The status interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
    #[inline(always)]
    pub fn scan_check_none_err(&self) -> SCAN_CHECK_NONE_ERR_R {
        SCAN_CHECK_NONE_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The status interrupt bit to sign that SOS header marker position wrong when decoding."]
    #[inline(always)]
    pub fn scan_check_pos_err(&self) -> SCAN_CHECK_POS_ERR_R {
        SCAN_CHECK_POS_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The status interrupt bit to sign that unsupported header marker is detected when decoding."]
    #[inline(always)]
    pub fn uxp_det(&self) -> UXP_DET_R {
        UXP_DET_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The status interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
    #[inline(always)]
    pub fn en_frame_eof_err(&self) -> EN_FRAME_EOF_ERR_R {
        EN_FRAME_EOF_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The status interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
    #[inline(always)]
    pub fn en_frame_eof_lack(&self) -> EN_FRAME_EOF_LACK_R {
        EN_FRAME_EOF_LACK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The status interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
    #[inline(always)]
    pub fn de_frame_eof_err(&self) -> DE_FRAME_EOF_ERR_R {
        DE_FRAME_EOF_ERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The status interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
    #[inline(always)]
    pub fn de_frame_eof_lack(&self) -> DE_FRAME_EOF_LACK_R {
        DE_FRAME_EOF_LACK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The status interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
    #[inline(always)]
    pub fn sos_unmatch_err(&self) -> SOS_UNMATCH_ERR_R {
        SOS_UNMATCH_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The status interrupt bit to sign that the first scan has header marker error when decoding."]
    #[inline(always)]
    pub fn marker_err_fst_scan(&self) -> MARKER_ERR_FST_SCAN_R {
        MARKER_ERR_FST_SCAN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The status interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
    #[inline(always)]
    pub fn marker_err_other_scan(&self) -> MARKER_ERR_OTHER_SCAN_R {
        MARKER_ERR_OTHER_SCAN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The status interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
    #[inline(always)]
    pub fn undet(&self) -> UNDET_R {
        UNDET_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The status interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
    #[inline(always)]
    pub fn decode_timeout(&self) -> DECODE_TIMEOUT_R {
        DECODE_TIMEOUT_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("done", &self.done())
            .field("rle_parallel_err", &self.rle_parallel_err())
            .field("cid_err", &self.cid_err())
            .field("c_dht_dc_id_err", &self.c_dht_dc_id_err())
            .field("c_dht_ac_id_err", &self.c_dht_ac_id_err())
            .field("c_dqt_id_err", &self.c_dqt_id_err())
            .field("rst_uxp_err", &self.rst_uxp_err())
            .field("rst_check_none_err", &self.rst_check_none_err())
            .field("rst_check_pos_err", &self.rst_check_pos_err())
            .field("out_eof", &self.out_eof())
            .field("sr_color_mode_err", &self.sr_color_mode_err())
            .field("dct_done", &self.dct_done())
            .field("bs_last_block_eof", &self.bs_last_block_eof())
            .field("scan_check_none_err", &self.scan_check_none_err())
            .field("scan_check_pos_err", &self.scan_check_pos_err())
            .field("uxp_det", &self.uxp_det())
            .field("en_frame_eof_err", &self.en_frame_eof_err())
            .field("en_frame_eof_lack", &self.en_frame_eof_lack())
            .field("de_frame_eof_err", &self.de_frame_eof_err())
            .field("de_frame_eof_lack", &self.de_frame_eof_lack())
            .field("sos_unmatch_err", &self.sos_unmatch_err())
            .field("marker_err_fst_scan", &self.marker_err_fst_scan())
            .field("marker_err_other_scan", &self.marker_err_other_scan())
            .field("undet", &self.undet())
            .field("decode_timeout", &self.decode_timeout())
            .finish()
    }
}
#[doc = "Interrupt status registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
