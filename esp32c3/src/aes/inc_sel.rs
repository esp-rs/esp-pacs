#[doc = "Register `INC_SEL` reader"]
pub struct R(crate::R<INC_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INC_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INC_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INC_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INC_SEL` writer"]
pub struct W(crate::W<INC_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INC_SEL_SPEC>;
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
impl From<crate::W<INC_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INC_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INC_SEL` reader - This bit decides the standard incrementing function. 0: INC32. 1: INC128."]
pub type INC_SEL_R = crate::BitReader<bool>;
#[doc = "Field `INC_SEL` writer - This bit decides the standard incrementing function. 0: INC32. 1: INC128."]
pub type INC_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INC_SEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit decides the standard incrementing function. 0: INC32. 1: INC128."]
    #[inline(always)]
    pub fn inc_sel(&self) -> INC_SEL_R {
        INC_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit decides the standard incrementing function. 0: INC32. 1: INC128."]
    #[inline(always)]
    pub fn inc_sel(&mut self) -> INC_SEL_W<0> {
        INC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Standard incrementing function configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inc_sel](index.html) module"]
pub struct INC_SEL_SPEC;
impl crate::RegisterSpec for INC_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inc_sel::R](R) reader structure"]
impl crate::Readable for INC_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inc_sel::W](W) writer structure"]
impl crate::Writable for INC_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INC_SEL to value 0"]
impl crate::Resettable for INC_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
