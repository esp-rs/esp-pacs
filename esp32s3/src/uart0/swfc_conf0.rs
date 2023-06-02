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
#[doc = "Field `XOFF_THRESHOLD` reader - When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1, it will send a Xoff char."]
pub type XOFF_THRESHOLD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XOFF_THRESHOLD` writer - When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1, it will send a Xoff char."]
pub type XOFF_THRESHOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, SWFC_CONF0_SPEC, 10, O, u16, u16>;
#[doc = "Field `XOFF_CHAR` reader - This register stores the Xoff flow control char."]
pub type XOFF_CHAR_R = crate::FieldReader;
#[doc = "Field `XOFF_CHAR` writer - This register stores the Xoff flow control char."]
pub type XOFF_CHAR_W<'a, const O: u8> = crate::FieldWriter<'a, SWFC_CONF0_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:9 - When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1, it will send a Xoff char."]
    #[inline(always)]
    pub fn xoff_threshold(&self) -> XOFF_THRESHOLD_R {
        XOFF_THRESHOLD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:17 - This register stores the Xoff flow control char."]
    #[inline(always)]
    pub fn xoff_char(&self) -> XOFF_CHAR_R {
        XOFF_CHAR_R::new(((self.bits >> 10) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWFC_CONF0")
            .field(
                "xoff_threshold",
                &format_args!("{}", self.xoff_threshold().bits()),
            )
            .field("xoff_char", &format_args!("{}", self.xoff_char().bits()))
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
    #[doc = "Bits 0:9 - When the data amount in Rx-FIFO is more than this register value with uart_sw_flow_con_en set to 1, it will send a Xoff char."]
    #[inline(always)]
    #[must_use]
    pub fn xoff_threshold(&mut self) -> XOFF_THRESHOLD_W<0> {
        XOFF_THRESHOLD_W::new(self)
    }
    #[doc = "Bits 10:17 - This register stores the Xoff flow control char."]
    #[inline(always)]
    #[must_use]
    pub fn xoff_char(&mut self) -> XOFF_CHAR_W<10> {
        XOFF_CHAR_W::new(self)
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
#[doc = "`reset()` method sets SWFC_CONF0 to value 0x4ce0"]
impl crate::Resettable for SWFC_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x4ce0;
}
