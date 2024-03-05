#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `DONE_INT_RAW` reader - This raw interrupt bit turns to high level when JPEG finishes encoding a picture.."]
pub type DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `DONE_INT_RAW` writer - This raw interrupt bit turns to high level when JPEG finishes encoding a picture.."]
pub type DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLE_PARALLEL_ERR_INT_RAW` reader - The raw interrupt bit to sign that rle parallel error when decoding."]
pub type RLE_PARALLEL_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `RLE_PARALLEL_ERR_INT_RAW` writer - The raw interrupt bit to sign that rle parallel error when decoding."]
pub type RLE_PARALLEL_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CID_ERR_INT_RAW` reader - The raw interrupt bit to sign that scan id check with component fails when decoding."]
pub type CID_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `CID_ERR_INT_RAW` writer - The raw interrupt bit to sign that scan id check with component fails when decoding."]
pub type CID_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_DHT_DC_ID_ERR_INT_RAW` reader - The raw interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
pub type C_DHT_DC_ID_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `C_DHT_DC_ID_ERR_INT_RAW` writer - The raw interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
pub type C_DHT_DC_ID_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_DHT_AC_ID_ERR_INT_RAW` reader - The raw interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
pub type C_DHT_AC_ID_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `C_DHT_AC_ID_ERR_INT_RAW` writer - The raw interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
pub type C_DHT_AC_ID_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_DQT_ID_ERR_INT_RAW` reader - The raw interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
pub type C_DQT_ID_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `C_DQT_ID_ERR_INT_RAW` writer - The raw interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
pub type C_DQT_ID_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_UXP_ERR_INT_RAW` reader - The raw interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
pub type RST_UXP_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `RST_UXP_ERR_INT_RAW` writer - The raw interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
pub type RST_UXP_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_CHECK_NONE_ERR_INT_RAW` reader - The raw interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
pub type RST_CHECK_NONE_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `RST_CHECK_NONE_ERR_INT_RAW` writer - The raw interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
pub type RST_CHECK_NONE_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_CHECK_POS_ERR_INT_RAW` reader - The raw interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
pub type RST_CHECK_POS_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `RST_CHECK_POS_ERR_INT_RAW` writer - The raw interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
pub type RST_CHECK_POS_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_INT_RAW` reader - The raw interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
pub type OUT_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_EOF_INT_RAW` writer - The raw interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
pub type OUT_EOF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_COLOR_MODE_ERR_INT_RAW` reader - The raw interrupt bit to sign that the selected source color mode is not supported."]
pub type SR_COLOR_MODE_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SR_COLOR_MODE_ERR_INT_RAW` writer - The raw interrupt bit to sign that the selected source color mode is not supported."]
pub type SR_COLOR_MODE_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCT_DONE_INT_RAW` reader - The raw interrupt bit to sign that one dct calculation is finished."]
pub type DCT_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `DCT_DONE_INT_RAW` writer - The raw interrupt bit to sign that one dct calculation is finished."]
pub type DCT_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_LAST_BLOCK_EOF_INT_RAW` reader - The raw interrupt bit to sign that the coding process for last block is finished."]
pub type BS_LAST_BLOCK_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `BS_LAST_BLOCK_EOF_INT_RAW` writer - The raw interrupt bit to sign that the coding process for last block is finished."]
pub type BS_LAST_BLOCK_EOF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_CHECK_NONE_ERR_INT_RAW` reader - The raw interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
pub type SCAN_CHECK_NONE_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SCAN_CHECK_NONE_ERR_INT_RAW` writer - The raw interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
pub type SCAN_CHECK_NONE_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_CHECK_POS_ERR_INT_RAW` reader - The raw interrupt bit to sign that SOS header marker position wrong when decoding."]
pub type SCAN_CHECK_POS_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SCAN_CHECK_POS_ERR_INT_RAW` writer - The raw interrupt bit to sign that SOS header marker position wrong when decoding."]
pub type SCAN_CHECK_POS_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UXP_DET_INT_RAW` reader - The raw interrupt bit to sign that unsupported header marker is detected when decoding."]
pub type UXP_DET_INT_RAW_R = crate::BitReader;
#[doc = "Field `UXP_DET_INT_RAW` writer - The raw interrupt bit to sign that unsupported header marker is detected when decoding."]
pub type UXP_DET_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_FRAME_EOF_ERR_INT_RAW` reader - The raw interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
pub type EN_FRAME_EOF_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `EN_FRAME_EOF_ERR_INT_RAW` writer - The raw interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
pub type EN_FRAME_EOF_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_FRAME_EOF_LACK_INT_RAW` reader - The raw interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
pub type EN_FRAME_EOF_LACK_INT_RAW_R = crate::BitReader;
#[doc = "Field `EN_FRAME_EOF_LACK_INT_RAW` writer - The raw interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
pub type EN_FRAME_EOF_LACK_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DE_FRAME_EOF_ERR_INT_RAW` reader - The raw interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
pub type DE_FRAME_EOF_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `DE_FRAME_EOF_ERR_INT_RAW` writer - The raw interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
pub type DE_FRAME_EOF_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DE_FRAME_EOF_LACK_INT_RAW` reader - The raw interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
pub type DE_FRAME_EOF_LACK_INT_RAW_R = crate::BitReader;
#[doc = "Field `DE_FRAME_EOF_LACK_INT_RAW` writer - The raw interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
pub type DE_FRAME_EOF_LACK_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOS_UNMATCH_ERR_INT_RAW` reader - The raw interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
pub type SOS_UNMATCH_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SOS_UNMATCH_ERR_INT_RAW` writer - The raw interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
pub type SOS_UNMATCH_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MARKER_ERR_FST_SCAN_INT_RAW` reader - The raw interrupt bit to sign that the first scan has header marker error when decoding."]
pub type MARKER_ERR_FST_SCAN_INT_RAW_R = crate::BitReader;
#[doc = "Field `MARKER_ERR_FST_SCAN_INT_RAW` writer - The raw interrupt bit to sign that the first scan has header marker error when decoding."]
pub type MARKER_ERR_FST_SCAN_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MARKER_ERR_OTHER_SCAN_INT_RAW` reader - The raw interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
pub type MARKER_ERR_OTHER_SCAN_INT_RAW_R = crate::BitReader;
#[doc = "Field `MARKER_ERR_OTHER_SCAN_INT_RAW` writer - The raw interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
pub type MARKER_ERR_OTHER_SCAN_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDET_INT_RAW` reader - The raw interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
pub type UNDET_INT_RAW_R = crate::BitReader;
#[doc = "Field `UNDET_INT_RAW` writer - The raw interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
pub type UNDET_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECODE_TIMEOUT_INT_RAW` reader - The raw interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
pub type DECODE_TIMEOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `DECODE_TIMEOUT_INT_RAW` writer - The raw interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
pub type DECODE_TIMEOUT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This raw interrupt bit turns to high level when JPEG finishes encoding a picture.."]
    #[inline(always)]
    pub fn done_int_raw(&self) -> DONE_INT_RAW_R {
        DONE_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit to sign that rle parallel error when decoding."]
    #[inline(always)]
    pub fn rle_parallel_err_int_raw(&self) -> RLE_PARALLEL_ERR_INT_RAW_R {
        RLE_PARALLEL_ERR_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit to sign that scan id check with component fails when decoding."]
    #[inline(always)]
    pub fn cid_err_int_raw(&self) -> CID_ERR_INT_RAW_R {
        CID_ERR_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dht_dc_id_err_int_raw(&self) -> C_DHT_DC_ID_ERR_INT_RAW_R {
        C_DHT_DC_ID_ERR_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dht_ac_id_err_int_raw(&self) -> C_DHT_AC_ID_ERR_INT_RAW_R {
        C_DHT_AC_ID_ERR_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
    #[inline(always)]
    pub fn c_dqt_id_err_int_raw(&self) -> C_DQT_ID_ERR_INT_RAW_R {
        C_DQT_ID_ERR_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
    #[inline(always)]
    pub fn rst_uxp_err_int_raw(&self) -> RST_UXP_ERR_INT_RAW_R {
        RST_UXP_ERR_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
    #[inline(always)]
    pub fn rst_check_none_err_int_raw(&self) -> RST_CHECK_NONE_ERR_INT_RAW_R {
        RST_CHECK_NONE_ERR_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
    #[inline(always)]
    pub fn rst_check_pos_err_int_raw(&self) -> RST_CHECK_POS_ERR_INT_RAW_R {
        RST_CHECK_POS_ERR_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
    #[inline(always)]
    pub fn out_eof_int_raw(&self) -> OUT_EOF_INT_RAW_R {
        OUT_EOF_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt bit to sign that the selected source color mode is not supported."]
    #[inline(always)]
    pub fn sr_color_mode_err_int_raw(&self) -> SR_COLOR_MODE_ERR_INT_RAW_R {
        SR_COLOR_MODE_ERR_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt bit to sign that one dct calculation is finished."]
    #[inline(always)]
    pub fn dct_done_int_raw(&self) -> DCT_DONE_INT_RAW_R {
        DCT_DONE_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw interrupt bit to sign that the coding process for last block is finished."]
    #[inline(always)]
    pub fn bs_last_block_eof_int_raw(&self) -> BS_LAST_BLOCK_EOF_INT_RAW_R {
        BS_LAST_BLOCK_EOF_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
    #[inline(always)]
    pub fn scan_check_none_err_int_raw(&self) -> SCAN_CHECK_NONE_ERR_INT_RAW_R {
        SCAN_CHECK_NONE_ERR_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw interrupt bit to sign that SOS header marker position wrong when decoding."]
    #[inline(always)]
    pub fn scan_check_pos_err_int_raw(&self) -> SCAN_CHECK_POS_ERR_INT_RAW_R {
        SCAN_CHECK_POS_ERR_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw interrupt bit to sign that unsupported header marker is detected when decoding."]
    #[inline(always)]
    pub fn uxp_det_int_raw(&self) -> UXP_DET_INT_RAW_R {
        UXP_DET_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The raw interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
    #[inline(always)]
    pub fn en_frame_eof_err_int_raw(&self) -> EN_FRAME_EOF_ERR_INT_RAW_R {
        EN_FRAME_EOF_ERR_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The raw interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
    #[inline(always)]
    pub fn en_frame_eof_lack_int_raw(&self) -> EN_FRAME_EOF_LACK_INT_RAW_R {
        EN_FRAME_EOF_LACK_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The raw interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
    #[inline(always)]
    pub fn de_frame_eof_err_int_raw(&self) -> DE_FRAME_EOF_ERR_INT_RAW_R {
        DE_FRAME_EOF_ERR_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The raw interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
    #[inline(always)]
    pub fn de_frame_eof_lack_int_raw(&self) -> DE_FRAME_EOF_LACK_INT_RAW_R {
        DE_FRAME_EOF_LACK_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The raw interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
    #[inline(always)]
    pub fn sos_unmatch_err_int_raw(&self) -> SOS_UNMATCH_ERR_INT_RAW_R {
        SOS_UNMATCH_ERR_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The raw interrupt bit to sign that the first scan has header marker error when decoding."]
    #[inline(always)]
    pub fn marker_err_fst_scan_int_raw(&self) -> MARKER_ERR_FST_SCAN_INT_RAW_R {
        MARKER_ERR_FST_SCAN_INT_RAW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The raw interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
    #[inline(always)]
    pub fn marker_err_other_scan_int_raw(&self) -> MARKER_ERR_OTHER_SCAN_INT_RAW_R {
        MARKER_ERR_OTHER_SCAN_INT_RAW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The raw interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
    #[inline(always)]
    pub fn undet_int_raw(&self) -> UNDET_INT_RAW_R {
        UNDET_INT_RAW_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The raw interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
    #[inline(always)]
    pub fn decode_timeout_int_raw(&self) -> DECODE_TIMEOUT_INT_RAW_R {
        DECODE_TIMEOUT_INT_RAW_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "done_int_raw",
                &format_args!("{}", self.done_int_raw().bit()),
            )
            .field(
                "rle_parallel_err_int_raw",
                &format_args!("{}", self.rle_parallel_err_int_raw().bit()),
            )
            .field(
                "cid_err_int_raw",
                &format_args!("{}", self.cid_err_int_raw().bit()),
            )
            .field(
                "c_dht_dc_id_err_int_raw",
                &format_args!("{}", self.c_dht_dc_id_err_int_raw().bit()),
            )
            .field(
                "c_dht_ac_id_err_int_raw",
                &format_args!("{}", self.c_dht_ac_id_err_int_raw().bit()),
            )
            .field(
                "c_dqt_id_err_int_raw",
                &format_args!("{}", self.c_dqt_id_err_int_raw().bit()),
            )
            .field(
                "rst_uxp_err_int_raw",
                &format_args!("{}", self.rst_uxp_err_int_raw().bit()),
            )
            .field(
                "rst_check_none_err_int_raw",
                &format_args!("{}", self.rst_check_none_err_int_raw().bit()),
            )
            .field(
                "rst_check_pos_err_int_raw",
                &format_args!("{}", self.rst_check_pos_err_int_raw().bit()),
            )
            .field(
                "out_eof_int_raw",
                &format_args!("{}", self.out_eof_int_raw().bit()),
            )
            .field(
                "sr_color_mode_err_int_raw",
                &format_args!("{}", self.sr_color_mode_err_int_raw().bit()),
            )
            .field(
                "dct_done_int_raw",
                &format_args!("{}", self.dct_done_int_raw().bit()),
            )
            .field(
                "bs_last_block_eof_int_raw",
                &format_args!("{}", self.bs_last_block_eof_int_raw().bit()),
            )
            .field(
                "scan_check_none_err_int_raw",
                &format_args!("{}", self.scan_check_none_err_int_raw().bit()),
            )
            .field(
                "scan_check_pos_err_int_raw",
                &format_args!("{}", self.scan_check_pos_err_int_raw().bit()),
            )
            .field(
                "uxp_det_int_raw",
                &format_args!("{}", self.uxp_det_int_raw().bit()),
            )
            .field(
                "en_frame_eof_err_int_raw",
                &format_args!("{}", self.en_frame_eof_err_int_raw().bit()),
            )
            .field(
                "en_frame_eof_lack_int_raw",
                &format_args!("{}", self.en_frame_eof_lack_int_raw().bit()),
            )
            .field(
                "de_frame_eof_err_int_raw",
                &format_args!("{}", self.de_frame_eof_err_int_raw().bit()),
            )
            .field(
                "de_frame_eof_lack_int_raw",
                &format_args!("{}", self.de_frame_eof_lack_int_raw().bit()),
            )
            .field(
                "sos_unmatch_err_int_raw",
                &format_args!("{}", self.sos_unmatch_err_int_raw().bit()),
            )
            .field(
                "marker_err_fst_scan_int_raw",
                &format_args!("{}", self.marker_err_fst_scan_int_raw().bit()),
            )
            .field(
                "marker_err_other_scan_int_raw",
                &format_args!("{}", self.marker_err_other_scan_int_raw().bit()),
            )
            .field(
                "undet_int_raw",
                &format_args!("{}", self.undet_int_raw().bit()),
            )
            .field(
                "decode_timeout_int_raw",
                &format_args!("{}", self.decode_timeout_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This raw interrupt bit turns to high level when JPEG finishes encoding a picture.."]
    #[inline(always)]
    #[must_use]
    pub fn done_int_raw(&mut self) -> DONE_INT_RAW_W<INT_RAW_SPEC> {
        DONE_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit to sign that rle parallel error when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn rle_parallel_err_int_raw(&mut self) -> RLE_PARALLEL_ERR_INT_RAW_W<INT_RAW_SPEC> {
        RLE_PARALLEL_ERR_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt bit to sign that scan id check with component fails when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn cid_err_int_raw(&mut self) -> CID_ERR_INT_RAW_W<INT_RAW_SPEC> {
        CID_ERR_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn c_dht_dc_id_err_int_raw(&mut self) -> C_DHT_DC_ID_ERR_INT_RAW_W<INT_RAW_SPEC> {
        C_DHT_DC_ID_ERR_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn c_dht_ac_id_err_int_raw(&mut self) -> C_DHT_AC_ID_ERR_INT_RAW_W<INT_RAW_SPEC> {
        C_DHT_AC_ID_ERR_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn c_dqt_id_err_int_raw(&mut self) -> C_DQT_ID_ERR_INT_RAW_W<INT_RAW_SPEC> {
        C_DQT_ID_ERR_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - The raw interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn rst_uxp_err_int_raw(&mut self) -> RST_UXP_ERR_INT_RAW_W<INT_RAW_SPEC> {
        RST_UXP_ERR_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - The raw interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn rst_check_none_err_int_raw(&mut self) -> RST_CHECK_NONE_ERR_INT_RAW_W<INT_RAW_SPEC> {
        RST_CHECK_NONE_ERR_INT_RAW_W::new(self, 7)
    }
    #[doc = "Bit 8 - The raw interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn rst_check_pos_err_int_raw(&mut self) -> RST_CHECK_POS_ERR_INT_RAW_W<INT_RAW_SPEC> {
        RST_CHECK_POS_ERR_INT_RAW_W::new(self, 8)
    }
    #[doc = "Bit 9 - The raw interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_int_raw(&mut self) -> OUT_EOF_INT_RAW_W<INT_RAW_SPEC> {
        OUT_EOF_INT_RAW_W::new(self, 9)
    }
    #[doc = "Bit 10 - The raw interrupt bit to sign that the selected source color mode is not supported."]
    #[inline(always)]
    #[must_use]
    pub fn sr_color_mode_err_int_raw(&mut self) -> SR_COLOR_MODE_ERR_INT_RAW_W<INT_RAW_SPEC> {
        SR_COLOR_MODE_ERR_INT_RAW_W::new(self, 10)
    }
    #[doc = "Bit 11 - The raw interrupt bit to sign that one dct calculation is finished."]
    #[inline(always)]
    #[must_use]
    pub fn dct_done_int_raw(&mut self) -> DCT_DONE_INT_RAW_W<INT_RAW_SPEC> {
        DCT_DONE_INT_RAW_W::new(self, 11)
    }
    #[doc = "Bit 12 - The raw interrupt bit to sign that the coding process for last block is finished."]
    #[inline(always)]
    #[must_use]
    pub fn bs_last_block_eof_int_raw(&mut self) -> BS_LAST_BLOCK_EOF_INT_RAW_W<INT_RAW_SPEC> {
        BS_LAST_BLOCK_EOF_INT_RAW_W::new(self, 12)
    }
    #[doc = "Bit 13 - The raw interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
    #[inline(always)]
    #[must_use]
    pub fn scan_check_none_err_int_raw(&mut self) -> SCAN_CHECK_NONE_ERR_INT_RAW_W<INT_RAW_SPEC> {
        SCAN_CHECK_NONE_ERR_INT_RAW_W::new(self, 13)
    }
    #[doc = "Bit 14 - The raw interrupt bit to sign that SOS header marker position wrong when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn scan_check_pos_err_int_raw(&mut self) -> SCAN_CHECK_POS_ERR_INT_RAW_W<INT_RAW_SPEC> {
        SCAN_CHECK_POS_ERR_INT_RAW_W::new(self, 14)
    }
    #[doc = "Bit 15 - The raw interrupt bit to sign that unsupported header marker is detected when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn uxp_det_int_raw(&mut self) -> UXP_DET_INT_RAW_W<INT_RAW_SPEC> {
        UXP_DET_INT_RAW_W::new(self, 15)
    }
    #[doc = "Bit 16 - The raw interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
    #[inline(always)]
    #[must_use]
    pub fn en_frame_eof_err_int_raw(&mut self) -> EN_FRAME_EOF_ERR_INT_RAW_W<INT_RAW_SPEC> {
        EN_FRAME_EOF_ERR_INT_RAW_W::new(self, 16)
    }
    #[doc = "Bit 17 - The raw interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
    #[inline(always)]
    #[must_use]
    pub fn en_frame_eof_lack_int_raw(&mut self) -> EN_FRAME_EOF_LACK_INT_RAW_W<INT_RAW_SPEC> {
        EN_FRAME_EOF_LACK_INT_RAW_W::new(self, 17)
    }
    #[doc = "Bit 18 - The raw interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn de_frame_eof_err_int_raw(&mut self) -> DE_FRAME_EOF_ERR_INT_RAW_W<INT_RAW_SPEC> {
        DE_FRAME_EOF_ERR_INT_RAW_W::new(self, 18)
    }
    #[doc = "Bit 19 - The raw interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
    #[inline(always)]
    #[must_use]
    pub fn de_frame_eof_lack_int_raw(&mut self) -> DE_FRAME_EOF_LACK_INT_RAW_W<INT_RAW_SPEC> {
        DE_FRAME_EOF_LACK_INT_RAW_W::new(self, 19)
    }
    #[doc = "Bit 20 - The raw interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn sos_unmatch_err_int_raw(&mut self) -> SOS_UNMATCH_ERR_INT_RAW_W<INT_RAW_SPEC> {
        SOS_UNMATCH_ERR_INT_RAW_W::new(self, 20)
    }
    #[doc = "Bit 21 - The raw interrupt bit to sign that the first scan has header marker error when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn marker_err_fst_scan_int_raw(&mut self) -> MARKER_ERR_FST_SCAN_INT_RAW_W<INT_RAW_SPEC> {
        MARKER_ERR_FST_SCAN_INT_RAW_W::new(self, 21)
    }
    #[doc = "Bit 22 - The raw interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn marker_err_other_scan_int_raw(
        &mut self,
    ) -> MARKER_ERR_OTHER_SCAN_INT_RAW_W<INT_RAW_SPEC> {
        MARKER_ERR_OTHER_SCAN_INT_RAW_W::new(self, 22)
    }
    #[doc = "Bit 23 - The raw interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn undet_int_raw(&mut self) -> UNDET_INT_RAW_W<INT_RAW_SPEC> {
        UNDET_INT_RAW_W::new(self, 23)
    }
    #[doc = "Bit 24 - The raw interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn decode_timeout_int_raw(&mut self) -> DECODE_TIMEOUT_INT_RAW_W<INT_RAW_SPEC> {
        DECODE_TIMEOUT_INT_RAW_W::new(self, 24)
    }
}
#[doc = "Interrupt raw registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
