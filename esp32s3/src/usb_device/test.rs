#[doc = "Register `TEST` reader"]
pub struct R(crate::R<TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST` writer"]
pub struct W(crate::W<TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_SPEC>;
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
impl From<crate::W<TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable test of the USB pad"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable test of the USB pad"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, TEST_SPEC, O>;
#[doc = "Field `USB_OE` reader - USB pad oen in test"]
pub type USB_OE_R = crate::BitReader;
#[doc = "Field `USB_OE` writer - USB pad oen in test"]
pub type USB_OE_W<'a, const O: u8> = crate::BitWriter<'a, TEST_SPEC, O>;
#[doc = "Field `TX_DP` reader - USB D+ tx value in test"]
pub type TX_DP_R = crate::BitReader;
#[doc = "Field `TX_DP` writer - USB D+ tx value in test"]
pub type TX_DP_W<'a, const O: u8> = crate::BitWriter<'a, TEST_SPEC, O>;
#[doc = "Field `TX_DM` reader - USB D- tx value in test"]
pub type TX_DM_R = crate::BitReader;
#[doc = "Field `TX_DM` writer - USB D- tx value in test"]
pub type TX_DM_W<'a, const O: u8> = crate::BitWriter<'a, TEST_SPEC, O>;
#[doc = "Field `RX_RCV` reader - USB differential rx value in test"]
pub type RX_RCV_R = crate::BitReader;
#[doc = "Field `RX_DP` reader - USB D+ rx value in test"]
pub type RX_DP_R = crate::BitReader;
#[doc = "Field `RX_DM` reader - USB D- rx value in test"]
pub type RX_DM_R = crate::BitReader;
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
    #[doc = "Bit 4 - USB differential rx value in test"]
    #[inline(always)]
    pub fn rx_rcv(&self) -> RX_RCV_R {
        RX_RCV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB D+ rx value in test"]
    #[inline(always)]
    pub fn rx_dp(&self) -> RX_DP_R {
        RX_DP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB D- rx value in test"]
    #[inline(always)]
    pub fn rx_dm(&self) -> RX_DM_R {
        RX_DM_R::new(((self.bits >> 6) & 1) != 0)
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
            .field("rx_rcv", &format_args!("{}", self.rx_rcv().bit()))
            .field("rx_dp", &format_args!("{}", self.rx_dp().bit()))
            .field("rx_dm", &format_args!("{}", self.rx_dm().bit()))
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
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - USB pad oen in test"]
    #[inline(always)]
    #[must_use]
    pub fn usb_oe(&mut self) -> USB_OE_W<1> {
        USB_OE_W::new(self)
    }
    #[doc = "Bit 2 - USB D+ tx value in test"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dp(&mut self) -> TX_DP_W<2> {
        TX_DP_W::new(self)
    }
    #[doc = "Bit 3 - USB D- tx value in test"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dm(&mut self) -> TX_DM_W<3> {
        TX_DM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Internal PHY test register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](index.html) module"]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test::R](R) reader structure"]
impl crate::Readable for TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test::W](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
