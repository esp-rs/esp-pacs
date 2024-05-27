///Register `INFIFO_STATUS` reader
pub type R = crate::R<INFIFO_STATUS_SPEC>;
///Field `INFIFO_FULL_L1` reader - L1 Rx FIFO full signal for Rx channel 0.
pub type INFIFO_FULL_L1_R = crate::BitReader;
///Field `INFIFO_EMPTY_L1` reader - L1 Rx FIFO empty signal for Rx channel 0.
pub type INFIFO_EMPTY_L1_R = crate::BitReader;
///Field `INFIFO_FULL_L2` reader - L2 Rx FIFO full signal for Rx channel 0.
pub type INFIFO_FULL_L2_R = crate::BitReader;
///Field `INFIFO_EMPTY_L2` reader - L2 Rx FIFO empty signal for Rx channel 0.
pub type INFIFO_EMPTY_L2_R = crate::BitReader;
///Field `INFIFO_FULL_L3` reader - L3 Rx FIFO full signal for Rx channel 0.
pub type INFIFO_FULL_L3_R = crate::BitReader;
///Field `INFIFO_EMPTY_L3` reader - L3 Rx FIFO empty signal for Rx channel 0.
pub type INFIFO_EMPTY_L3_R = crate::BitReader;
///Field `INFIFO_CNT_L1` reader - The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0.
pub type INFIFO_CNT_L1_R = crate::FieldReader;
///Field `INFIFO_CNT_L2` reader - The register stores the byte number of the data in L2 Rx FIFO for Rx channel 0.
pub type INFIFO_CNT_L2_R = crate::FieldReader;
///Field `INFIFO_CNT_L3` reader - The register stores the byte number of the data in L3 Rx FIFO for Rx channel 0.
pub type INFIFO_CNT_L3_R = crate::FieldReader;
///Field `IN_REMAIN_UNDER_1B_L3` reader - reserved
pub type IN_REMAIN_UNDER_1B_L3_R = crate::BitReader;
///Field `IN_REMAIN_UNDER_2B_L3` reader - reserved
pub type IN_REMAIN_UNDER_2B_L3_R = crate::BitReader;
///Field `IN_REMAIN_UNDER_3B_L3` reader - reserved
pub type IN_REMAIN_UNDER_3B_L3_R = crate::BitReader;
///Field `IN_REMAIN_UNDER_4B_L3` reader - reserved
pub type IN_REMAIN_UNDER_4B_L3_R = crate::BitReader;
///Field `IN_BUF_HUNGRY` reader - reserved
pub type IN_BUF_HUNGRY_R = crate::BitReader;
impl R {
    ///Bit 0 - L1 Rx FIFO full signal for Rx channel 0.
    #[inline(always)]
    pub fn infifo_full_l1(&self) -> INFIFO_FULL_L1_R {
        INFIFO_FULL_L1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - L1 Rx FIFO empty signal for Rx channel 0.
    #[inline(always)]
    pub fn infifo_empty_l1(&self) -> INFIFO_EMPTY_L1_R {
        INFIFO_EMPTY_L1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - L2 Rx FIFO full signal for Rx channel 0.
    #[inline(always)]
    pub fn infifo_full_l2(&self) -> INFIFO_FULL_L2_R {
        INFIFO_FULL_L2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - L2 Rx FIFO empty signal for Rx channel 0.
    #[inline(always)]
    pub fn infifo_empty_l2(&self) -> INFIFO_EMPTY_L2_R {
        INFIFO_EMPTY_L2_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - L3 Rx FIFO full signal for Rx channel 0.
    #[inline(always)]
    pub fn infifo_full_l3(&self) -> INFIFO_FULL_L3_R {
        INFIFO_FULL_L3_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - L3 Rx FIFO empty signal for Rx channel 0.
    #[inline(always)]
    pub fn infifo_empty_l3(&self) -> INFIFO_EMPTY_L3_R {
        INFIFO_EMPTY_L3_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:11 - The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0.
    #[inline(always)]
    pub fn infifo_cnt_l1(&self) -> INFIFO_CNT_L1_R {
        INFIFO_CNT_L1_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    ///Bits 12:18 - The register stores the byte number of the data in L2 Rx FIFO for Rx channel 0.
    #[inline(always)]
    pub fn infifo_cnt_l2(&self) -> INFIFO_CNT_L2_R {
        INFIFO_CNT_L2_R::new(((self.bits >> 12) & 0x7f) as u8)
    }
    ///Bits 19:23 - The register stores the byte number of the data in L3 Rx FIFO for Rx channel 0.
    #[inline(always)]
    pub fn infifo_cnt_l3(&self) -> INFIFO_CNT_L3_R {
        INFIFO_CNT_L3_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bit 24 - reserved
    #[inline(always)]
    pub fn in_remain_under_1b_l3(&self) -> IN_REMAIN_UNDER_1B_L3_R {
        IN_REMAIN_UNDER_1B_L3_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - reserved
    #[inline(always)]
    pub fn in_remain_under_2b_l3(&self) -> IN_REMAIN_UNDER_2B_L3_R {
        IN_REMAIN_UNDER_2B_L3_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - reserved
    #[inline(always)]
    pub fn in_remain_under_3b_l3(&self) -> IN_REMAIN_UNDER_3B_L3_R {
        IN_REMAIN_UNDER_3B_L3_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - reserved
    #[inline(always)]
    pub fn in_remain_under_4b_l3(&self) -> IN_REMAIN_UNDER_4B_L3_R {
        IN_REMAIN_UNDER_4B_L3_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - reserved
    #[inline(always)]
    pub fn in_buf_hungry(&self) -> IN_BUF_HUNGRY_R {
        IN_BUF_HUNGRY_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFIFO_STATUS")
            .field("infifo_full_l1", &self.infifo_full_l1())
            .field("infifo_empty_l1", &self.infifo_empty_l1())
            .field("infifo_full_l2", &self.infifo_full_l2())
            .field("infifo_empty_l2", &self.infifo_empty_l2())
            .field("infifo_full_l3", &self.infifo_full_l3())
            .field("infifo_empty_l3", &self.infifo_empty_l3())
            .field("infifo_cnt_l1", &self.infifo_cnt_l1())
            .field("infifo_cnt_l2", &self.infifo_cnt_l2())
            .field("infifo_cnt_l3", &self.infifo_cnt_l3())
            .field("in_remain_under_1b_l3", &self.in_remain_under_1b_l3())
            .field("in_remain_under_2b_l3", &self.in_remain_under_2b_l3())
            .field("in_remain_under_3b_l3", &self.in_remain_under_3b_l3())
            .field("in_remain_under_4b_l3", &self.in_remain_under_4b_l3())
            .field("in_buf_hungry", &self.in_buf_hungry())
            .finish()
    }
}
/**Receive FIFO status of Rx channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`infifo_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INFIFO_STATUS_SPEC;
impl crate::RegisterSpec for INFIFO_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`infifo_status::R`](R) reader structure
impl crate::Readable for INFIFO_STATUS_SPEC {}
///`reset()` method sets INFIFO_STATUS to value 0x0f00_003a
impl crate::Resettable for INFIFO_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x0f00_003a;
}
