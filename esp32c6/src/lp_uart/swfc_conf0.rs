#[doc = "Register `SWFC_CONF0` reader"]
pub struct R(crate::R<SWFC_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWFC_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWFC_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWFC_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWFC_CONF0` writer"]
pub struct W(crate::W<SWFC_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWFC_CONF0_SPEC>;
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
impl From<crate::W<SWFC_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWFC_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XON_CHAR` reader - This register stores the Xon flow control char."]
pub type XON_CHAR_R = crate::FieldReader;
#[doc = "Field `XON_CHAR` writer - This register stores the Xon flow control char."]
pub type XON_CHAR_W<'a, const O: u8> = crate::FieldWriter<'a, SWFC_CONF0_SPEC, 8, O>;
#[doc = "Field `XOFF_CHAR` reader - This register stores the Xoff flow control char."]
pub type XOFF_CHAR_R = crate::FieldReader;
#[doc = "Field `XOFF_CHAR` writer - This register stores the Xoff flow control char."]
pub type XOFF_CHAR_W<'a, const O: u8> = crate::FieldWriter<'a, SWFC_CONF0_SPEC, 8, O>;
#[doc = "Field `XON_XOFF_STILL_SEND` reader - In software flow control mode, UART Tx is disabled once UART Rx receives XOFF. In this status, UART Tx can not transmit XOFF even the received data number is larger than UART_XOFF_THRESHOLD. Set this bit to enable UART Tx can transmit XON/XOFF when UART Tx is disabled."]
pub type XON_XOFF_STILL_SEND_R = crate::BitReader;
#[doc = "Field `XON_XOFF_STILL_SEND` writer - In software flow control mode, UART Tx is disabled once UART Rx receives XOFF. In this status, UART Tx can not transmit XOFF even the received data number is larger than UART_XOFF_THRESHOLD. Set this bit to enable UART Tx can transmit XON/XOFF when UART Tx is disabled."]
pub type XON_XOFF_STILL_SEND_W<'a, const O: u8> = crate::BitWriter<'a, SWFC_CONF0_SPEC, O>;
#[doc = "Field `SW_FLOW_CON_EN` reader - Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff."]
pub type SW_FLOW_CON_EN_R = crate::BitReader;
#[doc = "Field `SW_FLOW_CON_EN` writer - Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff."]
pub type SW_FLOW_CON_EN_W<'a, const O: u8> = crate::BitWriter<'a, SWFC_CONF0_SPEC, O>;
#[doc = "Field `XONOFF_DEL` reader - Set this bit to remove flow control char from the received data."]
pub type XONOFF_DEL_R = crate::BitReader;
#[doc = "Field `XONOFF_DEL` writer - Set this bit to remove flow control char from the received data."]
pub type XONOFF_DEL_W<'a, const O: u8> = crate::BitWriter<'a, SWFC_CONF0_SPEC, O>;
#[doc = "Field `FORCE_XON` reader - Set this bit to enable the transmitter to go on sending data."]
pub type FORCE_XON_R = crate::BitReader;
#[doc = "Field `FORCE_XON` writer - Set this bit to enable the transmitter to go on sending data."]
pub type FORCE_XON_W<'a, const O: u8> = crate::BitWriter<'a, SWFC_CONF0_SPEC, O>;
#[doc = "Field `FORCE_XOFF` reader - Set this bit to stop the transmitter from sending data."]
pub type FORCE_XOFF_R = crate::BitReader;
#[doc = "Field `FORCE_XOFF` writer - Set this bit to stop the transmitter from sending data."]
pub type FORCE_XOFF_W<'a, const O: u8> = crate::BitWriter<'a, SWFC_CONF0_SPEC, O>;
#[doc = "Field `SEND_XON` reader - Set this bit to send Xon char. It is cleared by hardware automatically."]
pub type SEND_XON_R = crate::BitReader;
#[doc = "Field `SEND_XON` writer - Set this bit to send Xon char. It is cleared by hardware automatically."]
pub type SEND_XON_W<'a, const O: u8> = crate::BitWriter<'a, SWFC_CONF0_SPEC, O>;
#[doc = "Field `SEND_XOFF` reader - Set this bit to send Xoff char. It is cleared by hardware automatically."]
pub type SEND_XOFF_R = crate::BitReader;
#[doc = "Field `SEND_XOFF` writer - Set this bit to send Xoff char. It is cleared by hardware automatically."]
pub type SEND_XOFF_W<'a, const O: u8> = crate::BitWriter<'a, SWFC_CONF0_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - This register stores the Xon flow control char."]
    #[inline(always)]
    pub fn xon_char(&self) -> XON_CHAR_R {
        XON_CHAR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register stores the Xoff flow control char."]
    #[inline(always)]
    pub fn xoff_char(&self) -> XOFF_CHAR_R {
        XOFF_CHAR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - In software flow control mode, UART Tx is disabled once UART Rx receives XOFF. In this status, UART Tx can not transmit XOFF even the received data number is larger than UART_XOFF_THRESHOLD. Set this bit to enable UART Tx can transmit XON/XOFF when UART Tx is disabled."]
    #[inline(always)]
    pub fn xon_xoff_still_send(&self) -> XON_XOFF_STILL_SEND_R {
        XON_XOFF_STILL_SEND_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff."]
    #[inline(always)]
    pub fn sw_flow_con_en(&self) -> SW_FLOW_CON_EN_R {
        SW_FLOW_CON_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set this bit to remove flow control char from the received data."]
    #[inline(always)]
    pub fn xonoff_del(&self) -> XONOFF_DEL_R {
        XONOFF_DEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set this bit to enable the transmitter to go on sending data."]
    #[inline(always)]
    pub fn force_xon(&self) -> FORCE_XON_R {
        FORCE_XON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to stop the transmitter from sending data."]
    #[inline(always)]
    pub fn force_xoff(&self) -> FORCE_XOFF_R {
        FORCE_XOFF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to send Xon char. It is cleared by hardware automatically."]
    #[inline(always)]
    pub fn send_xon(&self) -> SEND_XON_R {
        SEND_XON_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to send Xoff char. It is cleared by hardware automatically."]
    #[inline(always)]
    pub fn send_xoff(&self) -> SEND_XOFF_R {
        SEND_XOFF_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWFC_CONF0")
            .field("xon_char", &format_args!("{}", self.xon_char().bits()))
            .field("xoff_char", &format_args!("{}", self.xoff_char().bits()))
            .field(
                "xon_xoff_still_send",
                &format_args!("{}", self.xon_xoff_still_send().bit()),
            )
            .field(
                "sw_flow_con_en",
                &format_args!("{}", self.sw_flow_con_en().bit()),
            )
            .field("xonoff_del", &format_args!("{}", self.xonoff_del().bit()))
            .field("force_xon", &format_args!("{}", self.force_xon().bit()))
            .field("force_xoff", &format_args!("{}", self.force_xoff().bit()))
            .field("send_xon", &format_args!("{}", self.send_xon().bit()))
            .field("send_xoff", &format_args!("{}", self.send_xoff().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SWFC_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register stores the Xon flow control char."]
    #[inline(always)]
    #[must_use]
    pub fn xon_char(&mut self) -> XON_CHAR_W<0> {
        XON_CHAR_W::new(self)
    }
    #[doc = "Bits 8:15 - This register stores the Xoff flow control char."]
    #[inline(always)]
    #[must_use]
    pub fn xoff_char(&mut self) -> XOFF_CHAR_W<8> {
        XOFF_CHAR_W::new(self)
    }
    #[doc = "Bit 16 - In software flow control mode, UART Tx is disabled once UART Rx receives XOFF. In this status, UART Tx can not transmit XOFF even the received data number is larger than UART_XOFF_THRESHOLD. Set this bit to enable UART Tx can transmit XON/XOFF when UART Tx is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn xon_xoff_still_send(&mut self) -> XON_XOFF_STILL_SEND_W<16> {
        XON_XOFF_STILL_SEND_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to enable software flow control. It is used with register sw_xon or sw_xoff."]
    #[inline(always)]
    #[must_use]
    pub fn sw_flow_con_en(&mut self) -> SW_FLOW_CON_EN_W<17> {
        SW_FLOW_CON_EN_W::new(self)
    }
    #[doc = "Bit 18 - Set this bit to remove flow control char from the received data."]
    #[inline(always)]
    #[must_use]
    pub fn xonoff_del(&mut self) -> XONOFF_DEL_W<18> {
        XONOFF_DEL_W::new(self)
    }
    #[doc = "Bit 19 - Set this bit to enable the transmitter to go on sending data."]
    #[inline(always)]
    #[must_use]
    pub fn force_xon(&mut self) -> FORCE_XON_W<19> {
        FORCE_XON_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to stop the transmitter from sending data."]
    #[inline(always)]
    #[must_use]
    pub fn force_xoff(&mut self) -> FORCE_XOFF_W<20> {
        FORCE_XOFF_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to send Xon char. It is cleared by hardware automatically."]
    #[inline(always)]
    #[must_use]
    pub fn send_xon(&mut self) -> SEND_XON_W<21> {
        SEND_XON_W::new(self)
    }
    #[doc = "Bit 22 - Set this bit to send Xoff char. It is cleared by hardware automatically."]
    #[inline(always)]
    #[must_use]
    pub fn send_xoff(&mut self) -> SEND_XOFF_W<22> {
        SEND_XOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software flow-control character configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swfc_conf0](index.html) module"]
pub struct SWFC_CONF0_SPEC;
impl crate::RegisterSpec for SWFC_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swfc_conf0::R](R) reader structure"]
impl crate::Readable for SWFC_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swfc_conf0::W](W) writer structure"]
impl crate::Writable for SWFC_CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWFC_CONF0 to value 0x1311"]
impl crate::Resettable for SWFC_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1311;
}
