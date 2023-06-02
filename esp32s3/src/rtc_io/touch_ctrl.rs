#[doc = "Register `TOUCH_CTRL` reader"]
pub struct R(crate::R<TOUCH_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_CTRL` writer"]
pub struct W(crate::W<TOUCH_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_CTRL_SPEC>;
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
impl From<crate::W<TOUCH_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IO_TOUCH_BUFSEL` reader - BUF_SEL when touch work without fsm"]
pub type IO_TOUCH_BUFSEL_R = crate::FieldReader;
#[doc = "Field `IO_TOUCH_BUFSEL` writer - BUF_SEL when touch work without fsm"]
pub type IO_TOUCH_BUFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_CTRL_SPEC, 4, O>;
#[doc = "Field `IO_TOUCH_BUFMODE` reader - BUF_MODE when touch work without fsm"]
pub type IO_TOUCH_BUFMODE_R = crate::BitReader;
#[doc = "Field `IO_TOUCH_BUFMODE` writer - BUF_MODE when touch work without fsm"]
pub type IO_TOUCH_BUFMODE_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_CTRL_SPEC, O>;
impl R {
    #[doc = "Bits 0:3 - BUF_SEL when touch work without fsm"]
    #[inline(always)]
    pub fn io_touch_bufsel(&self) -> IO_TOUCH_BUFSEL_R {
        IO_TOUCH_BUFSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - BUF_MODE when touch work without fsm"]
    #[inline(always)]
    pub fn io_touch_bufmode(&self) -> IO_TOUCH_BUFMODE_R {
        IO_TOUCH_BUFMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_CTRL")
            .field(
                "io_touch_bufsel",
                &format_args!("{}", self.io_touch_bufsel().bits()),
            )
            .field(
                "io_touch_bufmode",
                &format_args!("{}", self.io_touch_bufmode().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - BUF_SEL when touch work without fsm"]
    #[inline(always)]
    #[must_use]
    pub fn io_touch_bufsel(&mut self) -> IO_TOUCH_BUFSEL_W<0> {
        IO_TOUCH_BUFSEL_W::new(self)
    }
    #[doc = "Bit 4 - BUF_MODE when touch work without fsm"]
    #[inline(always)]
    #[must_use]
    pub fn io_touch_bufmode(&mut self) -> IO_TOUCH_BUFMODE_W<4> {
        IO_TOUCH_BUFMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure touch pad bufmode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_ctrl](index.html) module"]
pub struct TOUCH_CTRL_SPEC;
impl crate::RegisterSpec for TOUCH_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_ctrl::R](R) reader structure"]
impl crate::Readable for TOUCH_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_ctrl::W](W) writer structure"]
impl crate::Writable for TOUCH_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_CTRL to value 0"]
impl crate::Resettable for TOUCH_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
