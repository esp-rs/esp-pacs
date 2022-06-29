#[doc = "Register `BUS_TIMING_0` reader"]
pub struct R(crate::R<BUS_TIMING_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_TIMING_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_TIMING_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_TIMING_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUS_TIMING_0` writer"]
pub struct W(crate::W<BUS_TIMING_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUS_TIMING_0_SPEC>;
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
impl From<crate::W<BUS_TIMING_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUS_TIMING_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUD_PRESC` reader - Baud Rate Prescaler, determines the frequency dividing ratio."]
pub type BAUD_PRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BAUD_PRESC` writer - Baud Rate Prescaler, determines the frequency dividing ratio."]
pub type BAUD_PRESC_W<'a> = crate::FieldWriter<'a, u32, BUS_TIMING_0_SPEC, u8, u8, 6, 0>;
#[doc = "Field `SYNC_JUMP_WIDTH` reader - Synchronization Jump Width (SJW), 1 \\verb+~+ 14 Tq wide."]
pub type SYNC_JUMP_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNC_JUMP_WIDTH` writer - Synchronization Jump Width (SJW), 1 \\verb+~+ 14 Tq wide."]
pub type SYNC_JUMP_WIDTH_W<'a> = crate::FieldWriter<'a, u32, BUS_TIMING_0_SPEC, u8, u8, 2, 6>;
impl R {
    #[doc = "Bits 0:5 - Baud Rate Prescaler, determines the frequency dividing ratio."]
    #[inline(always)]
    pub fn baud_presc(&self) -> BAUD_PRESC_R {
        BAUD_PRESC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Synchronization Jump Width (SJW), 1 \\verb+~+ 14 Tq wide."]
    #[inline(always)]
    pub fn sync_jump_width(&self) -> SYNC_JUMP_WIDTH_R {
        SYNC_JUMP_WIDTH_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud Rate Prescaler, determines the frequency dividing ratio."]
    #[inline(always)]
    pub fn baud_presc(&mut self) -> BAUD_PRESC_W {
        BAUD_PRESC_W::new(self)
    }
    #[doc = "Bits 6:7 - Synchronization Jump Width (SJW), 1 \\verb+~+ 14 Tq wide."]
    #[inline(always)]
    pub fn sync_jump_width(&mut self) -> SYNC_JUMP_WIDTH_W {
        SYNC_JUMP_WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Timing Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_timing_0](index.html) module"]
pub struct BUS_TIMING_0_SPEC;
impl crate::RegisterSpec for BUS_TIMING_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_timing_0::R](R) reader structure"]
impl crate::Readable for BUS_TIMING_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bus_timing_0::W](W) writer structure"]
impl crate::Writable for BUS_TIMING_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUS_TIMING_0 to value 0"]
impl crate::Resettable for BUS_TIMING_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
