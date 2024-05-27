#[doc = "Register `TX_CH%sDATA` reader"]
pub type R = crate::R<TX_CHDATA_SPEC>;
#[doc = "Field `CHDATA` reader - Read and write data for channel %s via APB FIFO."]
pub type CHDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read and write data for channel %s via APB FIFO."]
    #[inline(always)]
    pub fn chdata(&self) -> CHDATA_R {
        CHDATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CHDATA")
            .field("chdata", &self.chdata())
            .finish()
    }
}
#[doc = "The read and write data register for CHANNEL%s by apb fifo access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_chdata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CHDATA_SPEC;
impl crate::RegisterSpec for TX_CHDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_chdata::R`](R) reader structure"]
impl crate::Readable for TX_CHDATA_SPEC {}
#[doc = "`reset()` method sets TX_CH%sDATA to value 0"]
impl crate::Resettable for TX_CHDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
