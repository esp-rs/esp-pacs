#[doc = "Register `ENA` reader"]
pub type R = crate::R<ENA_SPEC>;
#[doc = "Register `ENA` writer"]
pub type W = crate::W<ENA_SPEC>;
#[doc = "Field `OUT_DONE_CH0_INT_ENA` reader - Write 1 to enable AHB_DMA_OUT_DONE_CH0_INT"]
pub type OUT_DONE_CH0_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_DONE_CH0_INT_ENA` writer - Write 1 to enable AHB_DMA_OUT_DONE_CH0_INT"]
pub type OUT_DONE_CH0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_CH0_INT_ENA` reader - Write 1 to enable AHB_DMA_OUT_EOF_CH0_INT"]
pub type OUT_EOF_CH0_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_EOF_CH0_INT_ENA` writer - Write 1 to enable AHB_DMA_OUT_EOF_CH0_INT"]
pub type OUT_EOF_CH0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR_CH0_INT_ENA` reader - Write 1 to enable AHB_DMA_OUT_DSCR_ERR_CH0_INT"]
pub type OUT_DSCR_ERR_CH0_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR_CH0_INT_ENA` writer - Write 1 to enable AHB_DMA_OUT_DSCR_ERR_CH0_INT"]
pub type OUT_DSCR_ERR_CH0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF_CH0_INT_ENA` reader - Write 1 to enable AHB_DMA_OUT_TOTAL_EOF_CH0_INT"]
pub type OUT_TOTAL_EOF_CH0_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_CH0_INT_ENA` writer - Write 1 to enable AHB_DMA_OUT_TOTAL_EOF_CH0_INT"]
pub type OUT_TOTAL_EOF_CH0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_CH0_INT_ENA` reader - Write 1 to enable AHB_DMA_OUTFIFO_OVF_CH0_INT"]
pub type OUTFIFO_OVF_CH0_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_CH0_INT_ENA` writer - Write 1 to enable AHB_DMA_OUTFIFO_OVF_CH0_INT"]
pub type OUTFIFO_OVF_CH0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_CH0_INT_ENA` reader - Write 1 to enable AHB_DMA_OUTFIFO_UDF_CH0_INT"]
pub type OUTFIFO_UDF_CH0_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_CH0_INT_ENA` writer - Write 1 to enable AHB_DMA_OUTFIFO_UDF_CH0_INT"]
pub type OUTFIFO_UDF_CH0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_AHBINF_RESP_ERR_CH0_INT_ENA` reader - Write 1 to enable AHB_DMA_OUT_RESP_ERR_CH0_INT"]
pub type OUT_AHBINF_RESP_ERR_CH0_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_AHBINF_RESP_ERR_CH0_INT_ENA` writer - Write 1 to enable AHB_DMA_OUT_RESP_ERR_CH0_INT"]
pub type OUT_AHBINF_RESP_ERR_CH0_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable AHB_DMA_OUT_DONE_CH0_INT"]
    #[inline(always)]
    pub fn out_done_ch0_int_ena(&self) -> OUT_DONE_CH0_INT_ENA_R {
        OUT_DONE_CH0_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable AHB_DMA_OUT_EOF_CH0_INT"]
    #[inline(always)]
    pub fn out_eof_ch0_int_ena(&self) -> OUT_EOF_CH0_INT_ENA_R {
        OUT_EOF_CH0_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to enable AHB_DMA_OUT_DSCR_ERR_CH0_INT"]
    #[inline(always)]
    pub fn out_dscr_err_ch0_int_ena(&self) -> OUT_DSCR_ERR_CH0_INT_ENA_R {
        OUT_DSCR_ERR_CH0_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to enable AHB_DMA_OUT_TOTAL_EOF_CH0_INT"]
    #[inline(always)]
    pub fn out_total_eof_ch0_int_ena(&self) -> OUT_TOTAL_EOF_CH0_INT_ENA_R {
        OUT_TOTAL_EOF_CH0_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to enable AHB_DMA_OUTFIFO_OVF_CH0_INT"]
    #[inline(always)]
    pub fn outfifo_ovf_ch0_int_ena(&self) -> OUTFIFO_OVF_CH0_INT_ENA_R {
        OUTFIFO_OVF_CH0_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write 1 to enable AHB_DMA_OUTFIFO_UDF_CH0_INT"]
    #[inline(always)]
    pub fn outfifo_udf_ch0_int_ena(&self) -> OUTFIFO_UDF_CH0_INT_ENA_R {
        OUTFIFO_UDF_CH0_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write 1 to enable AHB_DMA_OUT_RESP_ERR_CH0_INT"]
    #[inline(always)]
    pub fn out_ahbinf_resp_err_ch0_int_ena(&self) -> OUT_AHBINF_RESP_ERR_CH0_INT_ENA_R {
        OUT_AHBINF_RESP_ERR_CH0_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENA")
            .field("out_done_ch0_int_ena", &self.out_done_ch0_int_ena())
            .field("out_eof_ch0_int_ena", &self.out_eof_ch0_int_ena())
            .field("out_dscr_err_ch0_int_ena", &self.out_dscr_err_ch0_int_ena())
            .field(
                "out_total_eof_ch0_int_ena",
                &self.out_total_eof_ch0_int_ena(),
            )
            .field("outfifo_ovf_ch0_int_ena", &self.outfifo_ovf_ch0_int_ena())
            .field("outfifo_udf_ch0_int_ena", &self.outfifo_udf_ch0_int_ena())
            .field(
                "out_ahbinf_resp_err_ch0_int_ena",
                &self.out_ahbinf_resp_err_ch0_int_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable AHB_DMA_OUT_DONE_CH0_INT"]
    #[inline(always)]
    pub fn out_done_ch0_int_ena(&mut self) -> OUT_DONE_CH0_INT_ENA_W<'_, ENA_SPEC> {
        OUT_DONE_CH0_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable AHB_DMA_OUT_EOF_CH0_INT"]
    #[inline(always)]
    pub fn out_eof_ch0_int_ena(&mut self) -> OUT_EOF_CH0_INT_ENA_W<'_, ENA_SPEC> {
        OUT_EOF_CH0_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to enable AHB_DMA_OUT_DSCR_ERR_CH0_INT"]
    #[inline(always)]
    pub fn out_dscr_err_ch0_int_ena(&mut self) -> OUT_DSCR_ERR_CH0_INT_ENA_W<'_, ENA_SPEC> {
        OUT_DSCR_ERR_CH0_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to enable AHB_DMA_OUT_TOTAL_EOF_CH0_INT"]
    #[inline(always)]
    pub fn out_total_eof_ch0_int_ena(&mut self) -> OUT_TOTAL_EOF_CH0_INT_ENA_W<'_, ENA_SPEC> {
        OUT_TOTAL_EOF_CH0_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to enable AHB_DMA_OUTFIFO_OVF_CH0_INT"]
    #[inline(always)]
    pub fn outfifo_ovf_ch0_int_ena(&mut self) -> OUTFIFO_OVF_CH0_INT_ENA_W<'_, ENA_SPEC> {
        OUTFIFO_OVF_CH0_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to enable AHB_DMA_OUTFIFO_UDF_CH0_INT"]
    #[inline(always)]
    pub fn outfifo_udf_ch0_int_ena(&mut self) -> OUTFIFO_UDF_CH0_INT_ENA_W<'_, ENA_SPEC> {
        OUTFIFO_UDF_CH0_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to enable AHB_DMA_OUT_RESP_ERR_CH0_INT"]
    #[inline(always)]
    pub fn out_ahbinf_resp_err_ch0_int_ena(
        &mut self,
    ) -> OUT_AHBINF_RESP_ERR_CH0_INT_ENA_W<'_, ENA_SPEC> {
        OUT_AHBINF_RESP_ERR_CH0_INT_ENA_W::new(self, 6)
    }
}
#[doc = "Interrupt enable bits of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENA_SPEC;
impl crate::RegisterSpec for ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ena::R`](R) reader structure"]
impl crate::Readable for ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ena::W`](W) writer structure"]
impl crate::Writable for ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENA to value 0"]
impl crate::Resettable for ENA_SPEC {}
