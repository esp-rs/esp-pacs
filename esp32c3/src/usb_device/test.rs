#[doc = "Register `TEST` reader"]
pub type R = crate::R<TEST_SPEC>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TEST_SPEC>;
#[doc = "Field `ENABLE` reader - Enable test of the USB pad"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable test of the USB pad"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_OE` reader - USB pad oen in test"]
pub type USB_OE_R = crate::BitReader;
#[doc = "Field `USB_OE` writer - USB pad oen in test"]
pub type USB_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DP` reader - USB D+ tx value in test"]
pub type TX_DP_R = crate::BitReader;
#[doc = "Field `TX_DP` writer - USB D+ tx value in test"]
pub type TX_DP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DM` reader - USB D- tx value in test"]
pub type TX_DM_R = crate::BitReader;
#[doc = "Field `TX_DM` writer - USB D- tx value in test"]
pub type TX_DM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable test of the USB pad"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB pad oen in test"]
    #[inline(always)]
    pub fn usb_oe(&self) -> USB_OE_R {
        USB_OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB D+ tx value in test"]
    #[inline(always)]
    pub fn tx_dp(&self) -> TX_DP_R {
        TX_DP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB D- tx value in test"]
    #[inline(always)]
    pub fn tx_dm(&self) -> TX_DM_R {
        TX_DM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST")
            .field("enable", &format_args!("{}", self.enable().bit()))
            .field("usb_oe", &format_args!("{}", self.usb_oe().bit()))
            .field("tx_dp", &format_args!("{}", self.tx_dp().bit()))
            .field("tx_dm", &format_args!("{}", self.tx_dm().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TEST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable test of the USB pad"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<TEST_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - USB pad oen in test"]
    #[inline(always)]
    #[must_use]
    pub fn usb_oe(&mut self) -> USB_OE_W<TEST_SPEC> {
        USB_OE_W::new(self, 1)
    }
    #[doc = "Bit 2 - USB D+ tx value in test"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dp(&mut self) -> TX_DP_W<TEST_SPEC> {
        TX_DP_W::new(self, 2)
    }
    #[doc = "Bit 3 - USB D- tx value in test"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dm(&mut self) -> TX_DM_W<TEST_SPEC> {
        TX_DM_W::new(self, 3)
    }
}
#[doc = "USB_DEVICE_TEST_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TEST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: u32 = 0;
}
