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
pub type BAUD_PRESC_R = crate::FieldReader<u16>;
#[doc = "Field `BAUD_PRESC` writer - Baud Rate Prescaler, determines the frequency dividing ratio."]
pub type BAUD_PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, BUS_TIMING_0_SPEC, 14, O, u16>;
#[doc = "Field `SYNC_JUMP_WIDTH` reader - Synchronization Jump Width (SJW), 1 \\verb+~+ 14 Tq wide."]
pub type SYNC_JUMP_WIDTH_R = crate::FieldReader;
#[doc = "Field `SYNC_JUMP_WIDTH` writer - Synchronization Jump Width (SJW), 1 \\verb+~+ 14 Tq wide."]
pub type SYNC_JUMP_WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, BUS_TIMING_0_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:13 - Baud Rate Prescaler, determines the frequency dividing ratio."]
    #[inline(always)]
    pub fn baud_presc(&self) -> BAUD_PRESC_R {
        BAUD_PRESC_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:15 - Synchronization Jump Width (SJW), 1 \\verb+~+ 14 Tq wide."]
    #[inline(always)]
    pub fn sync_jump_width(&self) -> SYNC_JUMP_WIDTH_R {
        SYNC_JUMP_WIDTH_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS_TIMING_0")
            .field("baud_presc", &format_args!("{}", self.baud_presc().bits()))
            .field(
                "sync_jump_width",
                &format_args!("{}", self.sync_jump_width().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUS_TIMING_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - Baud Rate Prescaler, determines the frequency dividing ratio."]
    #[inline(always)]
    #[must_use]
    pub fn baud_presc(&mut self) -> BAUD_PRESC_W<0> {
        BAUD_PRESC_W::new(self)
    }
    #[doc = "Bits 14:15 - Synchronization Jump Width (SJW), 1 \\verb+~+ 14 Tq wide."]
    #[inline(always)]
    #[must_use]
    pub fn sync_jump_width(&mut self) -> SYNC_JUMP_WIDTH_W<14> {
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUS_TIMING_0 to value 0"]
impl crate::Resettable for BUS_TIMING_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
