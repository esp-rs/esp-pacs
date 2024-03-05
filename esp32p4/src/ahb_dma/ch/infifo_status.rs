#[doc = "Register `INFIFO_STATUS` reader"]
pub type R = crate::R<INFIFO_STATUS_SPEC>;
#[doc = "Field `INFIFO_FULL_CH` reader - L1 Rx FIFO full signal for Rx channel 0."]
pub type INFIFO_FULL_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_EMPTY_CH` reader - L1 Rx FIFO empty signal for Rx channel 0."]
pub type INFIFO_EMPTY_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_CNT_CH` reader - The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0."]
pub type INFIFO_CNT_CH_R = crate::FieldReader;
#[doc = "Field `IN_REMAIN_UNDER_1B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_1B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_2B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_2B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_3B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_3B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_4B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_4B_CH_R = crate::BitReader;
#[doc = "Field `IN_BUF_HUNGRY_CH` reader - reserved"]
pub type IN_BUF_HUNGRY_CH_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - L1 Rx FIFO full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_full_ch(&self) -> INFIFO_FULL_CH_R {
        INFIFO_FULL_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - L1 Rx FIFO empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_empty_ch(&self) -> INFIFO_EMPTY_CH_R {
        INFIFO_EMPTY_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_cnt_ch(&self) -> INFIFO_CNT_CH_R {
        INFIFO_CNT_CH_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_1b_ch(&self) -> IN_REMAIN_UNDER_1B_CH_R {
        IN_REMAIN_UNDER_1B_CH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_2b_ch(&self) -> IN_REMAIN_UNDER_2B_CH_R {
        IN_REMAIN_UNDER_2B_CH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_3b_ch(&self) -> IN_REMAIN_UNDER_3B_CH_R {
        IN_REMAIN_UNDER_3B_CH_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_4b_ch(&self) -> IN_REMAIN_UNDER_4B_CH_R {
        IN_REMAIN_UNDER_4B_CH_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn in_buf_hungry_ch(&self) -> IN_BUF_HUNGRY_CH_R {
        IN_BUF_HUNGRY_CH_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFIFO_STATUS")
            .field(
                "infifo_full_ch",
                &format_args!("{}", self.infifo_full_ch().bit()),
            )
            .field(
                "infifo_empty_ch",
                &format_args!("{}", self.infifo_empty_ch().bit()),
            )
            .field(
                "infifo_cnt_ch",
                &format_args!("{}", self.infifo_cnt_ch().bits()),
            )
            .field(
                "in_remain_under_1b_ch",
                &format_args!("{}", self.in_remain_under_1b_ch().bit()),
            )
            .field(
                "in_remain_under_2b_ch",
                &format_args!("{}", self.in_remain_under_2b_ch().bit()),
            )
            .field(
                "in_remain_under_3b_ch",
                &format_args!("{}", self.in_remain_under_3b_ch().bit()),
            )
            .field(
                "in_remain_under_4b_ch",
                &format_args!("{}", self.in_remain_under_4b_ch().bit()),
            )
            .field(
                "in_buf_hungry_ch",
                &format_args!("{}", self.in_buf_hungry_ch().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INFIFO_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Receive FIFO status of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INFIFO_STATUS_SPEC;
impl crate::RegisterSpec for INFIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`infifo_status::R`](R) reader structure"]
impl crate::Readable for INFIFO_STATUS_SPEC {}
#[doc = "`reset()` method sets INFIFO_STATUS to value 0x0780_0003"]
impl crate::Resettable for INFIFO_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x0780_0003;
}
