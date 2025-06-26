#[doc = "Register `PIN%s` reader"]
pub type R = crate::R<PIN_SPEC>;
#[doc = "Register `PIN%s` writer"]
pub type W = crate::W<PIN_SPEC>;
#[doc = "Field `SYNC2_BYPASS` reader - For the second stage synchronization, GPIO input data can be syn- chronized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
pub type SYNC2_BYPASS_R = crate::FieldReader;
#[doc = "Field `SYNC2_BYPASS` writer - For the second stage synchronization, GPIO input data can be syn- chronized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
pub type SYNC2_BYPASS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PAD_DRIVER` reader - Pad driver selection. 0: normal output; 1: open drain output.."]
pub type PAD_DRIVER_R = crate::BitReader;
#[doc = "Field `PAD_DRIVER` writer - Pad driver selection. 0: normal output; 1: open drain output.."]
pub type PAD_DRIVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC1_BYPASS` reader - For the first stage synchronization, GPIO input data can be synchro- nized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
pub type SYNC1_BYPASS_R = crate::FieldReader;
#[doc = "Field `SYNC1_BYPASS` writer - For the first stage synchronization, GPIO input data can be synchro- nized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
pub type SYNC1_BYPASS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INT_TYPE` reader - Interrupt type selection. 0: GPIO interrupt disabled; 1: rising edge trigger; 2: falling edge trigger; 3: any edge trigger; 4: low level trigger; 5: high level trigger. (R/W)"]
pub type INT_TYPE_R = crate::FieldReader;
#[doc = "Field `INT_TYPE` writer - Interrupt type selection. 0: GPIO interrupt disabled; 1: rising edge trigger; 2: falling edge trigger; 3: any edge trigger; 4: low level trigger; 5: high level trigger. (R/W)"]
pub type INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WAKEUP_ENABLE` reader - GPIO wake-up enable bit, only wakes up the CPU from Light-sleep."]
pub type WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `WAKEUP_ENABLE` writer - GPIO wake-up enable bit, only wakes up the CPU from Light-sleep."]
pub type WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONFIG` reader - Reserved"]
pub type CONFIG_R = crate::FieldReader;
#[doc = "Field `CONFIG` writer - Reserved"]
pub type CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INT_ENA` reader - Interrupt enable bits. bit13: CPU interrupt enabled; bit14: CPU non-maskable interrupt enabled."]
pub type INT_ENA_R = crate::FieldReader;
#[doc = "Field `INT_ENA` writer - Interrupt enable bits. bit13: CPU interrupt enabled; bit14: CPU non-maskable interrupt enabled."]
pub type INT_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - For the second stage synchronization, GPIO input data can be syn- chronized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
    #[inline(always)]
    pub fn sync2_bypass(&self) -> SYNC2_BYPASS_R {
        SYNC2_BYPASS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Pad driver selection. 0: normal output; 1: open drain output.."]
    #[inline(always)]
    pub fn pad_driver(&self) -> PAD_DRIVER_R {
        PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - For the first stage synchronization, GPIO input data can be synchro- nized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
    #[inline(always)]
    pub fn sync1_bypass(&self) -> SYNC1_BYPASS_R {
        SYNC1_BYPASS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 7:9 - Interrupt type selection. 0: GPIO interrupt disabled; 1: rising edge trigger; 2: falling edge trigger; 3: any edge trigger; 4: low level trigger; 5: high level trigger. (R/W)"]
    #[inline(always)]
    pub fn int_type(&self) -> INT_TYPE_R {
        INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - GPIO wake-up enable bit, only wakes up the CPU from Light-sleep."]
    #[inline(always)]
    pub fn wakeup_enable(&self) -> WAKEUP_ENABLE_R {
        WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Reserved"]
    #[inline(always)]
    pub fn config(&self) -> CONFIG_R {
        CONFIG_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:17 - Interrupt enable bits. bit13: CPU interrupt enabled; bit14: CPU non-maskable interrupt enabled."]
    #[inline(always)]
    pub fn int_ena(&self) -> INT_ENA_R {
        INT_ENA_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN")
            .field("sync2_bypass", &self.sync2_bypass())
            .field("pad_driver", &self.pad_driver())
            .field("sync1_bypass", &self.sync1_bypass())
            .field("int_type", &self.int_type())
            .field("wakeup_enable", &self.wakeup_enable())
            .field("config", &self.config())
            .field("int_ena", &self.int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - For the second stage synchronization, GPIO input data can be syn- chronized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
    #[inline(always)]
    pub fn sync2_bypass(&mut self) -> SYNC2_BYPASS_W<PIN_SPEC> {
        SYNC2_BYPASS_W::new(self, 0)
    }
    #[doc = "Bit 2 - Pad driver selection. 0: normal output; 1: open drain output.."]
    #[inline(always)]
    pub fn pad_driver(&mut self) -> PAD_DRIVER_W<PIN_SPEC> {
        PAD_DRIVER_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - For the first stage synchronization, GPIO input data can be synchro- nized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
    #[inline(always)]
    pub fn sync1_bypass(&mut self) -> SYNC1_BYPASS_W<PIN_SPEC> {
        SYNC1_BYPASS_W::new(self, 3)
    }
    #[doc = "Bits 7:9 - Interrupt type selection. 0: GPIO interrupt disabled; 1: rising edge trigger; 2: falling edge trigger; 3: any edge trigger; 4: low level trigger; 5: high level trigger. (R/W)"]
    #[inline(always)]
    pub fn int_type(&mut self) -> INT_TYPE_W<PIN_SPEC> {
        INT_TYPE_W::new(self, 7)
    }
    #[doc = "Bit 10 - GPIO wake-up enable bit, only wakes up the CPU from Light-sleep."]
    #[inline(always)]
    pub fn wakeup_enable(&mut self) -> WAKEUP_ENABLE_W<PIN_SPEC> {
        WAKEUP_ENABLE_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - Reserved"]
    #[inline(always)]
    pub fn config(&mut self) -> CONFIG_W<PIN_SPEC> {
        CONFIG_W::new(self, 11)
    }
    #[doc = "Bits 13:17 - Interrupt enable bits. bit13: CPU interrupt enabled; bit14: CPU non-maskable interrupt enabled."]
    #[inline(always)]
    pub fn int_ena(&mut self) -> INT_ENA_W<PIN_SPEC> {
        INT_ENA_W::new(self, 13)
    }
}
#[doc = "Configuration for GPIO pin %s\n\nYou can [`read`](crate::Reg::read) this register and get [`pin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin::R`](R) reader structure"]
impl crate::Readable for PIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pin::W`](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIN%s to value 0"]
impl crate::Resettable for PIN_SPEC {}
