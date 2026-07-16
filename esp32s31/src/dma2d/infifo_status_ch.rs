#[doc = "Register `INFIFO_STATUS_CH%s` reader"]
pub type R = crate::R<INFIFO_STATUS_CH_SPEC>;
#[doc = "Field `INFIFO_FULL_L2_CH` reader - Rx FIFO full signal for Rx channel."]
pub type INFIFO_FULL_L2_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_EMPTY_L2_CH` reader - Rx FIFO empty signal for Rx channel."]
pub type INFIFO_EMPTY_L2_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_CNT_L2_CH` reader - The register stores the byte number of the data in Rx FIFO for Rx channel."]
pub type INFIFO_CNT_L2_CH_R = crate::FieldReader;
#[doc = "Field `IN_REMAIN_UNDER_1B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_1B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_2B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_2B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_3B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_3B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_4B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_4B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_5B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_5B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_6B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_6B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_7B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_7B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_8B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_8B_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_FULL_L1_CH` reader - Rx FIFO full signal for Rx channel 0."]
pub type INFIFO_FULL_L1_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_EMPTY_L1_CH` reader - Rx FIFO empty signal for Rx channel 0."]
pub type INFIFO_EMPTY_L1_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_CNT_L1_CH` reader - The register stores the byte number of the data in Rx FIFO for Rx channel 0."]
pub type INFIFO_CNT_L1_CH_R = crate::FieldReader;
#[doc = "Field `INFIFO_FULL_L3_CH` reader - Rx FIFO full signal for Rx channel 0."]
pub type INFIFO_FULL_L3_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_EMPTY_L3_CH` reader - Rx FIFO empty signal for Rx channel 0."]
pub type INFIFO_EMPTY_L3_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_CNT_L3_CH` reader - The register stores the byte number of the data in Rx FIFO for Rx channel 0."]
pub type INFIFO_CNT_L3_CH_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Rx FIFO full signal for Rx channel."]
    #[inline(always)]
    pub fn infifo_full_l2_ch(&self) -> INFIFO_FULL_L2_CH_R {
        INFIFO_FULL_L2_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO empty signal for Rx channel."]
    #[inline(always)]
    pub fn infifo_empty_l2_ch(&self) -> INFIFO_EMPTY_L2_CH_R {
        INFIFO_EMPTY_L2_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - The register stores the byte number of the data in Rx FIFO for Rx channel."]
    #[inline(always)]
    pub fn infifo_cnt_l2_ch(&self) -> INFIFO_CNT_L2_CH_R {
        INFIFO_CNT_L2_CH_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_1b_ch(&self) -> IN_REMAIN_UNDER_1B_CH_R {
        IN_REMAIN_UNDER_1B_CH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_2b_ch(&self) -> IN_REMAIN_UNDER_2B_CH_R {
        IN_REMAIN_UNDER_2B_CH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_3b_ch(&self) -> IN_REMAIN_UNDER_3B_CH_R {
        IN_REMAIN_UNDER_3B_CH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_4b_ch(&self) -> IN_REMAIN_UNDER_4B_CH_R {
        IN_REMAIN_UNDER_4B_CH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_5b_ch(&self) -> IN_REMAIN_UNDER_5B_CH_R {
        IN_REMAIN_UNDER_5B_CH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_6b_ch(&self) -> IN_REMAIN_UNDER_6B_CH_R {
        IN_REMAIN_UNDER_6B_CH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_7b_ch(&self) -> IN_REMAIN_UNDER_7B_CH_R {
        IN_REMAIN_UNDER_7B_CH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_8b_ch(&self) -> IN_REMAIN_UNDER_8B_CH_R {
        IN_REMAIN_UNDER_8B_CH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx FIFO full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_full_l1_ch(&self) -> INFIFO_FULL_L1_CH_R {
        INFIFO_FULL_L1_CH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rx FIFO empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_empty_l1_ch(&self) -> INFIFO_EMPTY_L1_CH_R {
        INFIFO_EMPTY_L1_CH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - The register stores the byte number of the data in Rx FIFO for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_cnt_l1_ch(&self) -> INFIFO_CNT_L1_CH_R {
        INFIFO_CNT_L1_CH_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - Rx FIFO full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_full_l3_ch(&self) -> INFIFO_FULL_L3_CH_R {
        INFIFO_FULL_L3_CH_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Rx FIFO empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_empty_l3_ch(&self) -> INFIFO_EMPTY_L3_CH_R {
        INFIFO_EMPTY_L3_CH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - The register stores the byte number of the data in Rx FIFO for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_cnt_l3_ch(&self) -> INFIFO_CNT_L3_CH_R {
        INFIFO_CNT_L3_CH_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFIFO_STATUS_CH")
            .field("infifo_full_l2_ch", &self.infifo_full_l2_ch())
            .field("infifo_empty_l2_ch", &self.infifo_empty_l2_ch())
            .field("infifo_cnt_l2_ch", &self.infifo_cnt_l2_ch())
            .field("in_remain_under_1b_ch", &self.in_remain_under_1b_ch())
            .field("in_remain_under_2b_ch", &self.in_remain_under_2b_ch())
            .field("in_remain_under_3b_ch", &self.in_remain_under_3b_ch())
            .field("in_remain_under_4b_ch", &self.in_remain_under_4b_ch())
            .field("in_remain_under_5b_ch", &self.in_remain_under_5b_ch())
            .field("in_remain_under_6b_ch", &self.in_remain_under_6b_ch())
            .field("in_remain_under_7b_ch", &self.in_remain_under_7b_ch())
            .field("in_remain_under_8b_ch", &self.in_remain_under_8b_ch())
            .field("infifo_full_l1_ch", &self.infifo_full_l1_ch())
            .field("infifo_empty_l1_ch", &self.infifo_empty_l1_ch())
            .field("infifo_cnt_l1_ch", &self.infifo_cnt_l1_ch())
            .field("infifo_full_l3_ch", &self.infifo_full_l3_ch())
            .field("infifo_empty_l3_ch", &self.infifo_empty_l3_ch())
            .field("infifo_cnt_l3_ch", &self.infifo_cnt_l3_ch())
            .finish()
    }
}
#[doc = "Represents the status of the rx fifo of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`infifo_status_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INFIFO_STATUS_CH_SPEC;
impl crate::RegisterSpec for INFIFO_STATUS_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`infifo_status_ch::R`](R) reader structure"]
impl crate::Readable for INFIFO_STATUS_CH_SPEC {}
#[doc = "`reset()` method sets INFIFO_STATUS_CH%s to value 0x0081_0002"]
impl crate::Resettable for INFIFO_STATUS_CH_SPEC {
    const RESET_VALUE: u32 = 0x0081_0002;
}
