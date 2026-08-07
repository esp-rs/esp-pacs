#[doc = "Register `PIN_CTRL` reader"]
pub type R = crate::R<PIN_CTRL_SPEC>;
#[doc = "Register `PIN_CTRL` writer"]
pub type W = crate::W<PIN_CTRL_SPEC>;
#[doc = "Field `GPIO_EXT_CLK_OUT1` reader - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals."]
pub type GPIO_EXT_CLK_OUT1_R = crate::FieldReader;
#[doc = "Field `GPIO_EXT_CLK_OUT1` writer - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals."]
pub type GPIO_EXT_CLK_OUT1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `GPIO_EXT_CLK_OUT2` reader - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals."]
pub type GPIO_EXT_CLK_OUT2_R = crate::FieldReader;
#[doc = "Field `GPIO_EXT_CLK_OUT2` writer - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals."]
pub type GPIO_EXT_CLK_OUT2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `GPIO_EXT_CLK_OUT3` reader - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals."]
pub type GPIO_EXT_CLK_OUT3_R = crate::FieldReader;
#[doc = "Field `GPIO_EXT_CLK_OUT3` writer - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals."]
pub type GPIO_EXT_CLK_OUT3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals."]
    #[inline(always)]
    pub fn gpio_ext_clk_out1(&self) -> GPIO_EXT_CLK_OUT1_R {
        GPIO_EXT_CLK_OUT1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals."]
    #[inline(always)]
    pub fn gpio_ext_clk_out2(&self) -> GPIO_EXT_CLK_OUT2_R {
        GPIO_EXT_CLK_OUT2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals."]
    #[inline(always)]
    pub fn gpio_ext_clk_out3(&self) -> GPIO_EXT_CLK_OUT3_R {
        GPIO_EXT_CLK_OUT3_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN_CTRL")
            .field("gpio_ext_clk_out1", &self.gpio_ext_clk_out1())
            .field("gpio_ext_clk_out2", &self.gpio_ext_clk_out2())
            .field("gpio_ext_clk_out3", &self.gpio_ext_clk_out3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - If you want to output clock for I2S to CLK_OUT_out1, set this register to 0x0. CLK_OUT_out1 can be found in peripheral output signals."]
    #[inline(always)]
    pub fn gpio_ext_clk_out1(&mut self) -> GPIO_EXT_CLK_OUT1_W<'_, PIN_CTRL_SPEC> {
        GPIO_EXT_CLK_OUT1_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - If you want to output clock for I2S to CLK_OUT_out2, set this register to 0x0. CLK_OUT_out2 can be found in peripheral output signals."]
    #[inline(always)]
    pub fn gpio_ext_clk_out2(&mut self) -> GPIO_EXT_CLK_OUT2_W<'_, PIN_CTRL_SPEC> {
        GPIO_EXT_CLK_OUT2_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - If you want to output clock for I2S to CLK_OUT_out3, set this register to 0x0. CLK_OUT_out3 can be found in peripheral output signals."]
    #[inline(always)]
    pub fn gpio_ext_clk_out3(&mut self) -> GPIO_EXT_CLK_OUT3_W<'_, PIN_CTRL_SPEC> {
        GPIO_EXT_CLK_OUT3_W::new(self, 10)
    }
}
#[doc = "Clock Output Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pin_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIN_CTRL_SPEC;
impl crate::RegisterSpec for PIN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin_ctrl::R`](R) reader structure"]
impl crate::Readable for PIN_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pin_ctrl::W`](W) writer structure"]
impl crate::Writable for PIN_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN_CTRL to value 0"]
impl crate::Resettable for PIN_CTRL_SPEC {}
