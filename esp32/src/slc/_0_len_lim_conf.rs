#[doc = "Register `_0_LEN_LIM_CONF` reader"]
pub struct R(crate::R<_0_LEN_LIM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_0_LEN_LIM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_0_LEN_LIM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_0_LEN_LIM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_0_LEN_LIM_CONF` writer"]
pub struct W(crate::W<_0_LEN_LIM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_0_LEN_LIM_CONF_SPEC>;
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
impl From<crate::W<_0_LEN_LIM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_0_LEN_LIM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_LEN_LIM` reader - "]
pub type SLC0_LEN_LIM_R = crate::FieldReader<u32>;
#[doc = "Field `SLC0_LEN_LIM` writer - "]
pub type SLC0_LEN_LIM_W<'a, const O: u8> = crate::FieldWriter<'a, _0_LEN_LIM_CONF_SPEC, 20, O, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc0_len_lim(&self) -> SLC0_LEN_LIM_R {
        SLC0_LEN_LIM_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_LEN_LIM_CONF")
            .field(
                "slc0_len_lim",
                &format_args!("{}", self.slc0_len_lim().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_0_LEN_LIM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_len_lim(&mut self) -> SLC0_LEN_LIM_W<0> {
        SLC0_LEN_LIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_len_lim_conf](index.html) module"]
pub struct _0_LEN_LIM_CONF_SPEC;
impl crate::RegisterSpec for _0_LEN_LIM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_0_len_lim_conf::R](R) reader structure"]
impl crate::Readable for _0_LEN_LIM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_0_len_lim_conf::W](W) writer structure"]
impl crate::Writable for _0_LEN_LIM_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _0_LEN_LIM_CONF to value 0x5400"]
impl crate::Resettable for _0_LEN_LIM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x5400;
}
