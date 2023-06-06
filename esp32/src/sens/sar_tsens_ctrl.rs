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
#[doc = "Field `TSENS_XPD_WAIT` reader - "]
pub type TSENS_XPD_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `TSENS_XPD_WAIT` writer - "]
pub type TSENS_XPD_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_TSENS_CTRL_SPEC, 12, O, u16>;
#[doc = "Field `TSENS_XPD_FORCE` reader - "]
pub type TSENS_XPD_FORCE_R = crate::BitReader;
#[doc = "Field `TSENS_XPD_FORCE` writer - "]
pub type TSENS_XPD_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TSENS_CTRL_SPEC, O>;
#[doc = "Field `TSENS_CLK_INV` reader - "]
pub type TSENS_CLK_INV_R = crate::BitReader;
#[doc = "Field `TSENS_CLK_INV` writer - "]
pub type TSENS_CLK_INV_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TSENS_CTRL_SPEC, O>;
#[doc = "Field `TSENS_CLK_GATED` reader - "]
pub type TSENS_CLK_GATED_R = crate::BitReader;
#[doc = "Field `TSENS_CLK_GATED` writer - "]
pub type TSENS_CLK_GATED_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TSENS_CTRL_SPEC, O>;
#[doc = "Field `TSENS_IN_INV` reader - invert temperature sensor data"]
pub type TSENS_IN_INV_R = crate::BitReader;
#[doc = "Field `TSENS_IN_INV` writer - invert temperature sensor data"]
pub type TSENS_IN_INV_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TSENS_CTRL_SPEC, O>;
#[doc = "Field `TSENS_CLK_DIV` reader - temperature sensor clock divider"]
pub type TSENS_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `TSENS_CLK_DIV` writer - temperature sensor clock divider"]
pub type TSENS_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_TSENS_CTRL_SPEC, 8, O>;
#[doc = "Field `TSENS_POWER_UP` reader - temperature sensor power up"]
pub type TSENS_POWER_UP_R = crate::BitReader;
#[doc = "Field `TSENS_POWER_UP` writer - temperature sensor power up"]
pub type TSENS_POWER_UP_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TSENS_CTRL_SPEC, O>;
#[doc = "Field `TSENS_POWER_UP_FORCE` reader - 1: dump out &amp; power up controlled by SW 0: by FSM"]
pub type TSENS_POWER_UP_FORCE_R = crate::BitReader;
#[doc = "Field `TSENS_POWER_UP_FORCE` writer - 1: dump out &amp; power up controlled by SW 0: by FSM"]
pub type TSENS_POWER_UP_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TSENS_CTRL_SPEC, O>;
#[doc = "Field `TSENS_DUMP_OUT` reader - temperature sensor dump out only active when reg_tsens_power_up_force = 1"]
pub type TSENS_DUMP_OUT_R = crate::BitReader;
#[doc = "Field `TSENS_DUMP_OUT` writer - temperature sensor dump out only active when reg_tsens_power_up_force = 1"]
pub type TSENS_DUMP_OUT_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TSENS_CTRL_SPEC, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsens_xpd_wait(&self) -> TSENS_XPD_WAIT_R {
        TSENS_XPD_WAIT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tsens_xpd_force(&self) -> TSENS_XPD_FORCE_R {
        TSENS_XPD_FORCE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tsens_clk_inv(&self) -> TSENS_CLK_INV_R {
        TSENS_CLK_INV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tsens_clk_gated(&self) -> TSENS_CLK_GATED_R {
        TSENS_CLK_GATED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - invert temperature sensor data"]
    #[inline(always)]
    pub fn tsens_in_inv(&self) -> TSENS_IN_INV_R {
        TSENS_IN_INV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - temperature sensor clock divider"]
    #[inline(always)]
    pub fn tsens_clk_div(&self) -> TSENS_CLK_DIV_R {
        TSENS_CLK_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - temperature sensor power up"]
    #[inline(always)]
    pub fn tsens_power_up(&self) -> TSENS_POWER_UP_R {
        TSENS_POWER_UP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: dump out &amp; power up controlled by SW 0: by FSM"]
    #[inline(always)]
    pub fn tsens_power_up_force(&self) -> TSENS_POWER_UP_FORCE_R {
        TSENS_POWER_UP_FORCE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - temperature sensor dump out only active when reg_tsens_power_up_force = 1"]
    #[inline(always)]
    pub fn tsens_dump_out(&self) -> TSENS_DUMP_OUT_R {
        TSENS_DUMP_OUT_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TSENS_CTRL")
            .field(
                "tsens_xpd_wait",
                &format_args!("{}", self.tsens_xpd_wait().bits()),
            )
            .field(
                "tsens_xpd_force",
                &format_args!("{}", self.tsens_xpd_force().bit()),
            )
            .field(
                "tsens_clk_inv",
                &format_args!("{}", self.tsens_clk_inv().bit()),
            )
            .field(
                "tsens_clk_gated",
                &format_args!("{}", self.tsens_clk_gated().bit()),
            )
            .field(
                "tsens_in_inv",
                &format_args!("{}", self.tsens_in_inv().bit()),
            )
            .field(
                "tsens_clk_div",
                &format_args!("{}", self.tsens_clk_div().bits()),
            )
            .field(
                "tsens_power_up",
                &format_args!("{}", self.tsens_power_up().bit()),
            )
            .field(
                "tsens_power_up_force",
                &format_args!("{}", self.tsens_power_up_force().bit()),
            )
            .field(
                "tsens_dump_out",
                &format_args!("{}", self.tsens_dump_out().bit()),
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
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_xpd_wait(&mut self) -> TSENS_XPD_WAIT_W<0> {
        TSENS_XPD_WAIT_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_xpd_force(&mut self) -> TSENS_XPD_FORCE_W<12> {
        TSENS_XPD_FORCE_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_inv(&mut self) -> TSENS_CLK_INV_W<13> {
        TSENS_CLK_INV_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_gated(&mut self) -> TSENS_CLK_GATED_W<14> {
        TSENS_CLK_GATED_W::new(self)
    }
    #[doc = "Bit 15 - invert temperature sensor data"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_in_inv(&mut self) -> TSENS_IN_INV_W<15> {
        TSENS_IN_INV_W::new(self)
    }
    #[doc = "Bits 16:23 - temperature sensor clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_div(&mut self) -> TSENS_CLK_DIV_W<16> {
        TSENS_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 24 - temperature sensor power up"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_power_up(&mut self) -> TSENS_POWER_UP_W<24> {
        TSENS_POWER_UP_W::new(self)
    }
    #[doc = "Bit 25 - 1: dump out &amp; power up controlled by SW 0: by FSM"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_power_up_force(&mut self) -> TSENS_POWER_UP_FORCE_W<25> {
        TSENS_POWER_UP_FORCE_W::new(self)
    }
    #[doc = "Bit 26 - temperature sensor dump out only active when reg_tsens_power_up_force = 1"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_dump_out(&mut self) -> TSENS_DUMP_OUT_W<26> {
        TSENS_DUMP_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_tsens_ctrl](index.html) module"]
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
#[doc = "`reset()` method sets SAR_TSENS_CTRL to value 0x0006_6002"]
impl crate::Resettable for SAR_TSENS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0006_6002;
}
