#[doc = "Register `INFIFO_STATUS1_CH1` reader"]
pub type R = crate::R<INFIFO_STATUS1_CH1_SPEC>;
#[doc = "Field `L1INFIFO_CNT_CH1` reader - The register stores the byte number of the data in L1 Rx FIFO for Rx channel 1."]
pub type L1INFIFO_CNT_CH1_R = crate::FieldReader;
#[doc = "Field `L2INFIFO_CNT_CH1` reader - The register stores the byte number of the data in L2 Rx FIFO for Rx channel 1."]
pub type L2INFIFO_CNT_CH1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - The register stores the byte number of the data in L1 Rx FIFO for Rx channel 1."]
    #[inline(always)]
    pub fn l1infifo_cnt_ch1(&self) -> L1INFIFO_CNT_CH1_R {
        L1INFIFO_CNT_CH1_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - The register stores the byte number of the data in L2 Rx FIFO for Rx channel 1."]
    #[inline(always)]
    pub fn l2infifo_cnt_ch1(&self) -> L2INFIFO_CNT_CH1_R {
        L2INFIFO_CNT_CH1_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFIFO_STATUS1_CH1")
            .field("l1infifo_cnt_ch1", &self.l1infifo_cnt_ch1())
            .field("l2infifo_cnt_ch1", &self.l2infifo_cnt_ch1())
            .finish()
    }
}
#[doc = "Receive FIFO status of Rx channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`infifo_status1_ch1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INFIFO_STATUS1_CH1_SPEC;
impl crate::RegisterSpec for INFIFO_STATUS1_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`infifo_status1_ch1::R`](R) reader structure"]
impl crate::Readable for INFIFO_STATUS1_CH1_SPEC {}
#[doc = "`reset()` method sets INFIFO_STATUS1_CH1 to value 0"]
impl crate::Resettable for INFIFO_STATUS1_CH1_SPEC {}
