#[doc = "Register `IN_LINK_CH2` reader"]
pub struct R(crate::R<IN_LINK_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_LINK_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_LINK_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_LINK_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_LINK_CH2` writer"]
pub struct W(crate::W<IN_LINK_CH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_LINK_CH2_SPEC>;
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
impl From<crate::W<IN_LINK_CH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_LINK_CH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INLINK_ADDR_CH2` reader - This register stores the 20 least significant bits of the first inlink descriptor's address."]
pub type INLINK_ADDR_CH2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INLINK_ADDR_CH2` writer - This register stores the 20 least significant bits of the first inlink descriptor's address."]
pub type INLINK_ADDR_CH2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IN_LINK_CH2_SPEC, u32, u32, 20, O>;
#[doc = "Field `INLINK_AUTO_RET_CH2` reader - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
pub type INLINK_AUTO_RET_CH2_R = crate::BitReader<bool>;
#[doc = "Field `INLINK_AUTO_RET_CH2` writer - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
pub type INLINK_AUTO_RET_CH2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IN_LINK_CH2_SPEC, bool, O>;
#[doc = "Field `INLINK_STOP_CH2` reader - Set this bit to stop dealing with the inlink descriptors."]
pub type INLINK_STOP_CH2_R = crate::BitReader<bool>;
#[doc = "Field `INLINK_STOP_CH2` writer - Set this bit to stop dealing with the inlink descriptors."]
pub type INLINK_STOP_CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IN_LINK_CH2_SPEC, bool, O>;
#[doc = "Field `INLINK_START_CH2` reader - Set this bit to start dealing with the inlink descriptors."]
pub type INLINK_START_CH2_R = crate::BitReader<bool>;
#[doc = "Field `INLINK_START_CH2` writer - Set this bit to start dealing with the inlink descriptors."]
pub type INLINK_START_CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IN_LINK_CH2_SPEC, bool, O>;
#[doc = "Field `INLINK_RESTART_CH2` reader - Set this bit to mount a new inlink descriptor."]
pub type INLINK_RESTART_CH2_R = crate::BitReader<bool>;
#[doc = "Field `INLINK_RESTART_CH2` writer - Set this bit to mount a new inlink descriptor."]
pub type INLINK_RESTART_CH2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IN_LINK_CH2_SPEC, bool, O>;
#[doc = "Field `INLINK_PARK_CH2` reader - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
pub type INLINK_PARK_CH2_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_addr_ch2(&self) -> INLINK_ADDR_CH2_R {
        INLINK_ADDR_CH2_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 20 - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn inlink_auto_ret_ch2(&self) -> INLINK_AUTO_RET_CH2_R {
        INLINK_AUTO_RET_CH2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to stop dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_stop_ch2(&self) -> INLINK_STOP_CH2_R {
        INLINK_STOP_CH2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to start dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_start_ch2(&self) -> INLINK_START_CH2_R {
        INLINK_START_CH2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to mount a new inlink descriptor."]
    #[inline(always)]
    pub fn inlink_restart_ch2(&self) -> INLINK_RESTART_CH2_R {
        INLINK_RESTART_CH2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn inlink_park_ch2(&self) -> INLINK_PARK_CH2_R {
        INLINK_PARK_CH2_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_addr_ch2(&mut self) -> INLINK_ADDR_CH2_W<0> {
        INLINK_ADDR_CH2_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn inlink_auto_ret_ch2(&mut self) -> INLINK_AUTO_RET_CH2_W<20> {
        INLINK_AUTO_RET_CH2_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to stop dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_stop_ch2(&mut self) -> INLINK_STOP_CH2_W<21> {
        INLINK_STOP_CH2_W::new(self)
    }
    #[doc = "Bit 22 - Set this bit to start dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_start_ch2(&mut self) -> INLINK_START_CH2_W<22> {
        INLINK_START_CH2_W::new(self)
    }
    #[doc = "Bit 23 - Set this bit to mount a new inlink descriptor."]
    #[inline(always)]
    pub fn inlink_restart_ch2(&mut self) -> INLINK_RESTART_CH2_W<23> {
        INLINK_RESTART_CH2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IN_LINK_CH2_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_link_ch2](index.html) module"]
pub struct IN_LINK_CH2_SPEC;
impl crate::RegisterSpec for IN_LINK_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_link_ch2::R](R) reader structure"]
impl crate::Readable for IN_LINK_CH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_link_ch2::W](W) writer structure"]
impl crate::Writable for IN_LINK_CH2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_LINK_CH2 to value 0x0110_0000"]
impl crate::Resettable for IN_LINK_CH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0110_0000
    }
}
