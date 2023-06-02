#[doc = "Register `ESCO_CONF0` reader"]
pub struct R(crate::R<ESCO_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESCO_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESCO_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESCO_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESCO_CONF0` writer"]
pub struct W(crate::W<ESCO_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESCO_CONF0_SPEC>;
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
impl From<crate::W<ESCO_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESCO_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESCO_EN` reader - "]
pub type ESCO_EN_R = crate::BitReader;
#[doc = "Field `ESCO_EN` writer - "]
pub type ESCO_EN_W<'a, const O: u8> = crate::BitWriter<'a, ESCO_CONF0_SPEC, O>;
#[doc = "Field `ESCO_CHAN_MOD` reader - "]
pub type ESCO_CHAN_MOD_R = crate::BitReader;
#[doc = "Field `ESCO_CHAN_MOD` writer - "]
pub type ESCO_CHAN_MOD_W<'a, const O: u8> = crate::BitWriter<'a, ESCO_CONF0_SPEC, O>;
#[doc = "Field `ESCO_CVSD_DEC_PACK_ERR` reader - "]
pub type ESCO_CVSD_DEC_PACK_ERR_R = crate::BitReader;
#[doc = "Field `ESCO_CVSD_DEC_PACK_ERR` writer - "]
pub type ESCO_CVSD_DEC_PACK_ERR_W<'a, const O: u8> = crate::BitWriter<'a, ESCO_CONF0_SPEC, O>;
#[doc = "Field `ESCO_CVSD_PACK_LEN_8K` reader - "]
pub type ESCO_CVSD_PACK_LEN_8K_R = crate::FieldReader;
#[doc = "Field `ESCO_CVSD_PACK_LEN_8K` writer - "]
pub type ESCO_CVSD_PACK_LEN_8K_W<'a, const O: u8> = crate::FieldWriter<'a, ESCO_CONF0_SPEC, 5, O>;
#[doc = "Field `ESCO_CVSD_INF_EN` reader - "]
pub type ESCO_CVSD_INF_EN_R = crate::BitReader;
#[doc = "Field `ESCO_CVSD_INF_EN` writer - "]
pub type ESCO_CVSD_INF_EN_W<'a, const O: u8> = crate::BitWriter<'a, ESCO_CONF0_SPEC, O>;
#[doc = "Field `CVSD_DEC_START` reader - "]
pub type CVSD_DEC_START_R = crate::BitReader;
#[doc = "Field `CVSD_DEC_START` writer - "]
pub type CVSD_DEC_START_W<'a, const O: u8> = crate::BitWriter<'a, ESCO_CONF0_SPEC, O>;
#[doc = "Field `CVSD_DEC_RESET` reader - "]
pub type CVSD_DEC_RESET_R = crate::BitReader;
#[doc = "Field `CVSD_DEC_RESET` writer - "]
pub type CVSD_DEC_RESET_W<'a, const O: u8> = crate::BitWriter<'a, ESCO_CONF0_SPEC, O>;
#[doc = "Field `PLC_EN` reader - "]
pub type PLC_EN_R = crate::BitReader;
#[doc = "Field `PLC_EN` writer - "]
pub type PLC_EN_W<'a, const O: u8> = crate::BitWriter<'a, ESCO_CONF0_SPEC, O>;
#[doc = "Field `PLC2DMA_EN` reader - "]
pub type PLC2DMA_EN_R = crate::BitReader;
#[doc = "Field `PLC2DMA_EN` writer - "]
pub type PLC2DMA_EN_W<'a, const O: u8> = crate::BitWriter<'a, ESCO_CONF0_SPEC, O>;
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
            .field("esco_en", &format_args!("{}", self.esco_en().bit()))
            .field(
                "esco_chan_mod",
                &format_args!("{}", self.esco_chan_mod().bit()),
            )
            .field(
                "esco_cvsd_dec_pack_err",
                &format_args!("{}", self.esco_cvsd_dec_pack_err().bit()),
            )
            .field(
                "esco_cvsd_pack_len_8k",
                &format_args!("{}", self.esco_cvsd_pack_len_8k().bits()),
            )
            .field(
                "esco_cvsd_inf_en",
                &format_args!("{}", self.esco_cvsd_inf_en().bit()),
            )
            .field(
                "cvsd_dec_start",
                &format_args!("{}", self.cvsd_dec_start().bit()),
            )
            .field(
                "cvsd_dec_reset",
                &format_args!("{}", self.cvsd_dec_reset().bit()),
            )
            .field("plc_en", &format_args!("{}", self.plc_en().bit()))
            .field("plc2dma_en", &format_args!("{}", self.plc2dma_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ESCO_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn esco_en(&mut self) -> ESCO_EN_W<0> {
        ESCO_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn esco_chan_mod(&mut self) -> ESCO_CHAN_MOD_W<1> {
        ESCO_CHAN_MOD_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn esco_cvsd_dec_pack_err(&mut self) -> ESCO_CVSD_DEC_PACK_ERR_W<2> {
        ESCO_CVSD_DEC_PACK_ERR_W::new(self)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    #[must_use]
    pub fn esco_cvsd_pack_len_8k(&mut self) -> ESCO_CVSD_PACK_LEN_8K_W<3> {
        ESCO_CVSD_PACK_LEN_8K_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn esco_cvsd_inf_en(&mut self) -> ESCO_CVSD_INF_EN_W<8> {
        ESCO_CVSD_INF_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cvsd_dec_start(&mut self) -> CVSD_DEC_START_W<9> {
        CVSD_DEC_START_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cvsd_dec_reset(&mut self) -> CVSD_DEC_RESET_W<10> {
        CVSD_DEC_RESET_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn plc_en(&mut self) -> PLC_EN_W<11> {
        PLC_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn plc2dma_en(&mut self) -> PLC2DMA_EN_W<12> {
        PLC2DMA_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esco_conf0](index.html) module"]
pub struct ESCO_CONF0_SPEC;
impl crate::RegisterSpec for ESCO_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esco_conf0::R](R) reader structure"]
impl crate::Readable for ESCO_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esco_conf0::W](W) writer structure"]
impl crate::Writable for ESCO_CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESCO_CONF0 to value 0"]
impl crate::Resettable for ESCO_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
