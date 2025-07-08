#[doc = "Register `OUT_INT_RAW_CH0` reader"]
pub type R = crate::R<OUT_INT_RAW_CH0_SPEC>;
#[doc = "Register `OUT_INT_RAW_CH0` writer"]
pub type W = crate::W<OUT_INT_RAW_CH0_SPEC>;
#[doc = "Field `OUT_DONE_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_OUT_DONE_CH0_INT"]
pub type OUT_DONE_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_DONE_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_OUT_DONE_CH0_INT"]
pub type OUT_DONE_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_OUT_EOF_CH0_INT"]
pub type OUT_EOF_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_EOF_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_OUT_EOF_CH0_INT"]
pub type OUT_EOF_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_OUT_DSCR_ERR_CH0_INT"]
pub type OUT_DSCR_ERR_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_OUT_DSCR_ERR_CH0_INT"]
pub type OUT_DSCR_ERR_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_OUT_TOTAL_EOF_CH0_INT"]
pub type OUT_TOTAL_EOF_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_OUT_TOTAL_EOF_CH0_INT"]
pub type OUT_TOTAL_EOF_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_OUTFIFO_OVF_CH0_INT"]
pub type OUTFIFO_OVF_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_OUTFIFO_OVF_CH0_INT"]
pub type OUTFIFO_OVF_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_OUTFIFO_UDF_CH0_INT"]
pub type OUTFIFO_UDF_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_OUTFIFO_UDF_CH0_INT"]
pub type OUTFIFO_UDF_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_AHBINF_RESP_ERR_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_OUT_RESP_ERR_CH0_INT"]
pub type OUT_AHBINF_RESP_ERR_CH0_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_AHBINF_RESP_ERR_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_OUT_RESP_ERR_CH0_INT"]
pub type OUT_AHBINF_RESP_ERR_CH0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt status of AHB_DMA_OUT_DONE_CH0_INT"]
    #[inline(always)]
    pub fn out_done_ch0_int_raw(&self) -> OUT_DONE_CH0_INT_RAW_R {
        OUT_DONE_CH0_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of AHB_DMA_OUT_EOF_CH0_INT"]
    #[inline(always)]
    pub fn out_eof_ch0_int_raw(&self) -> OUT_EOF_CH0_INT_RAW_R {
        OUT_EOF_CH0_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status of AHB_DMA_OUT_DSCR_ERR_CH0_INT"]
    #[inline(always)]
    pub fn out_dscr_err_ch0_int_raw(&self) -> OUT_DSCR_ERR_CH0_INT_RAW_R {
        OUT_DSCR_ERR_CH0_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status of AHB_DMA_OUT_TOTAL_EOF_CH0_INT"]
    #[inline(always)]
    pub fn out_total_eof_ch0_int_raw(&self) -> OUT_TOTAL_EOF_CH0_INT_RAW_R {
        OUT_TOTAL_EOF_CH0_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status of AHB_DMA_OUTFIFO_OVF_CH0_INT"]
    #[inline(always)]
    pub fn outfifo_ovf_ch0_int_raw(&self) -> OUTFIFO_OVF_CH0_INT_RAW_R {
        OUTFIFO_OVF_CH0_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status of AHB_DMA_OUTFIFO_UDF_CH0_INT"]
    #[inline(always)]
    pub fn outfifo_udf_ch0_int_raw(&self) -> OUTFIFO_UDF_CH0_INT_RAW_R {
        OUTFIFO_UDF_CH0_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status of AHB_DMA_OUT_RESP_ERR_CH0_INT"]
    #[inline(always)]
    pub fn out_ahbinf_resp_err_ch0_int_raw(&self) -> OUT_AHBINF_RESP_ERR_CH0_INT_RAW_R {
        OUT_AHBINF_RESP_ERR_CH0_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_INT_RAW_CH0")
            .field("out_done_ch0_int_raw", &self.out_done_ch0_int_raw())
            .field("out_eof_ch0_int_raw", &self.out_eof_ch0_int_raw())
            .field("out_dscr_err_ch0_int_raw", &self.out_dscr_err_ch0_int_raw())
            .field(
                "out_total_eof_ch0_int_raw",
                &self.out_total_eof_ch0_int_raw(),
            )
            .field("outfifo_ovf_ch0_int_raw", &self.outfifo_ovf_ch0_int_raw())
            .field("outfifo_udf_ch0_int_raw", &self.outfifo_udf_ch0_int_raw())
            .field(
                "out_ahbinf_resp_err_ch0_int_raw",
                &self.out_ahbinf_resp_err_ch0_int_raw(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt status of AHB_DMA_OUT_DONE_CH0_INT"]
    #[inline(always)]
    pub fn out_done_ch0_int_raw(&mut self) -> OUT_DONE_CH0_INT_RAW_W<OUT_INT_RAW_CH0_SPEC> {
        OUT_DONE_CH0_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of AHB_DMA_OUT_EOF_CH0_INT"]
    #[inline(always)]
    pub fn out_eof_ch0_int_raw(&mut self) -> OUT_EOF_CH0_INT_RAW_W<OUT_INT_RAW_CH0_SPEC> {
        OUT_EOF_CH0_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt status of AHB_DMA_OUT_DSCR_ERR_CH0_INT"]
    #[inline(always)]
    pub fn out_dscr_err_ch0_int_raw(&mut self) -> OUT_DSCR_ERR_CH0_INT_RAW_W<OUT_INT_RAW_CH0_SPEC> {
        OUT_DSCR_ERR_CH0_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt status of AHB_DMA_OUT_TOTAL_EOF_CH0_INT"]
    #[inline(always)]
    pub fn out_total_eof_ch0_int_raw(
        &mut self,
    ) -> OUT_TOTAL_EOF_CH0_INT_RAW_W<OUT_INT_RAW_CH0_SPEC> {
        OUT_TOTAL_EOF_CH0_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw interrupt status of AHB_DMA_OUTFIFO_OVF_CH0_INT"]
    #[inline(always)]
    pub fn outfifo_ovf_ch0_int_raw(&mut self) -> OUTFIFO_OVF_CH0_INT_RAW_W<OUT_INT_RAW_CH0_SPEC> {
        OUTFIFO_OVF_CH0_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw interrupt status of AHB_DMA_OUTFIFO_UDF_CH0_INT"]
    #[inline(always)]
    pub fn outfifo_udf_ch0_int_raw(&mut self) -> OUTFIFO_UDF_CH0_INT_RAW_W<OUT_INT_RAW_CH0_SPEC> {
        OUTFIFO_UDF_CH0_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - The raw interrupt status of AHB_DMA_OUT_RESP_ERR_CH0_INT"]
    #[inline(always)]
    pub fn out_ahbinf_resp_err_ch0_int_raw(
        &mut self,
    ) -> OUT_AHBINF_RESP_ERR_CH0_INT_RAW_W<OUT_INT_RAW_CH0_SPEC> {
        OUT_AHBINF_RESP_ERR_CH0_INT_RAW_W::new(self, 6)
    }
}
#[doc = "//Raw interrupt status of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_int_raw_ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_int_raw_ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_INT_RAW_CH0_SPEC;
impl crate::RegisterSpec for OUT_INT_RAW_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_int_raw_ch0::R`](R) reader structure"]
impl crate::Readable for OUT_INT_RAW_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_int_raw_ch0::W`](W) writer structure"]
impl crate::Writable for OUT_INT_RAW_CH0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_INT_RAW_CH0 to value 0"]
impl crate::Resettable for OUT_INT_RAW_CH0_SPEC {}
