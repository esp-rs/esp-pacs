#[doc = "Register `OUTFIFO_STATUS1_CH%s` reader"]
pub type R = crate::R<OUTFIFO_STATUS1_CH_SPEC>;
#[doc = "Field `L1OUTFIFO_CNT` reader - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0."]
pub type L1OUTFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `L2OUTFIFO_CNT` reader - The register stores the byte number of the data in L2 Tx FIFO for Tx channel 0."]
pub type L2OUTFIFO_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0."]
    #[inline(always)]
    pub fn l1outfifo_cnt(&self) -> L1OUTFIFO_CNT_R {
        L1OUTFIFO_CNT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - The register stores the byte number of the data in L2 Tx FIFO for Tx channel 0."]
    #[inline(always)]
    pub fn l2outfifo_cnt(&self) -> L2OUTFIFO_CNT_R {
        L2OUTFIFO_CNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTFIFO_STATUS1_CH")
            .field(
                "l1outfifo_cnt",
                &format_args!("{}", self.l1outfifo_cnt().bits()),
            )
            .field(
                "l2outfifo_cnt",
                &format_args!("{}", self.l2outfifo_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUTFIFO_STATUS1_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Receive FIFO status of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status1_ch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTFIFO_STATUS1_CH_SPEC;
impl crate::RegisterSpec for OUTFIFO_STATUS1_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outfifo_status1_ch::R`](R) reader structure"]
impl crate::Readable for OUTFIFO_STATUS1_CH_SPEC {}
#[doc = "`reset()` method sets OUTFIFO_STATUS1_CH%s to value 0"]
impl crate::Resettable for OUTFIFO_STATUS1_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
