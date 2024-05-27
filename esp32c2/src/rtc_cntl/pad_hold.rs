///Register `PAD_HOLD` reader
pub type R = crate::R<PAD_HOLD_SPEC>;
///Register `PAD_HOLD` writer
pub type W = crate::W<PAD_HOLD_SPEC>;
///Field `GPIO_PIN0_HOLD` reader - Need add desc
pub type GPIO_PIN0_HOLD_R = crate::BitReader;
///Field `GPIO_PIN0_HOLD` writer - Need add desc
pub type GPIO_PIN0_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN1_HOLD` reader - Need add desc
pub type GPIO_PIN1_HOLD_R = crate::BitReader;
///Field `GPIO_PIN1_HOLD` writer - Need add desc
pub type GPIO_PIN1_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN2_HOLD` reader - Need add desc
pub type GPIO_PIN2_HOLD_R = crate::BitReader;
///Field `GPIO_PIN2_HOLD` writer - Need add desc
pub type GPIO_PIN2_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN3_HOLD` reader - Need add desc
pub type GPIO_PIN3_HOLD_R = crate::BitReader;
///Field `GPIO_PIN3_HOLD` writer - Need add desc
pub type GPIO_PIN3_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN4_HOLD` reader - Need add desc
pub type GPIO_PIN4_HOLD_R = crate::BitReader;
///Field `GPIO_PIN4_HOLD` writer - Need add desc
pub type GPIO_PIN4_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN5_HOLD` reader - Need add desc
pub type GPIO_PIN5_HOLD_R = crate::BitReader;
///Field `GPIO_PIN5_HOLD` writer - Need add desc
pub type GPIO_PIN5_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Need add desc
    #[inline(always)]
    pub fn gpio_pin0_hold(&self) -> GPIO_PIN0_HOLD_R {
        GPIO_PIN0_HOLD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Need add desc
    #[inline(always)]
    pub fn gpio_pin1_hold(&self) -> GPIO_PIN1_HOLD_R {
        GPIO_PIN1_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Need add desc
    #[inline(always)]
    pub fn gpio_pin2_hold(&self) -> GPIO_PIN2_HOLD_R {
        GPIO_PIN2_HOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Need add desc
    #[inline(always)]
    pub fn gpio_pin3_hold(&self) -> GPIO_PIN3_HOLD_R {
        GPIO_PIN3_HOLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Need add desc
    #[inline(always)]
    pub fn gpio_pin4_hold(&self) -> GPIO_PIN4_HOLD_R {
        GPIO_PIN4_HOLD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Need add desc
    #[inline(always)]
    pub fn gpio_pin5_hold(&self) -> GPIO_PIN5_HOLD_R {
        GPIO_PIN5_HOLD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_HOLD")
            .field("gpio_pin0_hold", &self.gpio_pin0_hold())
            .field("gpio_pin1_hold", &self.gpio_pin1_hold())
            .field("gpio_pin2_hold", &self.gpio_pin2_hold())
            .field("gpio_pin3_hold", &self.gpio_pin3_hold())
            .field("gpio_pin4_hold", &self.gpio_pin4_hold())
            .field("gpio_pin5_hold", &self.gpio_pin5_hold())
            .finish()
    }
}
impl W {
    ///Bit 0 - Need add desc
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin0_hold(&mut self) -> GPIO_PIN0_HOLD_W<PAD_HOLD_SPEC> {
        GPIO_PIN0_HOLD_W::new(self, 0)
    }
    ///Bit 1 - Need add desc
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin1_hold(&mut self) -> GPIO_PIN1_HOLD_W<PAD_HOLD_SPEC> {
        GPIO_PIN1_HOLD_W::new(self, 1)
    }
    ///Bit 2 - Need add desc
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin2_hold(&mut self) -> GPIO_PIN2_HOLD_W<PAD_HOLD_SPEC> {
        GPIO_PIN2_HOLD_W::new(self, 2)
    }
    ///Bit 3 - Need add desc
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin3_hold(&mut self) -> GPIO_PIN3_HOLD_W<PAD_HOLD_SPEC> {
        GPIO_PIN3_HOLD_W::new(self, 3)
    }
    ///Bit 4 - Need add desc
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin4_hold(&mut self) -> GPIO_PIN4_HOLD_W<PAD_HOLD_SPEC> {
        GPIO_PIN4_HOLD_W::new(self, 4)
    }
    ///Bit 5 - Need add desc
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin5_hold(&mut self) -> GPIO_PIN5_HOLD_W<PAD_HOLD_SPEC> {
        GPIO_PIN5_HOLD_W::new(self, 5)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`pad_hold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_hold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PAD_HOLD_SPEC;
impl crate::RegisterSpec for PAD_HOLD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pad_hold::R`](R) reader structure
impl crate::Readable for PAD_HOLD_SPEC {}
///`write(|w| ..)` method takes [`pad_hold::W`](W) writer structure
impl crate::Writable for PAD_HOLD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PAD_HOLD to value 0
impl crate::Resettable for PAD_HOLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
