#[doc = "Register `IN_LINK_CH%s` reader"]
pub struct R(crate::R<IN_LINK_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_LINK_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_LINK_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_LINK_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_LINK_CH%s` writer"]
pub struct W(crate::W<IN_LINK_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_LINK_CH_SPEC>;
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
impl From<crate::W<IN_LINK_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_LINK_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INLINK_ADDR` reader - This register stores the 20 least significant bits of the first inlink descriptor's address."]
pub type INLINK_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INLINK_ADDR` writer - This register stores the 20 least significant bits of the first inlink descriptor's address."]
pub type INLINK_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, IN_LINK_CH_SPEC, 20, O, u32, u32>;
#[doc = "Field `INLINK_AUTO_RET` reader - Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data."]
pub type INLINK_AUTO_RET_R = crate::BitReader;
#[doc = "Field `INLINK_AUTO_RET` writer - Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data."]
pub type INLINK_AUTO_RET_W<'a, const O: u8> = crate::BitWriter<'a, IN_LINK_CH_SPEC, O>;
#[doc = "Field `INLINK_STOP` writer - Set this bit to stop dealing with the inlink descriptors."]
pub type INLINK_STOP_W<'a, const O: u8> = crate::BitWriter<'a, IN_LINK_CH_SPEC, O>;
#[doc = "Field `INLINK_START` writer - Set this bit to start dealing with the inlink descriptors."]
pub type INLINK_START_W<'a, const O: u8> = crate::BitWriter<'a, IN_LINK_CH_SPEC, O>;
#[doc = "Field `INLINK_RESTART` writer - Set this bit to mount a new inlink descriptor."]
pub type INLINK_RESTART_W<'a, const O: u8> = crate::BitWriter<'a, IN_LINK_CH_SPEC, O>;
#[doc = "Field `INLINK_PARK` reader - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
pub type INLINK_PARK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_addr(&self) -> INLINK_ADDR_R {
        INLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20 - Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn inlink_auto_ret(&self) -> INLINK_AUTO_RET_R {
        INLINK_AUTO_RET_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn inlink_park(&self) -> INLINK_PARK_R {
        INLINK_PARK_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_LINK_CH")
            .field(
                "inlink_addr",
                &format_args!("{}", self.inlink_addr().bits()),
            )
            .field(
                "inlink_auto_ret",
                &format_args!("{}", self.inlink_auto_ret().bit()),
            )
            .field("inlink_park", &format_args!("{}", self.inlink_park().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_LINK_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first inlink descriptor's address."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_addr(&mut self) -> INLINK_ADDR_W<0> {
        INLINK_ADDR_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_auto_ret(&mut self) -> INLINK_AUTO_RET_W<20> {
        INLINK_AUTO_RET_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to stop dealing with the inlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_stop(&mut self) -> INLINK_STOP_W<21> {
        INLINK_STOP_W::new(self)
    }
    #[doc = "Bit 22 - Set this bit to start dealing with the inlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_start(&mut self) -> INLINK_START_W<22> {
        INLINK_START_W::new(self)
    }
    #[doc = "Bit 23 - Set this bit to mount a new inlink descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_restart(&mut self) -> INLINK_RESTART_W<23> {
        INLINK_RESTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Link descriptor configure and control register of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_link_ch](index.html) module"]
pub struct IN_LINK_CH_SPEC;
impl crate::RegisterSpec for IN_LINK_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_link_ch::R](R) reader structure"]
impl crate::Readable for IN_LINK_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_link_ch::W](W) writer structure"]
impl crate::Writable for IN_LINK_CH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_LINK_CH%s to value 0x0110_0000"]
impl crate::Resettable for IN_LINK_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0x0110_0000;
}
