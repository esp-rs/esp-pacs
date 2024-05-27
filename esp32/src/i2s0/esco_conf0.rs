#[doc = "Register `ESCO_CONF0` reader"]
pub type R = crate::R<ESCO_CONF0_SPEC>;
#[doc = "Register `ESCO_CONF0` writer"]
pub type W = crate::W<ESCO_CONF0_SPEC>;
#[doc = "Field `ESCO_EN` reader - "]
pub type ESCO_EN_R = crate::BitReader;
#[doc = "Field `ESCO_EN` writer - "]
pub type ESCO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESCO_CHAN_MOD` reader - "]
pub type ESCO_CHAN_MOD_R = crate::BitReader;
#[doc = "Field `ESCO_CHAN_MOD` writer - "]
pub type ESCO_CHAN_MOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESCO_CVSD_DEC_PACK_ERR` reader - "]
pub type ESCO_CVSD_DEC_PACK_ERR_R = crate::BitReader;
#[doc = "Field `ESCO_CVSD_DEC_PACK_ERR` writer - "]
pub type ESCO_CVSD_DEC_PACK_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESCO_CVSD_PACK_LEN_8K` reader - "]
pub type ESCO_CVSD_PACK_LEN_8K_R = crate::FieldReader;
#[doc = "Field `ESCO_CVSD_PACK_LEN_8K` writer - "]
pub type ESCO_CVSD_PACK_LEN_8K_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ESCO_CVSD_INF_EN` reader - "]
pub type ESCO_CVSD_INF_EN_R = crate::BitReader;
#[doc = "Field `ESCO_CVSD_INF_EN` writer - "]
pub type ESCO_CVSD_INF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CVSD_DEC_START` reader - "]
pub type CVSD_DEC_START_R = crate::BitReader;
#[doc = "Field `CVSD_DEC_START` writer - "]
pub type CVSD_DEC_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CVSD_DEC_RESET` reader - "]
pub type CVSD_DEC_RESET_R = crate::BitReader;
#[doc = "Field `CVSD_DEC_RESET` writer - "]
pub type CVSD_DEC_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLC_EN` reader - "]
pub type PLC_EN_R = crate::BitReader;
#[doc = "Field `PLC_EN` writer - "]
pub type PLC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLC2DMA_EN` reader - "]
pub type PLC2DMA_EN_R = crate::BitReader;
#[doc = "Field `PLC2DMA_EN` writer - "]
pub type PLC2DMA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn esco_en(&self) -> ESCO_EN_R {
        ESCO_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn esco_chan_mod(&self) -> ESCO_CHAN_MOD_R {
        ESCO_CHAN_MOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn esco_cvsd_dec_pack_err(&self) -> ESCO_CVSD_DEC_PACK_ERR_R {
        ESCO_CVSD_DEC_PACK_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn esco_cvsd_pack_len_8k(&self) -> ESCO_CVSD_PACK_LEN_8K_R {
        ESCO_CVSD_PACK_LEN_8K_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn esco_cvsd_inf_en(&self) -> ESCO_CVSD_INF_EN_R {
        ESCO_CVSD_INF_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cvsd_dec_start(&self) -> CVSD_DEC_START_R {
        CVSD_DEC_START_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cvsd_dec_reset(&self) -> CVSD_DEC_RESET_R {
        CVSD_DEC_RESET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn plc_en(&self) -> PLC_EN_R {
        PLC_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn plc2dma_en(&self) -> PLC2DMA_EN_R {
        PLC2DMA_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESCO_CONF0")
            .field("esco_en", &self.esco_en())
            .field("esco_chan_mod", &self.esco_chan_mod())
            .field("esco_cvsd_dec_pack_err", &self.esco_cvsd_dec_pack_err())
            .field("esco_cvsd_pack_len_8k", &self.esco_cvsd_pack_len_8k())
            .field("esco_cvsd_inf_en", &self.esco_cvsd_inf_en())
            .field("cvsd_dec_start", &self.cvsd_dec_start())
            .field("cvsd_dec_reset", &self.cvsd_dec_reset())
            .field("plc_en", &self.plc_en())
            .field("plc2dma_en", &self.plc2dma_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn esco_en(&mut self) -> ESCO_EN_W<ESCO_CONF0_SPEC> {
        ESCO_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn esco_chan_mod(&mut self) -> ESCO_CHAN_MOD_W<ESCO_CONF0_SPEC> {
        ESCO_CHAN_MOD_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn esco_cvsd_dec_pack_err(&mut self) -> ESCO_CVSD_DEC_PACK_ERR_W<ESCO_CONF0_SPEC> {
        ESCO_CVSD_DEC_PACK_ERR_W::new(self, 2)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    #[must_use]
    pub fn esco_cvsd_pack_len_8k(&mut self) -> ESCO_CVSD_PACK_LEN_8K_W<ESCO_CONF0_SPEC> {
        ESCO_CVSD_PACK_LEN_8K_W::new(self, 3)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn esco_cvsd_inf_en(&mut self) -> ESCO_CVSD_INF_EN_W<ESCO_CONF0_SPEC> {
        ESCO_CVSD_INF_EN_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cvsd_dec_start(&mut self) -> CVSD_DEC_START_W<ESCO_CONF0_SPEC> {
        CVSD_DEC_START_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cvsd_dec_reset(&mut self) -> CVSD_DEC_RESET_W<ESCO_CONF0_SPEC> {
        CVSD_DEC_RESET_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn plc_en(&mut self) -> PLC_EN_W<ESCO_CONF0_SPEC> {
        PLC_EN_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn plc2dma_en(&mut self) -> PLC2DMA_EN_W<ESCO_CONF0_SPEC> {
        PLC2DMA_EN_W::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esco_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esco_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESCO_CONF0_SPEC;
impl crate::RegisterSpec for ESCO_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esco_conf0::R`](R) reader structure"]
impl crate::Readable for ESCO_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`esco_conf0::W`](W) writer structure"]
impl crate::Writable for ESCO_CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESCO_CONF0 to value 0"]
impl crate::Resettable for ESCO_CONF0_SPEC {
    const RESET_VALUE: u32 = 0;
}
