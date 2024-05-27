///Register `DMA_INT_ENA` reader
pub type R = crate::R<DMA_INT_ENA_SPEC>;
///Register `DMA_INT_ENA` writer
pub type W = crate::W<DMA_INT_ENA_SPEC>;
///Field `INLINK_DSCR_EMPTY` reader - The enable bit for lack of enough inlink descriptors.
pub type INLINK_DSCR_EMPTY_R = crate::BitReader;
///Field `INLINK_DSCR_EMPTY` writer - The enable bit for lack of enough inlink descriptors.
pub type INLINK_DSCR_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTLINK_DSCR_ERROR` reader - The enable bit for outlink descriptor error.
pub type OUTLINK_DSCR_ERROR_R = crate::BitReader;
///Field `OUTLINK_DSCR_ERROR` writer - The enable bit for outlink descriptor error.
pub type OUTLINK_DSCR_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INLINK_DSCR_ERROR` reader - The enable bit for inlink descriptor error.
pub type INLINK_DSCR_ERROR_R = crate::BitReader;
///Field `INLINK_DSCR_ERROR` writer - The enable bit for inlink descriptor error.
pub type INLINK_DSCR_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN_DONE` reader - The enable bit for completing usage of a inlink descriptor.
pub type IN_DONE_R = crate::BitReader;
///Field `IN_DONE` writer - The enable bit for completing usage of a inlink descriptor.
pub type IN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN_ERR_EOF` reader - The enable bit for receiving error.
pub type IN_ERR_EOF_R = crate::BitReader;
///Field `IN_ERR_EOF` writer - The enable bit for receiving error.
pub type IN_ERR_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN_SUC_EOF` reader - The enable bit for completing receiving all the packets from host.
pub type IN_SUC_EOF_R = crate::BitReader;
///Field `IN_SUC_EOF` writer - The enable bit for completing receiving all the packets from host.
pub type IN_SUC_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_DONE` reader - The enable bit for completing usage of a outlink descriptor .
pub type OUT_DONE_R = crate::BitReader;
///Field `OUT_DONE` writer - The enable bit for completing usage of a outlink descriptor .
pub type OUT_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_EOF` reader - The enable bit for sending a packet to host done.
pub type OUT_EOF_R = crate::BitReader;
///Field `OUT_EOF` writer - The enable bit for sending a packet to host done.
pub type OUT_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_TOTAL_EOF` reader - The enable bit for sending all the packets to host done.
pub type OUT_TOTAL_EOF_R = crate::BitReader;
///Field `OUT_TOTAL_EOF` writer - The enable bit for sending all the packets to host done.
pub type OUT_TOTAL_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The enable bit for lack of enough inlink descriptors.
    #[inline(always)]
    pub fn inlink_dscr_empty(&self) -> INLINK_DSCR_EMPTY_R {
        INLINK_DSCR_EMPTY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The enable bit for outlink descriptor error.
    #[inline(always)]
    pub fn outlink_dscr_error(&self) -> OUTLINK_DSCR_ERROR_R {
        OUTLINK_DSCR_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The enable bit for inlink descriptor error.
    #[inline(always)]
    pub fn inlink_dscr_error(&self) -> INLINK_DSCR_ERROR_R {
        INLINK_DSCR_ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The enable bit for completing usage of a inlink descriptor.
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - The enable bit for receiving error.
    #[inline(always)]
    pub fn in_err_eof(&self) -> IN_ERR_EOF_R {
        IN_ERR_EOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - The enable bit for completing receiving all the packets from host.
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - The enable bit for completing usage of a outlink descriptor .
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - The enable bit for sending a packet to host done.
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - The enable bit for sending all the packets to host done.
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_INT_ENA")
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
impl W {
    ///Bit 0 - The enable bit for lack of enough inlink descriptors.
    #[inline(always)]
    #[must_use]
    pub fn inlink_dscr_empty(&mut self) -> INLINK_DSCR_EMPTY_W<DMA_INT_ENA_SPEC> {
        INLINK_DSCR_EMPTY_W::new(self, 0)
    }
    ///Bit 1 - The enable bit for outlink descriptor error.
    #[inline(always)]
    #[must_use]
    pub fn outlink_dscr_error(&mut self) -> OUTLINK_DSCR_ERROR_W<DMA_INT_ENA_SPEC> {
        OUTLINK_DSCR_ERROR_W::new(self, 1)
    }
    ///Bit 2 - The enable bit for inlink descriptor error.
    #[inline(always)]
    #[must_use]
    pub fn inlink_dscr_error(&mut self) -> INLINK_DSCR_ERROR_W<DMA_INT_ENA_SPEC> {
        INLINK_DSCR_ERROR_W::new(self, 2)
    }
    ///Bit 3 - The enable bit for completing usage of a inlink descriptor.
    #[inline(always)]
    #[must_use]
    pub fn in_done(&mut self) -> IN_DONE_W<DMA_INT_ENA_SPEC> {
        IN_DONE_W::new(self, 3)
    }
    ///Bit 4 - The enable bit for receiving error.
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof(&mut self) -> IN_ERR_EOF_W<DMA_INT_ENA_SPEC> {
        IN_ERR_EOF_W::new(self, 4)
    }
    ///Bit 5 - The enable bit for completing receiving all the packets from host.
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<DMA_INT_ENA_SPEC> {
        IN_SUC_EOF_W::new(self, 5)
    }
    ///Bit 6 - The enable bit for completing usage of a outlink descriptor .
    #[inline(always)]
    #[must_use]
    pub fn out_done(&mut self) -> OUT_DONE_W<DMA_INT_ENA_SPEC> {
        OUT_DONE_W::new(self, 6)
    }
    ///Bit 7 - The enable bit for sending a packet to host done.
    #[inline(always)]
    #[must_use]
    pub fn out_eof(&mut self) -> OUT_EOF_W<DMA_INT_ENA_SPEC> {
        OUT_EOF_W::new(self, 7)
    }
    ///Bit 8 - The enable bit for sending all the packets to host done.
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof(&mut self) -> OUT_TOTAL_EOF_W<DMA_INT_ENA_SPEC> {
        OUT_TOTAL_EOF_W::new(self, 8)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`dma_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_INT_ENA_SPEC;
impl crate::RegisterSpec for DMA_INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma_int_ena::R`](R) reader structure
impl crate::Readable for DMA_INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`dma_int_ena::W`](W) writer structure
impl crate::Writable for DMA_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMA_INT_ENA to value 0
impl crate::Resettable for DMA_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
