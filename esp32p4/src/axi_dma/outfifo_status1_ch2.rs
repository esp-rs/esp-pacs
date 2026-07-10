#[doc = "Register `OUTFIFO_STATUS1_CH2` reader"]
pub type R = crate::R<OUTFIFO_STATUS1_CH2_SPEC>;
#[doc = "Field `L1OUTFIFO_CNT_CH2` reader - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 2."]
pub type L1OUTFIFO_CNT_CH2_R = crate::FieldReader;
#[doc = "Field `L2OUTFIFO_CNT_CH2` reader - The register stores the byte number of the data in L2 Tx FIFO for Tx channel 2."]
pub type L2OUTFIFO_CNT_CH2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 2."]
    #[inline(always)]
    pub fn l1outfifo_cnt_ch2(&self) -> L1OUTFIFO_CNT_CH2_R {
        L1OUTFIFO_CNT_CH2_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - The register stores the byte number of the data in L2 Tx FIFO for Tx channel 2."]
    #[inline(always)]
    pub fn l2outfifo_cnt_ch2(&self) -> L2OUTFIFO_CNT_CH2_R {
        L2OUTFIFO_CNT_CH2_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTFIFO_STATUS1_CH2")
            .field("l1outfifo_cnt_ch2", &self.l1outfifo_cnt_ch2())
            .field("l2outfifo_cnt_ch2", &self.l2outfifo_cnt_ch2())
            .finish()
    }
}
#[doc = "Receive FIFO status of Tx channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_status1_ch2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTFIFO_STATUS1_CH2_SPEC;
impl crate::RegisterSpec for OUTFIFO_STATUS1_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outfifo_status1_ch2::R`](R) reader structure"]
impl crate::Readable for OUTFIFO_STATUS1_CH2_SPEC {}
#[doc = "`reset()` method sets OUTFIFO_STATUS1_CH2 to value 0"]
impl crate::Resettable for OUTFIFO_STATUS1_CH2_SPEC {}
