#[doc = "Register `EVT_ST6` reader"]
pub type R = crate::R<EVT_ST6_SPEC>;
#[doc = "Register `EVT_ST6` writer"]
pub type W = crate::W<EVT_ST6_SPEC>;
#[doc = "Field `GDMA_EVT_OUT_FIFO_EMPTY_CH3_ST` reader - Represents GDMA_evt_out_fifo_empty_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_OUT_FIFO_EMPTY_CH3_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_OUT_FIFO_EMPTY_CH3_ST` writer - Represents GDMA_evt_out_fifo_empty_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_OUT_FIFO_EMPTY_CH3_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_OUT_FIFO_EMPTY_CH4_ST` reader - Represents GDMA_evt_out_fifo_empty_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_OUT_FIFO_EMPTY_CH4_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_OUT_FIFO_EMPTY_CH4_ST` writer - Represents GDMA_evt_out_fifo_empty_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_OUT_FIFO_EMPTY_CH4_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_OUT_FIFO_FULL_CH0_ST` reader - Represents GDMA_evt_out_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_OUT_FIFO_FULL_CH0_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_OUT_FIFO_FULL_CH0_ST` writer - Represents GDMA_evt_out_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_OUT_FIFO_FULL_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_OUT_FIFO_FULL_CH1_ST` reader - Represents GDMA_evt_out_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_OUT_FIFO_FULL_CH1_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_OUT_FIFO_FULL_CH1_ST` writer - Represents GDMA_evt_out_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_OUT_FIFO_FULL_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_OUT_FIFO_FULL_CH2_ST` reader - Represents GDMA_evt_out_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_OUT_FIFO_FULL_CH2_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_OUT_FIFO_FULL_CH2_ST` writer - Represents GDMA_evt_out_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_OUT_FIFO_FULL_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_OUT_FIFO_FULL_CH3_ST` reader - Represents GDMA_evt_out_fifo_full_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_OUT_FIFO_FULL_CH3_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_OUT_FIFO_FULL_CH3_ST` writer - Represents GDMA_evt_out_fifo_full_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_OUT_FIFO_FULL_CH3_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_OUT_FIFO_FULL_CH4_ST` reader - Represents GDMA_evt_out_fifo_full_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_OUT_FIFO_FULL_CH4_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_OUT_FIFO_FULL_CH4_ST` writer - Represents GDMA_evt_out_fifo_full_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_OUT_FIFO_FULL_CH4_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_EVT_SLEEP_WEEKUP_ST` reader - Represents PMU_evt_sleep_weekup trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PMU_EVT_SLEEP_WEEKUP_ST_R = crate::BitReader;
#[doc = "Field `PMU_EVT_SLEEP_WEEKUP_ST` writer - Represents PMU_evt_sleep_weekup trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PMU_EVT_SLEEP_WEEKUP_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_EVT_G0_ST` reader - Represents MODEM_evt_g0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_EVT_G0_ST_R = crate::BitReader;
#[doc = "Field `MODEM_EVT_G0_ST` writer - Represents MODEM_evt_g0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_EVT_G0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_EVT_G1_ST` reader - Represents MODEM_evt_g1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_EVT_G1_ST_R = crate::BitReader;
#[doc = "Field `MODEM_EVT_G1_ST` writer - Represents MODEM_evt_g1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_EVT_G1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_EVT_G2_ST` reader - Represents MODEM_evt_g2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_EVT_G2_ST_R = crate::BitReader;
#[doc = "Field `MODEM_EVT_G2_ST` writer - Represents MODEM_evt_g2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_EVT_G2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_EVT_G3_ST` reader - Represents MODEM_evt_g3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_EVT_G3_ST_R = crate::BitReader;
#[doc = "Field `MODEM_EVT_G3_ST` writer - Represents MODEM_evt_g3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_EVT_G3_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZERO_DET_EVT_CHANNEL_1_POS_ST` reader - Represents ZERO_DET_evt_channel_1_pos trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ZERO_DET_EVT_CHANNEL_1_POS_ST_R = crate::BitReader;
#[doc = "Field `ZERO_DET_EVT_CHANNEL_1_POS_ST` writer - Represents ZERO_DET_evt_channel_1_pos trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ZERO_DET_EVT_CHANNEL_1_POS_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZERO_DET_EVT_CHANNEL_2_POS_ST` reader - Represents ZERO_DET_evt_channel_2_pos trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ZERO_DET_EVT_CHANNEL_2_POS_ST_R = crate::BitReader;
#[doc = "Field `ZERO_DET_EVT_CHANNEL_2_POS_ST` writer - Represents ZERO_DET_evt_channel_2_pos trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ZERO_DET_EVT_CHANNEL_2_POS_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZERO_DET_EVT_CHANNEL_3_POS_ST` reader - Represents ZERO_DET_evt_channel_3_pos trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ZERO_DET_EVT_CHANNEL_3_POS_ST_R = crate::BitReader;
#[doc = "Field `ZERO_DET_EVT_CHANNEL_3_POS_ST` writer - Represents ZERO_DET_evt_channel_3_pos trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ZERO_DET_EVT_CHANNEL_3_POS_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZERO_DET_EVT_CHANNEL_1_NEG_ST` reader - Represents ZERO_DET_evt_channel_1_neg trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ZERO_DET_EVT_CHANNEL_1_NEG_ST_R = crate::BitReader;
#[doc = "Field `ZERO_DET_EVT_CHANNEL_1_NEG_ST` writer - Represents ZERO_DET_evt_channel_1_neg trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ZERO_DET_EVT_CHANNEL_1_NEG_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZERO_DET_EVT_CHANNEL_2_NEG_ST` reader - Represents ZERO_DET_evt_channel_2_neg trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ZERO_DET_EVT_CHANNEL_2_NEG_ST_R = crate::BitReader;
#[doc = "Field `ZERO_DET_EVT_CHANNEL_2_NEG_ST` writer - Represents ZERO_DET_evt_channel_2_neg trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ZERO_DET_EVT_CHANNEL_2_NEG_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZERO_DET_EVT_CHANNEL_3_NEG_ST` reader - Represents ZERO_DET_evt_channel_3_neg trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ZERO_DET_EVT_CHANNEL_3_NEG_ST_R = crate::BitReader;
#[doc = "Field `ZERO_DET_EVT_CHANNEL_3_NEG_ST` writer - Represents ZERO_DET_evt_channel_3_neg trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ZERO_DET_EVT_CHANNEL_3_NEG_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents GDMA_evt_out_fifo_empty_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_out_fifo_empty_ch3_st(&self) -> GDMA_EVT_OUT_FIFO_EMPTY_CH3_ST_R {
        GDMA_EVT_OUT_FIFO_EMPTY_CH3_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents GDMA_evt_out_fifo_empty_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_out_fifo_empty_ch4_st(&self) -> GDMA_EVT_OUT_FIFO_EMPTY_CH4_ST_R {
        GDMA_EVT_OUT_FIFO_EMPTY_CH4_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents GDMA_evt_out_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_out_fifo_full_ch0_st(&self) -> GDMA_EVT_OUT_FIFO_FULL_CH0_ST_R {
        GDMA_EVT_OUT_FIFO_FULL_CH0_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents GDMA_evt_out_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_out_fifo_full_ch1_st(&self) -> GDMA_EVT_OUT_FIFO_FULL_CH1_ST_R {
        GDMA_EVT_OUT_FIFO_FULL_CH1_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents GDMA_evt_out_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_out_fifo_full_ch2_st(&self) -> GDMA_EVT_OUT_FIFO_FULL_CH2_ST_R {
        GDMA_EVT_OUT_FIFO_FULL_CH2_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents GDMA_evt_out_fifo_full_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_out_fifo_full_ch3_st(&self) -> GDMA_EVT_OUT_FIFO_FULL_CH3_ST_R {
        GDMA_EVT_OUT_FIFO_FULL_CH3_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents GDMA_evt_out_fifo_full_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_out_fifo_full_ch4_st(&self) -> GDMA_EVT_OUT_FIFO_FULL_CH4_ST_R {
        GDMA_EVT_OUT_FIFO_FULL_CH4_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents PMU_evt_sleep_weekup trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pmu_evt_sleep_weekup_st(&self) -> PMU_EVT_SLEEP_WEEKUP_ST_R {
        PMU_EVT_SLEEP_WEEKUP_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents MODEM_evt_g0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_evt_g0_st(&self) -> MODEM_EVT_G0_ST_R {
        MODEM_EVT_G0_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents MODEM_evt_g1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_evt_g1_st(&self) -> MODEM_EVT_G1_ST_R {
        MODEM_EVT_G1_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents MODEM_evt_g2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_evt_g2_st(&self) -> MODEM_EVT_G2_ST_R {
        MODEM_EVT_G2_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents MODEM_evt_g3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_evt_g3_st(&self) -> MODEM_EVT_G3_ST_R {
        MODEM_EVT_G3_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents ZERO_DET_evt_channel_1_pos trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn zero_det_evt_channel_1_pos_st(&self) -> ZERO_DET_EVT_CHANNEL_1_POS_ST_R {
        ZERO_DET_EVT_CHANNEL_1_POS_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents ZERO_DET_evt_channel_2_pos trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn zero_det_evt_channel_2_pos_st(&self) -> ZERO_DET_EVT_CHANNEL_2_POS_ST_R {
        ZERO_DET_EVT_CHANNEL_2_POS_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents ZERO_DET_evt_channel_3_pos trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn zero_det_evt_channel_3_pos_st(&self) -> ZERO_DET_EVT_CHANNEL_3_POS_ST_R {
        ZERO_DET_EVT_CHANNEL_3_POS_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents ZERO_DET_evt_channel_1_neg trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn zero_det_evt_channel_1_neg_st(&self) -> ZERO_DET_EVT_CHANNEL_1_NEG_ST_R {
        ZERO_DET_EVT_CHANNEL_1_NEG_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents ZERO_DET_evt_channel_2_neg trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn zero_det_evt_channel_2_neg_st(&self) -> ZERO_DET_EVT_CHANNEL_2_NEG_ST_R {
        ZERO_DET_EVT_CHANNEL_2_NEG_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents ZERO_DET_evt_channel_3_neg trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn zero_det_evt_channel_3_neg_st(&self) -> ZERO_DET_EVT_CHANNEL_3_NEG_ST_R {
        ZERO_DET_EVT_CHANNEL_3_NEG_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_ST6")
            .field(
                "gdma_evt_out_fifo_empty_ch3_st",
                &self.gdma_evt_out_fifo_empty_ch3_st(),
            )
            .field(
                "gdma_evt_out_fifo_empty_ch4_st",
                &self.gdma_evt_out_fifo_empty_ch4_st(),
            )
            .field(
                "gdma_evt_out_fifo_full_ch0_st",
                &self.gdma_evt_out_fifo_full_ch0_st(),
            )
            .field(
                "gdma_evt_out_fifo_full_ch1_st",
                &self.gdma_evt_out_fifo_full_ch1_st(),
            )
            .field(
                "gdma_evt_out_fifo_full_ch2_st",
                &self.gdma_evt_out_fifo_full_ch2_st(),
            )
            .field(
                "gdma_evt_out_fifo_full_ch3_st",
                &self.gdma_evt_out_fifo_full_ch3_st(),
            )
            .field(
                "gdma_evt_out_fifo_full_ch4_st",
                &self.gdma_evt_out_fifo_full_ch4_st(),
            )
            .field("pmu_evt_sleep_weekup_st", &self.pmu_evt_sleep_weekup_st())
            .field("modem_evt_g0_st", &self.modem_evt_g0_st())
            .field("modem_evt_g1_st", &self.modem_evt_g1_st())
            .field("modem_evt_g2_st", &self.modem_evt_g2_st())
            .field("modem_evt_g3_st", &self.modem_evt_g3_st())
            .field(
                "zero_det_evt_channel_1_pos_st",
                &self.zero_det_evt_channel_1_pos_st(),
            )
            .field(
                "zero_det_evt_channel_2_pos_st",
                &self.zero_det_evt_channel_2_pos_st(),
            )
            .field(
                "zero_det_evt_channel_3_pos_st",
                &self.zero_det_evt_channel_3_pos_st(),
            )
            .field(
                "zero_det_evt_channel_1_neg_st",
                &self.zero_det_evt_channel_1_neg_st(),
            )
            .field(
                "zero_det_evt_channel_2_neg_st",
                &self.zero_det_evt_channel_2_neg_st(),
            )
            .field(
                "zero_det_evt_channel_3_neg_st",
                &self.zero_det_evt_channel_3_neg_st(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Represents GDMA_evt_out_fifo_empty_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_out_fifo_empty_ch3_st(
        &mut self,
    ) -> GDMA_EVT_OUT_FIFO_EMPTY_CH3_ST_W<'_, EVT_ST6_SPEC> {
        GDMA_EVT_OUT_FIFO_EMPTY_CH3_ST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Represents GDMA_evt_out_fifo_empty_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_out_fifo_empty_ch4_st(
        &mut self,
    ) -> GDMA_EVT_OUT_FIFO_EMPTY_CH4_ST_W<'_, EVT_ST6_SPEC> {
        GDMA_EVT_OUT_FIFO_EMPTY_CH4_ST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Represents GDMA_evt_out_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_out_fifo_full_ch0_st(
        &mut self,
    ) -> GDMA_EVT_OUT_FIFO_FULL_CH0_ST_W<'_, EVT_ST6_SPEC> {
        GDMA_EVT_OUT_FIFO_FULL_CH0_ST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Represents GDMA_evt_out_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_out_fifo_full_ch1_st(
        &mut self,
    ) -> GDMA_EVT_OUT_FIFO_FULL_CH1_ST_W<'_, EVT_ST6_SPEC> {
        GDMA_EVT_OUT_FIFO_FULL_CH1_ST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Represents GDMA_evt_out_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_out_fifo_full_ch2_st(
        &mut self,
    ) -> GDMA_EVT_OUT_FIFO_FULL_CH2_ST_W<'_, EVT_ST6_SPEC> {
        GDMA_EVT_OUT_FIFO_FULL_CH2_ST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Represents GDMA_evt_out_fifo_full_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_out_fifo_full_ch3_st(
        &mut self,
    ) -> GDMA_EVT_OUT_FIFO_FULL_CH3_ST_W<'_, EVT_ST6_SPEC> {
        GDMA_EVT_OUT_FIFO_FULL_CH3_ST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Represents GDMA_evt_out_fifo_full_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_out_fifo_full_ch4_st(
        &mut self,
    ) -> GDMA_EVT_OUT_FIFO_FULL_CH4_ST_W<'_, EVT_ST6_SPEC> {
        GDMA_EVT_OUT_FIFO_FULL_CH4_ST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Represents PMU_evt_sleep_weekup trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pmu_evt_sleep_weekup_st(&mut self) -> PMU_EVT_SLEEP_WEEKUP_ST_W<'_, EVT_ST6_SPEC> {
        PMU_EVT_SLEEP_WEEKUP_ST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Represents MODEM_evt_g0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_evt_g0_st(&mut self) -> MODEM_EVT_G0_ST_W<'_, EVT_ST6_SPEC> {
        MODEM_EVT_G0_ST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Represents MODEM_evt_g1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_evt_g1_st(&mut self) -> MODEM_EVT_G1_ST_W<'_, EVT_ST6_SPEC> {
        MODEM_EVT_G1_ST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Represents MODEM_evt_g2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_evt_g2_st(&mut self) -> MODEM_EVT_G2_ST_W<'_, EVT_ST6_SPEC> {
        MODEM_EVT_G2_ST_W::new(self, 10)
    }
    #[doc = "Bit 11 - Represents MODEM_evt_g3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_evt_g3_st(&mut self) -> MODEM_EVT_G3_ST_W<'_, EVT_ST6_SPEC> {
        MODEM_EVT_G3_ST_W::new(self, 11)
    }
    #[doc = "Bit 12 - Represents ZERO_DET_evt_channel_1_pos trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn zero_det_evt_channel_1_pos_st(
        &mut self,
    ) -> ZERO_DET_EVT_CHANNEL_1_POS_ST_W<'_, EVT_ST6_SPEC> {
        ZERO_DET_EVT_CHANNEL_1_POS_ST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Represents ZERO_DET_evt_channel_2_pos trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn zero_det_evt_channel_2_pos_st(
        &mut self,
    ) -> ZERO_DET_EVT_CHANNEL_2_POS_ST_W<'_, EVT_ST6_SPEC> {
        ZERO_DET_EVT_CHANNEL_2_POS_ST_W::new(self, 13)
    }
    #[doc = "Bit 14 - Represents ZERO_DET_evt_channel_3_pos trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn zero_det_evt_channel_3_pos_st(
        &mut self,
    ) -> ZERO_DET_EVT_CHANNEL_3_POS_ST_W<'_, EVT_ST6_SPEC> {
        ZERO_DET_EVT_CHANNEL_3_POS_ST_W::new(self, 14)
    }
    #[doc = "Bit 15 - Represents ZERO_DET_evt_channel_1_neg trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn zero_det_evt_channel_1_neg_st(
        &mut self,
    ) -> ZERO_DET_EVT_CHANNEL_1_NEG_ST_W<'_, EVT_ST6_SPEC> {
        ZERO_DET_EVT_CHANNEL_1_NEG_ST_W::new(self, 15)
    }
    #[doc = "Bit 16 - Represents ZERO_DET_evt_channel_2_neg trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn zero_det_evt_channel_2_neg_st(
        &mut self,
    ) -> ZERO_DET_EVT_CHANNEL_2_NEG_ST_W<'_, EVT_ST6_SPEC> {
        ZERO_DET_EVT_CHANNEL_2_NEG_ST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Represents ZERO_DET_evt_channel_3_neg trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn zero_det_evt_channel_3_neg_st(
        &mut self,
    ) -> ZERO_DET_EVT_CHANNEL_3_NEG_ST_W<'_, EVT_ST6_SPEC> {
        ZERO_DET_EVT_CHANNEL_3_NEG_ST_W::new(self, 17)
    }
}
#[doc = "Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVT_ST6_SPEC;
impl crate::RegisterSpec for EVT_ST6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_st6::R`](R) reader structure"]
impl crate::Readable for EVT_ST6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evt_st6::W`](W) writer structure"]
impl crate::Writable for EVT_ST6_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_ST6 to value 0"]
impl crate::Resettable for EVT_ST6_SPEC {}
