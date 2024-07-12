#[doc = "Register `FIFO_FLOW_STATUS` reader"]
pub type R = crate::R<FIFO_FLOW_STATUS_SPEC>;
#[doc = "Field `RAW_BUF_DEPTH` reader - this field configures the depth of dsi_bridge fifo depth"]
pub type RAW_BUF_DEPTH_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - this field configures the depth of dsi_bridge fifo depth"]
    #[inline(always)]
    pub fn raw_buf_depth(&self) -> RAW_BUF_DEPTH_R {
        RAW_BUF_DEPTH_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_FLOW_STATUS")
            .field("raw_buf_depth", &self.raw_buf_depth())
            .finish()
    }
}
#[doc = "dsi bridge raw buffer depth register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_flow_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_FLOW_STATUS_SPEC;
impl crate::RegisterSpec for FIFO_FLOW_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_flow_status::R`](R) reader structure"]
impl crate::Readable for FIFO_FLOW_STATUS_SPEC {}
#[doc = "`reset()` method sets FIFO_FLOW_STATUS to value 0"]
impl crate::Resettable for FIFO_FLOW_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
