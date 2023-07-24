#[doc = "Register `ENSHIFT` reader"]
pub struct R(crate::R<ENSHIFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENSHIFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENSHIFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENSHIFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENSHIFT` writer"]
pub struct W(crate::W<ENSHIFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENSHIFT_SPEC>;
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
impl From<crate::W<ENSHIFT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENSHIFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_SHIFT` reader - Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card. 2'b00-Default phase shift. 2'b01-Enables shifted to next immediate positive edge. 2'b10-Enables shifted to next immediate negative edge. 2'b11-Reserved."]
pub type ENABLE_SHIFT_R = crate::FieldReader;
#[doc = "Field `ENABLE_SHIFT` writer - Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card. 2'b00-Default phase shift. 2'b01-Enables shifted to next immediate positive edge. 2'b10-Enables shifted to next immediate negative edge. 2'b11-Reserved."]
pub type ENABLE_SHIFT_W<'a, const O: u8> = crate::FieldWriter<'a, ENSHIFT_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card. 2'b00-Default phase shift. 2'b01-Enables shifted to next immediate positive edge. 2'b10-Enables shifted to next immediate negative edge. 2'b11-Reserved."]
    #[inline(always)]
    pub fn enable_shift(&self) -> ENABLE_SHIFT_R {
        ENABLE_SHIFT_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENSHIFT")
            .field(
                "enable_shift",
                &format_args!("{}", self.enable_shift().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENSHIFT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card. 2'b00-Default phase shift. 2'b01-Enables shifted to next immediate positive edge. 2'b10-Enables shifted to next immediate negative edge. 2'b11-Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn enable_shift(&mut self) -> ENABLE_SHIFT_W<0> {
        ENABLE_SHIFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable Phase Shift register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enshift](index.html) module"]
pub struct ENSHIFT_SPEC;
impl crate::RegisterSpec for ENSHIFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enshift::R](R) reader structure"]
impl crate::Readable for ENSHIFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enshift::W](W) writer structure"]
impl crate::Writable for ENSHIFT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENSHIFT to value 0"]
impl crate::Resettable for ENSHIFT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
