#[doc = "Register `PIN23` reader"]
pub struct R(crate::R<PIN23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN23` writer"]
pub struct W(crate::W<PIN23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN23_SPEC>;
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
impl From<crate::W<PIN23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNC2_BYPASS` reader - set GPIO input_sync2 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
pub type SYNC2_BYPASS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNC2_BYPASS` writer - set GPIO input_sync2 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
pub type SYNC2_BYPASS_W<'a> = crate::FieldWriter<'a, u32, PIN23_SPEC, u8, u8, 2, 0>;
#[doc = "Field `PAD_DRIVER` reader - set this bit to select pad driver. 1:open-drain. 0:normal."]
pub type PAD_DRIVER_R = crate::BitReader<bool>;
#[doc = "Field `PAD_DRIVER` writer - set this bit to select pad driver. 1:open-drain. 0:normal."]
pub type PAD_DRIVER_W<'a> = crate::BitWriter<'a, u32, PIN23_SPEC, bool, 2>;
#[doc = "Field `SYNC1_BYPASS` reader - set GPIO input_sync1 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
pub type SYNC1_BYPASS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNC1_BYPASS` writer - set GPIO input_sync1 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
pub type SYNC1_BYPASS_W<'a> = crate::FieldWriter<'a, u32, PIN23_SPEC, u8, u8, 2, 3>;
#[doc = "Field `INT_TYPE` reader - set this value to choose interrupt mode. 0:disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
pub type INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_TYPE` writer - set this value to choose interrupt mode. 0:disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
pub type INT_TYPE_W<'a> = crate::FieldWriter<'a, u32, PIN23_SPEC, u8, u8, 3, 7>;
#[doc = "Field `WAKEUP_ENABLE` reader - set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
pub type WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUP_ENABLE` writer - set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
pub type WAKEUP_ENABLE_W<'a> = crate::BitWriter<'a, u32, PIN23_SPEC, bool, 10>;
#[doc = "Field `CONFIG` reader - reserved"]
pub type CONFIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONFIG` writer - reserved"]
pub type CONFIG_W<'a> = crate::FieldWriter<'a, u32, PIN23_SPEC, u8, u8, 2, 11>;
#[doc = "Field `INT_ENA` reader - set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
pub type INT_ENA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_ENA` writer - set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
pub type INT_ENA_W<'a> = crate::FieldWriter<'a, u32, PIN23_SPEC, u8, u8, 5, 13>;
impl R {
    #[doc = "Bits 0:1 - set GPIO input_sync2 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    #[inline(always)]
    pub fn sync2_bypass(&self) -> SYNC2_BYPASS_R {
        SYNC2_BYPASS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - set this bit to select pad driver. 1:open-drain. 0:normal."]
    #[inline(always)]
    pub fn pad_driver(&self) -> PAD_DRIVER_R {
        PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - set GPIO input_sync1 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    #[inline(always)]
    pub fn sync1_bypass(&self) -> SYNC1_BYPASS_R {
        SYNC1_BYPASS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 7:9 - set this value to choose interrupt mode. 0:disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
    #[inline(always)]
    pub fn int_type(&self) -> INT_TYPE_R {
        INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
    #[inline(always)]
    pub fn wakeup_enable(&self) -> WAKEUP_ENABLE_R {
        WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - reserved"]
    #[inline(always)]
    pub fn config(&self) -> CONFIG_R {
        CONFIG_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:17 - set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
    #[inline(always)]
    pub fn int_ena(&self) -> INT_ENA_R {
        INT_ENA_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - set GPIO input_sync2 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    #[inline(always)]
    pub fn sync2_bypass(&mut self) -> SYNC2_BYPASS_W {
        SYNC2_BYPASS_W::new(self)
    }
    #[doc = "Bit 2 - set this bit to select pad driver. 1:open-drain. 0:normal."]
    #[inline(always)]
    pub fn pad_driver(&mut self) -> PAD_DRIVER_W {
        PAD_DRIVER_W::new(self)
    }
    #[doc = "Bits 3:4 - set GPIO input_sync1 signal mode. 0:disable. 1:trigger at negedge. 2or3:trigger at posedge."]
    #[inline(always)]
    pub fn sync1_bypass(&mut self) -> SYNC1_BYPASS_W {
        SYNC1_BYPASS_W::new(self)
    }
    #[doc = "Bits 7:9 - set this value to choose interrupt mode. 0:disable GPIO interrupt. 1:trigger at posedge. 2:trigger at negedge. 3:trigger at any edge. 4:valid at low level. 5:valid at high level"]
    #[inline(always)]
    pub fn int_type(&mut self) -> INT_TYPE_W {
        INT_TYPE_W::new(self)
    }
    #[doc = "Bit 10 - set this bit to enable GPIO wakeup.(can only wakeup CPU from Light-sleep Mode)"]
    #[inline(always)]
    pub fn wakeup_enable(&mut self) -> WAKEUP_ENABLE_W {
        WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bits 11:12 - reserved"]
    #[inline(always)]
    pub fn config(&mut self) -> CONFIG_W {
        CONFIG_W::new(self)
    }
    #[doc = "Bits 13:17 - set bit 13 to enable CPU interrupt. set bit 14 to enable CPU(not shielded) interrupt."]
    #[inline(always)]
    pub fn int_ena(&mut self) -> INT_ENA_W {
        INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO pin configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin23](index.html) module"]
pub struct PIN23_SPEC;
impl crate::RegisterSpec for PIN23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin23::R](R) reader structure"]
impl crate::Readable for PIN23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin23::W](W) writer structure"]
impl crate::Writable for PIN23_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIN23 to value 0"]
impl crate::Resettable for PIN23_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
