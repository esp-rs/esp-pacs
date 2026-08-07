#[doc = "Register `CHNL1_CFG6` reader"]
pub type R = crate::R<CHNL1_CFG6_SPEC>;
#[doc = "Register `CHNL1_CFG6` writer"]
pub type W = crate::W<CHNL1_CFG6_SPEC>;
#[doc = "Field `CHNL1_OUT_EOF_GEN_MODE` reader - Write the bits to specify the which eof will be written to DMA. 0: counter eof, 1: DMA ineof, 2: both counter eof and DMA ineof, 3 none."]
pub type CHNL1_OUT_EOF_GEN_MODE_R = crate::FieldReader;
#[doc = "Field `CHNL1_OUT_EOF_GEN_MODE` writer - Write the bits to specify the which eof will be written to DMA. 0: counter eof, 1: DMA ineof, 2: both counter eof and DMA ineof, 3 none."]
pub type CHNL1_OUT_EOF_GEN_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHNL1_OUT_CNT_ENA` reader - Set this bit to enable out data byte counter."]
pub type CHNL1_OUT_CNT_ENA_R = crate::BitReader;
#[doc = "Field `CHNL1_OUT_CNT_ENA` writer - Set this bit to enable out data byte counter."]
pub type CHNL1_OUT_CNT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_OUT_CNT_CLR` writer - Set this bit to clear out data byte counter."]
pub type CHNL1_OUT_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_OUT_SAMPLES_COMP` reader - Set this bit to enable out data byte counter compensation when using fractional re-sampler and decimation by factor of 2 which results in reg_chnl1_out_cnt >= reg_chnl1_out_len"]
pub type CHNL1_OUT_SAMPLES_COMP_R = crate::BitReader;
#[doc = "Field `CHNL1_OUT_SAMPLES_COMP` writer - Set this bit to enable out data byte counter compensation when using fractional re-sampler and decimation by factor of 2 which results in reg_chnl1_out_cnt >= reg_chnl1_out_len"]
pub type CHNL1_OUT_SAMPLES_COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_OUT_LEN` reader - Write the bits to specify the data byte number of data to the DMA, the counter eof will be set when the counter approaches."]
pub type CHNL1_OUT_LEN_R = crate::FieldReader<u32>;
#[doc = "Field `CHNL1_OUT_LEN` writer - Write the bits to specify the data byte number of data to the DMA, the counter eof will be set when the counter approaches."]
pub type CHNL1_OUT_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:1 - Write the bits to specify the which eof will be written to DMA. 0: counter eof, 1: DMA ineof, 2: both counter eof and DMA ineof, 3 none."]
    #[inline(always)]
    pub fn chnl1_out_eof_gen_mode(&self) -> CHNL1_OUT_EOF_GEN_MODE_R {
        CHNL1_OUT_EOF_GEN_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set this bit to enable out data byte counter."]
    #[inline(always)]
    pub fn chnl1_out_cnt_ena(&self) -> CHNL1_OUT_CNT_ENA_R {
        CHNL1_OUT_CNT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable out data byte counter compensation when using fractional re-sampler and decimation by factor of 2 which results in reg_chnl1_out_cnt >= reg_chnl1_out_len"]
    #[inline(always)]
    pub fn chnl1_out_samples_comp(&self) -> CHNL1_OUT_SAMPLES_COMP_R {
        CHNL1_OUT_SAMPLES_COMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write the bits to specify the data byte number of data to the DMA, the counter eof will be set when the counter approaches."]
    #[inline(always)]
    pub fn chnl1_out_len(&self) -> CHNL1_OUT_LEN_R {
        CHNL1_OUT_LEN_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL1_CFG6")
            .field("chnl1_out_eof_gen_mode", &self.chnl1_out_eof_gen_mode())
            .field("chnl1_out_cnt_ena", &self.chnl1_out_cnt_ena())
            .field("chnl1_out_samples_comp", &self.chnl1_out_samples_comp())
            .field("chnl1_out_len", &self.chnl1_out_len())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Write the bits to specify the which eof will be written to DMA. 0: counter eof, 1: DMA ineof, 2: both counter eof and DMA ineof, 3 none."]
    #[inline(always)]
    pub fn chnl1_out_eof_gen_mode(&mut self) -> CHNL1_OUT_EOF_GEN_MODE_W<'_, CHNL1_CFG6_SPEC> {
        CHNL1_OUT_EOF_GEN_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set this bit to enable out data byte counter."]
    #[inline(always)]
    pub fn chnl1_out_cnt_ena(&mut self) -> CHNL1_OUT_CNT_ENA_W<'_, CHNL1_CFG6_SPEC> {
        CHNL1_OUT_CNT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear out data byte counter."]
    #[inline(always)]
    pub fn chnl1_out_cnt_clr(&mut self) -> CHNL1_OUT_CNT_CLR_W<'_, CHNL1_CFG6_SPEC> {
        CHNL1_OUT_CNT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable out data byte counter compensation when using fractional re-sampler and decimation by factor of 2 which results in reg_chnl1_out_cnt >= reg_chnl1_out_len"]
    #[inline(always)]
    pub fn chnl1_out_samples_comp(&mut self) -> CHNL1_OUT_SAMPLES_COMP_W<'_, CHNL1_CFG6_SPEC> {
        CHNL1_OUT_SAMPLES_COMP_W::new(self, 4)
    }
    #[doc = "Bits 8:31 - Write the bits to specify the data byte number of data to the DMA, the counter eof will be set when the counter approaches."]
    #[inline(always)]
    pub fn chnl1_out_len(&mut self) -> CHNL1_OUT_LEN_W<'_, CHNL1_CFG6_SPEC> {
        CHNL1_OUT_LEN_W::new(self, 8)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_cfg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_cfg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL1_CFG6_SPEC;
impl crate::RegisterSpec for CHNL1_CFG6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl1_cfg6::R`](R) reader structure"]
impl crate::Readable for CHNL1_CFG6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chnl1_cfg6::W`](W) writer structure"]
impl crate::Writable for CHNL1_CFG6_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL1_CFG6 to value 0"]
impl crate::Resettable for CHNL1_CFG6_SPEC {}
