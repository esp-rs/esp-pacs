#[doc = "Register `DMA_INT_ST` reader"]
pub type R = crate::R<DMA_INT_ST_SPEC>;
#[doc = "Register `DMA_INT_ST` writer"]
pub type W = crate::W<DMA_INT_ST_SPEC>;
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
#[doc = "Field `INFIFO_FULL_ERR` reader - The status bit for infifo full error."]
pub type INFIFO_FULL_ERR_R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY_ERR` reader - The status bit for outfifo empty error."]
pub type OUTFIFO_EMPTY_ERR_R = crate::BitReader;
#[doc = "Field `SLV_CMD6` reader - The status bit for SPI slave CMD6 interrupt."]
pub type SLV_CMD6_R = crate::BitReader;
#[doc = "Field `SLV_CMD6` writer - The status bit for SPI slave CMD6 interrupt."]
pub type SLV_CMD6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD7` reader - The status bit for SPI slave CMD7 interrupt."]
pub type SLV_CMD7_R = crate::BitReader;
#[doc = "Field `SLV_CMD7` writer - The status bit for SPI slave CMD7 interrupt."]
pub type SLV_CMD7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD8` reader - The status bit for SPI slave CMD8 interrupt."]
pub type SLV_CMD8_R = crate::BitReader;
#[doc = "Field `SLV_CMD8` writer - The status bit for SPI slave CMD8 interrupt."]
pub type SLV_CMD8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD9` reader - The status bit for SPI slave CMD9 interrupt."]
pub type SLV_CMD9_R = crate::BitReader;
#[doc = "Field `SLV_CMD9` writer - The status bit for SPI slave CMD9 interrupt."]
pub type SLV_CMD9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMDA` reader - The status bit for SPI slave CMDA interrupt."]
pub type SLV_CMDA_R = crate::BitReader;
#[doc = "Field `SLV_CMDA` writer - The status bit for SPI slave CMDA interrupt."]
pub type SLV_CMDA_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 9 - The status bit for infifo full error."]
    #[inline(always)]
    pub fn infifo_full_err(&self) -> INFIFO_FULL_ERR_R {
        INFIFO_FULL_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The status bit for outfifo empty error."]
    #[inline(always)]
    pub fn outfifo_empty_err(&self) -> OUTFIFO_EMPTY_ERR_R {
        OUTFIFO_EMPTY_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The status bit for SPI slave CMD6 interrupt."]
    #[inline(always)]
    pub fn slv_cmd6(&self) -> SLV_CMD6_R {
        SLV_CMD6_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The status bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    pub fn slv_cmd7(&self) -> SLV_CMD7_R {
        SLV_CMD7_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The status bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    pub fn slv_cmd8(&self) -> SLV_CMD8_R {
        SLV_CMD8_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The status bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    pub fn slv_cmd9(&self) -> SLV_CMD9_R {
        SLV_CMD9_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The status bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    pub fn slv_cmda(&self) -> SLV_CMDA_R {
        SLV_CMDA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_INT_ST")
            .field(
                "inlink_dscr_empty",
                &format_args!("{}", self.inlink_dscr_empty().bit()),
            )
            .field(
                "outlink_dscr_error",
                &format_args!("{}", self.outlink_dscr_error().bit()),
            )
            .field(
                "inlink_dscr_error",
                &format_args!("{}", self.inlink_dscr_error().bit()),
            )
            .field("in_done", &format_args!("{}", self.in_done().bit()))
            .field("in_err_eof", &format_args!("{}", self.in_err_eof().bit()))
            .field("in_suc_eof", &format_args!("{}", self.in_suc_eof().bit()))
            .field("out_done", &format_args!("{}", self.out_done().bit()))
            .field("out_eof", &format_args!("{}", self.out_eof().bit()))
            .field(
                "out_total_eof",
                &format_args!("{}", self.out_total_eof().bit()),
            )
            .field(
                "infifo_full_err",
                &format_args!("{}", self.infifo_full_err().bit()),
            )
            .field(
                "outfifo_empty_err",
                &format_args!("{}", self.outfifo_empty_err().bit()),
            )
            .field("slv_cmd6", &format_args!("{}", self.slv_cmd6().bit()))
            .field("slv_cmd7", &format_args!("{}", self.slv_cmd7().bit()))
            .field("slv_cmd8", &format_args!("{}", self.slv_cmd8().bit()))
            .field("slv_cmd9", &format_args!("{}", self.slv_cmd9().bit()))
            .field("slv_cmda", &format_args!("{}", self.slv_cmda().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 11 - The status bit for SPI slave CMD6 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd6(&mut self) -> SLV_CMD6_W<DMA_INT_ST_SPEC> {
        SLV_CMD6_W::new(self, 11)
    }
    #[doc = "Bit 12 - The status bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd7(&mut self) -> SLV_CMD7_W<DMA_INT_ST_SPEC> {
        SLV_CMD7_W::new(self, 12)
    }
    #[doc = "Bit 13 - The status bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd8(&mut self) -> SLV_CMD8_W<DMA_INT_ST_SPEC> {
        SLV_CMD8_W::new(self, 13)
    }
    #[doc = "Bit 14 - The status bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd9(&mut self) -> SLV_CMD9_W<DMA_INT_ST_SPEC> {
        SLV_CMD9_W::new(self, 14)
    }
    #[doc = "Bit 15 - The status bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmda(&mut self) -> SLV_CMDA_W<DMA_INT_ST_SPEC> {
        SLV_CMDA_W::new(self, 15)
    }
}
#[doc = "SPI DMA interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_int_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_INT_ST_SPEC;
impl crate::RegisterSpec for DMA_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_st::R`](R) reader structure"]
impl crate::Readable for DMA_INT_ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_int_st::W`](W) writer structure"]
impl crate::Writable for DMA_INT_ST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_INT_ST to value 0"]
impl crate::Resettable for DMA_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
