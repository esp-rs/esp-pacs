#[doc = "Register `IN_INT_ENA_CH%s` reader"]
pub type R = crate::R<IN_INT_ENA_CH_SPEC>;
#[doc = "Register `IN_INT_ENA_CH%s` writer"]
pub type W = crate::W<IN_INT_ENA_CH_SPEC>;
#[doc = "Field `IN_DONE_CH_INT_ENA` reader - Write 1 to enable AHB_DMA_IN_DONE_CH%s_INT."]
pub type IN_DONE_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_DONE_CH_INT_ENA` writer - Write 1 to enable AHB_DMA_IN_DONE_CH%s_INT."]
pub type IN_DONE_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF_CH_INT_ENA` reader - Write 1 to enable AHB_DMA_IN_SUC_EOF_CH%s_INT."]
pub type IN_SUC_EOF_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_CH_INT_ENA` writer - Write 1 to enable AHB_DMA_IN_SUC_EOF_CH%s_INT."]
pub type IN_SUC_EOF_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF_CH_INT_ENA` reader - Write 1 to enable AHB_DMA_IN_ERR_EOF_CH%s_INT."]
pub type IN_ERR_EOF_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_CH_INT_ENA` writer - Write 1 to enable AHB_DMA_IN_ERR_EOF_CH%s_INT."]
pub type IN_ERR_EOF_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR_CH_INT_ENA` reader - Write 1 to enable AHB_DMA_IN_DSCR_ERR_CH%s_INT."]
pub type IN_DSCR_ERR_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR_CH_INT_ENA` writer - Write 1 to enable AHB_DMA_IN_DSCR_ERR_CH%s_INT."]
pub type IN_DSCR_ERR_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY_CH_INT_ENA` reader - Write 1 to enable AHB_DMA_IN_DSCR_EMPTY_CH%s_INT."]
pub type IN_DSCR_EMPTY_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY_CH_INT_ENA` writer - Write 1 to enable AHB_DMA_IN_DSCR_EMPTY_CH%s_INT."]
pub type IN_DSCR_EMPTY_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_CH_INT_ENA` reader - Write 1 to enable AHB_DMA_INFIFO_OVF_CH%s_INT."]
pub type INFIFO_OVF_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_CH_INT_ENA` writer - Write 1 to enable AHB_DMA_INFIFO_OVF_CH%s_INT."]
pub type INFIFO_OVF_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_CH_INT_ENA` reader - Write 1 to enable AHB_DMA_INFIFO_UDF_CH%s_INT."]
pub type INFIFO_UDF_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_CH_INT_ENA` writer - Write 1 to enable AHB_DMA_INFIFO_UDF_CH%s_INT."]
pub type INFIFO_UDF_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable AHB_DMA_IN_DONE_CH%s_INT."]
    #[inline(always)]
    pub fn in_done_ch_int_ena(&self) -> IN_DONE_CH_INT_ENA_R {
        IN_DONE_CH_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable AHB_DMA_IN_SUC_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn in_suc_eof_ch_int_ena(&self) -> IN_SUC_EOF_CH_INT_ENA_R {
        IN_SUC_EOF_CH_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to enable AHB_DMA_IN_ERR_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn in_err_eof_ch_int_ena(&self) -> IN_ERR_EOF_CH_INT_ENA_R {
        IN_ERR_EOF_CH_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to enable AHB_DMA_IN_DSCR_ERR_CH%s_INT."]
    #[inline(always)]
    pub fn in_dscr_err_ch_int_ena(&self) -> IN_DSCR_ERR_CH_INT_ENA_R {
        IN_DSCR_ERR_CH_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to enable AHB_DMA_IN_DSCR_EMPTY_CH%s_INT."]
    #[inline(always)]
    pub fn in_dscr_empty_ch_int_ena(&self) -> IN_DSCR_EMPTY_CH_INT_ENA_R {
        IN_DSCR_EMPTY_CH_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write 1 to enable AHB_DMA_INFIFO_OVF_CH%s_INT."]
    #[inline(always)]
    pub fn infifo_ovf_ch_int_ena(&self) -> INFIFO_OVF_CH_INT_ENA_R {
        INFIFO_OVF_CH_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write 1 to enable AHB_DMA_INFIFO_UDF_CH%s_INT."]
    #[inline(always)]
    pub fn infifo_udf_ch_int_ena(&self) -> INFIFO_UDF_CH_INT_ENA_R {
        INFIFO_UDF_CH_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_INT_ENA_CH")
            .field("in_done_ch_int_ena", &self.in_done_ch_int_ena())
            .field("in_suc_eof_ch_int_ena", &self.in_suc_eof_ch_int_ena())
            .field("in_err_eof_ch_int_ena", &self.in_err_eof_ch_int_ena())
            .field("in_dscr_err_ch_int_ena", &self.in_dscr_err_ch_int_ena())
            .field("in_dscr_empty_ch_int_ena", &self.in_dscr_empty_ch_int_ena())
            .field("infifo_ovf_ch_int_ena", &self.infifo_ovf_ch_int_ena())
            .field("infifo_udf_ch_int_ena", &self.infifo_udf_ch_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable AHB_DMA_IN_DONE_CH%s_INT."]
    #[inline(always)]
    pub fn in_done_ch_int_ena(&mut self) -> IN_DONE_CH_INT_ENA_W<IN_INT_ENA_CH_SPEC> {
        IN_DONE_CH_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable AHB_DMA_IN_SUC_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn in_suc_eof_ch_int_ena(&mut self) -> IN_SUC_EOF_CH_INT_ENA_W<IN_INT_ENA_CH_SPEC> {
        IN_SUC_EOF_CH_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to enable AHB_DMA_IN_ERR_EOF_CH%s_INT."]
    #[inline(always)]
    pub fn in_err_eof_ch_int_ena(&mut self) -> IN_ERR_EOF_CH_INT_ENA_W<IN_INT_ENA_CH_SPEC> {
        IN_ERR_EOF_CH_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to enable AHB_DMA_IN_DSCR_ERR_CH%s_INT."]
    #[inline(always)]
    pub fn in_dscr_err_ch_int_ena(&mut self) -> IN_DSCR_ERR_CH_INT_ENA_W<IN_INT_ENA_CH_SPEC> {
        IN_DSCR_ERR_CH_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to enable AHB_DMA_IN_DSCR_EMPTY_CH%s_INT."]
    #[inline(always)]
    pub fn in_dscr_empty_ch_int_ena(&mut self) -> IN_DSCR_EMPTY_CH_INT_ENA_W<IN_INT_ENA_CH_SPEC> {
        IN_DSCR_EMPTY_CH_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to enable AHB_DMA_INFIFO_OVF_CH%s_INT."]
    #[inline(always)]
    pub fn infifo_ovf_ch_int_ena(&mut self) -> INFIFO_OVF_CH_INT_ENA_W<IN_INT_ENA_CH_SPEC> {
        INFIFO_OVF_CH_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to enable AHB_DMA_INFIFO_UDF_CH%s_INT."]
    #[inline(always)]
    pub fn infifo_udf_ch_int_ena(&mut self) -> INFIFO_UDF_CH_INT_ENA_W<IN_INT_ENA_CH_SPEC> {
        INFIFO_UDF_CH_INT_ENA_W::new(self, 6)
    }
}
#[doc = "Interrupt enable bits of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_ena_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_ena_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_INT_ENA_CH_SPEC;
impl crate::RegisterSpec for IN_INT_ENA_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_int_ena_ch::R`](R) reader structure"]
impl crate::Readable for IN_INT_ENA_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_int_ena_ch::W`](W) writer structure"]
impl crate::Writable for IN_INT_ENA_CH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_INT_ENA_CH%s to value 0"]
impl crate::Resettable for IN_INT_ENA_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
