#[doc = "Register `PIN_CTRL` reader"]
pub type R = crate::R<PIN_CTRL_SPEC>;
#[doc = "Register `PIN_CTRL` writer"]
pub type W = crate::W<PIN_CTRL_SPEC>;
#[doc = "Field `CLK_OUT1` reader - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals."]
pub type CLK_OUT1_R = crate::FieldReader;
#[doc = "Field `CLK_OUT1` writer - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals."]
pub type CLK_OUT1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLK_OUT2` reader - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals."]
pub type CLK_OUT2_R = crate::FieldReader;
#[doc = "Field `CLK_OUT2` writer - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals."]
pub type CLK_OUT2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLK_OUT3` reader - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals."]
pub type CLK_OUT3_R = crate::FieldReader;
#[doc = "Field `CLK_OUT3` writer - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals."]
pub type CLK_OUT3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals."]
    #[inline(always)]
    #[must_use]
    pub fn clk_out1(&mut self) -> CLK_OUT1_W<PIN_CTRL_SPEC> {
        CLK_OUT1_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals."]
    #[inline(always)]
    #[must_use]
    pub fn clk_out2(&mut self) -> CLK_OUT2_W<PIN_CTRL_SPEC> {
        CLK_OUT2_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals."]
    #[inline(always)]
    #[must_use]
    pub fn clk_out3(&mut self) -> CLK_OUT3_W<PIN_CTRL_SPEC> {
        CLK_OUT3_W::new(self, 8)
    }
}
#[doc = "Clock Output Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIN_CTRL_SPEC;
impl crate::RegisterSpec for PIN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin_ctrl::R`](R) reader structure"]
impl crate::Readable for PIN_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pin_ctrl::W`](W) writer structure"]
impl crate::Writable for PIN_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIN_CTRL to value 0x07ff"]
impl crate::Resettable for PIN_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x07ff;
}
