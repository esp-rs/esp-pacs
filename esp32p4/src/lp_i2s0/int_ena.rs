#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `RX_DONE_INT_ENA` reader - The interrupt enable bit for the i2s_rx_done_int interrupt"]
pub type RX_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_DONE_INT_ENA` writer - The interrupt enable bit for the i2s_rx_done_int interrupt"]
pub type RX_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HUNG_INT_ENA` reader - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
pub type RX_HUNG_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_HUNG_INT_ENA` writer - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
pub type RX_HUNG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFOMEM_UDF_INT_ENA` reader - The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt"]
pub type RX_FIFOMEM_UDF_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_FIFOMEM_UDF_INT_ENA` writer - The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt"]
pub type RX_FIFOMEM_UDF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_VAD_DONE_INT_ENA` reader - The interrupt enable bit for the vad_done_int interrupt"]
pub type LP_VAD_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_VAD_DONE_INT_ENA` writer - The interrupt enable bit for the vad_done_int interrupt"]
pub type LP_VAD_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_VAD_RESET_DONE_INT_ENA` reader - The interrupt enable bit for the vad_reset_done_int interrupt"]
pub type LP_VAD_RESET_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_VAD_RESET_DONE_INT_ENA` writer - The interrupt enable bit for the vad_reset_done_int interrupt"]
pub type LP_VAD_RESET_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MEM_THRESHOLD_INT_ENA` reader - The interrupt enable bit for the rx_mem_threshold_int interrupt"]
pub type RX_MEM_THRESHOLD_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_MEM_THRESHOLD_INT_ENA` writer - The interrupt enable bit for the rx_mem_threshold_int interrupt"]
pub type RX_MEM_THRESHOLD_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    pub fn rx_done_int_ena(&self) -> RX_DONE_INT_ENA_R {
        RX_DONE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    pub fn rx_hung_int_ena(&self) -> RX_HUNG_INT_ENA_R {
        RX_HUNG_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt"]
    #[inline(always)]
    pub fn rx_fifomem_udf_int_ena(&self) -> RX_FIFOMEM_UDF_INT_ENA_R {
        RX_FIFOMEM_UDF_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the vad_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_done_int_ena(&self) -> LP_VAD_DONE_INT_ENA_R {
        LP_VAD_DONE_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the vad_reset_done_int interrupt"]
    #[inline(always)]
    pub fn lp_vad_reset_done_int_ena(&self) -> LP_VAD_RESET_DONE_INT_ENA_R {
        LP_VAD_RESET_DONE_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the rx_mem_threshold_int interrupt"]
    #[inline(always)]
    pub fn rx_mem_threshold_int_ena(&self) -> RX_MEM_THRESHOLD_INT_ENA_R {
        RX_MEM_THRESHOLD_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "rx_done_int_ena",
                &format_args!("{}", self.rx_done_int_ena().bit()),
            )
            .field(
                "rx_hung_int_ena",
                &format_args!("{}", self.rx_hung_int_ena().bit()),
            )
            .field(
                "rx_fifomem_udf_int_ena",
                &format_args!("{}", self.rx_fifomem_udf_int_ena().bit()),
            )
            .field(
                "lp_vad_done_int_ena",
                &format_args!("{}", self.lp_vad_done_int_ena().bit()),
            )
            .field(
                "lp_vad_reset_done_int_ena",
                &format_args!("{}", self.lp_vad_reset_done_int_ena().bit()),
            )
            .field(
                "rx_mem_threshold_int_ena",
                &format_args!("{}", self.rx_mem_threshold_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the i2s_rx_done_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_done_int_ena(&mut self) -> RX_DONE_INT_ENA_W<INT_ENA_SPEC> {
        RX_DONE_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the i2s_rx_hung_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_hung_int_ena(&mut self) -> RX_HUNG_INT_ENA_W<INT_ENA_SPEC> {
        RX_HUNG_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the i2s_rx_fifomem_udf_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifomem_udf_int_ena(&mut self) -> RX_FIFOMEM_UDF_INT_ENA_W<INT_ENA_SPEC> {
        RX_FIFOMEM_UDF_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the vad_done_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lp_vad_done_int_ena(&mut self) -> LP_VAD_DONE_INT_ENA_W<INT_ENA_SPEC> {
        LP_VAD_DONE_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the vad_reset_done_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lp_vad_reset_done_int_ena(&mut self) -> LP_VAD_RESET_DONE_INT_ENA_W<INT_ENA_SPEC> {
        LP_VAD_RESET_DONE_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the rx_mem_threshold_int interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_mem_threshold_int_ena(&mut self) -> RX_MEM_THRESHOLD_INT_ENA_W<INT_ENA_SPEC> {
        RX_MEM_THRESHOLD_INT_ENA_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2S interrupt enable register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
