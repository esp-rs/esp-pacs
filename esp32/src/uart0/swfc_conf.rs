#[doc = "Register `SWFC_CONF` reader"]
pub type R = crate::R<SWFC_CONF_SPEC>;
#[doc = "Register `SWFC_CONF` writer"]
pub type W = crate::W<SWFC_CONF_SPEC>;
#[doc = "Field `XON_THRESHOLD` reader - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
pub type XON_THRESHOLD_R = crate::FieldReader;
#[doc = "Field `XON_THRESHOLD` writer - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
pub type XON_THRESHOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `XOFF_THRESHOLD` reader - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
pub type XOFF_THRESHOLD_R = crate::FieldReader;
#[doc = "Field `XOFF_THRESHOLD` writer - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
pub type XOFF_THRESHOLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `XON_CHAR` reader - This register stores the xon flow control char."]
pub type XON_CHAR_R = crate::FieldReader;
#[doc = "Field `XON_CHAR` writer - This register stores the xon flow control char."]
pub type XON_CHAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `XOFF_CHAR` reader - This register stores the xoff flow control char."]
pub type XOFF_CHAR_R = crate::FieldReader;
#[doc = "Field `XOFF_CHAR` writer - This register stores the xoff flow control char."]
pub type XOFF_CHAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
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
    pub fn xon_threshold(&mut self) -> XON_THRESHOLD_W<SWFC_CONF_SPEC, 0> {
        XON_THRESHOLD_W::new(self)
    }
    #[doc = "Bits 8:15 - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn xoff_threshold(&mut self) -> XOFF_THRESHOLD_W<SWFC_CONF_SPEC, 8> {
        XOFF_THRESHOLD_W::new(self)
    }
    #[doc = "Bits 16:23 - This register stores the xon flow control char."]
    #[inline(always)]
    #[must_use]
    pub fn xon_char(&mut self) -> XON_CHAR_W<SWFC_CONF_SPEC, 16> {
        XON_CHAR_W::new(self)
    }
    #[doc = "Bits 24:31 - This register stores the xoff flow control char."]
    #[inline(always)]
    #[must_use]
    pub fn xoff_char(&mut self) -> XOFF_CHAR_W<SWFC_CONF_SPEC, 24> {
        XOFF_CHAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swfc_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swfc_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWFC_CONF_SPEC;
impl crate::RegisterSpec for SWFC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swfc_conf::R`](R) reader structure"]
impl crate::Readable for SWFC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swfc_conf::W`](W) writer structure"]
impl crate::Writable for SWFC_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWFC_CONF to value 0x1311_e000"]
impl crate::Resettable for SWFC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x1311_e000;
}
