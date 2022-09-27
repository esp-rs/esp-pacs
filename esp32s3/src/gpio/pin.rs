#[doc = "Register `PIN%s` reader"]
pub struct R(crate::R<PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN%s` writer"]
pub struct W(crate::W<PIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_SPEC>;
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
impl From<crate::W<PIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN_SYNC2_BYPASS` reader - set GPIO input_sync2 signal mode. :disable. 1:trigger at negedge. 2or3:trigger at posedge."]
pub type PIN_SYNC2_BYPASS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN_SYNC2_BYPASS` writer - set GPIO input_sync2 signal mode. :disable. 1:trigger at negedge. 2or3:trigger at posedge."]
pub type PIN_SYNC2_BYPASS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIN_SPEC, u8, u8, 2, O>;
#[doc = "Field `PIN_PAD_DRIVER` reader - set this bit to select pad driver. 1:open-drain. :normal."]
pub type PIN_PAD_DRIVER_R = crate::BitReader<bool>;
#[doc = "Field `PIN_PAD_DRIVER` writer - set this bit to select pad driver. 1:open-drain. :normal."]
pub type PIN_PAD_DRIVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PIN_SYNC1_BYPASS` reader - set GPIO input_sync1 signal mode. :disable. 1:trigger at negedge. 2or3:trigger at posedge."]
pub type PIN_SYNC1_BYPASS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN_SYNC1_BYPASS` writer - set GPIO input_sync1 signal mode. :disable. 1:trigger at negedge. 2or3:trigger at posedge."]
pub type PIN_SYNC1_BYPASS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIN_SPEC, u8, u8, 2, O>;
#[doc = "Field `PIN_INT_TYPE` reader - set this value to choose interrupt mode. :disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
pub type PIN_INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN_INT_TYPE` writer - set this value to choose interrupt mode. :disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
pub type PIN_INT_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIN_SPEC, u8, u8, 3, O>;
#[doc = "Field `PIN_WAKEUP_ENABLE` reader - set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
pub type PIN_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PIN_WAKEUP_ENABLE` writer - set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
pub type PIN_WAKEUP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_SPEC, bool, O>;
#[doc = "Field `PIN_CONFIG` reader - reserved"]
pub type PIN_CONFIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN_CONFIG` writer - reserved"]
pub type PIN_CONFIG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIN_SPEC, u8, u8, 2, O>;
#[doc = "Field `PIN_INT_ENA` reader - set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
pub type PIN_INT_ENA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN_INT_ENA` writer - set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
pub type PIN_INT_ENA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIN_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:1 - set GPIO input_sync2 signal mode. :disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    #[inline(always)]
    pub fn pin_sync2_bypass(&self) -> PIN_SYNC2_BYPASS_R {
        PIN_SYNC2_BYPASS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - set this bit to select pad driver. 1:open-drain. :normal."]
    #[inline(always)]
    pub fn pin_pad_driver(&self) -> PIN_PAD_DRIVER_R {
        PIN_PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - set GPIO input_sync1 signal mode. :disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    #[inline(always)]
    pub fn pin_sync1_bypass(&self) -> PIN_SYNC1_BYPASS_R {
        PIN_SYNC1_BYPASS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 7:9 - set this value to choose interrupt mode. :disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
    #[inline(always)]
    pub fn pin_int_type(&self) -> PIN_INT_TYPE_R {
        PIN_INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
    #[inline(always)]
    pub fn pin_wakeup_enable(&self) -> PIN_WAKEUP_ENABLE_R {
        PIN_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - reserved"]
    #[inline(always)]
    pub fn pin_config(&self) -> PIN_CONFIG_R {
        PIN_CONFIG_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:17 - set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
    #[inline(always)]
    pub fn pin_int_ena(&self) -> PIN_INT_ENA_R {
        PIN_INT_ENA_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - set GPIO input_sync2 signal mode. :disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    #[inline(always)]
    pub fn pin_sync2_bypass(&mut self) -> PIN_SYNC2_BYPASS_W<0> {
        PIN_SYNC2_BYPASS_W::new(self)
    }
    #[doc = "Bit 2 - set this bit to select pad driver. 1:open-drain. :normal."]
    #[inline(always)]
    pub fn pin_pad_driver(&mut self) -> PIN_PAD_DRIVER_W<2> {
        PIN_PAD_DRIVER_W::new(self)
    }
    #[doc = "Bits 3:4 - set GPIO input_sync1 signal mode. :disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    #[inline(always)]
    pub fn pin_sync1_bypass(&mut self) -> PIN_SYNC1_BYPASS_W<3> {
        PIN_SYNC1_BYPASS_W::new(self)
    }
    #[doc = "Bits 7:9 - set this value to choose interrupt mode. :disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
    #[inline(always)]
    pub fn pin_int_type(&mut self) -> PIN_INT_TYPE_W<7> {
        PIN_INT_TYPE_W::new(self)
    }
    #[doc = "Bit 10 - set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
    #[inline(always)]
    pub fn pin_wakeup_enable(&mut self) -> PIN_WAKEUP_ENABLE_W<10> {
        PIN_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bits 11:12 - reserved"]
    #[inline(always)]
    pub fn pin_config(&mut self) -> PIN_CONFIG_W<11> {
        PIN_CONFIG_W::new(self)
    }
    #[doc = "Bits 13:17 - set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
    #[inline(always)]
    pub fn pin_int_ena(&mut self) -> PIN_INT_ENA_W<13> {
        PIN_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO pin configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](index.html) module"]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin::R](R) reader structure"]
impl crate::Readable for PIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin::W](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIN%s to value 0"]
impl crate::Resettable for PIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
