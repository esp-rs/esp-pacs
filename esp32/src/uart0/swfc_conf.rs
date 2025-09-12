#[doc = "Register `SWFC_CONF` reader"]
pub type R = crate::R<SWFC_CONF_SPEC>;
#[doc = "Register `SWFC_CONF` writer"]
pub type W = crate::W<SWFC_CONF_SPEC>;
#[doc = "Field `XON_THRESHOLD` reader - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
pub type XON_THRESHOLD_R = crate::FieldReader;
#[doc = "Field `XON_THRESHOLD` writer - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
pub type XON_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `XOFF_THRESHOLD` reader - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
pub type XOFF_THRESHOLD_R = crate::FieldReader;
#[doc = "Field `XOFF_THRESHOLD` writer - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
pub type XOFF_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `XON_CHAR` reader - This register stores the xon flow control char."]
pub type XON_CHAR_R = crate::FieldReader;
#[doc = "Field `XON_CHAR` writer - This register stores the xon flow control char."]
pub type XON_CHAR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `XOFF_CHAR` reader - This register stores the xoff flow control char."]
pub type XOFF_CHAR_R = crate::FieldReader;
#[doc = "Field `XOFF_CHAR` writer - This register stores the xoff flow control char."]
pub type XOFF_CHAR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
            .field("xon_threshold", &self.xon_threshold())
            .field("xoff_threshold", &self.xoff_threshold())
            .field("xon_char", &self.xon_char())
            .field("xoff_char", &self.xoff_char())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - when the data amount in receiver's fifo is more than this register value. it will send a xoff char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    pub fn xon_threshold(&mut self) -> XON_THRESHOLD_W<'_, SWFC_CONF_SPEC> {
        XON_THRESHOLD_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - When the data amount in receiver's fifo is less than this register value. it will send a xon char with uart_sw_flow_con_en set to 1."]
    #[inline(always)]
    pub fn xoff_threshold(&mut self) -> XOFF_THRESHOLD_W<'_, SWFC_CONF_SPEC> {
        XOFF_THRESHOLD_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - This register stores the xon flow control char."]
    #[inline(always)]
    pub fn xon_char(&mut self) -> XON_CHAR_W<'_, SWFC_CONF_SPEC> {
        XON_CHAR_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - This register stores the xoff flow control char."]
    #[inline(always)]
    pub fn xoff_char(&mut self) -> XOFF_CHAR_W<'_, SWFC_CONF_SPEC> {
        XOFF_CHAR_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`swfc_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swfc_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWFC_CONF_SPEC;
impl crate::RegisterSpec for SWFC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swfc_conf::R`](R) reader structure"]
impl crate::Readable for SWFC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swfc_conf::W`](W) writer structure"]
impl crate::Writable for SWFC_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWFC_CONF to value 0x1311_e000"]
impl crate::Resettable for SWFC_CONF_SPEC {
    const RESET_VALUE: u32 = 0x1311_e000;
}
