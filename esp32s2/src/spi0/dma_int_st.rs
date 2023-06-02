#[doc = "Register `DMA_INT_ST` reader"]
pub struct R(crate::R<DMA_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_INT_ST` writer"]
pub struct W(crate::W<DMA_INT_ST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_INT_ST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INT_ST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INLINK_DSCR_EMPTY_INT_ST` reader - The status bit for lack of enough inlink descriptors."]
pub type INLINK_DSCR_EMPTY_INT_ST_R = crate::BitReader;
#[doc = "Field `OUTLINK_DSCR_ERROR_INT_ST` reader - The status bit for outlink descriptor error."]
pub type OUTLINK_DSCR_ERROR_INT_ST_R = crate::BitReader;
#[doc = "Field `INLINK_DSCR_ERROR_INT_ST` reader - The status bit for inlink descriptor error."]
pub type INLINK_DSCR_ERROR_INT_ST_R = crate::BitReader;
#[doc = "Field `IN_DONE_INT_ST` reader - The status bit for completing usage of a inlink descriptor."]
pub type IN_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_INT_ST` reader - The status bit for receiving error."]
pub type IN_ERR_EOF_INT_ST_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_INT_ST` reader - The status bit for completing receiving all the packets from host."]
pub type IN_SUC_EOF_INT_ST_R = crate::BitReader;
#[doc = "Field `OUT_DONE_INT_ST` reader - The status bit for completing usage of a outlink descriptor."]
pub type OUT_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `OUT_EOF_INT_ST` reader - The status bit for sending a packet to host done."]
pub type OUT_EOF_INT_ST_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_INT_ST` reader - The status bit for sending all the packets to host done."]
pub type OUT_TOTAL_EOF_INT_ST_R = crate::BitReader;
#[doc = "Field `INFIFO_FULL_ERR_INT_ST` reader - The status bit for infifo full error."]
pub type INFIFO_FULL_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY_ERR_INT_ST` reader - The status bit for outfifo empty error."]
pub type OUTFIFO_EMPTY_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_CMD6_INT_ST` reader - The status bit for SPI slave CMD6 interrupt."]
pub type SLV_CMD6_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_CMD6_INT_ST` writer - The status bit for SPI slave CMD6 interrupt."]
pub type SLV_CMD6_INT_ST_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ST_SPEC, O>;
#[doc = "Field `SLV_CMD7_INT_ST` reader - The status bit for SPI slave CMD7 interrupt."]
pub type SLV_CMD7_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_CMD7_INT_ST` writer - The status bit for SPI slave CMD7 interrupt."]
pub type SLV_CMD7_INT_ST_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ST_SPEC, O>;
#[doc = "Field `SLV_CMD8_INT_ST` reader - The status bit for SPI slave CMD8 interrupt."]
pub type SLV_CMD8_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_CMD8_INT_ST` writer - The status bit for SPI slave CMD8 interrupt."]
pub type SLV_CMD8_INT_ST_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ST_SPEC, O>;
#[doc = "Field `SLV_CMD9_INT_ST` reader - The status bit for SPI slave CMD9 interrupt."]
pub type SLV_CMD9_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_CMD9_INT_ST` writer - The status bit for SPI slave CMD9 interrupt."]
pub type SLV_CMD9_INT_ST_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ST_SPEC, O>;
#[doc = "Field `SLV_CMDA_INT_ST` reader - The status bit for SPI slave CMDA interrupt."]
pub type SLV_CMDA_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_CMDA_INT_ST` writer - The status bit for SPI slave CMDA interrupt."]
pub type SLV_CMDA_INT_ST_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ST_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The status bit for lack of enough inlink descriptors."]
    #[inline(always)]
    pub fn inlink_dscr_empty_int_st(&self) -> INLINK_DSCR_EMPTY_INT_ST_R {
        INLINK_DSCR_EMPTY_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The status bit for outlink descriptor error."]
    #[inline(always)]
    pub fn outlink_dscr_error_int_st(&self) -> OUTLINK_DSCR_ERROR_INT_ST_R {
        OUTLINK_DSCR_ERROR_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The status bit for inlink descriptor error."]
    #[inline(always)]
    pub fn inlink_dscr_error_int_st(&self) -> INLINK_DSCR_ERROR_INT_ST_R {
        INLINK_DSCR_ERROR_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The status bit for completing usage of a inlink descriptor."]
    #[inline(always)]
    pub fn in_done_int_st(&self) -> IN_DONE_INT_ST_R {
        IN_DONE_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status bit for receiving error."]
    #[inline(always)]
    pub fn in_err_eof_int_st(&self) -> IN_ERR_EOF_INT_ST_R {
        IN_ERR_EOF_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The status bit for completing receiving all the packets from host."]
    #[inline(always)]
    pub fn in_suc_eof_int_st(&self) -> IN_SUC_EOF_INT_ST_R {
        IN_SUC_EOF_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The status bit for completing usage of a outlink descriptor."]
    #[inline(always)]
    pub fn out_done_int_st(&self) -> OUT_DONE_INT_ST_R {
        OUT_DONE_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The status bit for sending a packet to host done."]
    #[inline(always)]
    pub fn out_eof_int_st(&self) -> OUT_EOF_INT_ST_R {
        OUT_EOF_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The status bit for sending all the packets to host done."]
    #[inline(always)]
    pub fn out_total_eof_int_st(&self) -> OUT_TOTAL_EOF_INT_ST_R {
        OUT_TOTAL_EOF_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The status bit for infifo full error."]
    #[inline(always)]
    pub fn infifo_full_err_int_st(&self) -> INFIFO_FULL_ERR_INT_ST_R {
        INFIFO_FULL_ERR_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The status bit for outfifo empty error."]
    #[inline(always)]
    pub fn outfifo_empty_err_int_st(&self) -> OUTFIFO_EMPTY_ERR_INT_ST_R {
        OUTFIFO_EMPTY_ERR_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The status bit for SPI slave CMD6 interrupt."]
    #[inline(always)]
    pub fn slv_cmd6_int_st(&self) -> SLV_CMD6_INT_ST_R {
        SLV_CMD6_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The status bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    pub fn slv_cmd7_int_st(&self) -> SLV_CMD7_INT_ST_R {
        SLV_CMD7_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The status bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    pub fn slv_cmd8_int_st(&self) -> SLV_CMD8_INT_ST_R {
        SLV_CMD8_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The status bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    pub fn slv_cmd9_int_st(&self) -> SLV_CMD9_INT_ST_R {
        SLV_CMD9_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The status bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    pub fn slv_cmda_int_st(&self) -> SLV_CMDA_INT_ST_R {
        SLV_CMDA_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_INT_ST")
            .field(
                "inlink_dscr_empty_int_st",
                &format_args!("{}", self.inlink_dscr_empty_int_st().bit()),
            )
            .field(
                "outlink_dscr_error_int_st",
                &format_args!("{}", self.outlink_dscr_error_int_st().bit()),
            )
            .field(
                "inlink_dscr_error_int_st",
                &format_args!("{}", self.inlink_dscr_error_int_st().bit()),
            )
            .field(
                "in_done_int_st",
                &format_args!("{}", self.in_done_int_st().bit()),
            )
            .field(
                "in_err_eof_int_st",
                &format_args!("{}", self.in_err_eof_int_st().bit()),
            )
            .field(
                "in_suc_eof_int_st",
                &format_args!("{}", self.in_suc_eof_int_st().bit()),
            )
            .field(
                "out_done_int_st",
                &format_args!("{}", self.out_done_int_st().bit()),
            )
            .field(
                "out_eof_int_st",
                &format_args!("{}", self.out_eof_int_st().bit()),
            )
            .field(
                "out_total_eof_int_st",
                &format_args!("{}", self.out_total_eof_int_st().bit()),
            )
            .field(
                "infifo_full_err_int_st",
                &format_args!("{}", self.infifo_full_err_int_st().bit()),
            )
            .field(
                "outfifo_empty_err_int_st",
                &format_args!("{}", self.outfifo_empty_err_int_st().bit()),
            )
            .field(
                "slv_cmd6_int_st",
                &format_args!("{}", self.slv_cmd6_int_st().bit()),
            )
            .field(
                "slv_cmd7_int_st",
                &format_args!("{}", self.slv_cmd7_int_st().bit()),
            )
            .field(
                "slv_cmd8_int_st",
                &format_args!("{}", self.slv_cmd8_int_st().bit()),
            )
            .field(
                "slv_cmd9_int_st",
                &format_args!("{}", self.slv_cmd9_int_st().bit()),
            )
            .field(
                "slv_cmda_int_st",
                &format_args!("{}", self.slv_cmda_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 11 - The status bit for SPI slave CMD6 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd6_int_st(&mut self) -> SLV_CMD6_INT_ST_W<11> {
        SLV_CMD6_INT_ST_W::new(self)
    }
    #[doc = "Bit 12 - The status bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd7_int_st(&mut self) -> SLV_CMD7_INT_ST_W<12> {
        SLV_CMD7_INT_ST_W::new(self)
    }
    #[doc = "Bit 13 - The status bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd8_int_st(&mut self) -> SLV_CMD8_INT_ST_W<13> {
        SLV_CMD8_INT_ST_W::new(self)
    }
    #[doc = "Bit 14 - The status bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd9_int_st(&mut self) -> SLV_CMD9_INT_ST_W<14> {
        SLV_CMD9_INT_ST_W::new(self)
    }
    #[doc = "Bit 15 - The status bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmda_int_st(&mut self) -> SLV_CMDA_INT_ST_W<15> {
        SLV_CMDA_INT_ST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI DMA interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_st](index.html) module"]
pub struct DMA_INT_ST_SPEC;
impl crate::RegisterSpec for DMA_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int_st::R](R) reader structure"]
impl crate::Readable for DMA_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_int_st::W](W) writer structure"]
impl crate::Writable for DMA_INT_ST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_INT_ST to value 0"]
impl crate::Resettable for DMA_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
