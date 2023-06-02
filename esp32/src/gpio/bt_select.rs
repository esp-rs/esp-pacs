#[doc = "Register `BT_SELECT` reader"]
pub struct R(crate::R<BT_SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BT_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BT_SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BT_SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BT_SELECT` writer"]
pub struct W(crate::W<BT_SELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BT_SELECT_SPEC>;
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
impl From<crate::W<BT_SELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BT_SELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BT_SEL` reader - NA"]
pub type BT_SEL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BT_SEL` writer - NA"]
pub type BT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, BT_SELECT_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn bt_sel(&self) -> BT_SEL_R {
        BT_SEL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BT_SELECT")
            .field("bt_sel", &format_args!("{}", self.bt_sel().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BT_SELECT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn bt_sel(&mut self) -> BT_SEL_W<0> {
        BT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt_select](index.html) module"]
pub struct BT_SELECT_SPEC;
impl crate::RegisterSpec for BT_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bt_select::R](R) reader structure"]
impl crate::Readable for BT_SELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bt_select::W](W) writer structure"]
impl crate::Writable for BT_SELECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BT_SELECT to value 0"]
impl crate::Resettable for BT_SELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
