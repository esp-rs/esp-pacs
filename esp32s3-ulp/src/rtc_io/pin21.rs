#[doc = "Register `PIN21` reader"]
pub type R = crate::R<PIN21_SPEC>;
#[doc = "Register `PIN21` writer"]
pub type W = crate::W<PIN21_SPEC>;
#[doc = "Field `PAD_DRIVER` reader - if set to 0: normal output, if set to 1: open drain"]
pub type PAD_DRIVER_R = crate::BitReader;
#[doc = "Field `PAD_DRIVER` writer - if set to 0: normal output, if set to 1: open drain"]
pub type PAD_DRIVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_TYPE` reader - if set to 0: GPIO interrupt disable, if set to 1: rising edge trigger, if set to 2: falling edge trigger, if set to 3: any edge trigger, if set to 4: low level trigger, if set to 5: high level trigger"]
pub type INT_TYPE_R = crate::FieldReader;
#[doc = "Field `INT_TYPE` writer - if set to 0: GPIO interrupt disable, if set to 1: rising edge trigger, if set to 2: falling edge trigger, if set to 3: any edge trigger, if set to 4: low level trigger, if set to 5: high level trigger"]
pub type INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WAKEUP_ENABLE` reader - RTC GPIO wakeup enable bit"]
pub type WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `WAKEUP_ENABLE` writer - RTC GPIO wakeup enable bit"]
pub type WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - if set to 0: normal output, if set to 1: open drain"]
    #[inline(always)]
    pub fn pad_driver(&self) -> PAD_DRIVER_R {
        PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:9 - if set to 0: GPIO interrupt disable, if set to 1: rising edge trigger, if set to 2: falling edge trigger, if set to 3: any edge trigger, if set to 4: low level trigger, if set to 5: high level trigger"]
    #[inline(always)]
    pub fn int_type(&self) -> INT_TYPE_R {
        INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - RTC GPIO wakeup enable bit"]
    #[inline(always)]
    pub fn wakeup_enable(&self) -> WAKEUP_ENABLE_R {
        WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN21")
            .field("pad_driver", &self.pad_driver())
            .field("int_type", &self.int_type())
            .field("wakeup_enable", &self.wakeup_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - if set to 0: normal output, if set to 1: open drain"]
    #[inline(always)]
    pub fn pad_driver(&mut self) -> PAD_DRIVER_W<'_, PIN21_SPEC> {
        PAD_DRIVER_W::new(self, 2)
    }
    #[doc = "Bits 7:9 - if set to 0: GPIO interrupt disable, if set to 1: rising edge trigger, if set to 2: falling edge trigger, if set to 3: any edge trigger, if set to 4: low level trigger, if set to 5: high level trigger"]
    #[inline(always)]
    pub fn int_type(&mut self) -> INT_TYPE_W<'_, PIN21_SPEC> {
        INT_TYPE_W::new(self, 7)
    }
    #[doc = "Bit 10 - RTC GPIO wakeup enable bit"]
    #[inline(always)]
    pub fn wakeup_enable(&mut self) -> WAKEUP_ENABLE_W<'_, PIN21_SPEC> {
        WAKEUP_ENABLE_W::new(self, 10)
    }
}
#[doc = "configure RTC GPIO21\n\nYou can [`read`](crate::Reg::read) this register and get [`pin21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIN21_SPEC;
impl crate::RegisterSpec for PIN21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin21::R`](R) reader structure"]
impl crate::Readable for PIN21_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pin21::W`](W) writer structure"]
impl crate::Writable for PIN21_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN21 to value 0"]
impl crate::Resettable for PIN21_SPEC {}
