#[doc = "Register `PIN%s` reader"]
pub struct R(crate::R<PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN%s` writer"]
pub struct W(crate::W<PIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_SPEC>;
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
impl From<crate::W<PIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNC2_BYPASS` reader - For the second stage synchronization, GPIO input data can be syn- chronized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
pub type SYNC2_BYPASS_R = crate::FieldReader;
#[doc = "Field `SYNC2_BYPASS` writer - For the second stage synchronization, GPIO input data can be syn- chronized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
pub type SYNC2_BYPASS_W<'a, const O: u8> = crate::FieldWriter<'a, PIN_SPEC, 2, O>;
#[doc = "Field `PAD_DRIVER` reader - Pad driver selection. 0: normal output; 1: open drain output.."]
pub type PAD_DRIVER_R = crate::BitReader;
#[doc = "Field `PAD_DRIVER` writer - Pad driver selection. 0: normal output; 1: open drain output.."]
pub type PAD_DRIVER_W<'a, const O: u8> = crate::BitWriter<'a, PIN_SPEC, O>;
#[doc = "Field `SYNC1_BYPASS` reader - For the first stage synchronization, GPIO input data can be synchro- nized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
pub type SYNC1_BYPASS_R = crate::FieldReader;
#[doc = "Field `SYNC1_BYPASS` writer - For the first stage synchronization, GPIO input data can be synchro- nized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
pub type SYNC1_BYPASS_W<'a, const O: u8> = crate::FieldWriter<'a, PIN_SPEC, 2, O>;
#[doc = "Field `INT_TYPE` reader - Interrupt type selection. 0: GPIO interrupt disabled; 1: rising edge trigger; 2: falling edge trigger; 3: any edge trigger; 4: low level trigger; 5: high level trigger. (R/W)"]
pub type INT_TYPE_R = crate::FieldReader;
#[doc = "Field `INT_TYPE` writer - Interrupt type selection. 0: GPIO interrupt disabled; 1: rising edge trigger; 2: falling edge trigger; 3: any edge trigger; 4: low level trigger; 5: high level trigger. (R/W)"]
pub type INT_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, PIN_SPEC, 3, O>;
#[doc = "Field `WAKEUP_ENABLE` reader - GPIO wake-up enable bit, only wakes up the CPU from Light-sleep."]
pub type WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `WAKEUP_ENABLE` writer - GPIO wake-up enable bit, only wakes up the CPU from Light-sleep."]
pub type WAKEUP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, PIN_SPEC, O>;
#[doc = "Field `CONFIG` reader - Reserved"]
pub type CONFIG_R = crate::FieldReader;
#[doc = "Field `CONFIG` writer - Reserved"]
pub type CONFIG_W<'a, const O: u8> = crate::FieldWriter<'a, PIN_SPEC, 2, O>;
#[doc = "Field `INT_ENA` reader - Interrupt enable bits. bit13: CPU interrupt enabled; bit14: CPU non-maskable interrupt enabled."]
pub type INT_ENA_R = crate::FieldReader;
#[doc = "Field `INT_ENA` writer - Interrupt enable bits. bit13: CPU interrupt enabled; bit14: CPU non-maskable interrupt enabled."]
pub type INT_ENA_W<'a, const O: u8> = crate::FieldWriter<'a, PIN_SPEC, 5, O>;
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
            .field(
                "sync2_bypass",
                &format_args!("{}", self.sync2_bypass().bits()),
            )
            .field("pad_driver", &format_args!("{}", self.pad_driver().bit()))
            .field(
                "sync1_bypass",
                &format_args!("{}", self.sync1_bypass().bits()),
            )
            .field("int_type", &format_args!("{}", self.int_type().bits()))
            .field(
                "wakeup_enable",
                &format_args!("{}", self.wakeup_enable().bit()),
            )
            .field("config", &format_args!("{}", self.config().bits()))
            .field("int_ena", &format_args!("{}", self.int_ena().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PIN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - For the second stage synchronization, GPIO input data can be syn- chronized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn sync2_bypass(&mut self) -> SYNC2_BYPASS_W<0> {
        SYNC2_BYPASS_W::new(self)
    }
    #[doc = "Bit 2 - Pad driver selection. 0: normal output; 1: open drain output.."]
    #[inline(always)]
    #[must_use]
    pub fn pad_driver(&mut self) -> PAD_DRIVER_W<2> {
        PAD_DRIVER_W::new(self)
    }
    #[doc = "Bits 3:4 - For the first stage synchronization, GPIO input data can be synchro- nized on either edge of the APB clock. 0: no synchronization; 1: synchronized on falling edge; 2 and 3: synchronized on rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn sync1_bypass(&mut self) -> SYNC1_BYPASS_W<3> {
        SYNC1_BYPASS_W::new(self)
    }
    #[doc = "Bits 7:9 - Interrupt type selection. 0: GPIO interrupt disabled; 1: rising edge trigger; 2: falling edge trigger; 3: any edge trigger; 4: low level trigger; 5: high level trigger. (R/W)"]
    #[inline(always)]
    #[must_use]
    pub fn int_type(&mut self) -> INT_TYPE_W<7> {
        INT_TYPE_W::new(self)
    }
    #[doc = "Bit 10 - GPIO wake-up enable bit, only wakes up the CPU from Light-sleep."]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_enable(&mut self) -> WAKEUP_ENABLE_W<10> {
        WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bits 11:12 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn config(&mut self) -> CONFIG_W<11> {
        CONFIG_W::new(self)
    }
    #[doc = "Bits 13:17 - Interrupt enable bits. bit13: CPU interrupt enabled; bit14: CPU non-maskable interrupt enabled."]
    #[inline(always)]
    #[must_use]
    pub fn int_ena(&mut self) -> INT_ENA_W<13> {
        INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for GPIO pin %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](index.html) module"]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin::R](R) reader structure"]
impl crate::Readable for PIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin::W](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIN%s to value 0"]
impl crate::Resettable for PIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
