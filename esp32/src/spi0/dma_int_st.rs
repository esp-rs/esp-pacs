#[doc = "Register `DMA_INT_ST` reader"]
pub type R = crate::R<DMA_INT_ST_SPEC>;
#[doc = "Field `INLINK_DSCR_EMPTY` reader - The status bit for lack of enough inlink descriptors."]
pub type INLINK_DSCR_EMPTY_R = crate::BitReader;
#[doc = "Field `OUTLINK_DSCR_ERROR` reader - The status bit for outlink descriptor error."]
pub type OUTLINK_DSCR_ERROR_R = crate::BitReader;
#[doc = "Field `INLINK_DSCR_ERROR` reader - The status bit for inlink descriptor error."]
pub type INLINK_DSCR_ERROR_R = crate::BitReader;
#[doc = "Field `IN_DONE` reader - The status bit for completing usage of a inlink descriptor."]
pub type IN_DONE_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF` reader - The status bit for receiving error."]
pub type IN_ERR_EOF_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` reader - The status bit for completing receiving all the packets from host."]
pub type IN_SUC_EOF_R = crate::BitReader;
#[doc = "Field `OUT_DONE` reader - The status bit for completing usage of a outlink descriptor."]
pub type OUT_DONE_R = crate::BitReader;
#[doc = "Field `OUT_EOF` reader - The status bit for sending a packet to host done."]
pub type OUT_EOF_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF` reader - The status bit for sending all the packets to host done."]
pub type OUT_TOTAL_EOF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The status bit for lack of enough inlink descriptors."]
    #[inline(always)]
    pub fn inlink_dscr_empty(&self) -> INLINK_DSCR_EMPTY_R {
        INLINK_DSCR_EMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The status bit for outlink descriptor error."]
    #[inline(always)]
    pub fn outlink_dscr_error(&self) -> OUTLINK_DSCR_ERROR_R {
        OUTLINK_DSCR_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The status bit for inlink descriptor error."]
    #[inline(always)]
    pub fn inlink_dscr_error(&self) -> INLINK_DSCR_ERROR_R {
        INLINK_DSCR_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The status bit for completing usage of a inlink descriptor."]
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status bit for receiving error."]
    #[inline(always)]
    pub fn in_err_eof(&self) -> IN_ERR_EOF_R {
        IN_ERR_EOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The status bit for completing receiving all the packets from host."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The status bit for completing usage of a outlink descriptor."]
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The status bit for sending a packet to host done."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The status bit for sending all the packets to host done."]
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_INT_ST")
            .field("inlink_dscr_empty", &self.inlink_dscr_empty())
            .field("outlink_dscr_error", &self.outlink_dscr_error())
            .field("inlink_dscr_error", &self.inlink_dscr_error())
            .field("in_done", &self.in_done())
            .field("in_err_eof", &self.in_err_eof())
            .field("in_suc_eof", &self.in_suc_eof())
            .field("out_done", &self.out_done())
            .field("out_eof", &self.out_eof())
            .field("out_total_eof", &self.out_total_eof())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_INT_ST_SPEC;
impl crate::RegisterSpec for DMA_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_st::R`](R) reader structure"]
impl crate::Readable for DMA_INT_ST_SPEC {}
#[doc = "`reset()` method sets DMA_INT_ST to value 0"]
impl crate::Resettable for DMA_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
