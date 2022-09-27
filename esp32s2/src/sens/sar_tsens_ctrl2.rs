#[doc = "Register `SAR_TSENS_CTRL2` reader"]
pub struct R(crate::R<SAR_TSENS_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TSENS_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TSENS_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TSENS_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TSENS_CTRL2` writer"]
pub struct W(crate::W<SAR_TSENS_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TSENS_CTRL2_SPEC>;
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
impl From<crate::W<SAR_TSENS_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TSENS_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSENS_XPD_WAIT` reader - "]
pub type TSENS_XPD_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSENS_XPD_WAIT` writer - "]
pub type TSENS_XPD_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_TSENS_CTRL2_SPEC, u16, u16, 12, O>;
#[doc = "Field `TSENS_XPD_FORCE` reader - "]
pub type TSENS_XPD_FORCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSENS_XPD_FORCE` writer - "]
pub type TSENS_XPD_FORCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_TSENS_CTRL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `TSENS_CLK_INV` reader - "]
pub type TSENS_CLK_INV_R = crate::BitReader<bool>;
#[doc = "Field `TSENS_CLK_INV` writer - "]
pub type TSENS_CLK_INV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_TSENS_CTRL2_SPEC, bool, O>;
#[doc = "Field `TSENS_CLKGATE_EN` reader - Enable temperature sensor clock."]
pub type TSENS_CLKGATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `TSENS_CLKGATE_EN` writer - Enable temperature sensor clock."]
pub type TSENS_CLKGATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_TSENS_CTRL2_SPEC, bool, O>;
#[doc = "Field `TSENS_RESET` reader - Reset temperature sensor."]
pub type TSENS_RESET_R = crate::BitReader<bool>;
#[doc = "Field `TSENS_RESET` writer - Reset temperature sensor."]
pub type TSENS_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_TSENS_CTRL2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsens_xpd_wait(&self) -> TSENS_XPD_WAIT_R {
        TSENS_XPD_WAIT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tsens_xpd_force(&self) -> TSENS_XPD_FORCE_R {
        TSENS_XPD_FORCE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tsens_clk_inv(&self) -> TSENS_CLK_INV_R {
        TSENS_CLK_INV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable temperature sensor clock."]
    #[inline(always)]
    pub fn tsens_clkgate_en(&self) -> TSENS_CLKGATE_EN_R {
        TSENS_CLKGATE_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset temperature sensor."]
    #[inline(always)]
    pub fn tsens_reset(&self) -> TSENS_RESET_R {
        TSENS_RESET_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsens_xpd_wait(&mut self) -> TSENS_XPD_WAIT_W<0> {
        TSENS_XPD_WAIT_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tsens_xpd_force(&mut self) -> TSENS_XPD_FORCE_W<12> {
        TSENS_XPD_FORCE_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tsens_clk_inv(&mut self) -> TSENS_CLK_INV_W<14> {
        TSENS_CLK_INV_W::new(self)
    }
    #[doc = "Bit 15 - Enable temperature sensor clock."]
    #[inline(always)]
    pub fn tsens_clkgate_en(&mut self) -> TSENS_CLKGATE_EN_W<15> {
        TSENS_CLKGATE_EN_W::new(self)
    }
    #[doc = "Bit 16 - Reset temperature sensor."]
    #[inline(always)]
    pub fn tsens_reset(&mut self) -> TSENS_RESET_W<16> {
        TSENS_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Temperature sensor control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_tsens_ctrl2](index.html) module"]
pub struct SAR_TSENS_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_TSENS_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_tsens_ctrl2::R](R) reader structure"]
impl crate::Readable for SAR_TSENS_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_tsens_ctrl2::W](W) writer structure"]
impl crate::Writable for SAR_TSENS_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_TSENS_CTRL2 to value 0x4002"]
impl crate::Resettable for SAR_TSENS_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4002
    }
}
