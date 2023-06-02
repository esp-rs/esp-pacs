#[doc = "Register `CONF` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl From<crate::W<CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRC_SDIO11` reader - *******Description***********"]
pub type FRC_SDIO11_R = crate::FieldReader;
#[doc = "Field `FRC_SDIO11` writer - *******Description***********"]
pub type FRC_SDIO11_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_SPEC, 5, O>;
#[doc = "Field `FRC_SDIO20` reader - *******Description***********"]
pub type FRC_SDIO20_R = crate::FieldReader;
#[doc = "Field `FRC_SDIO20` writer - *******Description***********"]
pub type FRC_SDIO20_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_SPEC, 5, O>;
#[doc = "Field `FRC_NEG_SAMP` reader - *******Description***********"]
pub type FRC_NEG_SAMP_R = crate::FieldReader;
#[doc = "Field `FRC_NEG_SAMP` writer - *******Description***********"]
pub type FRC_NEG_SAMP_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_SPEC, 5, O>;
#[doc = "Field `FRC_POS_SAMP` reader - *******Description***********"]
pub type FRC_POS_SAMP_R = crate::FieldReader;
#[doc = "Field `FRC_POS_SAMP` writer - *******Description***********"]
pub type FRC_POS_SAMP_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_SPEC, 5, O>;
#[doc = "Field `FRC_QUICK_IN` reader - *******Description***********"]
pub type FRC_QUICK_IN_R = crate::FieldReader;
#[doc = "Field `FRC_QUICK_IN` writer - *******Description***********"]
pub type FRC_QUICK_IN_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_SPEC, 5, O>;
#[doc = "Field `SDIO20_INT_DELAY` reader - *******Description***********"]
pub type SDIO20_INT_DELAY_R = crate::BitReader;
#[doc = "Field `SDIO20_INT_DELAY` writer - *******Description***********"]
pub type SDIO20_INT_DELAY_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `SDIO_PAD_PULLUP` reader - *******Description***********"]
pub type SDIO_PAD_PULLUP_R = crate::BitReader;
#[doc = "Field `SDIO_PAD_PULLUP` writer - *******Description***********"]
pub type SDIO_PAD_PULLUP_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
#[doc = "Field `HSPEED_CON_EN` reader - *******Description***********"]
pub type HSPEED_CON_EN_R = crate::BitReader;
#[doc = "Field `HSPEED_CON_EN` writer - *******Description***********"]
pub type HSPEED_CON_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:4 - *******Description***********"]
    #[inline(always)]
    pub fn frc_sdio11(&self) -> FRC_SDIO11_R {
        FRC_SDIO11_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - *******Description***********"]
    #[inline(always)]
    pub fn frc_sdio20(&self) -> FRC_SDIO20_R {
        FRC_SDIO20_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - *******Description***********"]
    #[inline(always)]
    pub fn frc_neg_samp(&self) -> FRC_NEG_SAMP_R {
        FRC_NEG_SAMP_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - *******Description***********"]
    #[inline(always)]
    pub fn frc_pos_samp(&self) -> FRC_POS_SAMP_R {
        FRC_POS_SAMP_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - *******Description***********"]
    #[inline(always)]
    pub fn frc_quick_in(&self) -> FRC_QUICK_IN_R {
        FRC_QUICK_IN_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - *******Description***********"]
    #[inline(always)]
    pub fn sdio20_int_delay(&self) -> SDIO20_INT_DELAY_R {
        SDIO20_INT_DELAY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - *******Description***********"]
    #[inline(always)]
    pub fn sdio_pad_pullup(&self) -> SDIO_PAD_PULLUP_R {
        SDIO_PAD_PULLUP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - *******Description***********"]
    #[inline(always)]
    pub fn hspeed_con_en(&self) -> HSPEED_CON_EN_R {
        HSPEED_CON_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("frc_sdio11", &format_args!("{}", self.frc_sdio11().bits()))
            .field("frc_sdio20", &format_args!("{}", self.frc_sdio20().bits()))
            .field(
                "frc_neg_samp",
                &format_args!("{}", self.frc_neg_samp().bits()),
            )
            .field(
                "frc_pos_samp",
                &format_args!("{}", self.frc_pos_samp().bits()),
            )
            .field(
                "frc_quick_in",
                &format_args!("{}", self.frc_quick_in().bits()),
            )
            .field(
                "sdio20_int_delay",
                &format_args!("{}", self.sdio20_int_delay().bit()),
            )
            .field(
                "sdio_pad_pullup",
                &format_args!("{}", self.sdio_pad_pullup().bit()),
            )
            .field(
                "hspeed_con_en",
                &format_args!("{}", self.hspeed_con_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn frc_sdio11(&mut self) -> FRC_SDIO11_W<0> {
        FRC_SDIO11_W::new(self)
    }
    #[doc = "Bits 5:9 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn frc_sdio20(&mut self) -> FRC_SDIO20_W<5> {
        FRC_SDIO20_W::new(self)
    }
    #[doc = "Bits 10:14 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn frc_neg_samp(&mut self) -> FRC_NEG_SAMP_W<10> {
        FRC_NEG_SAMP_W::new(self)
    }
    #[doc = "Bits 15:19 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn frc_pos_samp(&mut self) -> FRC_POS_SAMP_W<15> {
        FRC_POS_SAMP_W::new(self)
    }
    #[doc = "Bits 20:24 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn frc_quick_in(&mut self) -> FRC_QUICK_IN_W<20> {
        FRC_QUICK_IN_W::new(self)
    }
    #[doc = "Bit 25 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn sdio20_int_delay(&mut self) -> SDIO20_INT_DELAY_W<25> {
        SDIO20_INT_DELAY_W::new(self)
    }
    #[doc = "Bit 26 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_pad_pullup(&mut self) -> SDIO_PAD_PULLUP_W<26> {
        SDIO_PAD_PULLUP_W::new(self)
    }
    #[doc = "Bit 27 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn hspeed_con_en(&mut self) -> HSPEED_CON_EN_W<27> {
        HSPEED_CON_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R](R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
