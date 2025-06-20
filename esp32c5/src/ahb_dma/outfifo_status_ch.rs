#[doc = "Register `OUTFIFO_STATUS_CH%s` reader"]
pub type R = crate::R<OUTFIFO_STATUS_CH_SPEC>;
#[doc = "Field `OUTFIFO_FULL_CH` reader - Represents whether or not L1 TX FIFO is full.\\\\0: Not Full\\\\1: Full\\\\"]
pub type OUTFIFO_FULL_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY_CH` reader - Represents whether or not L1 TX FIFO is empty.\\\\0: Not empty\\\\1: Empty\\\\"]
pub type OUTFIFO_EMPTY_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_CNT_CH` reader - Represents the number of data bytes in L1 TX FIFO for TX channel %s."]
pub type OUTFIFO_CNT_CH_R = crate::FieldReader;
#[doc = "Field `OUT_REMAIN_UNDER_1B_CH` reader - Reserved."]
pub type OUT_REMAIN_UNDER_1B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_2B_CH` reader - Reserved."]
pub type OUT_REMAIN_UNDER_2B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_3B_CH` reader - Reserved."]
pub type OUT_REMAIN_UNDER_3B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_4B_CH` reader - Reserved."]
pub type OUT_REMAIN_UNDER_4B_CH_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents whether or not L1 TX FIFO is full.\\\\0: Not Full\\\\1: Full\\\\"]
    #[inline(always)]
    pub fn outfifo_full_ch(&self) -> OUTFIFO_FULL_CH_R {
        OUTFIFO_FULL_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents whether or not L1 TX FIFO is empty.\\\\0: Not empty\\\\1: Empty\\\\"]
    #[inline(always)]
    pub fn outfifo_empty_ch(&self) -> OUTFIFO_EMPTY_CH_R {
        OUTFIFO_EMPTY_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Represents the number of data bytes in L1 TX FIFO for TX channel %s."]
    #[inline(always)]
    pub fn outfifo_cnt_ch(&self) -> OUTFIFO_CNT_CH_R {
        OUTFIFO_CNT_CH_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Reserved."]
    #[inline(always)]
    pub fn out_remain_under_1b_ch(&self) -> OUT_REMAIN_UNDER_1B_CH_R {
        OUT_REMAIN_UNDER_1B_CH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reserved."]
    #[inline(always)]
    pub fn out_remain_under_2b_ch(&self) -> OUT_REMAIN_UNDER_2B_CH_R {
        OUT_REMAIN_UNDER_2B_CH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved."]
    #[inline(always)]
    pub fn out_remain_under_3b_ch(&self) -> OUT_REMAIN_UNDER_3B_CH_R {
        OUT_REMAIN_UNDER_3B_CH_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reserved."]
    #[inline(always)]
    pub fn out_remain_under_4b_ch(&self) -> OUT_REMAIN_UNDER_4B_CH_R {
        OUT_REMAIN_UNDER_4B_CH_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTFIFO_STATUS_CH")
            .field("outfifo_full_ch", &self.outfifo_full_ch())
            .field("outfifo_empty_ch", &self.outfifo_empty_ch())
            .field("outfifo_cnt_ch", &self.outfifo_cnt_ch())
            .field("out_remain_under_1b_ch", &self.out_remain_under_1b_ch())
            .field("out_remain_under_2b_ch", &self.out_remain_under_2b_ch())
            .field("out_remain_under_3b_ch", &self.out_remain_under_3b_ch())
            .field("out_remain_under_4b_ch", &self.out_remain_under_4b_ch())
            .finish()
    }
}
#[doc = "Transmit FIFO status of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_status_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTFIFO_STATUS_CH_SPEC;
impl crate::RegisterSpec for OUTFIFO_STATUS_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outfifo_status_ch::R`](R) reader structure"]
impl crate::Readable for OUTFIFO_STATUS_CH_SPEC {}
#[doc = "`reset()` method sets OUTFIFO_STATUS_CH%s to value 0x0780_0002"]
impl crate::Resettable for OUTFIFO_STATUS_CH_SPEC {
    const RESET_VALUE: u32 = 0x0780_0002;
}
