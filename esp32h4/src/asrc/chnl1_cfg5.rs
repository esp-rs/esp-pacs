#[doc = "Register `CHNL1_CFG5` reader"]
pub type R = crate::R<CHNL1_CFG5_SPEC>;
#[doc = "Register `CHNL1_CFG5` writer"]
pub type W = crate::W<CHNL1_CFG5_SPEC>;
#[doc = "Field `CHNL1_IN_CNT_ENA` reader - Set this bit to enable in data byte counter."]
pub type CHNL1_IN_CNT_ENA_R = crate::BitReader;
#[doc = "Field `CHNL1_IN_CNT_ENA` writer - Set this bit to enable in data byte counter."]
pub type CHNL1_IN_CNT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_IN_CNT_CLR` writer - Set this bit to clear in data byte counter."]
pub type CHNL1_IN_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_IN_LEN` reader - Write the bits to specify the data byte numbers of data from the DMA"]
pub type CHNL1_IN_LEN_R = crate::FieldReader<u32>;
#[doc = "Field `CHNL1_IN_LEN` writer - Write the bits to specify the data byte numbers of data from the DMA"]
pub type CHNL1_IN_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable in data byte counter."]
    #[inline(always)]
    pub fn chnl1_in_cnt_ena(&self) -> CHNL1_IN_CNT_ENA_R {
        CHNL1_IN_CNT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write the bits to specify the data byte numbers of data from the DMA"]
    #[inline(always)]
    pub fn chnl1_in_len(&self) -> CHNL1_IN_LEN_R {
        CHNL1_IN_LEN_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL1_CFG5")
            .field("chnl1_in_cnt_ena", &self.chnl1_in_cnt_ena())
            .field("chnl1_in_len", &self.chnl1_in_len())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable in data byte counter."]
    #[inline(always)]
    pub fn chnl1_in_cnt_ena(&mut self) -> CHNL1_IN_CNT_ENA_W<'_, CHNL1_CFG5_SPEC> {
        CHNL1_IN_CNT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear in data byte counter."]
    #[inline(always)]
    pub fn chnl1_in_cnt_clr(&mut self) -> CHNL1_IN_CNT_CLR_W<'_, CHNL1_CFG5_SPEC> {
        CHNL1_IN_CNT_CLR_W::new(self, 1)
    }
    #[doc = "Bits 8:31 - Write the bits to specify the data byte numbers of data from the DMA"]
    #[inline(always)]
    pub fn chnl1_in_len(&mut self) -> CHNL1_IN_LEN_W<'_, CHNL1_CFG5_SPEC> {
        CHNL1_IN_LEN_W::new(self, 8)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_cfg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_cfg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL1_CFG5_SPEC;
impl crate::RegisterSpec for CHNL1_CFG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl1_cfg5::R`](R) reader structure"]
impl crate::Readable for CHNL1_CFG5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chnl1_cfg5::W`](W) writer structure"]
impl crate::Writable for CHNL1_CFG5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL1_CFG5 to value 0"]
impl crate::Resettable for CHNL1_CFG5_SPEC {}
