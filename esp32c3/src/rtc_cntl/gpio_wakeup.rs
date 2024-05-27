///Register `GPIO_WAKEUP` reader
pub type R = crate::R<GPIO_WAKEUP_SPEC>;
///Register `GPIO_WAKEUP` writer
pub type W = crate::W<GPIO_WAKEUP_SPEC>;
///Field `GPIO_WAKEUP_STATUS` reader - rtc gpio wakeup flag
pub type GPIO_WAKEUP_STATUS_R = crate::FieldReader;
///Field `GPIO_WAKEUP_STATUS_CLR` reader - clear rtc gpio wakeup flag
pub type GPIO_WAKEUP_STATUS_CLR_R = crate::BitReader;
///Field `GPIO_WAKEUP_STATUS_CLR` writer - clear rtc gpio wakeup flag
pub type GPIO_WAKEUP_STATUS_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN_CLK_GATE` reader - enable rtc io clk gate
pub type GPIO_PIN_CLK_GATE_R = crate::BitReader;
///Field `GPIO_PIN_CLK_GATE` writer - enable rtc io clk gate
pub type GPIO_PIN_CLK_GATE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN5_INT_TYPE` reader - configure gpio wakeup type
pub type GPIO_PIN5_INT_TYPE_R = crate::FieldReader;
///Field `GPIO_PIN5_INT_TYPE` writer - configure gpio wakeup type
pub type GPIO_PIN5_INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GPIO_PIN4_INT_TYPE` reader - configure gpio wakeup type
pub type GPIO_PIN4_INT_TYPE_R = crate::FieldReader;
///Field `GPIO_PIN4_INT_TYPE` writer - configure gpio wakeup type
pub type GPIO_PIN4_INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GPIO_PIN3_INT_TYPE` reader - configure gpio wakeup type
pub type GPIO_PIN3_INT_TYPE_R = crate::FieldReader;
///Field `GPIO_PIN3_INT_TYPE` writer - configure gpio wakeup type
pub type GPIO_PIN3_INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GPIO_PIN2_INT_TYPE` reader - configure gpio wakeup type
pub type GPIO_PIN2_INT_TYPE_R = crate::FieldReader;
///Field `GPIO_PIN2_INT_TYPE` writer - configure gpio wakeup type
pub type GPIO_PIN2_INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GPIO_PIN1_INT_TYPE` reader - configure gpio wakeup type
pub type GPIO_PIN1_INT_TYPE_R = crate::FieldReader;
///Field `GPIO_PIN1_INT_TYPE` writer - configure gpio wakeup type
pub type GPIO_PIN1_INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GPIO_PIN0_INT_TYPE` reader - configure gpio wakeup type
pub type GPIO_PIN0_INT_TYPE_R = crate::FieldReader;
///Field `GPIO_PIN0_INT_TYPE` writer - configure gpio wakeup type
pub type GPIO_PIN0_INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `GPIO_PIN5_WAKEUP_ENABLE` reader - enable wakeup from rtc gpio5
pub type GPIO_PIN5_WAKEUP_ENABLE_R = crate::BitReader;
///Field `GPIO_PIN5_WAKEUP_ENABLE` writer - enable wakeup from rtc gpio5
pub type GPIO_PIN5_WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN4_WAKEUP_ENABLE` reader - enable wakeup from rtc gpio4
pub type GPIO_PIN4_WAKEUP_ENABLE_R = crate::BitReader;
///Field `GPIO_PIN4_WAKEUP_ENABLE` writer - enable wakeup from rtc gpio4
pub type GPIO_PIN4_WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN3_WAKEUP_ENABLE` reader - enable wakeup from rtc gpio3
pub type GPIO_PIN3_WAKEUP_ENABLE_R = crate::BitReader;
///Field `GPIO_PIN3_WAKEUP_ENABLE` writer - enable wakeup from rtc gpio3
pub type GPIO_PIN3_WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN2_WAKEUP_ENABLE` reader - enable wakeup from rtc gpio2
pub type GPIO_PIN2_WAKEUP_ENABLE_R = crate::BitReader;
///Field `GPIO_PIN2_WAKEUP_ENABLE` writer - enable wakeup from rtc gpio2
pub type GPIO_PIN2_WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN1_WAKEUP_ENABLE` reader - enable wakeup from rtc gpio1
pub type GPIO_PIN1_WAKEUP_ENABLE_R = crate::BitReader;
///Field `GPIO_PIN1_WAKEUP_ENABLE` writer - enable wakeup from rtc gpio1
pub type GPIO_PIN1_WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN0_WAKEUP_ENABLE` reader - enable wakeup from rtc gpio0
pub type GPIO_PIN0_WAKEUP_ENABLE_R = crate::BitReader;
///Field `GPIO_PIN0_WAKEUP_ENABLE` writer - enable wakeup from rtc gpio0
pub type GPIO_PIN0_WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - rtc gpio wakeup flag
    #[inline(always)]
    pub fn gpio_wakeup_status(&self) -> GPIO_WAKEUP_STATUS_R {
        GPIO_WAKEUP_STATUS_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 6 - clear rtc gpio wakeup flag
    #[inline(always)]
    pub fn gpio_wakeup_status_clr(&self) -> GPIO_WAKEUP_STATUS_CLR_R {
        GPIO_WAKEUP_STATUS_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - enable rtc io clk gate
    #[inline(always)]
    pub fn gpio_pin_clk_gate(&self) -> GPIO_PIN_CLK_GATE_R {
        GPIO_PIN_CLK_GATE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - configure gpio wakeup type
    #[inline(always)]
    pub fn gpio_pin5_int_type(&self) -> GPIO_PIN5_INT_TYPE_R {
        GPIO_PIN5_INT_TYPE_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - configure gpio wakeup type
    #[inline(always)]
    pub fn gpio_pin4_int_type(&self) -> GPIO_PIN4_INT_TYPE_R {
        GPIO_PIN4_INT_TYPE_R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bits 14:16 - configure gpio wakeup type
    #[inline(always)]
    pub fn gpio_pin3_int_type(&self) -> GPIO_PIN3_INT_TYPE_R {
        GPIO_PIN3_INT_TYPE_R::new(((self.bits >> 14) & 7) as u8)
    }
    ///Bits 17:19 - configure gpio wakeup type
    #[inline(always)]
    pub fn gpio_pin2_int_type(&self) -> GPIO_PIN2_INT_TYPE_R {
        GPIO_PIN2_INT_TYPE_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 20:22 - configure gpio wakeup type
    #[inline(always)]
    pub fn gpio_pin1_int_type(&self) -> GPIO_PIN1_INT_TYPE_R {
        GPIO_PIN1_INT_TYPE_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 23:25 - configure gpio wakeup type
    #[inline(always)]
    pub fn gpio_pin0_int_type(&self) -> GPIO_PIN0_INT_TYPE_R {
        GPIO_PIN0_INT_TYPE_R::new(((self.bits >> 23) & 7) as u8)
    }
    ///Bit 26 - enable wakeup from rtc gpio5
    #[inline(always)]
    pub fn gpio_pin5_wakeup_enable(&self) -> GPIO_PIN5_WAKEUP_ENABLE_R {
        GPIO_PIN5_WAKEUP_ENABLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - enable wakeup from rtc gpio4
    #[inline(always)]
    pub fn gpio_pin4_wakeup_enable(&self) -> GPIO_PIN4_WAKEUP_ENABLE_R {
        GPIO_PIN4_WAKEUP_ENABLE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - enable wakeup from rtc gpio3
    #[inline(always)]
    pub fn gpio_pin3_wakeup_enable(&self) -> GPIO_PIN3_WAKEUP_ENABLE_R {
        GPIO_PIN3_WAKEUP_ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - enable wakeup from rtc gpio2
    #[inline(always)]
    pub fn gpio_pin2_wakeup_enable(&self) -> GPIO_PIN2_WAKEUP_ENABLE_R {
        GPIO_PIN2_WAKEUP_ENABLE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - enable wakeup from rtc gpio1
    #[inline(always)]
    pub fn gpio_pin1_wakeup_enable(&self) -> GPIO_PIN1_WAKEUP_ENABLE_R {
        GPIO_PIN1_WAKEUP_ENABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - enable wakeup from rtc gpio0
    #[inline(always)]
    pub fn gpio_pin0_wakeup_enable(&self) -> GPIO_PIN0_WAKEUP_ENABLE_R {
        GPIO_PIN0_WAKEUP_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_WAKEUP")
            .field("gpio_wakeup_status", &self.gpio_wakeup_status())
            .field("gpio_wakeup_status_clr", &self.gpio_wakeup_status_clr())
            .field("gpio_pin_clk_gate", &self.gpio_pin_clk_gate())
            .field("gpio_pin5_int_type", &self.gpio_pin5_int_type())
            .field("gpio_pin4_int_type", &self.gpio_pin4_int_type())
            .field("gpio_pin3_int_type", &self.gpio_pin3_int_type())
            .field("gpio_pin2_int_type", &self.gpio_pin2_int_type())
            .field("gpio_pin1_int_type", &self.gpio_pin1_int_type())
            .field("gpio_pin0_int_type", &self.gpio_pin0_int_type())
            .field("gpio_pin5_wakeup_enable", &self.gpio_pin5_wakeup_enable())
            .field("gpio_pin4_wakeup_enable", &self.gpio_pin4_wakeup_enable())
            .field("gpio_pin3_wakeup_enable", &self.gpio_pin3_wakeup_enable())
            .field("gpio_pin2_wakeup_enable", &self.gpio_pin2_wakeup_enable())
            .field("gpio_pin1_wakeup_enable", &self.gpio_pin1_wakeup_enable())
            .field("gpio_pin0_wakeup_enable", &self.gpio_pin0_wakeup_enable())
            .finish()
    }
}
impl W {
    ///Bit 6 - clear rtc gpio wakeup flag
    #[inline(always)]
    #[must_use]
    pub fn gpio_wakeup_status_clr(&mut self) -> GPIO_WAKEUP_STATUS_CLR_W<GPIO_WAKEUP_SPEC> {
        GPIO_WAKEUP_STATUS_CLR_W::new(self, 6)
    }
    ///Bit 7 - enable rtc io clk gate
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin_clk_gate(&mut self) -> GPIO_PIN_CLK_GATE_W<GPIO_WAKEUP_SPEC> {
        GPIO_PIN_CLK_GATE_W::new(self, 7)
    }
    ///Bits 8:10 - configure gpio wakeup type
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin5_int_type(&mut self) -> GPIO_PIN5_INT_TYPE_W<GPIO_WAKEUP_SPEC> {
        GPIO_PIN5_INT_TYPE_W::new(self, 8)
    }
    ///Bits 11:13 - configure gpio wakeup type
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin4_int_type(&mut self) -> GPIO_PIN4_INT_TYPE_W<GPIO_WAKEUP_SPEC> {
        GPIO_PIN4_INT_TYPE_W::new(self, 11)
    }
    ///Bits 14:16 - configure gpio wakeup type
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin3_int_type(&mut self) -> GPIO_PIN3_INT_TYPE_W<GPIO_WAKEUP_SPEC> {
        GPIO_PIN3_INT_TYPE_W::new(self, 14)
    }
    ///Bits 17:19 - configure gpio wakeup type
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin2_int_type(&mut self) -> GPIO_PIN2_INT_TYPE_W<GPIO_WAKEUP_SPEC> {
        GPIO_PIN2_INT_TYPE_W::new(self, 17)
    }
    ///Bits 20:22 - configure gpio wakeup type
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin1_int_type(&mut self) -> GPIO_PIN1_INT_TYPE_W<GPIO_WAKEUP_SPEC> {
        GPIO_PIN1_INT_TYPE_W::new(self, 20)
    }
    ///Bits 23:25 - configure gpio wakeup type
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin0_int_type(&mut self) -> GPIO_PIN0_INT_TYPE_W<GPIO_WAKEUP_SPEC> {
        GPIO_PIN0_INT_TYPE_W::new(self, 23)
    }
    ///Bit 26 - enable wakeup from rtc gpio5
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin5_wakeup_enable(&mut self) -> GPIO_PIN5_WAKEUP_ENABLE_W<GPIO_WAKEUP_SPEC> {
        GPIO_PIN5_WAKEUP_ENABLE_W::new(self, 26)
    }
    ///Bit 27 - enable wakeup from rtc gpio4
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin4_wakeup_enable(&mut self) -> GPIO_PIN4_WAKEUP_ENABLE_W<GPIO_WAKEUP_SPEC> {
        GPIO_PIN4_WAKEUP_ENABLE_W::new(self, 27)
    }
    ///Bit 28 - enable wakeup from rtc gpio3
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin3_wakeup_enable(&mut self) -> GPIO_PIN3_WAKEUP_ENABLE_W<GPIO_WAKEUP_SPEC> {
        GPIO_PIN3_WAKEUP_ENABLE_W::new(self, 28)
    }
    ///Bit 29 - enable wakeup from rtc gpio2
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin2_wakeup_enable(&mut self) -> GPIO_PIN2_WAKEUP_ENABLE_W<GPIO_WAKEUP_SPEC> {
        GPIO_PIN2_WAKEUP_ENABLE_W::new(self, 29)
    }
    ///Bit 30 - enable wakeup from rtc gpio1
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin1_wakeup_enable(&mut self) -> GPIO_PIN1_WAKEUP_ENABLE_W<GPIO_WAKEUP_SPEC> {
        GPIO_PIN1_WAKEUP_ENABLE_W::new(self, 30)
    }
    ///Bit 31 - enable wakeup from rtc gpio0
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin0_wakeup_enable(&mut self) -> GPIO_PIN0_WAKEUP_ENABLE_W<GPIO_WAKEUP_SPEC> {
        GPIO_PIN0_WAKEUP_ENABLE_W::new(self, 31)
    }
}
/**rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_wakeup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_wakeup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GPIO_WAKEUP_SPEC;
impl crate::RegisterSpec for GPIO_WAKEUP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gpio_wakeup::R`](R) reader structure
impl crate::Readable for GPIO_WAKEUP_SPEC {}
///`write(|w| ..)` method takes [`gpio_wakeup::W`](W) writer structure
impl crate::Writable for GPIO_WAKEUP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPIO_WAKEUP to value 0
impl crate::Resettable for GPIO_WAKEUP_SPEC {
    const RESET_VALUE: u32 = 0;
}
