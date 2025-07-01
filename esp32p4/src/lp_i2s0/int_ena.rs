#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `RX_DONE` reader - The interrupt enable bit for the i2s_rx_done_int interrupt"]
pub type RX_DONE_R = crate::BitReader;
#[doc = "Field `RX_DONE` writer - The interrupt enable bit for the i2s_rx_done_int interrupt"]
pub type RX_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HUNG` reader - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
pub type RX_HUNG_R = crate::BitReader;
#[doc = "Field `RX_HUNG` writer - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
pub type RX_HUNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFOMEM_UDF` reader - The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt"]
pub type RX_FIFOMEM_UDF_R = crate::BitReader;
#[doc = "Field `RX_FIFOMEM_UDF` writer - The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt"]
pub type RX_FIFOMEM_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_VAD_DONE` reader - The interrupt enable bit for the vad_done_int interrupt"]
pub type LP_VAD_DONE_R = crate::BitReader;
#[doc = "Field `LP_VAD_DONE` writer - The interrupt enable bit for the vad_done_int interrupt"]
pub type LP_VAD_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_VAD_RESET_DONE` reader - The interrupt enable bit for the vad_reset_done_int interrupt"]
pub type LP_VAD_RESET_DONE_R = crate::BitReader;
#[doc = "Field `LP_VAD_RESET_DONE` writer - The interrupt enable bit for the vad_reset_done_int interrupt"]
pub type LP_VAD_RESET_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MEM_THRESHOLD` reader - The interrupt enable bit for the rx_mem_threshold_int interrupt"]
pub type RX_MEM_THRESHOLD_R = crate::BitReader;
#[doc = "Field `RX_MEM_THRESHOLD` writer - The interrupt enable bit for the rx_mem_threshold_int interrupt"]
pub type RX_MEM_THRESHOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn rx_hung(&self) -> RX_HUNG_R {
        RX_HUNG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt"]
    #[inline(always)]
    pub fn rx_fifomem_udf(&self) -> RX_FIFOMEM_UDF_R {
        RX_FIFOMEM_UDF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the vad_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_done(&self) -> LP_VAD_DONE_R {
        LP_VAD_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the vad_reset_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_reset_done(&self) -> LP_VAD_RESET_DONE_R {
        LP_VAD_RESET_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the rx_mem_threshold_int interrupt"]
    #[inline(always)]
    pub fn rx_mem_threshold(&self) -> RX_MEM_THRESHOLD_R {
        RX_MEM_THRESHOLD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("rx_done", &self.rx_done())
            .field("rx_hung", &self.rx_hung())
            .field("rx_fifomem_udf", &self.rx_fifomem_udf())
            .field("lp_vad_done", &self.lp_vad_done())
            .field("lp_vad_reset_done", &self.lp_vad_reset_done())
            .field("rx_mem_threshold", &self.rx_mem_threshold())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn rx_done(&mut self) -> RX_DONE_W<INT_ENA_SPEC> {
        RX_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn rx_hung(&mut self) -> RX_HUNG_W<INT_ENA_SPEC> {
        RX_HUNG_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt"]
    #[inline(always)]
    pub fn rx_fifomem_udf(&mut self) -> RX_FIFOMEM_UDF_W<INT_ENA_SPEC> {
        RX_FIFOMEM_UDF_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the vad_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_done(&mut self) -> LP_VAD_DONE_W<INT_ENA_SPEC> {
        LP_VAD_DONE_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the vad_reset_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_reset_done(&mut self) -> LP_VAD_RESET_DONE_W<INT_ENA_SPEC> {
        LP_VAD_RESET_DONE_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the rx_mem_threshold_int interrupt"]
    #[inline(always)]
    pub fn rx_mem_threshold(&mut self) -> RX_MEM_THRESHOLD_W<INT_ENA_SPEC> {
        RX_MEM_THRESHOLD_W::new(self, 5)
    }
}
#[doc = "I2S interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
