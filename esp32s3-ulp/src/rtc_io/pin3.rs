///Register `PIN3` reader
pub type R = crate::R<PIN3_SPEC>;
///Register `PIN3` writer
pub type W = crate::W<PIN3_SPEC>;
///Field `PAD_DRIVER` reader - if set to 0: normal output, if set to 1: open drain
pub type PAD_DRIVER_R = crate::BitReader;
///Field `PAD_DRIVER` writer - if set to 0: normal output, if set to 1: open drain
pub type PAD_DRIVER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INT_TYPE` reader - if set to 0: GPIO interrupt disable, if set to 1: rising edge trigger, if set to 2: falling edge trigger, if set to 3: any edge trigger, if set to 4: low level trigger, if set to 5: high level trigger
pub type INT_TYPE_R = crate::FieldReader;
///Field `INT_TYPE` writer - if set to 0: GPIO interrupt disable, if set to 1: rising edge trigger, if set to 2: falling edge trigger, if set to 3: any edge trigger, if set to 4: low level trigger, if set to 5: high level trigger
pub type INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `WAKEUP_ENABLE` reader - RTC GPIO wakeup enable bit
pub type WAKEUP_ENABLE_R = crate::BitReader;
///Field `WAKEUP_ENABLE` writer - RTC GPIO wakeup enable bit
pub type WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - if set to 0: normal output, if set to 1: open drain
    #[inline(always)]
    pub fn pad_driver(&self) -> PAD_DRIVER_R {
        PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 7:9 - if set to 0: GPIO interrupt disable, if set to 1: rising edge trigger, if set to 2: falling edge trigger, if set to 3: any edge trigger, if set to 4: low level trigger, if set to 5: high level trigger
    #[inline(always)]
    pub fn int_type(&self) -> INT_TYPE_R {
        INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bit 10 - RTC GPIO wakeup enable bit
    #[inline(always)]
    pub fn wakeup_enable(&self) -> WAKEUP_ENABLE_R {
        WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN3")
            .field("pad_driver", &self.pad_driver())
            .field("int_type", &self.int_type())
            .field("wakeup_enable", &self.wakeup_enable())
            .finish()
    }
}
impl W {
    ///Bit 2 - if set to 0: normal output, if set to 1: open drain
    #[inline(always)]
    #[must_use]
    pub fn pad_driver(&mut self) -> PAD_DRIVER_W<PIN3_SPEC> {
        PAD_DRIVER_W::new(self, 2)
    }
    ///Bits 7:9 - if set to 0: GPIO interrupt disable, if set to 1: rising edge trigger, if set to 2: falling edge trigger, if set to 3: any edge trigger, if set to 4: low level trigger, if set to 5: high level trigger
    #[inline(always)]
    #[must_use]
    pub fn int_type(&mut self) -> INT_TYPE_W<PIN3_SPEC> {
        INT_TYPE_W::new(self, 7)
    }
    ///Bit 10 - RTC GPIO wakeup enable bit
    #[inline(always)]
    #[must_use]
    pub fn wakeup_enable(&mut self) -> WAKEUP_ENABLE_W<PIN3_SPEC> {
        WAKEUP_ENABLE_W::new(self, 10)
    }
}
/**configure RTC GPIO3

You can [`read`](crate::generic::Reg::read) this register and get [`pin3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PIN3_SPEC;
impl crate::RegisterSpec for PIN3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pin3::R`](R) reader structure
impl crate::Readable for PIN3_SPEC {}
///`write(|w| ..)` method takes [`pin3::W`](W) writer structure
impl crate::Writable for PIN3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PIN3 to value 0
impl crate::Resettable for PIN3_SPEC {
    const RESET_VALUE: u32 = 0;
}
