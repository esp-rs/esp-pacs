#[doc = "Register `HOST_SLCHOST_WIN_CMD` reader"]
pub struct R(crate::R<HOST_SLCHOST_WIN_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_WIN_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_WIN_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_WIN_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOST_WIN_CMD` writer"]
pub struct W(crate::W<HOST_SLCHOST_WIN_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_WIN_CMD_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_WIN_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_WIN_CMD_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_win_cmd](index.html) module"]
pub struct HOST_SLCHOST_WIN_CMD_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_WIN_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_win_cmd::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_WIN_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_win_cmd::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_WIN_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_SLCHOST_WIN_CMD to value 0"]
impl crate::Resettable for HOST_SLCHOST_WIN_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
