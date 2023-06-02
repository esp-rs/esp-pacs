#[doc = "Register `CFG_DATA16` reader"]
pub struct R(crate::R<CFG_DATA16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_DATA16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_DATA16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_DATA16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_DATA16` writer"]
pub struct W(crate::W<CFG_DATA16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_DATA16_SPEC>;
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
impl From<crate::W<CFG_DATA16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_DATA16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USER_ID_FN2` reader - "]
pub type USER_ID_FN2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USER_ID_FN2` writer - "]
pub type USER_ID_FN2_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_DATA16_SPEC, 16, O, u16, u16>;
#[doc = "Field `DEVICE_ID_FN2` reader - "]
pub type DEVICE_ID_FN2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DEVICE_ID_FN2` writer - "]
pub type DEVICE_ID_FN2_W<'a, const O: u8> =
    crate::FieldWriter<'a, CFG_DATA16_SPEC, 16, O, u16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn user_id_fn2(&self) -> USER_ID_FN2_R {
        USER_ID_FN2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn device_id_fn2(&self) -> DEVICE_ID_FN2_R {
        DEVICE_ID_FN2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG_DATA16")
            .field(
                "user_id_fn2",
                &format_args!("{}", self.user_id_fn2().bits()),
            )
            .field(
                "device_id_fn2",
                &format_args!("{}", self.device_id_fn2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CFG_DATA16_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn user_id_fn2(&mut self) -> USER_ID_FN2_W<0> {
        USER_ID_FN2_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn device_id_fn2(&mut self) -> DEVICE_ID_FN2_W<16> {
        DEVICE_ID_FN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_data16](index.html) module"]
pub struct CFG_DATA16_SPEC;
impl crate::RegisterSpec for CFG_DATA16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_data16::R](R) reader structure"]
impl crate::Readable for CFG_DATA16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_data16::W](W) writer structure"]
impl crate::Writable for CFG_DATA16_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_DATA16 to value 0x3333_6666"]
impl crate::Resettable for CFG_DATA16_SPEC {
    const RESET_VALUE: Self::Ux = 0x3333_6666;
}
