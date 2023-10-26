#[doc = "Register `TEST` reader"]
pub type R = crate::R<TEST_SPEC>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TEST_SPEC>;
#[doc = "Field `TEST_ENABLE` reader - Enable test of the USB pad"]
pub type TEST_ENABLE_R = crate::BitReader;
#[doc = "Field `TEST_ENABLE` writer - Enable test of the USB pad"]
pub type TEST_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEST_USB_OE` reader - USB pad oen in test"]
pub type TEST_USB_OE_R = crate::BitReader;
#[doc = "Field `TEST_USB_OE` writer - USB pad oen in test"]
pub type TEST_USB_OE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEST_TX_DP` reader - USB D+ tx value in test"]
pub type TEST_TX_DP_R = crate::BitReader;
#[doc = "Field `TEST_TX_DP` writer - USB D+ tx value in test"]
pub type TEST_TX_DP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEST_TX_DM` reader - USB D- tx value in test"]
pub type TEST_TX_DM_R = crate::BitReader;
#[doc = "Field `TEST_TX_DM` writer - USB D- tx value in test"]
pub type TEST_TX_DM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEST_RX_RCV` reader - USB RCV value in test"]
pub type TEST_RX_RCV_R = crate::BitReader;
#[doc = "Field `TEST_RX_DP` reader - USB D+ rx value in test"]
pub type TEST_RX_DP_R = crate::BitReader;
#[doc = "Field `TEST_RX_DM` reader - USB D- rx value in test"]
pub type TEST_RX_DM_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enable test of the USB pad"]
    #[inline(always)]
    pub fn test_enable(&self) -> TEST_ENABLE_R {
        TEST_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB pad oen in test"]
    #[inline(always)]
    pub fn test_usb_oe(&self) -> TEST_USB_OE_R {
        TEST_USB_OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB D+ tx value in test"]
    #[inline(always)]
    pub fn test_tx_dp(&self) -> TEST_TX_DP_R {
        TEST_TX_DP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB D- tx value in test"]
    #[inline(always)]
    pub fn test_tx_dm(&self) -> TEST_TX_DM_R {
        TEST_TX_DM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB RCV value in test"]
    #[inline(always)]
    pub fn test_rx_rcv(&self) -> TEST_RX_RCV_R {
        TEST_RX_RCV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB D+ rx value in test"]
    #[inline(always)]
    pub fn test_rx_dp(&self) -> TEST_RX_DP_R {
        TEST_RX_DP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB D- rx value in test"]
    #[inline(always)]
    pub fn test_rx_dm(&self) -> TEST_RX_DM_R {
        TEST_RX_DM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST")
            .field("test_enable", &format_args!("{}", self.test_enable().bit()))
            .field("test_usb_oe", &format_args!("{}", self.test_usb_oe().bit()))
            .field("test_tx_dp", &format_args!("{}", self.test_tx_dp().bit()))
            .field("test_tx_dm", &format_args!("{}", self.test_tx_dm().bit()))
            .field("test_rx_rcv", &format_args!("{}", self.test_rx_rcv().bit()))
            .field("test_rx_dp", &format_args!("{}", self.test_rx_dp().bit()))
            .field("test_rx_dm", &format_args!("{}", self.test_rx_dm().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TEST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable test of the USB pad"]
    #[inline(always)]
    #[must_use]
    pub fn test_enable(&mut self) -> TEST_ENABLE_W<TEST_SPEC, 0> {
        TEST_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - USB pad oen in test"]
    #[inline(always)]
    #[must_use]
    pub fn test_usb_oe(&mut self) -> TEST_USB_OE_W<TEST_SPEC, 1> {
        TEST_USB_OE_W::new(self)
    }
    #[doc = "Bit 2 - USB D+ tx value in test"]
    #[inline(always)]
    #[must_use]
    pub fn test_tx_dp(&mut self) -> TEST_TX_DP_W<TEST_SPEC, 2> {
        TEST_TX_DP_W::new(self)
    }
    #[doc = "Bit 3 - USB D- tx value in test"]
    #[inline(always)]
    #[must_use]
    pub fn test_tx_dm(&mut self) -> TEST_TX_DM_W<TEST_SPEC, 3> {
        TEST_TX_DM_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Registers used for debugging the PHY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TEST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST to value 0x30"]
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}
