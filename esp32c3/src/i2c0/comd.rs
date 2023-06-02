#[doc = "Register `COMD%s` reader"]
pub struct R(crate::R<COMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMD%s` writer"]
pub struct W(crate::W<COMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMD_SPEC>;
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
impl From<crate::W<COMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND` reader - reg_command"]
pub type COMMAND_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMMAND` writer - reg_command"]
pub type COMMAND_W<'a, const O: u8> = crate::FieldWriter<'a, COMD_SPEC, 14, O, u16, u16>;
#[doc = "Field `COMMAND_DONE` reader - reg_command_done"]
pub type COMMAND_DONE_R = crate::BitReader;
#[doc = "Field `COMMAND_DONE` writer - reg_command_done"]
pub type COMMAND_DONE_W<'a, const O: u8> = crate::BitWriter<'a, COMD_SPEC, O>;
impl R {
    #[doc = "Bits 0:13 - reg_command"]
    #[inline(always)]
    pub fn command(&self) -> COMMAND_R {
        COMMAND_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - reg_command_done"]
    #[inline(always)]
    pub fn command_done(&self) -> COMMAND_DONE_R {
        COMMAND_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMD")
            .field("command", &format_args!("{}", self.command().bits()))
            .field(
                "command_done",
                &format_args!("{}", self.command_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:13 - reg_command"]
    #[inline(always)]
    #[must_use]
    pub fn command(&mut self) -> COMMAND_W<0> {
        COMMAND_W::new(self)
    }
    #[doc = "Bit 31 - reg_command_done"]
    #[inline(always)]
    #[must_use]
    pub fn command_done(&mut self) -> COMMAND_DONE_W<31> {
        COMMAND_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_COMD%s_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd](index.html) module"]
pub struct COMD_SPEC;
impl crate::RegisterSpec for COMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comd::R](R) reader structure"]
impl crate::Readable for COMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comd::W](W) writer structure"]
impl crate::Writable for COMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMD%s to value 0"]
impl crate::Resettable for COMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
