#[doc = "Register `FIFO_CNT` reader"]
pub type R = crate::R<FIFO_CNT_SPEC>;
#[doc = "Field `IN_CMDFIFO_INFIFO_CNT` reader - only for debug"]
pub type IN_CMDFIFO_INFIFO_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - only for debug"]
    #[inline(always)]
    pub fn in_cmdfifo_infifo_cnt(&self) -> IN_CMDFIFO_INFIFO_CNT_R {
        IN_CMDFIFO_INFIFO_CNT_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CNT")
            .field("in_cmdfifo_infifo_cnt", &self.in_cmdfifo_infifo_cnt())
            .finish()
    }
}
#[doc = "rx CH5 fifo cnt register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_CNT_SPEC;
impl crate::RegisterSpec for FIFO_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_cnt::R`](R) reader structure"]
impl crate::Readable for FIFO_CNT_SPEC {}
#[doc = "`reset()` method sets FIFO_CNT to value 0"]
impl crate::Resettable for FIFO_CNT_SPEC {}
