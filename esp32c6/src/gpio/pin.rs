#[doc = "Register `PIN%s` reader"]
pub type R = crate::R<PIN_SPEC>;
#[doc = "Register `PIN%s` writer"]
pub type W = crate::W<PIN_SPEC>;
#[doc = "Field `SYNC2_BYPASS` reader - set GPIO input_sync2 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
pub type SYNC2_BYPASS_R = crate::FieldReader;
#[doc = "Field `SYNC2_BYPASS` writer - set GPIO input_sync2 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
pub type SYNC2_BYPASS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PAD_DRIVER` reader - set this bit to select pad driver. 1:open-drain. 0:normal."]
pub type PAD_DRIVER_R = crate::BitReader;
#[doc = "Field `PAD_DRIVER` writer - set this bit to select pad driver. 1:open-drain. 0:normal."]
pub type PAD_DRIVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC1_BYPASS` reader - set GPIO input_sync1 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
pub type SYNC1_BYPASS_R = crate::FieldReader;
#[doc = "Field `SYNC1_BYPASS` writer - set GPIO input_sync1 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
pub type SYNC1_BYPASS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INT_TYPE` reader - set this value to choose interrupt mode. 0:disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
pub type INT_TYPE_R = crate::FieldReader;
#[doc = "Field `INT_TYPE` writer - set this value to choose interrupt mode. 0:disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
pub type INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WAKEUP_ENABLE` reader - set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
pub type WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `WAKEUP_ENABLE` writer - set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
pub type WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONFIG` reader - reserved"]
pub type CONFIG_R = crate::FieldReader;
#[doc = "Field `CONFIG` writer - reserved"]
pub type CONFIG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INT_ENA` reader - set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
pub type INT_ENA_R = crate::FieldReader;
#[doc = "Field `INT_ENA` writer - set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
pub type INT_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - set GPIO input_sync2 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    #[inline(always)]
    pub fn sync2_bypass(&self) -> SYNC2_BYPASS_R {
        SYNC2_BYPASS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - set this bit to select pad driver. 1:open-drain. 0:normal."]
    #[inline(always)]
    pub fn pad_driver(&self) -> PAD_DRIVER_R {
        PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - set GPIO input_sync1 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    #[inline(always)]
    pub fn sync1_bypass(&self) -> SYNC1_BYPASS_R {
        SYNC1_BYPASS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 7:9 - set this value to choose interrupt mode. 0:disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
    #[inline(always)]
    pub fn int_type(&self) -> INT_TYPE_R {
        INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
    #[inline(always)]
    pub fn wakeup_enable(&self) -> WAKEUP_ENABLE_R {
        WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - reserved"]
    #[inline(always)]
    pub fn config(&self) -> CONFIG_R {
        CONFIG_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:17 - set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - set GPIO input_sync2 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    #[inline(always)]
    #[must_use]
    pub fn sync2_bypass(&mut self) -> SYNC2_BYPASS_W<PIN_SPEC> {
        SYNC2_BYPASS_W::new(self, 0)
    }
    #[doc = "Bit 2 - set this bit to select pad driver. 1:open-drain. 0:normal."]
    #[inline(always)]
    #[must_use]
    pub fn pad_driver(&mut self) -> PAD_DRIVER_W<PIN_SPEC> {
        PAD_DRIVER_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - set GPIO input_sync1 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    #[inline(always)]
    #[must_use]
    pub fn sync1_bypass(&mut self) -> SYNC1_BYPASS_W<PIN_SPEC> {
        SYNC1_BYPASS_W::new(self, 3)
    }
    #[doc = "Bits 7:9 - set this value to choose interrupt mode. 0:disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
    #[inline(always)]
    #[must_use]
    pub fn int_type(&mut self) -> INT_TYPE_W<PIN_SPEC> {
        INT_TYPE_W::new(self, 7)
    }
    #[doc = "Bit 10 - set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_enable(&mut self) -> WAKEUP_ENABLE_W<PIN_SPEC> {
        WAKEUP_ENABLE_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn config(&mut self) -> CONFIG_W<PIN_SPEC> {
        CONFIG_W::new(self, 11)
    }
    #[doc = "Bits 13:17 - set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn int_ena(&mut self) -> INT_ENA_W<PIN_SPEC> {
        INT_ENA_W::new(self, 13)
    }
}
#[doc = "GPIO pin configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin::R`](R) reader structure"]
impl crate::Readable for PIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pin::W`](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIN%s to value 0"]
impl crate::Resettable for PIN_SPEC {
    const RESET_VALUE: u32 = 0;
}
