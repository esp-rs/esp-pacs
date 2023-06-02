#[doc = "Register `M14_MODE_CTRL` reader"]
pub struct R(crate::R<M14_MODE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M14_MODE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M14_MODE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M14_MODE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M14_MODE_CTRL` writer"]
pub struct W(crate::W<M14_MODE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M14_MODE_CTRL_SPEC>;
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
impl From<crate::W<M14_MODE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M14_MODE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M14_MODE` reader - M14 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
pub type M14_MODE_R = crate::FieldReader;
#[doc = "Field `M14_MODE` writer - M14 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
pub type M14_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, M14_MODE_CTRL_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - M14 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
    #[inline(always)]
    pub fn m14_mode(&self) -> M14_MODE_R {
        M14_MODE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M14_MODE_CTRL")
            .field("m14_mode", &format_args!("{}", self.m14_mode().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<M14_MODE_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - M14 security level mode: 2'd3: ree_mode2. 2'd2: ree_mode1. 2'd1: ree_mode0. 2'd0: tee_mode"]
    #[inline(always)]
    #[must_use]
    pub fn m14_mode(&mut self) -> M14_MODE_W<0> {
        M14_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tee mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m14_mode_ctrl](index.html) module"]
pub struct M14_MODE_CTRL_SPEC;
impl crate::RegisterSpec for M14_MODE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m14_mode_ctrl::R](R) reader structure"]
impl crate::Readable for M14_MODE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m14_mode_ctrl::W](W) writer structure"]
impl crate::Writable for M14_MODE_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets M14_MODE_CTRL to value 0x03"]
impl crate::Resettable for M14_MODE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
