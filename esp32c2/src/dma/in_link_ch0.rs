#[doc = "Register `IN_LINK_CH0` reader"]
pub struct R(crate::R<IN_LINK_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_LINK_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_LINK_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_LINK_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_LINK_CH0` writer"]
pub struct W(crate::W<IN_LINK_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_LINK_CH0_SPEC>;
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
impl From<crate::W<IN_LINK_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_LINK_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INLINK_ADDR_CH0` reader - This register stores the 20 least significant bits of the first inlink descriptor's address."]
pub type INLINK_ADDR_CH0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INLINK_ADDR_CH0` writer - This register stores the 20 least significant bits of the first inlink descriptor's address."]
pub type INLINK_ADDR_CH0_W<'a> = crate::FieldWriter<'a, u32, IN_LINK_CH0_SPEC, u32, u32, 20, 0>;
#[doc = "Field `INLINK_AUTO_RET_CH0` reader - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
pub type INLINK_AUTO_RET_CH0_R = crate::BitReader<bool>;
#[doc = "Field `INLINK_AUTO_RET_CH0` writer - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
pub type INLINK_AUTO_RET_CH0_W<'a> = crate::BitWriter<'a, u32, IN_LINK_CH0_SPEC, bool, 20>;
#[doc = "Field `INLINK_STOP_CH0` reader - Set this bit to stop dealing with the inlink descriptors."]
pub type INLINK_STOP_CH0_R = crate::BitReader<bool>;
#[doc = "Field `INLINK_STOP_CH0` writer - Set this bit to stop dealing with the inlink descriptors."]
pub type INLINK_STOP_CH0_W<'a> = crate::BitWriter<'a, u32, IN_LINK_CH0_SPEC, bool, 21>;
#[doc = "Field `INLINK_START_CH0` reader - Set this bit to start dealing with the inlink descriptors."]
pub type INLINK_START_CH0_R = crate::BitReader<bool>;
#[doc = "Field `INLINK_START_CH0` writer - Set this bit to start dealing with the inlink descriptors."]
pub type INLINK_START_CH0_W<'a> = crate::BitWriter<'a, u32, IN_LINK_CH0_SPEC, bool, 22>;
#[doc = "Field `INLINK_RESTART_CH0` reader - Set this bit to mount a new inlink descriptor."]
pub type INLINK_RESTART_CH0_R = crate::BitReader<bool>;
#[doc = "Field `INLINK_RESTART_CH0` writer - Set this bit to mount a new inlink descriptor."]
pub type INLINK_RESTART_CH0_W<'a> = crate::BitWriter<'a, u32, IN_LINK_CH0_SPEC, bool, 23>;
#[doc = "Field `INLINK_PARK_CH0` reader - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
pub type INLINK_PARK_CH0_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_addr_ch0(&self) -> INLINK_ADDR_CH0_R {
        INLINK_ADDR_CH0_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 20 - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn inlink_auto_ret_ch0(&self) -> INLINK_AUTO_RET_CH0_R {
        INLINK_AUTO_RET_CH0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to stop dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_stop_ch0(&self) -> INLINK_STOP_CH0_R {
        INLINK_STOP_CH0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to start dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_start_ch0(&self) -> INLINK_START_CH0_R {
        INLINK_START_CH0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to mount a new inlink descriptor."]
    #[inline(always)]
    pub fn inlink_restart_ch0(&self) -> INLINK_RESTART_CH0_R {
        INLINK_RESTART_CH0_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn inlink_park_ch0(&self) -> INLINK_PARK_CH0_R {
        INLINK_PARK_CH0_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_addr_ch0(&mut self) -> INLINK_ADDR_CH0_W {
        INLINK_ADDR_CH0_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn inlink_auto_ret_ch0(&mut self) -> INLINK_AUTO_RET_CH0_W {
        INLINK_AUTO_RET_CH0_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to stop dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_stop_ch0(&mut self) -> INLINK_STOP_CH0_W {
        INLINK_STOP_CH0_W::new(self)
    }
    #[doc = "Bit 22 - Set this bit to start dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_start_ch0(&mut self) -> INLINK_START_CH0_W {
        INLINK_START_CH0_W::new(self)
    }
    #[doc = "Bit 23 - Set this bit to mount a new inlink descriptor."]
    #[inline(always)]
    pub fn inlink_restart_ch0(&mut self) -> INLINK_RESTART_CH0_W {
        INLINK_RESTART_CH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IN_LINK_CH0_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_link_ch0](index.html) module"]
pub struct IN_LINK_CH0_SPEC;
impl crate::RegisterSpec for IN_LINK_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_link_ch0::R](R) reader structure"]
impl crate::Readable for IN_LINK_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_link_ch0::W](W) writer structure"]
impl crate::Writable for IN_LINK_CH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_LINK_CH0 to value 0x0110_0000"]
impl crate::Resettable for IN_LINK_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0110_0000
    }
}
