#[doc = "Register `QUICK_SENT` reader"]
pub struct R(crate::R<QUICK_SENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUICK_SENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUICK_SENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUICK_SENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QUICK_SENT` writer"]
pub struct W(crate::W<QUICK_SENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QUICK_SENT_SPEC>;
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
impl From<crate::W<QUICK_SENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QUICK_SENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SINGLE_SEND_NUM` reader - This register is used to specify the single_send register."]
pub type SINGLE_SEND_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SINGLE_SEND_NUM` writer - This register is used to specify the single_send register."]
pub type SINGLE_SEND_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QUICK_SENT_SPEC, u8, u8, 3, O>;
#[doc = "Field `SINGLE_SEND_EN` reader - Set this bit to enable single_send mode to send short packet."]
pub type SINGLE_SEND_EN_R = crate::BitReader<bool>;
#[doc = "Field `SINGLE_SEND_EN` writer - Set this bit to enable single_send mode to send short packet."]
pub type SINGLE_SEND_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUICK_SENT_SPEC, bool, O>;
#[doc = "Field `ALWAYS_SEND_NUM` reader - This register is used to specify the always_send register."]
pub type ALWAYS_SEND_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALWAYS_SEND_NUM` writer - This register is used to specify the always_send register."]
pub type ALWAYS_SEND_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QUICK_SENT_SPEC, u8, u8, 3, O>;
#[doc = "Field `ALWAYS_SEND_EN` reader - Set this bit to enable always_send mode to send short packet."]
pub type ALWAYS_SEND_EN_R = crate::BitReader<bool>;
#[doc = "Field `ALWAYS_SEND_EN` writer - Set this bit to enable always_send mode to send short packet."]
pub type ALWAYS_SEND_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, QUICK_SENT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - This register is used to specify the single_send register."]
    #[inline(always)]
    pub fn single_send_num(&self) -> SINGLE_SEND_NUM_R {
        SINGLE_SEND_NUM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Set this bit to enable single_send mode to send short packet."]
    #[inline(always)]
    pub fn single_send_en(&self) -> SINGLE_SEND_EN_R {
        SINGLE_SEND_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - This register is used to specify the always_send register."]
    #[inline(always)]
    pub fn always_send_num(&self) -> ALWAYS_SEND_NUM_R {
        ALWAYS_SEND_NUM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Set this bit to enable always_send mode to send short packet."]
    #[inline(always)]
    pub fn always_send_en(&self) -> ALWAYS_SEND_EN_R {
        ALWAYS_SEND_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - This register is used to specify the single_send register."]
    #[inline(always)]
    pub fn single_send_num(&mut self) -> SINGLE_SEND_NUM_W<0> {
        SINGLE_SEND_NUM_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to enable single_send mode to send short packet."]
    #[inline(always)]
    pub fn single_send_en(&mut self) -> SINGLE_SEND_EN_W<3> {
        SINGLE_SEND_EN_W::new(self)
    }
    #[doc = "Bits 4:6 - This register is used to specify the always_send register."]
    #[inline(always)]
    pub fn always_send_num(&mut self) -> ALWAYS_SEND_NUM_W<4> {
        ALWAYS_SEND_NUM_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to enable always_send mode to send short packet."]
    #[inline(always)]
    pub fn always_send_en(&mut self) -> ALWAYS_SEND_EN_W<7> {
        ALWAYS_SEND_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHCI quick send configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quick_sent](index.html) module"]
pub struct QUICK_SENT_SPEC;
impl crate::RegisterSpec for QUICK_SENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quick_sent::R](R) reader structure"]
impl crate::Readable for QUICK_SENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [quick_sent::W](W) writer structure"]
impl crate::Writable for QUICK_SENT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QUICK_SENT to value 0"]
impl crate::Resettable for QUICK_SENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
