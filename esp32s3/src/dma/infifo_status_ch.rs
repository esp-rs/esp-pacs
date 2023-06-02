#[doc = "Register `INFIFO_STATUS_CH%s` reader"]
pub struct R(crate::R<INFIFO_STATUS_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INFIFO_STATUS_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INFIFO_STATUS_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INFIFO_STATUS_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INFIFO_FULL_L1` reader - L1 Rx FIFO full signal for Rx channel 0."]
pub type INFIFO_FULL_L1_R = crate::BitReader;
#[doc = "Field `INFIFO_EMPTY_L1` reader - L1 Rx FIFO empty signal for Rx channel 0."]
pub type INFIFO_EMPTY_L1_R = crate::BitReader;
#[doc = "Field `INFIFO_FULL_L2` reader - L2 Rx FIFO full signal for Rx channel 0."]
pub type INFIFO_FULL_L2_R = crate::BitReader;
#[doc = "Field `INFIFO_EMPTY_L2` reader - L2 Rx FIFO empty signal for Rx channel 0."]
pub type INFIFO_EMPTY_L2_R = crate::BitReader;
#[doc = "Field `INFIFO_FULL_L3` reader - L3 Rx FIFO full signal for Rx channel 0."]
pub type INFIFO_FULL_L3_R = crate::BitReader;
#[doc = "Field `INFIFO_EMPTY_L3` reader - L3 Rx FIFO empty signal for Rx channel 0."]
pub type INFIFO_EMPTY_L3_R = crate::BitReader;
#[doc = "Field `INFIFO_CNT_L1` reader - The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0."]
pub type INFIFO_CNT_L1_R = crate::FieldReader;
#[doc = "Field `INFIFO_CNT_L2` reader - The register stores the byte number of the data in L2 Rx FIFO for Rx channel 0."]
pub type INFIFO_CNT_L2_R = crate::FieldReader;
#[doc = "Field `INFIFO_CNT_L3` reader - The register stores the byte number of the data in L3 Rx FIFO for Rx channel 0."]
pub type INFIFO_CNT_L3_R = crate::FieldReader;
#[doc = "Field `IN_REMAIN_UNDER_1B_L3` reader - reserved"]
pub type IN_REMAIN_UNDER_1B_L3_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_2B_L3` reader - reserved"]
pub type IN_REMAIN_UNDER_2B_L3_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_3B_L3` reader - reserved"]
pub type IN_REMAIN_UNDER_3B_L3_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_4B_L3` reader - reserved"]
pub type IN_REMAIN_UNDER_4B_L3_R = crate::BitReader;
#[doc = "Field `IN_BUF_HUNGRY` reader - reserved"]
pub type IN_BUF_HUNGRY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - L1 Rx FIFO full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_full_l1(&self) -> INFIFO_FULL_L1_R {
        INFIFO_FULL_L1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - L1 Rx FIFO empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_empty_l1(&self) -> INFIFO_EMPTY_L1_R {
        INFIFO_EMPTY_L1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - L2 Rx FIFO full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_full_l2(&self) -> INFIFO_FULL_L2_R {
        INFIFO_FULL_L2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - L2 Rx FIFO empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_empty_l2(&self) -> INFIFO_EMPTY_L2_R {
        INFIFO_EMPTY_L2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - L3 Rx FIFO full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_full_l3(&self) -> INFIFO_FULL_L3_R {
        INFIFO_FULL_L3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - L3 Rx FIFO empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_empty_l3(&self) -> INFIFO_EMPTY_L3_R {
        INFIFO_EMPTY_L3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:11 - The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_cnt_l1(&self) -> INFIFO_CNT_L1_R {
        INFIFO_CNT_L1_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:18 - The register stores the byte number of the data in L2 Rx FIFO for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_cnt_l2(&self) -> INFIFO_CNT_L2_R {
        INFIFO_CNT_L2_R::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bits 19:23 - The register stores the byte number of the data in L3 Rx FIFO for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_cnt_l3(&self) -> INFIFO_CNT_L3_R {
        INFIFO_CNT_L3_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_1b_l3(&self) -> IN_REMAIN_UNDER_1B_L3_R {
        IN_REMAIN_UNDER_1B_L3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_2b_l3(&self) -> IN_REMAIN_UNDER_2B_L3_R {
        IN_REMAIN_UNDER_2B_L3_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_3b_l3(&self) -> IN_REMAIN_UNDER_3B_L3_R {
        IN_REMAIN_UNDER_3B_L3_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_4b_l3(&self) -> IN_REMAIN_UNDER_4B_L3_R {
        IN_REMAIN_UNDER_4B_L3_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - reserved"]
    #[inline(always)]
    pub fn in_buf_hungry(&self) -> IN_BUF_HUNGRY_R {
        IN_BUF_HUNGRY_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFIFO_STATUS_CH")
            .field(
                "infifo_full_l1",
                &format_args!("{}", self.infifo_full_l1().bit()),
            )
            .field(
                "infifo_empty_l1",
                &format_args!("{}", self.infifo_empty_l1().bit()),
            )
            .field(
                "infifo_full_l2",
                &format_args!("{}", self.infifo_full_l2().bit()),
            )
            .field(
                "infifo_empty_l2",
                &format_args!("{}", self.infifo_empty_l2().bit()),
            )
            .field(
                "infifo_full_l3",
                &format_args!("{}", self.infifo_full_l3().bit()),
            )
            .field(
                "infifo_empty_l3",
                &format_args!("{}", self.infifo_empty_l3().bit()),
            )
            .field(
                "infifo_cnt_l1",
                &format_args!("{}", self.infifo_cnt_l1().bits()),
            )
            .field(
                "infifo_cnt_l2",
                &format_args!("{}", self.infifo_cnt_l2().bits()),
            )
            .field(
                "infifo_cnt_l3",
                &format_args!("{}", self.infifo_cnt_l3().bits()),
            )
            .field(
                "in_remain_under_1b_l3",
                &format_args!("{}", self.in_remain_under_1b_l3().bit()),
            )
            .field(
                "in_remain_under_2b_l3",
                &format_args!("{}", self.in_remain_under_2b_l3().bit()),
            )
            .field(
                "in_remain_under_3b_l3",
                &format_args!("{}", self.in_remain_under_3b_l3().bit()),
            )
            .field(
                "in_remain_under_4b_l3",
                &format_args!("{}", self.in_remain_under_4b_l3().bit()),
            )
            .field(
                "in_buf_hungry",
                &format_args!("{}", self.in_buf_hungry().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INFIFO_STATUS_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Receive FIFO status of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [infifo_status_ch](index.html) module"]
pub struct INFIFO_STATUS_CH_SPEC;
impl crate::RegisterSpec for INFIFO_STATUS_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [infifo_status_ch::R](R) reader structure"]
impl crate::Readable for INFIFO_STATUS_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INFIFO_STATUS_CH%s to value 0x0f00_003a"]
impl crate::Resettable for INFIFO_STATUS_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00_003a;
}
