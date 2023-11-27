#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `DONE_INT_CLR` writer - This clear interrupt bit turns to high level when JPEG finishes encoding a picture.."]
pub type DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLE_PARALLEL_ERR_INT_CLR` writer - The clear interrupt bit to sign that rle parallel error when decoding."]
pub type RLE_PARALLEL_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CID_ERR_INT_CLR` writer - The clear interrupt bit to sign that scan id check with component fails when decoding."]
pub type CID_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_DHT_DC_ID_ERR_INT_CLR` writer - The clear interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
pub type C_DHT_DC_ID_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_DHT_AC_ID_ERR_INT_CLR` writer - The clear interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
pub type C_DHT_AC_ID_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_DQT_ID_ERR_INT_CLR` writer - The clear interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
pub type C_DQT_ID_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_UXP_ERR_INT_CLR` writer - The clear interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
pub type RST_UXP_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_CHECK_NONE_ERR_INT_CLR` writer - The clear interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
pub type RST_CHECK_NONE_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_CHECK_POS_ERR_INT_CLR` writer - The clear interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
pub type RST_CHECK_POS_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_INT_CLR` writer - The clear interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
pub type OUT_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_COLOR_MODE_ERR_INT_CLR` writer - The clear interrupt bit to sign that the selected source color mode is not supported."]
pub type SR_COLOR_MODE_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCT_DONE_INT_CLR` writer - The clear interrupt bit to sign that one dct calculation is finished."]
pub type DCT_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_LAST_BLOCK_EOF_INT_CLR` writer - The clear interrupt bit to sign that the coding process for last block is finished."]
pub type BS_LAST_BLOCK_EOF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_CHECK_NONE_ERR_INT_CLR` writer - The clear interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
pub type SCAN_CHECK_NONE_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCAN_CHECK_POS_ERR_INT_CLR` writer - The clear interrupt bit to sign that SOS header marker position wrong when decoding."]
pub type SCAN_CHECK_POS_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UXP_DET_INT_CLR` writer - The clear interrupt bit to sign that unsupported header marker is detected when decoding."]
pub type UXP_DET_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_FRAME_EOF_ERR_INT_CLR` writer - The clear interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
pub type EN_FRAME_EOF_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_FRAME_EOF_LACK_INT_CLR` writer - The clear interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
pub type EN_FRAME_EOF_LACK_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DE_FRAME_EOF_ERR_INT_CLR` writer - The clear interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
pub type DE_FRAME_EOF_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DE_FRAME_EOF_LACK_INT_CLR` writer - The clear interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
pub type DE_FRAME_EOF_LACK_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOS_UNMATCH_ERR_INT_CLR` writer - The clear interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
pub type SOS_UNMATCH_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MARKER_ERR_FST_SCAN_INT_CLR` writer - The clear interrupt bit to sign that the first scan has header marker error when decoding."]
pub type MARKER_ERR_FST_SCAN_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MARKER_ERR_OTHER_SCAN_INT_CLR` writer - The clear interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
pub type MARKER_ERR_OTHER_SCAN_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDET_INT_CLR` writer - The clear interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
pub type UNDET_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECODE_TIMEOUT_INT_CLR` writer - The clear interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
pub type DECODE_TIMEOUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - This clear interrupt bit turns to high level when JPEG finishes encoding a picture.."]
    #[inline(always)]
    #[must_use]
    pub fn done_int_clr(&mut self) -> DONE_INT_CLR_W<INT_CLR_SPEC> {
        DONE_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The clear interrupt bit to sign that rle parallel error when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn rle_parallel_err_int_clr(&mut self) -> RLE_PARALLEL_ERR_INT_CLR_W<INT_CLR_SPEC> {
        RLE_PARALLEL_ERR_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - The clear interrupt bit to sign that scan id check with component fails when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn cid_err_int_clr(&mut self) -> CID_ERR_INT_CLR_W<INT_CLR_SPEC> {
        CID_ERR_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - The clear interrupt bit to sign that scan component's dc dht id check with dc dht table's id fails when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn c_dht_dc_id_err_int_clr(&mut self) -> C_DHT_DC_ID_ERR_INT_CLR_W<INT_CLR_SPEC> {
        C_DHT_DC_ID_ERR_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - The clear interrupt bit to sign that scan component's ac dht id check with ac dht table's id fails when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn c_dht_ac_id_err_int_clr(&mut self) -> C_DHT_AC_ID_ERR_INT_CLR_W<INT_CLR_SPEC> {
        C_DHT_AC_ID_ERR_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - The clear interrupt bit to sign that scan component's dqt id check with dqt table's id fails when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn c_dqt_id_err_int_clr(&mut self) -> C_DQT_ID_ERR_INT_CLR_W<INT_CLR_SPEC> {
        C_DQT_ID_ERR_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - The clear interrupt bit to sign that RST header marker is detected but restart interval is 0 when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn rst_uxp_err_int_clr(&mut self) -> RST_UXP_ERR_INT_CLR_W<INT_CLR_SPEC> {
        RST_UXP_ERR_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - The clear interrupt bit to sign that RST header marker is not detected but restart interval is not 0 when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn rst_check_none_err_int_clr(&mut self) -> RST_CHECK_NONE_ERR_INT_CLR_W<INT_CLR_SPEC> {
        RST_CHECK_NONE_ERR_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - The clear interrupt bit to sign that RST header marker position mismatches with restart interval when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn rst_check_pos_err_int_clr(&mut self) -> RST_CHECK_POS_ERR_INT_CLR_W<INT_CLR_SPEC> {
        RST_CHECK_POS_ERR_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - The clear interrupt bit turns to high level when the last pixel of one square has been transmitted for Tx channel."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_int_clr(&mut self) -> OUT_EOF_INT_CLR_W<INT_CLR_SPEC> {
        OUT_EOF_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - The clear interrupt bit to sign that the selected source color mode is not supported."]
    #[inline(always)]
    #[must_use]
    pub fn sr_color_mode_err_int_clr(&mut self) -> SR_COLOR_MODE_ERR_INT_CLR_W<INT_CLR_SPEC> {
        SR_COLOR_MODE_ERR_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - The clear interrupt bit to sign that one dct calculation is finished."]
    #[inline(always)]
    #[must_use]
    pub fn dct_done_int_clr(&mut self) -> DCT_DONE_INT_CLR_W<INT_CLR_SPEC> {
        DCT_DONE_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - The clear interrupt bit to sign that the coding process for last block is finished."]
    #[inline(always)]
    #[must_use]
    pub fn bs_last_block_eof_int_clr(&mut self) -> BS_LAST_BLOCK_EOF_INT_CLR_W<INT_CLR_SPEC> {
        BS_LAST_BLOCK_EOF_INT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - The clear interrupt bit to sign that SOS header marker is not detected but there are still components left to be decoded."]
    #[inline(always)]
    #[must_use]
    pub fn scan_check_none_err_int_clr(&mut self) -> SCAN_CHECK_NONE_ERR_INT_CLR_W<INT_CLR_SPEC> {
        SCAN_CHECK_NONE_ERR_INT_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - The clear interrupt bit to sign that SOS header marker position wrong when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn scan_check_pos_err_int_clr(&mut self) -> SCAN_CHECK_POS_ERR_INT_CLR_W<INT_CLR_SPEC> {
        SCAN_CHECK_POS_ERR_INT_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15 - The clear interrupt bit to sign that unsupported header marker is detected when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn uxp_det_int_clr(&mut self) -> UXP_DET_INT_CLR_W<INT_CLR_SPEC> {
        UXP_DET_INT_CLR_W::new(self, 15)
    }
    #[doc = "Bit 16 - The clear interrupt bit to sign that received pixel blocks are smaller than expected when encoding."]
    #[inline(always)]
    #[must_use]
    pub fn en_frame_eof_err_int_clr(&mut self) -> EN_FRAME_EOF_ERR_INT_CLR_W<INT_CLR_SPEC> {
        EN_FRAME_EOF_ERR_INT_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17 - The clear interrupt bit to sign that the frame eof sign bit from dma input is missing when encoding. But the number of pixel blocks is enough."]
    #[inline(always)]
    #[must_use]
    pub fn en_frame_eof_lack_int_clr(&mut self) -> EN_FRAME_EOF_LACK_INT_CLR_W<INT_CLR_SPEC> {
        EN_FRAME_EOF_LACK_INT_CLR_W::new(self, 17)
    }
    #[doc = "Bit 18 - The clear interrupt bit to sign that decoded blocks are smaller than expected when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn de_frame_eof_err_int_clr(&mut self) -> DE_FRAME_EOF_ERR_INT_CLR_W<INT_CLR_SPEC> {
        DE_FRAME_EOF_ERR_INT_CLR_W::new(self, 18)
    }
    #[doc = "Bit 19 - The clear interrupt bit to sign that the either frame eof from dma input or eoi marker is missing when encoding. But the number of decoded blocks is enough."]
    #[inline(always)]
    #[must_use]
    pub fn de_frame_eof_lack_int_clr(&mut self) -> DE_FRAME_EOF_LACK_INT_CLR_W<INT_CLR_SPEC> {
        DE_FRAME_EOF_LACK_INT_CLR_W::new(self, 19)
    }
    #[doc = "Bit 20 - The clear interrupt bit to sign that the component number of a scan is 0 or does not match the sos marker's length when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn sos_unmatch_err_int_clr(&mut self) -> SOS_UNMATCH_ERR_INT_CLR_W<INT_CLR_SPEC> {
        SOS_UNMATCH_ERR_INT_CLR_W::new(self, 20)
    }
    #[doc = "Bit 21 - The clear interrupt bit to sign that the first scan has header marker error when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn marker_err_fst_scan_int_clr(&mut self) -> MARKER_ERR_FST_SCAN_INT_CLR_W<INT_CLR_SPEC> {
        MARKER_ERR_FST_SCAN_INT_CLR_W::new(self, 21)
    }
    #[doc = "Bit 22 - The clear interrupt bit to sign that the following scans but not the first scan have header marker error when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn marker_err_other_scan_int_clr(
        &mut self,
    ) -> MARKER_ERR_OTHER_SCAN_INT_CLR_W<INT_CLR_SPEC> {
        MARKER_ERR_OTHER_SCAN_INT_CLR_W::new(self, 22)
    }
    #[doc = "Bit 23 - The clear interrupt bit to sign that JPEG format is not detected at the eof data of a packet when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn undet_int_clr(&mut self) -> UNDET_INT_CLR_W<INT_CLR_SPEC> {
        UNDET_INT_CLR_W::new(self, 23)
    }
    #[doc = "Bit 24 - The clear interrupt bit to sign that decode pause time is longer than the setting decode timeout time when decoding."]
    #[inline(always)]
    #[must_use]
    pub fn decode_timeout_int_clr(&mut self) -> DECODE_TIMEOUT_INT_CLR_W<INT_CLR_SPEC> {
        DECODE_TIMEOUT_INT_CLR_W::new(self, 24)
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
#[doc = "Interrupt clear registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
