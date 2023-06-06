#[doc = "Register `OUT_LINK_CH2` reader"]
pub struct R(crate::R<OUT_LINK_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_LINK_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_LINK_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_LINK_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_LINK_CH2` writer"]
pub struct W(crate::W<OUT_LINK_CH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_LINK_CH2_SPEC>;
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
impl From<crate::W<OUT_LINK_CH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_LINK_CH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTLINK_ADDR` reader - This register stores the 20 least significant bits of the first outlink descriptor's address."]
pub type OUTLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `OUTLINK_ADDR` writer - This register stores the 20 least significant bits of the first outlink descriptor's address."]
pub type OUTLINK_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, OUT_LINK_CH2_SPEC, 20, O, u32>;
#[doc = "Field `OUTLINK_STOP` reader - Set this bit to stop dealing with the outlink descriptors."]
pub type OUTLINK_STOP_R = crate::BitReader;
#[doc = "Field `OUTLINK_STOP` writer - Set this bit to stop dealing with the outlink descriptors."]
pub type OUTLINK_STOP_W<'a, const O: u8> = crate::BitWriter<'a, OUT_LINK_CH2_SPEC, O>;
#[doc = "Field `OUTLINK_START` reader - Set this bit to start dealing with the outlink descriptors."]
pub type OUTLINK_START_R = crate::BitReader;
#[doc = "Field `OUTLINK_START` writer - Set this bit to start dealing with the outlink descriptors."]
pub type OUTLINK_START_W<'a, const O: u8> = crate::BitWriter<'a, OUT_LINK_CH2_SPEC, O>;
#[doc = "Field `OUTLINK_RESTART` reader - Set this bit to restart a new outlink from the last address."]
pub type OUTLINK_RESTART_R = crate::BitReader;
#[doc = "Field `OUTLINK_RESTART` writer - Set this bit to restart a new outlink from the last address."]
pub type OUTLINK_RESTART_W<'a, const O: u8> = crate::BitWriter<'a, OUT_LINK_CH2_SPEC, O>;
#[doc = "Field `OUTLINK_PARK` reader - 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
pub type OUTLINK_PARK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr(&self) -> OUTLINK_ADDR_R {
        OUTLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20 - Set this bit to stop dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_stop(&self) -> OUTLINK_STOP_R {
        OUTLINK_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to start dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_start(&self) -> OUTLINK_START_R {
        OUTLINK_START_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to restart a new outlink from the last address."]
    #[inline(always)]
    pub fn outlink_restart(&self) -> OUTLINK_RESTART_R {
        OUTLINK_RESTART_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn outlink_park(&self) -> OUTLINK_PARK_R {
        OUTLINK_PARK_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_LINK_CH2")
            .field(
                "outlink_addr",
                &format_args!("{}", self.outlink_addr().bits()),
            )
            .field(
                "outlink_stop",
                &format_args!("{}", self.outlink_stop().bit()),
            )
            .field(
                "outlink_start",
                &format_args!("{}", self.outlink_start().bit()),
            )
            .field(
                "outlink_restart",
                &format_args!("{}", self.outlink_restart().bit()),
            )
            .field(
                "outlink_park",
                &format_args!("{}", self.outlink_park().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_LINK_CH2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first outlink descriptor's address."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_addr(&mut self) -> OUTLINK_ADDR_W<0> {
        OUTLINK_ADDR_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to stop dealing with the outlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_stop(&mut self) -> OUTLINK_STOP_W<20> {
        OUTLINK_STOP_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to start dealing with the outlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_start(&mut self) -> OUTLINK_START_W<21> {
        OUTLINK_START_W::new(self)
    }
    #[doc = "Bit 22 - Set this bit to restart a new outlink from the last address."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_restart(&mut self) -> OUTLINK_RESTART_W<22> {
        OUTLINK_RESTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_OUT_LINK_CH2_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_link_ch2](index.html) module"]
pub struct OUT_LINK_CH2_SPEC;
impl crate::RegisterSpec for OUT_LINK_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_link_ch2::R](R) reader structure"]
impl crate::Readable for OUT_LINK_CH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_link_ch2::W](W) writer structure"]
impl crate::Writable for OUT_LINK_CH2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_LINK_CH2 to value 0x0080_0000"]
impl crate::Resettable for OUT_LINK_CH2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0080_0000;
}
