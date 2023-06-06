#[doc = "Register `BROWN_OUT` reader"]
pub struct R(crate::R<BROWN_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BROWN_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BROWN_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BROWN_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BROWN_OUT` writer"]
pub struct W(crate::W<BROWN_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BROWN_OUT_SPEC>;
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
impl From<crate::W<BROWN_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BROWN_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BROWN_OUT_INT_WAIT` reader - brown out interrupt wait cycles"]
pub type BROWN_OUT_INT_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `BROWN_OUT_INT_WAIT` writer - brown out interrupt wait cycles"]
pub type BROWN_OUT_INT_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, BROWN_OUT_SPEC, 10, O, u16>;
#[doc = "Field `BROWN_OUT_CLOSE_FLASH_ENA` reader - enable close flash when brown out happens"]
pub type BROWN_OUT_CLOSE_FLASH_ENA_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_CLOSE_FLASH_ENA` writer - enable close flash when brown out happens"]
pub type BROWN_OUT_CLOSE_FLASH_ENA_W<'a, const O: u8> = crate::BitWriter<'a, BROWN_OUT_SPEC, O>;
#[doc = "Field `BROWN_OUT_PD_RF_ENA` reader - enable power down RF when brown out happens"]
pub type BROWN_OUT_PD_RF_ENA_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_PD_RF_ENA` writer - enable power down RF when brown out happens"]
pub type BROWN_OUT_PD_RF_ENA_W<'a, const O: u8> = crate::BitWriter<'a, BROWN_OUT_SPEC, O>;
#[doc = "Field `BROWN_OUT_RST_WAIT` reader - brown out reset wait cycles"]
pub type BROWN_OUT_RST_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `BROWN_OUT_RST_WAIT` writer - brown out reset wait cycles"]
pub type BROWN_OUT_RST_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, BROWN_OUT_SPEC, 10, O, u16>;
#[doc = "Field `BROWN_OUT_RST_ENA` reader - enable brown out reset"]
pub type BROWN_OUT_RST_ENA_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_RST_ENA` writer - enable brown out reset"]
pub type BROWN_OUT_RST_ENA_W<'a, const O: u8> = crate::BitWriter<'a, BROWN_OUT_SPEC, O>;
#[doc = "Field `BROWN_OUT_RST_SEL` reader - 1: 4-pos reset"]
pub type BROWN_OUT_RST_SEL_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_RST_SEL` writer - 1: 4-pos reset"]
pub type BROWN_OUT_RST_SEL_W<'a, const O: u8> = crate::BitWriter<'a, BROWN_OUT_SPEC, O>;
#[doc = "Field `BROWN_OUT_ANA_RST_EN` reader - Need add desc"]
pub type BROWN_OUT_ANA_RST_EN_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_ANA_RST_EN` writer - Need add desc"]
pub type BROWN_OUT_ANA_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, BROWN_OUT_SPEC, O>;
#[doc = "Field `BROWN_OUT_CNT_CLR` reader - clear brown out counter"]
pub type BROWN_OUT_CNT_CLR_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_CNT_CLR` writer - clear brown out counter"]
pub type BROWN_OUT_CNT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, BROWN_OUT_SPEC, O>;
#[doc = "Field `BROWN_OUT_ENA` reader - enable brown out"]
pub type BROWN_OUT_ENA_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_ENA` writer - enable brown out"]
pub type BROWN_OUT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, BROWN_OUT_SPEC, O>;
#[doc = "Field `DET` reader - Need add desc"]
pub type DET_R = crate::BitReader;
#[doc = "Field `DET` writer - Need add desc"]
pub type DET_W<'a, const O: u8> = crate::BitWriter<'a, BROWN_OUT_SPEC, O>;
impl R {
    #[doc = "Bits 4:13 - brown out interrupt wait cycles"]
    #[inline(always)]
    pub fn brown_out_int_wait(&self) -> BROWN_OUT_INT_WAIT_R {
        BROWN_OUT_INT_WAIT_R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
    #[doc = "Bit 14 - enable close flash when brown out happens"]
    #[inline(always)]
    pub fn brown_out_close_flash_ena(&self) -> BROWN_OUT_CLOSE_FLASH_ENA_R {
        BROWN_OUT_CLOSE_FLASH_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - enable power down RF when brown out happens"]
    #[inline(always)]
    pub fn brown_out_pd_rf_ena(&self) -> BROWN_OUT_PD_RF_ENA_R {
        BROWN_OUT_PD_RF_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:25 - brown out reset wait cycles"]
    #[inline(always)]
    pub fn brown_out_rst_wait(&self) -> BROWN_OUT_RST_WAIT_R {
        BROWN_OUT_RST_WAIT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - enable brown out reset"]
    #[inline(always)]
    pub fn brown_out_rst_ena(&self) -> BROWN_OUT_RST_ENA_R {
        BROWN_OUT_RST_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: 4-pos reset"]
    #[inline(always)]
    pub fn brown_out_rst_sel(&self) -> BROWN_OUT_RST_SEL_R {
        BROWN_OUT_RST_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Need add desc"]
    #[inline(always)]
    pub fn brown_out_ana_rst_en(&self) -> BROWN_OUT_ANA_RST_EN_R {
        BROWN_OUT_ANA_RST_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - clear brown out counter"]
    #[inline(always)]
    pub fn brown_out_cnt_clr(&self) -> BROWN_OUT_CNT_CLR_R {
        BROWN_OUT_CNT_CLR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - enable brown out"]
    #[inline(always)]
    pub fn brown_out_ena(&self) -> BROWN_OUT_ENA_R {
        BROWN_OUT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn det(&self) -> DET_R {
        DET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BROWN_OUT")
            .field(
                "brown_out_int_wait",
                &format_args!("{}", self.brown_out_int_wait().bits()),
            )
            .field(
                "brown_out_close_flash_ena",
                &format_args!("{}", self.brown_out_close_flash_ena().bit()),
            )
            .field(
                "brown_out_pd_rf_ena",
                &format_args!("{}", self.brown_out_pd_rf_ena().bit()),
            )
            .field(
                "brown_out_rst_wait",
                &format_args!("{}", self.brown_out_rst_wait().bits()),
            )
            .field(
                "brown_out_rst_ena",
                &format_args!("{}", self.brown_out_rst_ena().bit()),
            )
            .field(
                "brown_out_rst_sel",
                &format_args!("{}", self.brown_out_rst_sel().bit()),
            )
            .field(
                "brown_out_ana_rst_en",
                &format_args!("{}", self.brown_out_ana_rst_en().bit()),
            )
            .field(
                "brown_out_cnt_clr",
                &format_args!("{}", self.brown_out_cnt_clr().bit()),
            )
            .field(
                "brown_out_ena",
                &format_args!("{}", self.brown_out_ena().bit()),
            )
            .field("det", &format_args!("{}", self.det().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BROWN_OUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 4:13 - brown out interrupt wait cycles"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_int_wait(&mut self) -> BROWN_OUT_INT_WAIT_W<4> {
        BROWN_OUT_INT_WAIT_W::new(self)
    }
    #[doc = "Bit 14 - enable close flash when brown out happens"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_close_flash_ena(&mut self) -> BROWN_OUT_CLOSE_FLASH_ENA_W<14> {
        BROWN_OUT_CLOSE_FLASH_ENA_W::new(self)
    }
    #[doc = "Bit 15 - enable power down RF when brown out happens"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_pd_rf_ena(&mut self) -> BROWN_OUT_PD_RF_ENA_W<15> {
        BROWN_OUT_PD_RF_ENA_W::new(self)
    }
    #[doc = "Bits 16:25 - brown out reset wait cycles"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_rst_wait(&mut self) -> BROWN_OUT_RST_WAIT_W<16> {
        BROWN_OUT_RST_WAIT_W::new(self)
    }
    #[doc = "Bit 26 - enable brown out reset"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_rst_ena(&mut self) -> BROWN_OUT_RST_ENA_W<26> {
        BROWN_OUT_RST_ENA_W::new(self)
    }
    #[doc = "Bit 27 - 1: 4-pos reset"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_rst_sel(&mut self) -> BROWN_OUT_RST_SEL_W<27> {
        BROWN_OUT_RST_SEL_W::new(self)
    }
    #[doc = "Bit 28 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_ana_rst_en(&mut self) -> BROWN_OUT_ANA_RST_EN_W<28> {
        BROWN_OUT_ANA_RST_EN_W::new(self)
    }
    #[doc = "Bit 29 - clear brown out counter"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_cnt_clr(&mut self) -> BROWN_OUT_CNT_CLR_W<29> {
        BROWN_OUT_CNT_CLR_W::new(self)
    }
    #[doc = "Bit 30 - enable brown out"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_ena(&mut self) -> BROWN_OUT_ENA_W<30> {
        BROWN_OUT_ENA_W::new(self)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn det(&mut self) -> DET_W<31> {
        DET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brown_out](index.html) module"]
pub struct BROWN_OUT_SPEC;
impl crate::RegisterSpec for BROWN_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brown_out::R](R) reader structure"]
impl crate::Readable for BROWN_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brown_out::W](W) writer structure"]
impl crate::Writable for BROWN_OUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BROWN_OUT to value 0x43ff_0010"]
impl crate::Resettable for BROWN_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0x43ff_0010;
}
