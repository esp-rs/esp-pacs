#[doc = "Register `OUT_LINK_CH0` reader"]
pub struct R(crate::R<OUT_LINK_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_LINK_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_LINK_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_LINK_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_LINK_CH0` writer"]
pub struct W(crate::W<OUT_LINK_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_LINK_CH0_SPEC>;
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
impl From<crate::W<OUT_LINK_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_LINK_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTLINK_ADDR_CH0` reader - This register stores the 20 least significant bits of the first outlink descriptor's address."]
pub type OUTLINK_ADDR_CH0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OUTLINK_ADDR_CH0` writer - This register stores the 20 least significant bits of the first outlink descriptor's address."]
pub type OUTLINK_ADDR_CH0_W<'a> = crate::FieldWriter<'a, u32, OUT_LINK_CH0_SPEC, u32, u32, 20, 0>;
#[doc = "Field `OUTLINK_STOP_CH0` reader - Set this bit to stop dealing with the outlink descriptors."]
pub type OUTLINK_STOP_CH0_R = crate::BitReader<bool>;
#[doc = "Field `OUTLINK_STOP_CH0` writer - Set this bit to stop dealing with the outlink descriptors."]
pub type OUTLINK_STOP_CH0_W<'a> = crate::BitWriter<'a, u32, OUT_LINK_CH0_SPEC, bool, 20>;
#[doc = "Field `OUTLINK_START_CH0` reader - Set this bit to start dealing with the outlink descriptors."]
pub type OUTLINK_START_CH0_R = crate::BitReader<bool>;
#[doc = "Field `OUTLINK_START_CH0` writer - Set this bit to start dealing with the outlink descriptors."]
pub type OUTLINK_START_CH0_W<'a> = crate::BitWriter<'a, u32, OUT_LINK_CH0_SPEC, bool, 21>;
#[doc = "Field `OUTLINK_RESTART_CH0` reader - Set this bit to restart a new outlink from the last address."]
pub type OUTLINK_RESTART_CH0_R = crate::BitReader<bool>;
#[doc = "Field `OUTLINK_RESTART_CH0` writer - Set this bit to restart a new outlink from the last address."]
pub type OUTLINK_RESTART_CH0_W<'a> = crate::BitWriter<'a, u32, OUT_LINK_CH0_SPEC, bool, 22>;
#[doc = "Field `OUTLINK_PARK_CH0` reader - 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
pub type OUTLINK_PARK_CH0_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr_ch0(&self) -> OUTLINK_ADDR_CH0_R {
        OUTLINK_ADDR_CH0_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 20 - Set this bit to stop dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_stop_ch0(&self) -> OUTLINK_STOP_CH0_R {
        OUTLINK_STOP_CH0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to start dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_start_ch0(&self) -> OUTLINK_START_CH0_R {
        OUTLINK_START_CH0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to restart a new outlink from the last address."]
    #[inline(always)]
    pub fn outlink_restart_ch0(&self) -> OUTLINK_RESTART_CH0_R {
        OUTLINK_RESTART_CH0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn outlink_park_ch0(&self) -> OUTLINK_PARK_CH0_R {
        OUTLINK_PARK_CH0_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr_ch0(&mut self) -> OUTLINK_ADDR_CH0_W {
        OUTLINK_ADDR_CH0_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to stop dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_stop_ch0(&mut self) -> OUTLINK_STOP_CH0_W {
        OUTLINK_STOP_CH0_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to start dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_start_ch0(&mut self) -> OUTLINK_START_CH0_W {
        OUTLINK_START_CH0_W::new(self)
    }
    #[doc = "Bit 22 - Set this bit to restart a new outlink from the last address."]
    #[inline(always)]
    pub fn outlink_restart_ch0(&mut self) -> OUTLINK_RESTART_CH0_W {
        OUTLINK_RESTART_CH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_OUT_LINK_CH0_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_link_ch0](index.html) module"]
pub struct OUT_LINK_CH0_SPEC;
impl crate::RegisterSpec for OUT_LINK_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_link_ch0::R](R) reader structure"]
impl crate::Readable for OUT_LINK_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_link_ch0::W](W) writer structure"]
impl crate::Writable for OUT_LINK_CH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_LINK_CH0 to value 0x0080_0000"]
impl crate::Resettable for OUT_LINK_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0000
    }
}
