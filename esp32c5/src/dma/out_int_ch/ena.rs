#[doc = "Register `ENA` reader"]
pub type R = crate::R<ENA_SPEC>;
#[doc = "Register `ENA` writer"]
pub type W = crate::W<ENA_SPEC>;
#[doc = "Field `OUT_DONE` reader - Write 1 to enable AHB_DMA_OUT_DONE_CH%s_INT."]
pub type OUT_DONE_R = crate::BitReader;
#[doc = "Field `OUT_DONE` writer - Write 1 to enable AHB_DMA_OUT_DONE_CH%s_INT."]
pub type OUT_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF` reader - Write 1 to enable AHB_DMA_OUT_EOF_CH%s_INT."]
pub type OUT_EOF_R = crate::BitReader;
#[doc = "Field `OUT_EOF` writer - Write 1 to enable AHB_DMA_OUT_EOF_CH%s_INT."]
pub type OUT_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR` reader - Write 1 to enable AHB_DMA_OUT_DSCR_ERR_CH%s_INT."]
pub type OUT_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR` writer - Write 1 to enable AHB_DMA_OUT_DSCR_ERR_CH%s_INT."]
pub type OUT_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF` reader - Write 1 to enable AHB_DMA_OUT_TOTAL_EOF_CH%s_INT."]
pub type OUT_TOTAL_EOF_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF` writer - Write 1 to enable AHB_DMA_OUT_TOTAL_EOF_CH%s_INT."]
pub type OUT_TOTAL_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_OVF` reader - Write 1 to enable AHB_DMA_OUTFIFO_OVF_CH%s_INT."]
pub type OUTFIFO_OVF_R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF` writer - Write 1 to enable AHB_DMA_OUTFIFO_OVF_CH%s_INT."]
pub type OUTFIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_UDF` reader - Write 1 to enable AHB_DMA_OUTFIFO_UDF_CH%s_INT."]
pub type OUTFIFO_UDF_R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF` writer - Write 1 to enable AHB_DMA_OUTFIFO_UDF_CH%s_INT."]
pub type OUTFIFO_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable AHB_DMA_OUT_DONE_CH%s_INT."]
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable AHB_DMA_OUT_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to enable AHB_DMA_OUT_DSCR_ERR_CH%s_INT."]
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OUT_DSCR_ERR_R {
        OUT_DSCR_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to enable AHB_DMA_OUT_TOTAL_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to enable AHB_DMA_OUTFIFO_OVF_CH%s_INT."]
    #[inline(always)]
    pub fn outfifo_ovf(&self) -> OUTFIFO_OVF_R {
        OUTFIFO_OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write 1 to enable AHB_DMA_OUTFIFO_UDF_CH%s_INT."]
    #[inline(always)]
    pub fn outfifo_udf(&self) -> OUTFIFO_UDF_R {
        OUTFIFO_UDF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENA")
            .field("out_done", &self.out_done())
            .field("out_eof", &self.out_eof())
            .field("out_dscr_err", &self.out_dscr_err())
            .field("out_total_eof", &self.out_total_eof())
            .field("outfifo_ovf", &self.outfifo_ovf())
            .field("outfifo_udf", &self.outfifo_udf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable AHB_DMA_OUT_DONE_CH%s_INT."]
    #[inline(always)]
    pub fn out_done(&mut self) -> OUT_DONE_W<'_, ENA_SPEC> {
        OUT_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable AHB_DMA_OUT_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn out_eof(&mut self) -> OUT_EOF_W<'_, ENA_SPEC> {
        OUT_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to enable AHB_DMA_OUT_DSCR_ERR_CH%s_INT."]
    #[inline(always)]
    pub fn out_dscr_err(&mut self) -> OUT_DSCR_ERR_W<'_, ENA_SPEC> {
        OUT_DSCR_ERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to enable AHB_DMA_OUT_TOTAL_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn out_total_eof(&mut self) -> OUT_TOTAL_EOF_W<'_, ENA_SPEC> {
        OUT_TOTAL_EOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to enable AHB_DMA_OUTFIFO_OVF_CH%s_INT."]
    #[inline(always)]
    pub fn outfifo_ovf(&mut self) -> OUTFIFO_OVF_W<'_, ENA_SPEC> {
        OUTFIFO_OVF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to enable AHB_DMA_OUTFIFO_UDF_CH%s_INT."]
    #[inline(always)]
    pub fn outfifo_udf(&mut self) -> OUTFIFO_UDF_W<'_, ENA_SPEC> {
        OUTFIFO_UDF_W::new(self, 5)
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
