#[doc = "Register `CH%s_GAMMA_CONF` reader"]
pub struct R(crate::R<CH_GAMMA_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_GAMMA_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_GAMMA_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_GAMMA_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_GAMMA_CONF` writer"]
pub struct W(crate::W<CH_GAMMA_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_GAMMA_CONF_SPEC>;
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
impl From<crate::W<CH_GAMMA_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_GAMMA_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_GAMMA_ENTRY_NUM` reader - Ledc ch%s gamma entry num."]
pub type CH_GAMMA_ENTRY_NUM_R = crate::FieldReader;
#[doc = "Field `CH_GAMMA_ENTRY_NUM` writer - Ledc ch%s gamma entry num."]
pub type CH_GAMMA_ENTRY_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, CH_GAMMA_CONF_SPEC, 5, O>;
#[doc = "Field `CH_GAMMA_PAUSE` writer - Ledc ch%s gamma pause, write 1 to pause."]
pub type CH_GAMMA_PAUSE_W<'a, const O: u8> = crate::BitWriter<'a, CH_GAMMA_CONF_SPEC, O>;
#[doc = "Field `CH_GAMMA_RESUME` writer - Ledc ch%s gamma resume, write 1 to resume."]
pub type CH_GAMMA_RESUME_W<'a, const O: u8> = crate::BitWriter<'a, CH_GAMMA_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:4 - Ledc ch%s gamma entry num."]
    #[inline(always)]
    pub fn ch_gamma_entry_num(&self) -> CH_GAMMA_ENTRY_NUM_R {
        CH_GAMMA_ENTRY_NUM_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_GAMMA_CONF")
            .field(
                "ch_gamma_entry_num",
                &format_args!("{}", self.ch_gamma_entry_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_GAMMA_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Ledc ch%s gamma entry num."]
    #[inline(always)]
    #[must_use]
    pub fn ch_gamma_entry_num(&mut self) -> CH_GAMMA_ENTRY_NUM_W<0> {
        CH_GAMMA_ENTRY_NUM_W::new(self)
    }
    #[doc = "Bit 5 - Ledc ch%s gamma pause, write 1 to pause."]
    #[inline(always)]
    #[must_use]
    pub fn ch_gamma_pause(&mut self) -> CH_GAMMA_PAUSE_W<5> {
        CH_GAMMA_PAUSE_W::new(self)
    }
    #[doc = "Bit 6 - Ledc ch%s gamma resume, write 1 to resume."]
    #[inline(always)]
    #[must_use]
    pub fn ch_gamma_resume(&mut self) -> CH_GAMMA_RESUME_W<6> {
        CH_GAMMA_RESUME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ledc ch%s gamma config register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_gamma_conf](index.html) module"]
pub struct CH_GAMMA_CONF_SPEC;
impl crate::RegisterSpec for CH_GAMMA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_gamma_conf::R](R) reader structure"]
impl crate::Readable for CH_GAMMA_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_gamma_conf::W](W) writer structure"]
impl crate::Writable for CH_GAMMA_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_GAMMA_CONF to value 0"]
impl crate::Resettable for CH_GAMMA_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
