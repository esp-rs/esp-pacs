#[doc = "Register `INFIFO_STATUS1_CH%s` reader"]
pub type R = crate::R<INFIFO_STATUS1_CH_SPEC>;
#[doc = "Field `L1INFIFO_CNT` reader - The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0."]
pub type L1INFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `L2INFIFO_CNT` reader - The register stores the byte number of the data in L2 Rx FIFO for Rx channel 0."]
pub type L2INFIFO_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0."]
    #[inline(always)]
    pub fn l1infifo_cnt(&self) -> L1INFIFO_CNT_R {
        L1INFIFO_CNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - The register stores the byte number of the data in L2 Rx FIFO for Rx channel 0."]
    #[inline(always)]
    pub fn l2infifo_cnt(&self) -> L2INFIFO_CNT_R {
        L2INFIFO_CNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFIFO_STATUS1_CH")
            .field("l1infifo_cnt", &self.l1infifo_cnt())
            .field("l2infifo_cnt", &self.l2infifo_cnt())
            .finish()
    }
}
#[doc = "Receive FIFO status of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`infifo_status1_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INFIFO_STATUS1_CH_SPEC;
impl crate::RegisterSpec for INFIFO_STATUS1_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`infifo_status1_ch::R`](R) reader structure"]
impl crate::Readable for INFIFO_STATUS1_CH_SPEC {}
#[doc = "`reset()` method sets INFIFO_STATUS1_CH%s to value 0"]
impl crate::Resettable for INFIFO_STATUS1_CH_SPEC {}
