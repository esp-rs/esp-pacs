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
pub type CLK_OUT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLK_OUT1` writer - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals."]
pub type CLK_OUT1_W<'a> = crate::FieldWriter<'a, u32, PIN_CTRL_SPEC, u8, u8, 4, 0>;
#[doc = "Field `CLK_OUT2` reader - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals."]
pub type CLK_OUT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLK_OUT2` writer - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals."]
pub type CLK_OUT2_W<'a> = crate::FieldWriter<'a, u32, PIN_CTRL_SPEC, u8, u8, 4, 4>;
#[doc = "Field `CLK_OUT3` reader - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals."]
pub type CLK_OUT3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLK_OUT3` writer - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals."]
pub type CLK_OUT3_W<'a> = crate::FieldWriter<'a, u32, PIN_CTRL_SPEC, u8, u8, 4, 8>;
impl R {
    #[doc = "Bits 0:3 - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals."]
    #[inline(always)]
    pub fn clk_out1(&self) -> CLK_OUT1_R {
        CLK_OUT1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals."]
    #[inline(always)]
    pub fn clk_out2(&self) -> CLK_OUT2_R {
        CLK_OUT2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals."]
    #[inline(always)]
    pub fn clk_out3(&self) -> CLK_OUT3_R {
        CLK_OUT3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals."]
    #[inline(always)]
    pub fn clk_out1(&mut self) -> CLK_OUT1_W {
        CLK_OUT1_W::new(self)
    }
    #[doc = "Bits 4:7 - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals."]
    #[inline(always)]
    pub fn clk_out2(&mut self) -> CLK_OUT2_W {
        CLK_OUT2_W::new(self)
    }
    #[doc = "Bits 8:11 - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals."]
    #[inline(always)]
    pub fn clk_out3(&mut self) -> CLK_OUT3_W {
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
}
#[doc = "`reset()` method sets PIN_CTRL to value 0x07ff"]
impl crate::Resettable for PIN_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07ff
    }
}
