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
#[doc = "Field `INT_WAIT` reader - brown out interrupt wait cycles"]
pub type INT_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INT_WAIT` writer - brown out interrupt wait cycles"]
pub type INT_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BROWN_OUT_SPEC, u16, u16, 10, O>;
#[doc = "Field `CLOSE_FLASH_ENA` reader - enable close flash when brown out happens"]
pub type CLOSE_FLASH_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CLOSE_FLASH_ENA` writer - enable close flash when brown out happens"]
pub type CLOSE_FLASH_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BROWN_OUT_SPEC, bool, O>;
#[doc = "Field `PD_RF_ENA` reader - enable power down RF when brown out happens"]
pub type PD_RF_ENA_R = crate::BitReader<bool>;
#[doc = "Field `PD_RF_ENA` writer - enable power down RF when brown out happens"]
pub type PD_RF_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BROWN_OUT_SPEC, bool, O>;
#[doc = "Field `RST_WAIT` reader - brown out reset wait cycles"]
pub type RST_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RST_WAIT` writer - brown out reset wait cycles"]
pub type RST_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BROWN_OUT_SPEC, u16, u16, 10, O>;
#[doc = "Field `RST_ENA` reader - enable brown out reset"]
pub type RST_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RST_ENA` writer - enable brown out reset"]
pub type RST_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BROWN_OUT_SPEC, bool, O>;
#[doc = "Field `RST_SEL` reader - 1: 4-pos reset"]
pub type RST_SEL_R = crate::BitReader<bool>;
#[doc = "Field `RST_SEL` writer - 1: 4-pos reset"]
pub type RST_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BROWN_OUT_SPEC, bool, O>;
#[doc = "Field `ANA_RST_EN` reader - brown_out origin reset enable"]
pub type ANA_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `ANA_RST_EN` writer - brown_out origin reset enable"]
pub type ANA_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BROWN_OUT_SPEC, bool, O>;
#[doc = "Field `CNT_CLR` writer - clear brown out counter"]
pub type CNT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BROWN_OUT_SPEC, bool, O>;
#[doc = "Field `ENA` reader - enable brown out"]
pub type ENA_R = crate::BitReader<bool>;
#[doc = "Field `ENA` writer - enable brown out"]
pub type ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BROWN_OUT_SPEC, bool, O>;
#[doc = "Field `DET` reader - the flag of brown det from analog"]
pub type DET_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 4:13 - brown out interrupt wait cycles"]
    #[inline(always)]
    pub fn int_wait(&self) -> INT_WAIT_R {
        INT_WAIT_R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
    #[doc = "Bit 14 - enable close flash when brown out happens"]
    #[inline(always)]
    pub fn close_flash_ena(&self) -> CLOSE_FLASH_ENA_R {
        CLOSE_FLASH_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - enable power down RF when brown out happens"]
    #[inline(always)]
    pub fn pd_rf_ena(&self) -> PD_RF_ENA_R {
        PD_RF_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:25 - brown out reset wait cycles"]
    #[inline(always)]
    pub fn rst_wait(&self) -> RST_WAIT_R {
        RST_WAIT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - enable brown out reset"]
    #[inline(always)]
    pub fn rst_ena(&self) -> RST_ENA_R {
        RST_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: 4-pos reset"]
    #[inline(always)]
    pub fn rst_sel(&self) -> RST_SEL_R {
        RST_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - brown_out origin reset enable"]
    #[inline(always)]
    pub fn ana_rst_en(&self) -> ANA_RST_EN_R {
        ANA_RST_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - enable brown out"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - the flag of brown det from analog"]
    #[inline(always)]
    pub fn det(&self) -> DET_R {
        DET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:13 - brown out interrupt wait cycles"]
    #[inline(always)]
    pub fn int_wait(&mut self) -> INT_WAIT_W<4> {
        INT_WAIT_W::new(self)
    }
    #[doc = "Bit 14 - enable close flash when brown out happens"]
    #[inline(always)]
    pub fn close_flash_ena(&mut self) -> CLOSE_FLASH_ENA_W<14> {
        CLOSE_FLASH_ENA_W::new(self)
    }
    #[doc = "Bit 15 - enable power down RF when brown out happens"]
    #[inline(always)]
    pub fn pd_rf_ena(&mut self) -> PD_RF_ENA_W<15> {
        PD_RF_ENA_W::new(self)
    }
    #[doc = "Bits 16:25 - brown out reset wait cycles"]
    #[inline(always)]
    pub fn rst_wait(&mut self) -> RST_WAIT_W<16> {
        RST_WAIT_W::new(self)
    }
    #[doc = "Bit 26 - enable brown out reset"]
    #[inline(always)]
    pub fn rst_ena(&mut self) -> RST_ENA_W<26> {
        RST_ENA_W::new(self)
    }
    #[doc = "Bit 27 - 1: 4-pos reset"]
    #[inline(always)]
    pub fn rst_sel(&mut self) -> RST_SEL_W<27> {
        RST_SEL_W::new(self)
    }
    #[doc = "Bit 28 - brown_out origin reset enable"]
    #[inline(always)]
    pub fn ana_rst_en(&mut self) -> ANA_RST_EN_W<28> {
        ANA_RST_EN_W::new(self)
    }
    #[doc = "Bit 29 - clear brown out counter"]
    #[inline(always)]
    pub fn cnt_clr(&mut self) -> CNT_CLR_W<29> {
        CNT_CLR_W::new(self)
    }
    #[doc = "Bit 30 - enable brown out"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W<30> {
        ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brown_out](index.html) module"]
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
}
#[doc = "`reset()` method sets BROWN_OUT to value 0x43ff_0010"]
impl crate::Resettable for BROWN_OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x43ff_0010
    }
}
