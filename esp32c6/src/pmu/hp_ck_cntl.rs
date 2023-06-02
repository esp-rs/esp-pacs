#[doc = "Register `HP_CK_CNTL` reader"]
pub struct R(crate::R<HP_CK_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_CK_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_CK_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_CK_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_CK_CNTL` writer"]
pub struct W(crate::W<HP_CK_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_CK_CNTL_SPEC>;
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
impl From<crate::W<HP_CK_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_CK_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODIFY_ICG_CNTL_WAIT` reader - need_des"]
pub type MODIFY_ICG_CNTL_WAIT_R = crate::FieldReader;
#[doc = "Field `MODIFY_ICG_CNTL_WAIT` writer - need_des"]
pub type MODIFY_ICG_CNTL_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, HP_CK_CNTL_SPEC, 8, O>;
#[doc = "Field `SWITCH_ICG_CNTL_WAIT` reader - need_des"]
pub type SWITCH_ICG_CNTL_WAIT_R = crate::FieldReader;
#[doc = "Field `SWITCH_ICG_CNTL_WAIT` writer - need_des"]
pub type SWITCH_ICG_CNTL_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, HP_CK_CNTL_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn modify_icg_cntl_wait(&self) -> MODIFY_ICG_CNTL_WAIT_R {
        MODIFY_ICG_CNTL_WAIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn switch_icg_cntl_wait(&self) -> SWITCH_ICG_CNTL_WAIT_R {
        SWITCH_ICG_CNTL_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CK_CNTL")
            .field(
                "modify_icg_cntl_wait",
                &format_args!("{}", self.modify_icg_cntl_wait().bits()),
            )
            .field(
                "switch_icg_cntl_wait",
                &format_args!("{}", self.switch_icg_cntl_wait().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_CK_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modify_icg_cntl_wait(&mut self) -> MODIFY_ICG_CNTL_WAIT_W<0> {
        MODIFY_ICG_CNTL_WAIT_W::new(self)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn switch_icg_cntl_wait(&mut self) -> SWITCH_ICG_CNTL_WAIT_W<8> {
        SWITCH_ICG_CNTL_WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_ck_cntl](index.html) module"]
pub struct HP_CK_CNTL_SPEC;
impl crate::RegisterSpec for HP_CK_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_ck_cntl::R](R) reader structure"]
impl crate::Readable for HP_CK_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_ck_cntl::W](W) writer structure"]
impl crate::Writable for HP_CK_CNTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_CK_CNTL to value 0x0a0a"]
impl crate::Resettable for HP_CK_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a0a;
}
