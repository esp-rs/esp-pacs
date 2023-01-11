#[doc = "Register `IO_MUX` reader"]
pub struct R(crate::R<IO_MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IO_MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IO_MUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IO_MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IO_MUX` writer"]
pub struct W(crate::W<IO_MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IO_MUX_SPEC>;
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
impl From<crate::W<IO_MUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IO_MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PULL_LDO` reader - need_des"]
pub type PULL_LDO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PULL_LDO` writer - need_des"]
pub type PULL_LDO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IO_MUX_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESET_DISABLE` reader - need_des"]
pub type RESET_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `RESET_DISABLE` writer - need_des"]
pub type RESET_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IO_MUX_SPEC, bool, O>;
impl R {
    #[doc = "Bits 28:30 - need_des"]
    #[inline(always)]
    pub fn pull_ldo(&self) -> PULL_LDO_R {
        PULL_LDO_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn reset_disable(&self) -> RESET_DISABLE_R {
        RESET_DISABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 28:30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pull_ldo(&mut self) -> PULL_LDO_W<28> {
        PULL_LDO_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn reset_disable(&mut self) -> RESET_DISABLE_W<31> {
        RESET_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io_mux](index.html) module"]
pub struct IO_MUX_SPEC;
impl crate::RegisterSpec for IO_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [io_mux::R](R) reader structure"]
impl crate::Readable for IO_MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [io_mux::W](W) writer structure"]
impl crate::Writable for IO_MUX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IO_MUX to value 0"]
impl crate::Resettable for IO_MUX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
