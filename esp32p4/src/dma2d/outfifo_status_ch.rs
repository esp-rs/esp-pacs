#[doc = "Register `OUTFIFO_STATUS_CH%s` reader"]
pub type R = crate::R<OUTFIFO_STATUS_CH_SPEC>;
#[doc = "Field `OUTFIFO_FULL_L2_CH` reader - Tx FIFO full signal for Tx channel 0."]
pub type OUTFIFO_FULL_L2_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY_L2_CH` reader - Tx FIFO empty signal for Tx channel 0."]
pub type OUTFIFO_EMPTY_L2_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_CNT_L2_CH` reader - The register stores the byte number of the data in Tx FIFO for Tx channel 0."]
pub type OUTFIFO_CNT_L2_CH_R = crate::FieldReader;
#[doc = "Field `OUT_REMAIN_UNDER_1B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_1B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_2B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_2B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_3B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_3B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_4B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_4B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_5B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_5B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_6B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_6B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_7B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_7B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_8B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_8B_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_FULL_L1_CH` reader - Tx FIFO full signal for Tx channel 0."]
pub type OUTFIFO_FULL_L1_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY_L1_CH` reader - Tx FIFO empty signal for Tx channel 0."]
pub type OUTFIFO_EMPTY_L1_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_CNT_L1_CH` reader - The register stores the byte number of the data in Tx FIFO for Tx channel 0."]
pub type OUTFIFO_CNT_L1_CH_R = crate::FieldReader;
#[doc = "Field `OUTFIFO_FULL_L3_CH` reader - Tx FIFO full signal for Tx channel 0."]
pub type OUTFIFO_FULL_L3_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY_L3_CH` reader - Tx FIFO empty signal for Tx channel 0."]
pub type OUTFIFO_EMPTY_L3_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_CNT_L3_CH` reader - The register stores the byte number of the data in Tx FIFO for Tx channel 0."]
pub type OUTFIFO_CNT_L3_CH_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Tx FIFO full signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_full_l2_ch(&self) -> OUTFIFO_FULL_L2_CH_R {
        OUTFIFO_FULL_L2_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx FIFO empty signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_empty_l2_ch(&self) -> OUTFIFO_EMPTY_L2_CH_R {
        OUTFIFO_EMPTY_L2_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - The register stores the byte number of the data in Tx FIFO for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_cnt_l2_ch(&self) -> OUTFIFO_CNT_L2_CH_R {
        OUTFIFO_CNT_L2_CH_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_1b_ch(&self) -> OUT_REMAIN_UNDER_1B_CH_R {
        OUT_REMAIN_UNDER_1B_CH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_2b_ch(&self) -> OUT_REMAIN_UNDER_2B_CH_R {
        OUT_REMAIN_UNDER_2B_CH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_3b_ch(&self) -> OUT_REMAIN_UNDER_3B_CH_R {
        OUT_REMAIN_UNDER_3B_CH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_4b_ch(&self) -> OUT_REMAIN_UNDER_4B_CH_R {
        OUT_REMAIN_UNDER_4B_CH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_5b_ch(&self) -> OUT_REMAIN_UNDER_5B_CH_R {
        OUT_REMAIN_UNDER_5B_CH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_6b_ch(&self) -> OUT_REMAIN_UNDER_6B_CH_R {
        OUT_REMAIN_UNDER_6B_CH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_7b_ch(&self) -> OUT_REMAIN_UNDER_7B_CH_R {
        OUT_REMAIN_UNDER_7B_CH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_8b_ch(&self) -> OUT_REMAIN_UNDER_8B_CH_R {
        OUT_REMAIN_UNDER_8B_CH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx FIFO full signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_full_l1_ch(&self) -> OUTFIFO_FULL_L1_CH_R {
        OUTFIFO_FULL_L1_CH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tx FIFO empty signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_empty_l1_ch(&self) -> OUTFIFO_EMPTY_L1_CH_R {
        OUTFIFO_EMPTY_L1_CH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - The register stores the byte number of the data in Tx FIFO for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_cnt_l1_ch(&self) -> OUTFIFO_CNT_L1_CH_R {
        OUTFIFO_CNT_L1_CH_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - Tx FIFO full signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_full_l3_ch(&self) -> OUTFIFO_FULL_L3_CH_R {
        OUTFIFO_FULL_L3_CH_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Tx FIFO empty signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_empty_l3_ch(&self) -> OUTFIFO_EMPTY_L3_CH_R {
        OUTFIFO_EMPTY_L3_CH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - The register stores the byte number of the data in Tx FIFO for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_cnt_l3_ch(&self) -> OUTFIFO_CNT_L3_CH_R {
        OUTFIFO_CNT_L3_CH_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTFIFO_STATUS_CH")
            .field("outfifo_full_l2_ch", &self.outfifo_full_l2_ch())
            .field("outfifo_empty_l2_ch", &self.outfifo_empty_l2_ch())
            .field("outfifo_cnt_l2_ch", &self.outfifo_cnt_l2_ch())
            .field("out_remain_under_1b_ch", &self.out_remain_under_1b_ch())
            .field("out_remain_under_2b_ch", &self.out_remain_under_2b_ch())
            .field("out_remain_under_3b_ch", &self.out_remain_under_3b_ch())
            .field("out_remain_under_4b_ch", &self.out_remain_under_4b_ch())
            .field("out_remain_under_5b_ch", &self.out_remain_under_5b_ch())
            .field("out_remain_under_6b_ch", &self.out_remain_under_6b_ch())
            .field("out_remain_under_7b_ch", &self.out_remain_under_7b_ch())
            .field("out_remain_under_8b_ch", &self.out_remain_under_8b_ch())
            .field("outfifo_full_l1_ch", &self.outfifo_full_l1_ch())
            .field("outfifo_empty_l1_ch", &self.outfifo_empty_l1_ch())
            .field("outfifo_cnt_l1_ch", &self.outfifo_cnt_l1_ch())
            .field("outfifo_full_l3_ch", &self.outfifo_full_l3_ch())
            .field("outfifo_empty_l3_ch", &self.outfifo_empty_l3_ch())
            .field("outfifo_cnt_l3_ch", &self.outfifo_cnt_l3_ch())
            .finish()
    }
}
#[doc = "Represents the status of the tx fifo of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_status_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTFIFO_STATUS_CH_SPEC;
impl crate::RegisterSpec for OUTFIFO_STATUS_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outfifo_status_ch::R`](R) reader structure"]
impl crate::Readable for OUTFIFO_STATUS_CH_SPEC {}
#[doc = "`reset()` method sets OUTFIFO_STATUS_CH%s to value 0x0081_7f82"]
impl crate::Resettable for OUTFIFO_STATUS_CH_SPEC {
    const RESET_VALUE: u32 = 0x0081_7f82;
}
