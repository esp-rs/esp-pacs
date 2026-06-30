#[doc = "Register `REGISTER55_WATCHDOGTIMEOUTREGISTER` reader"]
pub type R = crate::R<REGISTER55_WATCHDOGTIMEOUTREGISTER_SPEC>;
#[doc = "Register `REGISTER55_WATCHDOGTIMEOUTREGISTER` writer"]
pub type W = crate::W<REGISTER55_WATCHDOGTIMEOUTREGISTER_SPEC>;
#[doc = "Field `WTO` reader - Watchdog Timeout When Bit 16 _PWE_ is set and Bit 23 _WD_ of Register 0 _MAC Configuration Register_ is reset, this field is used as watchdog timeout for a received frame If the length of a received frame exceeds the value of this field, such frame is terminated and declared as an error frame Note: When Bit 16 _PWE_ is set, the value in this field should be more than 1,522 _0x05F2_ Otherwise, the IEEE Std 8023specified valid tagged frames are declared as error frames and are dropped"]
pub type WTO_R = crate::FieldReader<u16>;
#[doc = "Field `WTO` writer - Watchdog Timeout When Bit 16 _PWE_ is set and Bit 23 _WD_ of Register 0 _MAC Configuration Register_ is reset, this field is used as watchdog timeout for a received frame If the length of a received frame exceeds the value of this field, such frame is terminated and declared as an error frame Note: When Bit 16 _PWE_ is set, the value in this field should be more than 1,522 _0x05F2_ Otherwise, the IEEE Std 8023specified valid tagged frames are declared as error frames and are dropped"]
pub type WTO_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PWE` reader - Programmable Watchdog Enable When this bit is set and Bit 23 _WD_ of Register 0 _MAC Configuration Register_ is reset, the WTO field _Bits\\[13:0\\]_ is used as watchdog timeout for a received frame When this bit is cleared, the watchdog timeout for a received frame is controlled by the setting of Bit 23 _WD_ and Bit 20 _JE_ in Register 0 _MAC Configuration Register_"]
pub type PWE_R = crate::BitReader;
#[doc = "Field `PWE` writer - Programmable Watchdog Enable When this bit is set and Bit 23 _WD_ of Register 0 _MAC Configuration Register_ is reset, the WTO field _Bits\\[13:0\\]_ is used as watchdog timeout for a received frame When this bit is cleared, the watchdog timeout for a received frame is controlled by the setting of Bit 23 _WD_ and Bit 20 _JE_ in Register 0 _MAC Configuration Register_"]
pub type PWE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - Watchdog Timeout When Bit 16 _PWE_ is set and Bit 23 _WD_ of Register 0 _MAC Configuration Register_ is reset, this field is used as watchdog timeout for a received frame If the length of a received frame exceeds the value of this field, such frame is terminated and declared as an error frame Note: When Bit 16 _PWE_ is set, the value in this field should be more than 1,522 _0x05F2_ Otherwise, the IEEE Std 8023specified valid tagged frames are declared as error frames and are dropped"]
    #[inline(always)]
    pub fn wto(&self) -> WTO_R {
        WTO_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - Programmable Watchdog Enable When this bit is set and Bit 23 _WD_ of Register 0 _MAC Configuration Register_ is reset, the WTO field _Bits\\[13:0\\]_ is used as watchdog timeout for a received frame When this bit is cleared, the watchdog timeout for a received frame is controlled by the setting of Bit 23 _WD_ and Bit 20 _JE_ in Register 0 _MAC Configuration Register_"]
    #[inline(always)]
    pub fn pwe(&self) -> PWE_R {
        PWE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER55_WATCHDOGTIMEOUTREGISTER")
            .field("wto", &self.wto())
            .field("pwe", &self.pwe())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Watchdog Timeout When Bit 16 _PWE_ is set and Bit 23 _WD_ of Register 0 _MAC Configuration Register_ is reset, this field is used as watchdog timeout for a received frame If the length of a received frame exceeds the value of this field, such frame is terminated and declared as an error frame Note: When Bit 16 _PWE_ is set, the value in this field should be more than 1,522 _0x05F2_ Otherwise, the IEEE Std 8023specified valid tagged frames are declared as error frames and are dropped"]
    #[inline(always)]
    pub fn wto(&mut self) -> WTO_W<'_, REGISTER55_WATCHDOGTIMEOUTREGISTER_SPEC> {
        WTO_W::new(self, 0)
    }
    #[doc = "Bit 16 - Programmable Watchdog Enable When this bit is set and Bit 23 _WD_ of Register 0 _MAC Configuration Register_ is reset, the WTO field _Bits\\[13:0\\]_ is used as watchdog timeout for a received frame When this bit is cleared, the watchdog timeout for a received frame is controlled by the setting of Bit 23 _WD_ and Bit 20 _JE_ in Register 0 _MAC Configuration Register_"]
    #[inline(always)]
    pub fn pwe(&mut self) -> PWE_W<'_, REGISTER55_WATCHDOGTIMEOUTREGISTER_SPEC> {
        PWE_W::new(self, 16)
    }
}
#[doc = "Controls the watchdog timeout for received frames\n\nYou can [`read`](crate::Reg::read) this register and get [`register55_watchdogtimeoutregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register55_watchdogtimeoutregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER55_WATCHDOGTIMEOUTREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER55_WATCHDOGTIMEOUTREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register55_watchdogtimeoutregister::R`](R) reader structure"]
impl crate::Readable for REGISTER55_WATCHDOGTIMEOUTREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register55_watchdogtimeoutregister::W`](W) writer structure"]
impl crate::Writable for REGISTER55_WATCHDOGTIMEOUTREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER55_WATCHDOGTIMEOUTREGISTER to value 0"]
impl crate::Resettable for REGISTER55_WATCHDOGTIMEOUTREGISTER_SPEC {}
