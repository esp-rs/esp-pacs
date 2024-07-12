#[doc = "Register `FIFO_STATUS` reader"]
pub type R = crate::R<FIFO_STATUS_SPEC>;
#[doc = "Field `OUTFIFO_FULL_L2` reader - Tx FIFO full signal for Tx channel 0."]
pub type OUTFIFO_FULL_L2_R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY_L2` reader - Tx FIFO empty signal for Tx channel 0."]
pub type OUTFIFO_EMPTY_L2_R = crate::BitReader;
#[doc = "Field `OUTFIFO_CNT_L2` reader - The register stores the byte number of the data in Tx FIFO for Tx channel 0."]
pub type OUTFIFO_CNT_L2_R = crate::FieldReader;
#[doc = "Field `OUTFIFO_FULL_L1` reader - Tx FIFO full signal for Tx channel 0."]
pub type OUTFIFO_FULL_L1_R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY_L1` reader - Tx FIFO empty signal for Tx channel 0."]
pub type OUTFIFO_EMPTY_L1_R = crate::BitReader;
#[doc = "Field `OUTFIFO_CNT_L1` reader - The register stores the byte number of the data in Tx FIFO for Tx channel 0."]
pub type OUTFIFO_CNT_L1_R = crate::FieldReader;
#[doc = "Field `OUTFIFO_FULL_L3` reader - Tx FIFO full signal for Tx channel 0."]
pub type OUTFIFO_FULL_L3_R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY_L3` reader - Tx FIFO empty signal for Tx channel 0."]
pub type OUTFIFO_EMPTY_L3_R = crate::BitReader;
#[doc = "Field `OUTFIFO_CNT_L3` reader - The register stores the 8byte number of the data in Tx FIFO for Tx channel 0."]
pub type OUTFIFO_CNT_L3_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Tx FIFO full signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_full_l2(&self) -> OUTFIFO_FULL_L2_R {
        OUTFIFO_FULL_L2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx FIFO empty signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_empty_l2(&self) -> OUTFIFO_EMPTY_L2_R {
        OUTFIFO_EMPTY_L2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - The register stores the byte number of the data in Tx FIFO for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_cnt_l2(&self) -> OUTFIFO_CNT_L2_R {
        OUTFIFO_CNT_L2_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Tx FIFO full signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_full_l1(&self) -> OUTFIFO_FULL_L1_R {
        OUTFIFO_FULL_L1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx FIFO empty signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_empty_l1(&self) -> OUTFIFO_EMPTY_L1_R {
        OUTFIFO_EMPTY_L1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - The register stores the byte number of the data in Tx FIFO for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_cnt_l1(&self) -> OUTFIFO_CNT_L1_R {
        OUTFIFO_CNT_L1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Tx FIFO full signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_full_l3(&self) -> OUTFIFO_FULL_L3_R {
        OUTFIFO_FULL_L3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tx FIFO empty signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_empty_l3(&self) -> OUTFIFO_EMPTY_L3_R {
        OUTFIFO_EMPTY_L3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - The register stores the 8byte number of the data in Tx FIFO for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_cnt_l3(&self) -> OUTFIFO_CNT_L3_R {
        OUTFIFO_CNT_L3_R::new(((self.bits >> 18) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_STATUS")
            .field("outfifo_full_l2", &self.outfifo_full_l2())
            .field("outfifo_empty_l2", &self.outfifo_empty_l2())
            .field("outfifo_cnt_l2", &self.outfifo_cnt_l2())
            .field("outfifo_full_l1", &self.outfifo_full_l1())
            .field("outfifo_empty_l1", &self.outfifo_empty_l1())
            .field("outfifo_cnt_l1", &self.outfifo_cnt_l1())
            .field("outfifo_full_l3", &self.outfifo_full_l3())
            .field("outfifo_empty_l3", &self.outfifo_empty_l3())
            .field("outfifo_cnt_l3", &self.outfifo_cnt_l3())
            .finish()
    }
}
#[doc = "TX CHx outfifo status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_STATUS_SPEC;
impl crate::RegisterSpec for FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_status::R`](R) reader structure"]
impl crate::Readable for FIFO_STATUS_SPEC {}
#[doc = "`reset()` method sets FIFO_STATUS to value 0x0002_0082"]
impl crate::Resettable for FIFO_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x0002_0082;
}
