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
pub type IO_TOUCH_BUFSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO_TOUCH_BUFSEL` writer - BUF_SEL when touch work without fsm"]
pub type IO_TOUCH_BUFSEL_W<'a> = crate::FieldWriter<'a, u32, TOUCH_CTRL_SPEC, u8, u8, 4, 0>;
#[doc = "Field `IO_TOUCH_BUFMODE` reader - BUF_MODE when touch work without fsm"]
pub type IO_TOUCH_BUFMODE_R = crate::BitReader<bool>;
#[doc = "Field `IO_TOUCH_BUFMODE` writer - BUF_MODE when touch work without fsm"]
pub type IO_TOUCH_BUFMODE_W<'a> = crate::BitWriter<'a, u32, TOUCH_CTRL_SPEC, bool, 4>;
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
impl W {
    #[doc = "Bits 0:3 - BUF_SEL when touch work without fsm"]
    #[inline(always)]
    pub fn io_touch_bufsel(&mut self) -> IO_TOUCH_BUFSEL_W {
        IO_TOUCH_BUFSEL_W::new(self)
    }
    #[doc = "Bit 4 - BUF_MODE when touch work without fsm"]
    #[inline(always)]
    pub fn io_touch_bufmode(&mut self) -> IO_TOUCH_BUFMODE_W {
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
}
#[doc = "`reset()` method sets TOUCH_CTRL to value 0"]
impl crate::Resettable for TOUCH_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
