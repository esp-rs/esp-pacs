#[doc = "Register `SWFC_CONF` reader"]
pub struct R(crate::R<SWFC_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWFC_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWFC_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWFC_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWFC_CONF` writer"]
pub struct W(crate::W<SWFC_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWFC_CONF_SPEC>;
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
impl From<crate::W<SWFC_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWFC_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XON_THRESHOLD` reader - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
pub type XON_THRESHOLD_R = crate::FieldReader;
#[doc = "Field `XON_THRESHOLD` writer - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
pub type XON_THRESHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, SWFC_CONF_SPEC, 8, O>;
#[doc = "Field `XOFF_THRESHOLD` reader - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
pub type XOFF_THRESHOLD_R = crate::FieldReader;
#[doc = "Field `XOFF_THRESHOLD` writer - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
pub type XOFF_THRESHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, SWFC_CONF_SPEC, 8, O>;
#[doc = "Field `XON_CHAR` reader - This register stores the xon flow control char."]
pub type XON_CHAR_R = crate::FieldReader;
#[doc = "Field `XON_CHAR` writer - This register stores the xon flow control char."]
pub type XON_CHAR_W<'a, const O: u8> = crate::FieldWriter<'a, SWFC_CONF_SPEC, 8, O>;
#[doc = "Field `XOFF_CHAR` reader - This register stores the xoff flow control char."]
pub type XOFF_CHAR_R = crate::FieldReader;
#[doc = "Field `XOFF_CHAR` writer - This register stores the xoff flow control char."]
pub type XOFF_CHAR_W<'a, const O: u8> = crate::FieldWriter<'a, SWFC_CONF_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    pub fn xon_threshold(&self) -> XON_THRESHOLD_R {
        XON_THRESHOLD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    pub fn xoff_threshold(&self) -> XOFF_THRESHOLD_R {
        XOFF_THRESHOLD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register stores the xon flow control char."]
    #[inline(always)]
    pub fn xon_char(&self) -> XON_CHAR_R {
        XON_CHAR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This register stores the xoff flow control char."]
    #[inline(always)]
    pub fn xoff_char(&self) -> XOFF_CHAR_R {
        XOFF_CHAR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWFC_CONF")
            .field(
                "xon_threshold",
                &format_args!("{}", self.xon_threshold().bits()),
            )
            .field(
                "xoff_threshold",
                &format_args!("{}", self.xoff_threshold().bits()),
            )
            .field("xon_char", &format_args!("{}", self.xon_char().bits()))
            .field("xoff_char", &format_args!("{}", self.xoff_char().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SWFC_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn xon_threshold(&mut self) -> XON_THRESHOLD_W<0> {
        XON_THRESHOLD_W::new(self)
    }
    #[doc = "Bits 8:15 - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn xoff_threshold(&mut self) -> XOFF_THRESHOLD_W<8> {
        XOFF_THRESHOLD_W::new(self)
    }
    #[doc = "Bits 16:23 - This register stores the xon flow control char."]
    #[inline(always)]
    #[must_use]
    pub fn xon_char(&mut self) -> XON_CHAR_W<16> {
        XON_CHAR_W::new(self)
    }
    #[doc = "Bits 24:31 - This register stores the xoff flow control char."]
    #[inline(always)]
    #[must_use]
    pub fn xoff_char(&mut self) -> XOFF_CHAR_W<24> {
        XOFF_CHAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swfc_conf](index.html) module"]
pub struct SWFC_CONF_SPEC;
impl crate::RegisterSpec for SWFC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swfc_conf::R](R) reader structure"]
impl crate::Readable for SWFC_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swfc_conf::W](W) writer structure"]
impl crate::Writable for SWFC_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWFC_CONF to value 0x1311_e000"]
impl crate::Resettable for SWFC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x1311_e000;
}
