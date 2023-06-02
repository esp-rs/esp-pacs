#[doc = "Register `SAR_TSENS_CTRL` reader"]
pub struct R(crate::R<SAR_TSENS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TSENS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TSENS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TSENS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TSENS_CTRL` writer"]
pub struct W(crate::W<SAR_TSENS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TSENS_CTRL_SPEC>;
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
impl From<crate::W<SAR_TSENS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TSENS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_TSENS_OUT` reader - temperature sensor data out"]
pub type SAR_TSENS_OUT_R = crate::FieldReader;
#[doc = "Field `SAR_TSENS_READY` reader - indicate temperature sensor out ready"]
pub type SAR_TSENS_READY_R = crate::BitReader;
#[doc = "Field `SAR_TSENS_INT_EN` reader - enable temperature sensor to send out interrupt"]
pub type SAR_TSENS_INT_EN_R = crate::BitReader;
#[doc = "Field `SAR_TSENS_INT_EN` writer - enable temperature sensor to send out interrupt"]
pub type SAR_TSENS_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TSENS_CTRL_SPEC, O>;
#[doc = "Field `SAR_TSENS_IN_INV` reader - invert temperature sensor data"]
pub type SAR_TSENS_IN_INV_R = crate::BitReader;
#[doc = "Field `SAR_TSENS_IN_INV` writer - invert temperature sensor data"]
pub type SAR_TSENS_IN_INV_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TSENS_CTRL_SPEC, O>;
#[doc = "Field `SAR_TSENS_CLK_DIV` reader - temperature sensor clock divider"]
pub type SAR_TSENS_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `SAR_TSENS_CLK_DIV` writer - temperature sensor clock divider"]
pub type SAR_TSENS_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_TSENS_CTRL_SPEC, 8, O>;
#[doc = "Field `SAR_TSENS_POWER_UP` reader - temperature sensor power up"]
pub type SAR_TSENS_POWER_UP_R = crate::BitReader;
#[doc = "Field `SAR_TSENS_POWER_UP` writer - temperature sensor power up"]
pub type SAR_TSENS_POWER_UP_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TSENS_CTRL_SPEC, O>;
#[doc = "Field `SAR_TSENS_POWER_UP_FORCE` reader - 1: dump out &amp; power up controlled by SW 0: by FSM"]
pub type SAR_TSENS_POWER_UP_FORCE_R = crate::BitReader;
#[doc = "Field `SAR_TSENS_POWER_UP_FORCE` writer - 1: dump out &amp; power up controlled by SW 0: by FSM"]
pub type SAR_TSENS_POWER_UP_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TSENS_CTRL_SPEC, O>;
#[doc = "Field `SAR_TSENS_DUMP_OUT` reader - temperature sensor dump out only active when reg_tsens_power_up_force = 1"]
pub type SAR_TSENS_DUMP_OUT_R = crate::BitReader;
#[doc = "Field `SAR_TSENS_DUMP_OUT` writer - temperature sensor dump out only active when reg_tsens_power_up_force = 1"]
pub type SAR_TSENS_DUMP_OUT_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TSENS_CTRL_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - temperature sensor data out"]
    #[inline(always)]
    pub fn sar_tsens_out(&self) -> SAR_TSENS_OUT_R {
        SAR_TSENS_OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - indicate temperature sensor out ready"]
    #[inline(always)]
    pub fn sar_tsens_ready(&self) -> SAR_TSENS_READY_R {
        SAR_TSENS_READY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - enable temperature sensor to send out interrupt"]
    #[inline(always)]
    pub fn sar_tsens_int_en(&self) -> SAR_TSENS_INT_EN_R {
        SAR_TSENS_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - invert temperature sensor data"]
    #[inline(always)]
    pub fn sar_tsens_in_inv(&self) -> SAR_TSENS_IN_INV_R {
        SAR_TSENS_IN_INV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:21 - temperature sensor clock divider"]
    #[inline(always)]
    pub fn sar_tsens_clk_div(&self) -> SAR_TSENS_CLK_DIV_R {
        SAR_TSENS_CLK_DIV_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 22 - temperature sensor power up"]
    #[inline(always)]
    pub fn sar_tsens_power_up(&self) -> SAR_TSENS_POWER_UP_R {
        SAR_TSENS_POWER_UP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: dump out &amp; power up controlled by SW 0: by FSM"]
    #[inline(always)]
    pub fn sar_tsens_power_up_force(&self) -> SAR_TSENS_POWER_UP_FORCE_R {
        SAR_TSENS_POWER_UP_FORCE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - temperature sensor dump out only active when reg_tsens_power_up_force = 1"]
    #[inline(always)]
    pub fn sar_tsens_dump_out(&self) -> SAR_TSENS_DUMP_OUT_R {
        SAR_TSENS_DUMP_OUT_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TSENS_CTRL")
            .field(
                "sar_tsens_out",
                &format_args!("{}", self.sar_tsens_out().bits()),
            )
            .field(
                "sar_tsens_ready",
                &format_args!("{}", self.sar_tsens_ready().bit()),
            )
            .field(
                "sar_tsens_int_en",
                &format_args!("{}", self.sar_tsens_int_en().bit()),
            )
            .field(
                "sar_tsens_in_inv",
                &format_args!("{}", self.sar_tsens_in_inv().bit()),
            )
            .field(
                "sar_tsens_clk_div",
                &format_args!("{}", self.sar_tsens_clk_div().bits()),
            )
            .field(
                "sar_tsens_power_up",
                &format_args!("{}", self.sar_tsens_power_up().bit()),
            )
            .field(
                "sar_tsens_power_up_force",
                &format_args!("{}", self.sar_tsens_power_up_force().bit()),
            )
            .field(
                "sar_tsens_dump_out",
                &format_args!("{}", self.sar_tsens_dump_out().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TSENS_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 12 - enable temperature sensor to send out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sar_tsens_int_en(&mut self) -> SAR_TSENS_INT_EN_W<12> {
        SAR_TSENS_INT_EN_W::new(self)
    }
    #[doc = "Bit 13 - invert temperature sensor data"]
    #[inline(always)]
    #[must_use]
    pub fn sar_tsens_in_inv(&mut self) -> SAR_TSENS_IN_INV_W<13> {
        SAR_TSENS_IN_INV_W::new(self)
    }
    #[doc = "Bits 14:21 - temperature sensor clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn sar_tsens_clk_div(&mut self) -> SAR_TSENS_CLK_DIV_W<14> {
        SAR_TSENS_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 22 - temperature sensor power up"]
    #[inline(always)]
    #[must_use]
    pub fn sar_tsens_power_up(&mut self) -> SAR_TSENS_POWER_UP_W<22> {
        SAR_TSENS_POWER_UP_W::new(self)
    }
    #[doc = "Bit 23 - 1: dump out &amp; power up controlled by SW 0: by FSM"]
    #[inline(always)]
    #[must_use]
    pub fn sar_tsens_power_up_force(&mut self) -> SAR_TSENS_POWER_UP_FORCE_W<23> {
        SAR_TSENS_POWER_UP_FORCE_W::new(self)
    }
    #[doc = "Bit 24 - temperature sensor dump out only active when reg_tsens_power_up_force = 1"]
    #[inline(always)]
    #[must_use]
    pub fn sar_tsens_dump_out(&mut self) -> SAR_TSENS_DUMP_OUT_W<24> {
        SAR_TSENS_DUMP_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure tsens controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_tsens_ctrl](index.html) module"]
pub struct SAR_TSENS_CTRL_SPEC;
impl crate::RegisterSpec for SAR_TSENS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_tsens_ctrl::R](R) reader structure"]
impl crate::Readable for SAR_TSENS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_tsens_ctrl::W](W) writer structure"]
impl crate::Writable for SAR_TSENS_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_TSENS_CTRL to value 0x0001_9000"]
impl crate::Resettable for SAR_TSENS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_9000;
}
