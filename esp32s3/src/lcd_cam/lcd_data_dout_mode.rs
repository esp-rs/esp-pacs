#[doc = "Register `LCD_DATA_DOUT_MODE` reader"]
pub struct R(crate::R<LCD_DATA_DOUT_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_DATA_DOUT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_DATA_DOUT_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_DATA_DOUT_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_DATA_DOUT_MODE` writer"]
pub struct W(crate::W<LCD_DATA_DOUT_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_DATA_DOUT_MODE_SPEC>;
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
impl From<crate::W<LCD_DATA_DOUT_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_DATA_DOUT_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT0_MODE` reader - The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT0_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT0_MODE` writer - The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT0_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_DATA_DOUT_MODE_SPEC, 2, O>;
#[doc = "Field `DOUT1_MODE` reader - The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT1_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT1_MODE` writer - The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT1_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_DATA_DOUT_MODE_SPEC, 2, O>;
#[doc = "Field `DOUT2_MODE` reader - The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT2_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT2_MODE` writer - The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT2_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_DATA_DOUT_MODE_SPEC, 2, O>;
#[doc = "Field `DOUT3_MODE` reader - The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT3_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT3_MODE` writer - The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT3_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_DATA_DOUT_MODE_SPEC, 2, O>;
#[doc = "Field `DOUT4_MODE` reader - The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT4_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT4_MODE` writer - The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT4_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_DATA_DOUT_MODE_SPEC, 2, O>;
#[doc = "Field `DOUT5_MODE` reader - The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT5_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT5_MODE` writer - The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT5_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_DATA_DOUT_MODE_SPEC, 2, O>;
#[doc = "Field `DOUT6_MODE` reader - The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT6_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT6_MODE` writer - The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT6_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_DATA_DOUT_MODE_SPEC, 2, O>;
#[doc = "Field `DOUT7_MODE` reader - The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT7_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT7_MODE` writer - The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT7_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_DATA_DOUT_MODE_SPEC, 2, O>;
#[doc = "Field `DOUT8_MODE` reader - The output data bit 16 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT8_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT8_MODE` writer - The output data bit 16 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT8_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_DATA_DOUT_MODE_SPEC, 2, O>;
#[doc = "Field `DOUT9_MODE` reader - The output data bit 18 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT9_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT9_MODE` writer - The output data bit 18 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT9_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_DATA_DOUT_MODE_SPEC, 2, O>;
#[doc = "Field `DOUT10_MODE` reader - The output data bit 20 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT10_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT10_MODE` writer - The output data bit 20 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT10_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_DATA_DOUT_MODE_SPEC, 2, O>;
#[doc = "Field `DOUT11_MODE` reader - The output data bit 22 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT11_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT11_MODE` writer - The output data bit 22 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT11_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_DATA_DOUT_MODE_SPEC, 2, O>;
#[doc = "Field `DOUT12_MODE` reader - The output data bit 24 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT12_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT12_MODE` writer - The output data bit 24 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT12_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_DATA_DOUT_MODE_SPEC, 2, O>;
#[doc = "Field `DOUT13_MODE` reader - The output data bit 26 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT13_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT13_MODE` writer - The output data bit 26 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT13_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_DATA_DOUT_MODE_SPEC, 2, O>;
#[doc = "Field `DOUT14_MODE` reader - The output data bit 28 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT14_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT14_MODE` writer - The output data bit 28 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT14_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_DATA_DOUT_MODE_SPEC, 2, O>;
#[doc = "Field `DOUT15_MODE` reader - The output data bit 30 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT15_MODE_R = crate::FieldReader;
#[doc = "Field `DOUT15_MODE` writer - The output data bit 30 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
pub type DOUT15_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_DATA_DOUT_MODE_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout0_mode(&self) -> DOUT0_MODE_R {
        DOUT0_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout1_mode(&self) -> DOUT1_MODE_R {
        DOUT1_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout2_mode(&self) -> DOUT2_MODE_R {
        DOUT2_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout3_mode(&self) -> DOUT3_MODE_R {
        DOUT3_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout4_mode(&self) -> DOUT4_MODE_R {
        DOUT4_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout5_mode(&self) -> DOUT5_MODE_R {
        DOUT5_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout6_mode(&self) -> DOUT6_MODE_R {
        DOUT6_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout7_mode(&self) -> DOUT7_MODE_R {
        DOUT7_MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - The output data bit 16 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout8_mode(&self) -> DOUT8_MODE_R {
        DOUT8_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - The output data bit 18 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout9_mode(&self) -> DOUT9_MODE_R {
        DOUT9_MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - The output data bit 20 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout10_mode(&self) -> DOUT10_MODE_R {
        DOUT10_MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - The output data bit 22 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout11_mode(&self) -> DOUT11_MODE_R {
        DOUT11_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - The output data bit 24 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout12_mode(&self) -> DOUT12_MODE_R {
        DOUT12_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - The output data bit 26 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout13_mode(&self) -> DOUT13_MODE_R {
        DOUT13_MODE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - The output data bit 28 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout14_mode(&self) -> DOUT14_MODE_R {
        DOUT14_MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - The output data bit 30 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    pub fn dout15_mode(&self) -> DOUT15_MODE_R {
        DOUT15_MODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_DATA_DOUT_MODE")
            .field("dout0_mode", &format_args!("{}", self.dout0_mode().bits()))
            .field("dout1_mode", &format_args!("{}", self.dout1_mode().bits()))
            .field("dout2_mode", &format_args!("{}", self.dout2_mode().bits()))
            .field("dout3_mode", &format_args!("{}", self.dout3_mode().bits()))
            .field("dout4_mode", &format_args!("{}", self.dout4_mode().bits()))
            .field("dout5_mode", &format_args!("{}", self.dout5_mode().bits()))
            .field("dout6_mode", &format_args!("{}", self.dout6_mode().bits()))
            .field("dout7_mode", &format_args!("{}", self.dout7_mode().bits()))
            .field("dout8_mode", &format_args!("{}", self.dout8_mode().bits()))
            .field("dout9_mode", &format_args!("{}", self.dout9_mode().bits()))
            .field(
                "dout10_mode",
                &format_args!("{}", self.dout10_mode().bits()),
            )
            .field(
                "dout11_mode",
                &format_args!("{}", self.dout11_mode().bits()),
            )
            .field(
                "dout12_mode",
                &format_args!("{}", self.dout12_mode().bits()),
            )
            .field(
                "dout13_mode",
                &format_args!("{}", self.dout13_mode().bits()),
            )
            .field(
                "dout14_mode",
                &format_args!("{}", self.dout14_mode().bits()),
            )
            .field(
                "dout15_mode",
                &format_args!("{}", self.dout15_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LCD_DATA_DOUT_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout0_mode(&mut self) -> DOUT0_MODE_W<0> {
        DOUT0_MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout1_mode(&mut self) -> DOUT1_MODE_W<2> {
        DOUT1_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout2_mode(&mut self) -> DOUT2_MODE_W<4> {
        DOUT2_MODE_W::new(self)
    }
    #[doc = "Bits 6:7 - The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout3_mode(&mut self) -> DOUT3_MODE_W<6> {
        DOUT3_MODE_W::new(self)
    }
    #[doc = "Bits 8:9 - The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout4_mode(&mut self) -> DOUT4_MODE_W<8> {
        DOUT4_MODE_W::new(self)
    }
    #[doc = "Bits 10:11 - The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout5_mode(&mut self) -> DOUT5_MODE_W<10> {
        DOUT5_MODE_W::new(self)
    }
    #[doc = "Bits 12:13 - The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout6_mode(&mut self) -> DOUT6_MODE_W<12> {
        DOUT6_MODE_W::new(self)
    }
    #[doc = "Bits 14:15 - The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout7_mode(&mut self) -> DOUT7_MODE_W<14> {
        DOUT7_MODE_W::new(self)
    }
    #[doc = "Bits 16:17 - The output data bit 16 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout8_mode(&mut self) -> DOUT8_MODE_W<16> {
        DOUT8_MODE_W::new(self)
    }
    #[doc = "Bits 18:19 - The output data bit 18 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout9_mode(&mut self) -> DOUT9_MODE_W<18> {
        DOUT9_MODE_W::new(self)
    }
    #[doc = "Bits 20:21 - The output data bit 20 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout10_mode(&mut self) -> DOUT10_MODE_W<20> {
        DOUT10_MODE_W::new(self)
    }
    #[doc = "Bits 22:23 - The output data bit 22 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout11_mode(&mut self) -> DOUT11_MODE_W<22> {
        DOUT11_MODE_W::new(self)
    }
    #[doc = "Bits 24:25 - The output data bit 24 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout12_mode(&mut self) -> DOUT12_MODE_W<24> {
        DOUT12_MODE_W::new(self)
    }
    #[doc = "Bits 26:27 - The output data bit 26 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout13_mode(&mut self) -> DOUT13_MODE_W<26> {
        DOUT13_MODE_W::new(self)
    }
    #[doc = "Bits 28:29 - The output data bit 28 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout14_mode(&mut self) -> DOUT14_MODE_W<28> {
        DOUT14_MODE_W::new(self)
    }
    #[doc = "Bits 30:31 - The output data bit 30 is delayed by module clock LCD_CLK. 0: output without delayed. 1: delay by the positive edge of LCD_CLK. 2: delay by the negative edge of LCD_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn dout15_mode(&mut self) -> DOUT15_MODE_W<30> {
        DOUT15_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_data_dout_mode](index.html) module"]
pub struct LCD_DATA_DOUT_MODE_SPEC;
impl crate::RegisterSpec for LCD_DATA_DOUT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_data_dout_mode::R](R) reader structure"]
impl crate::Readable for LCD_DATA_DOUT_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_data_dout_mode::W](W) writer structure"]
impl crate::Writable for LCD_DATA_DOUT_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCD_DATA_DOUT_MODE to value 0"]
impl crate::Resettable for LCD_DATA_DOUT_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
