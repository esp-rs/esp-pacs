#[doc = "Register `WIN_CMD` reader"]
pub struct R(crate::R<WIN_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIN_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIN_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIN_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIN_CMD` writer"]
pub struct W(crate::W<WIN_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIN_CMD_SPEC>;
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
impl From<crate::W<WIN_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIN_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLCHOST_WIN_CMD` reader - *******Description***********"]
pub type SLCHOST_WIN_CMD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SLCHOST_WIN_CMD` writer - *******Description***********"]
pub type SLCHOST_WIN_CMD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WIN_CMD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_win_cmd(&self) -> SLCHOST_WIN_CMD_R {
        SLCHOST_WIN_CMD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_win_cmd(&mut self) -> SLCHOST_WIN_CMD_W<0> {
        SLCHOST_WIN_CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [win_cmd](index.html) module"]
pub struct WIN_CMD_SPEC;
impl crate::RegisterSpec for WIN_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [win_cmd::R](R) reader structure"]
impl crate::Readable for WIN_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [win_cmd::W](W) writer structure"]
impl crate::Writable for WIN_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WIN_CMD to value 0"]
impl crate::Resettable for WIN_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
