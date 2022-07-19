#[doc = "Register `RTC_BROWN_OUT` reader"]
pub struct R(crate::R<RTC_BROWN_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_BROWN_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_BROWN_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_BROWN_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_BROWN_OUT` writer"]
pub struct W(crate::W<RTC_BROWN_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_BROWN_OUT_SPEC>;
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
impl From<crate::W<RTC_BROWN_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_BROWN_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BROWN_OUT_INT_WAIT` reader - brown out interrupt wait cycles"]
pub type BROWN_OUT_INT_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BROWN_OUT_INT_WAIT` writer - brown out interrupt wait cycles"]
pub type BROWN_OUT_INT_WAIT_W<'a> =
    crate::FieldWriter<'a, u32, RTC_BROWN_OUT_SPEC, u16, u16, 10, 4>;
#[doc = "Field `BROWN_OUT_CLOSE_FLASH_ENA` reader - enable close flash when brown out happens"]
pub type BROWN_OUT_CLOSE_FLASH_ENA_R = crate::BitReader<bool>;
#[doc = "Field `BROWN_OUT_CLOSE_FLASH_ENA` writer - enable close flash when brown out happens"]
pub type BROWN_OUT_CLOSE_FLASH_ENA_W<'a> = crate::BitWriter<'a, u32, RTC_BROWN_OUT_SPEC, bool, 14>;
#[doc = "Field `BROWN_OUT_PD_RF_ENA` reader - enable power down RF when brown out happens"]
pub type BROWN_OUT_PD_RF_ENA_R = crate::BitReader<bool>;
#[doc = "Field `BROWN_OUT_PD_RF_ENA` writer - enable power down RF when brown out happens"]
pub type BROWN_OUT_PD_RF_ENA_W<'a> = crate::BitWriter<'a, u32, RTC_BROWN_OUT_SPEC, bool, 15>;
#[doc = "Field `BROWN_OUT_RST_WAIT` reader - brown out reset wait cycles"]
pub type BROWN_OUT_RST_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BROWN_OUT_RST_WAIT` writer - brown out reset wait cycles"]
pub type BROWN_OUT_RST_WAIT_W<'a> =
    crate::FieldWriter<'a, u32, RTC_BROWN_OUT_SPEC, u16, u16, 10, 16>;
#[doc = "Field `BROWN_OUT_RST_ENA` reader - enable brown out reset"]
pub type BROWN_OUT_RST_ENA_R = crate::BitReader<bool>;
#[doc = "Field `BROWN_OUT_RST_ENA` writer - enable brown out reset"]
pub type BROWN_OUT_RST_ENA_W<'a> = crate::BitWriter<'a, u32, RTC_BROWN_OUT_SPEC, bool, 26>;
#[doc = "Field `BROWN_OUT_RST_SEL` reader - 1: 4-pos reset"]
pub type BROWN_OUT_RST_SEL_R = crate::BitReader<bool>;
#[doc = "Field `BROWN_OUT_RST_SEL` writer - 1: 4-pos reset"]
pub type BROWN_OUT_RST_SEL_W<'a> = crate::BitWriter<'a, u32, RTC_BROWN_OUT_SPEC, bool, 27>;
#[doc = "Field `BROWN_OUT_ANA_RST_EN` reader - Need add desc"]
pub type BROWN_OUT_ANA_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `BROWN_OUT_ANA_RST_EN` writer - Need add desc"]
pub type BROWN_OUT_ANA_RST_EN_W<'a> = crate::BitWriter<'a, u32, RTC_BROWN_OUT_SPEC, bool, 28>;
#[doc = "Field `BROWN_OUT_CNT_CLR` reader - clear brown out counter"]
pub type BROWN_OUT_CNT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `BROWN_OUT_CNT_CLR` writer - clear brown out counter"]
pub type BROWN_OUT_CNT_CLR_W<'a> = crate::BitWriter<'a, u32, RTC_BROWN_OUT_SPEC, bool, 29>;
#[doc = "Field `BROWN_OUT_ENA` reader - enable brown out"]
pub type BROWN_OUT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `BROWN_OUT_ENA` writer - enable brown out"]
pub type BROWN_OUT_ENA_W<'a> = crate::BitWriter<'a, u32, RTC_BROWN_OUT_SPEC, bool, 30>;
#[doc = "Field `DET` reader - Need add desc"]
pub type DET_R = crate::BitReader<bool>;
#[doc = "Field `DET` writer - Need add desc"]
pub type DET_W<'a> = crate::BitWriter<'a, u32, RTC_BROWN_OUT_SPEC, bool, 31>;
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
impl W {
    #[doc = "Bits 4:13 - brown out interrupt wait cycles"]
    #[inline(always)]
    pub fn brown_out_int_wait(&mut self) -> BROWN_OUT_INT_WAIT_W {
        BROWN_OUT_INT_WAIT_W::new(self)
    }
    #[doc = "Bit 14 - enable close flash when brown out happens"]
    #[inline(always)]
    pub fn brown_out_close_flash_ena(&mut self) -> BROWN_OUT_CLOSE_FLASH_ENA_W {
        BROWN_OUT_CLOSE_FLASH_ENA_W::new(self)
    }
    #[doc = "Bit 15 - enable power down RF when brown out happens"]
    #[inline(always)]
    pub fn brown_out_pd_rf_ena(&mut self) -> BROWN_OUT_PD_RF_ENA_W {
        BROWN_OUT_PD_RF_ENA_W::new(self)
    }
    #[doc = "Bits 16:25 - brown out reset wait cycles"]
    #[inline(always)]
    pub fn brown_out_rst_wait(&mut self) -> BROWN_OUT_RST_WAIT_W {
        BROWN_OUT_RST_WAIT_W::new(self)
    }
    #[doc = "Bit 26 - enable brown out reset"]
    #[inline(always)]
    pub fn brown_out_rst_ena(&mut self) -> BROWN_OUT_RST_ENA_W {
        BROWN_OUT_RST_ENA_W::new(self)
    }
    #[doc = "Bit 27 - 1: 4-pos reset"]
    #[inline(always)]
    pub fn brown_out_rst_sel(&mut self) -> BROWN_OUT_RST_SEL_W {
        BROWN_OUT_RST_SEL_W::new(self)
    }
    #[doc = "Bit 28 - Need add desc"]
    #[inline(always)]
    pub fn brown_out_ana_rst_en(&mut self) -> BROWN_OUT_ANA_RST_EN_W {
        BROWN_OUT_ANA_RST_EN_W::new(self)
    }
    #[doc = "Bit 29 - clear brown out counter"]
    #[inline(always)]
    pub fn brown_out_cnt_clr(&mut self) -> BROWN_OUT_CNT_CLR_W {
        BROWN_OUT_CNT_CLR_W::new(self)
    }
    #[doc = "Bit 30 - enable brown out"]
    #[inline(always)]
    pub fn brown_out_ena(&mut self) -> BROWN_OUT_ENA_W {
        BROWN_OUT_ENA_W::new(self)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn det(&mut self) -> DET_W {
        DET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_brown_out](index.html) module"]
pub struct RTC_BROWN_OUT_SPEC;
impl crate::RegisterSpec for RTC_BROWN_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_brown_out::R](R) reader structure"]
impl crate::Readable for RTC_BROWN_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_brown_out::W](W) writer structure"]
impl crate::Writable for RTC_BROWN_OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_BROWN_OUT to value 0x43ff_0010"]
impl crate::Resettable for RTC_BROWN_OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x43ff_0010
    }
}
