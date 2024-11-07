#[doc = "Register `PIN_CTRL` reader"]
pub type R = crate::R<PIN_CTRL_SPEC>;
#[doc = "Register `PIN_CTRL` writer"]
pub type W = crate::W<PIN_CTRL_SPEC>;
#[doc = "Field `PIN_CLK_OUT1` reader - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT1. 15: disabled."]
pub type PIN_CLK_OUT1_R = crate::FieldReader;
#[doc = "Field `PIN_CLK_OUT1` writer - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT1. 15: disabled."]
pub type PIN_CLK_OUT1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIN_CLK_OUT2` reader - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT2. 15: disabled."]
pub type PIN_CLK_OUT2_R = crate::FieldReader;
#[doc = "Field `PIN_CLK_OUT2` writer - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT2. 15: disabled."]
pub type PIN_CLK_OUT2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIN_CLK_OUT3` reader - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT3. 15: disabled."]
pub type PIN_CLK_OUT3_R = crate::FieldReader;
#[doc = "Field `PIN_CLK_OUT3` writer - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT3. 15: disabled."]
pub type PIN_CLK_OUT3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SWITCH_PRT_NUM` reader - IO pin power switch delay, delay unit is one APB clock."]
pub type SWITCH_PRT_NUM_R = crate::FieldReader;
#[doc = "Field `SWITCH_PRT_NUM` writer - IO pin power switch delay, delay unit is one APB clock."]
pub type SWITCH_PRT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PAD_POWER_CTRL` reader - Select power voltage for GPIO33 ~ GPIO37. 1: select VDD_SPI 1.8 V. 0: select VDD3P3_CPU 3.3 V."]
pub type PAD_POWER_CTRL_R = crate::BitReader;
#[doc = "Field `PAD_POWER_CTRL` writer - Select power voltage for GPIO33 ~ GPIO37. 1: select VDD_SPI 1.8 V. 0: select VDD3P3_CPU 3.3 V."]
pub type PAD_POWER_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT1. 15: disabled."]
    #[inline(always)]
    pub fn pin_clk_out1(&self) -> PIN_CLK_OUT1_R {
        PIN_CLK_OUT1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT2. 15: disabled."]
    #[inline(always)]
    pub fn pin_clk_out2(&self) -> PIN_CLK_OUT2_R {
        PIN_CLK_OUT2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT3. 15: disabled."]
    #[inline(always)]
    pub fn pin_clk_out3(&self) -> PIN_CLK_OUT3_R {
        PIN_CLK_OUT3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - IO pin power switch delay, delay unit is one APB clock."]
    #[inline(always)]
    pub fn switch_prt_num(&self) -> SWITCH_PRT_NUM_R {
        SWITCH_PRT_NUM_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Select power voltage for GPIO33 ~ GPIO37. 1: select VDD_SPI 1.8 V. 0: select VDD3P3_CPU 3.3 V."]
    #[inline(always)]
    pub fn pad_power_ctrl(&self) -> PAD_POWER_CTRL_R {
        PAD_POWER_CTRL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN_CTRL")
            .field("pin_clk_out1", &self.pin_clk_out1())
            .field("pin_clk_out2", &self.pin_clk_out2())
            .field("pin_clk_out3", &self.pin_clk_out3())
            .field("switch_prt_num", &self.switch_prt_num())
            .field("pad_power_ctrl", &self.pad_power_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT1. 15: disabled."]
    #[inline(always)]
    pub fn pin_clk_out1(&mut self) -> PIN_CLK_OUT1_W<PIN_CTRL_SPEC> {
        PIN_CLK_OUT1_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT2. 15: disabled."]
    #[inline(always)]
    pub fn pin_clk_out2(&mut self) -> PIN_CLK_OUT2_W<PIN_CTRL_SPEC> {
        PIN_CLK_OUT2_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Configure I2S0 clock output. 0: output I2S0 clock to CLK_OUT3. 15: disabled."]
    #[inline(always)]
    pub fn pin_clk_out3(&mut self) -> PIN_CLK_OUT3_W<PIN_CTRL_SPEC> {
        PIN_CLK_OUT3_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - IO pin power switch delay, delay unit is one APB clock."]
    #[inline(always)]
    pub fn switch_prt_num(&mut self) -> SWITCH_PRT_NUM_W<PIN_CTRL_SPEC> {
        SWITCH_PRT_NUM_W::new(self, 12)
    }
    #[doc = "Bit 15 - Select power voltage for GPIO33 ~ GPIO37. 1: select VDD_SPI 1.8 V. 0: select VDD3P3_CPU 3.3 V."]
    #[inline(always)]
    pub fn pad_power_ctrl(&mut self) -> PAD_POWER_CTRL_W<PIN_CTRL_SPEC> {
        PAD_POWER_CTRL_W::new(self, 15)
    }
}
#[doc = "Clock output configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pin_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets PIN_CTRL to value 0x27ff"]
impl crate::Resettable for PIN_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x27ff;
}
