#[doc = "Register `HWFC_CONF` reader"]
pub struct R(crate::R<HWFC_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWFC_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWFC_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWFC_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWFC_CONF` writer"]
pub struct W(crate::W<HWFC_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWFC_CONF_SPEC>;
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
impl From<crate::W<HWFC_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWFC_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_FLOW_THRHD` reader - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
pub type RX_FLOW_THRHD_R = crate::FieldReader;
#[doc = "Field `RX_FLOW_THRHD` writer - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
pub type RX_FLOW_THRHD_W<'a, const O: u8> = crate::FieldWriter<'a, HWFC_CONF_SPEC, 5, O>;
#[doc = "Field `RX_FLOW_EN` reader - This is the flow enable bit for UART receiver."]
pub type RX_FLOW_EN_R = crate::BitReader;
#[doc = "Field `RX_FLOW_EN` writer - This is the flow enable bit for UART receiver."]
pub type RX_FLOW_EN_W<'a, const O: u8> = crate::BitWriter<'a, HWFC_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 3:7 - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
    #[inline(always)]
    pub fn rx_flow_thrhd(&self) -> RX_FLOW_THRHD_R {
        RX_FLOW_THRHD_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - This is the flow enable bit for UART receiver."]
    #[inline(always)]
    pub fn rx_flow_en(&self) -> RX_FLOW_EN_R {
        RX_FLOW_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWFC_CONF")
            .field(
                "rx_flow_thrhd",
                &format_args!("{}", self.rx_flow_thrhd().bits()),
            )
            .field("rx_flow_en", &format_args!("{}", self.rx_flow_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HWFC_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 3:7 - This register is used to configure the maximum amount of data that can be received when hardware flow control works."]
    #[inline(always)]
    #[must_use]
    pub fn rx_flow_thrhd(&mut self) -> RX_FLOW_THRHD_W<3> {
        RX_FLOW_THRHD_W::new(self)
    }
    #[doc = "Bit 8 - This is the flow enable bit for UART receiver."]
    #[inline(always)]
    #[must_use]
    pub fn rx_flow_en(&mut self) -> RX_FLOW_EN_W<8> {
        RX_FLOW_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware flow-control configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwfc_conf](index.html) module"]
pub struct HWFC_CONF_SPEC;
impl crate::RegisterSpec for HWFC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwfc_conf::R](R) reader structure"]
impl crate::Readable for HWFC_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwfc_conf::W](W) writer structure"]
impl crate::Writable for HWFC_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWFC_CONF to value 0"]
impl crate::Resettable for HWFC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
