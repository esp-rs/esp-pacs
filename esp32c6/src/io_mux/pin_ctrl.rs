#[doc = "Register `PIN_CTRL` reader"]
pub struct R(crate::R<PIN_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN_CTRL` writer"]
pub struct W(crate::W<PIN_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_CTRL_SPEC>;
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
impl From<crate::W<PIN_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_OUT1` reader - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals."]
pub type CLK_OUT1_R = crate::FieldReader;
#[doc = "Field `CLK_OUT1` writer - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals."]
pub type CLK_OUT1_W<'a, const O: u8> = crate::FieldWriter<'a, PIN_CTRL_SPEC, 5, O>;
#[doc = "Field `CLK_OUT2` reader - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals."]
pub type CLK_OUT2_R = crate::FieldReader;
#[doc = "Field `CLK_OUT2` writer - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals."]
pub type CLK_OUT2_W<'a, const O: u8> = crate::FieldWriter<'a, PIN_CTRL_SPEC, 5, O>;
#[doc = "Field `CLK_OUT3` reader - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals."]
pub type CLK_OUT3_R = crate::FieldReader;
#[doc = "Field `CLK_OUT3` writer - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals."]
pub type CLK_OUT3_W<'a, const O: u8> = crate::FieldWriter<'a, PIN_CTRL_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals."]
    #[inline(always)]
    pub fn clk_out1(&self) -> CLK_OUT1_R {
        CLK_OUT1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals."]
    #[inline(always)]
    pub fn clk_out2(&self) -> CLK_OUT2_R {
        CLK_OUT2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals."]
    #[inline(always)]
    pub fn clk_out3(&self) -> CLK_OUT3_R {
        CLK_OUT3_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN_CTRL")
            .field("clk_out1", &format_args!("{}", self.clk_out1().bits()))
            .field("clk_out2", &format_args!("{}", self.clk_out2().bits()))
            .field("clk_out3", &format_args!("{}", self.clk_out3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PIN_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals."]
    #[inline(always)]
    #[must_use]
    pub fn clk_out1(&mut self) -> CLK_OUT1_W<0> {
        CLK_OUT1_W::new(self)
    }
    #[doc = "Bits 5:9 - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals."]
    #[inline(always)]
    #[must_use]
    pub fn clk_out2(&mut self) -> CLK_OUT2_W<5> {
        CLK_OUT2_W::new(self)
    }
    #[doc = "Bits 10:14 - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals."]
    #[inline(always)]
    #[must_use]
    pub fn clk_out3(&mut self) -> CLK_OUT3_W<10> {
        CLK_OUT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Output Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin_ctrl](index.html) module"]
pub struct PIN_CTRL_SPEC;
impl crate::RegisterSpec for PIN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin_ctrl::R](R) reader structure"]
impl crate::Readable for PIN_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin_ctrl::W](W) writer structure"]
impl crate::Writable for PIN_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIN_CTRL to value 0x1def"]
impl crate::Resettable for PIN_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1def;
}
