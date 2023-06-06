#[doc = "Register `DTHRCTL` reader"]
pub struct R(crate::R<DTHRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTHRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTHRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTHRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTHRCTL` writer"]
pub struct W(crate::W<DTHRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTHRCTL_SPEC>;
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
impl From<crate::W<DTHRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTHRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NONISOTHREN` reader - "]
pub type NONISOTHREN_R = crate::BitReader;
#[doc = "Field `NONISOTHREN` writer - "]
pub type NONISOTHREN_W<'a, const O: u8> = crate::BitWriter<'a, DTHRCTL_SPEC, O>;
#[doc = "Field `ISOTHREN` reader - "]
pub type ISOTHREN_R = crate::BitReader;
#[doc = "Field `ISOTHREN` writer - "]
pub type ISOTHREN_W<'a, const O: u8> = crate::BitWriter<'a, DTHRCTL_SPEC, O>;
#[doc = "Field `TXTHRLEN` reader - "]
pub type TXTHRLEN_R = crate::FieldReader<u16>;
#[doc = "Field `TXTHRLEN` writer - "]
pub type TXTHRLEN_W<'a, const O: u8> = crate::FieldWriter<'a, DTHRCTL_SPEC, 9, O, u16>;
#[doc = "Field `AHBTHRRATIO` reader - "]
pub type AHBTHRRATIO_R = crate::FieldReader;
#[doc = "Field `AHBTHRRATIO` writer - "]
pub type AHBTHRRATIO_W<'a, const O: u8> = crate::FieldWriter<'a, DTHRCTL_SPEC, 2, O>;
#[doc = "Field `RXTHREN` reader - "]
pub type RXTHREN_R = crate::BitReader;
#[doc = "Field `RXTHREN` writer - "]
pub type RXTHREN_W<'a, const O: u8> = crate::BitWriter<'a, DTHRCTL_SPEC, O>;
#[doc = "Field `RXTHRLEN` reader - "]
pub type RXTHRLEN_R = crate::FieldReader<u16>;
#[doc = "Field `RXTHRLEN` writer - "]
pub type RXTHRLEN_W<'a, const O: u8> = crate::FieldWriter<'a, DTHRCTL_SPEC, 9, O, u16>;
#[doc = "Field `ARBPRKEN` reader - "]
pub type ARBPRKEN_R = crate::BitReader;
#[doc = "Field `ARBPRKEN` writer - "]
pub type ARBPRKEN_W<'a, const O: u8> = crate::BitWriter<'a, DTHRCTL_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn nonisothren(&self) -> NONISOTHREN_R {
        NONISOTHREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn isothren(&self) -> ISOTHREN_R {
        ISOTHREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:10"]
    #[inline(always)]
    pub fn txthrlen(&self) -> TXTHRLEN_R {
        TXTHRLEN_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn ahbthrratio(&self) -> AHBTHRRATIO_R {
        AHBTHRRATIO_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rxthren(&self) -> RXTHREN_R {
        RXTHREN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:25"]
    #[inline(always)]
    pub fn rxthrlen(&self) -> RXTHRLEN_R {
        RXTHRLEN_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn arbprken(&self) -> ARBPRKEN_R {
        ARBPRKEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTHRCTL")
            .field("nonisothren", &format_args!("{}", self.nonisothren().bit()))
            .field("isothren", &format_args!("{}", self.isothren().bit()))
            .field("txthrlen", &format_args!("{}", self.txthrlen().bits()))
            .field(
                "ahbthrratio",
                &format_args!("{}", self.ahbthrratio().bits()),
            )
            .field("rxthren", &format_args!("{}", self.rxthren().bit()))
            .field("rxthrlen", &format_args!("{}", self.rxthrlen().bits()))
            .field("arbprken", &format_args!("{}", self.arbprken().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DTHRCTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn nonisothren(&mut self) -> NONISOTHREN_W<0> {
        NONISOTHREN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn isothren(&mut self) -> ISOTHREN_W<1> {
        ISOTHREN_W::new(self)
    }
    #[doc = "Bits 2:10"]
    #[inline(always)]
    #[must_use]
    pub fn txthrlen(&mut self) -> TXTHRLEN_W<2> {
        TXTHRLEN_W::new(self)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    #[must_use]
    pub fn ahbthrratio(&mut self) -> AHBTHRRATIO_W<11> {
        AHBTHRRATIO_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn rxthren(&mut self) -> RXTHREN_W<16> {
        RXTHREN_W::new(self)
    }
    #[doc = "Bits 17:25"]
    #[inline(always)]
    #[must_use]
    pub fn rxthrlen(&mut self) -> RXTHRLEN_W<17> {
        RXTHRLEN_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn arbprken(&mut self) -> ARBPRKEN_W<27> {
        ARBPRKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dthrctl](index.html) module"]
pub struct DTHRCTL_SPEC;
impl crate::RegisterSpec for DTHRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dthrctl::R](R) reader structure"]
impl crate::Readable for DTHRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dthrctl::W](W) writer structure"]
impl crate::Writable for DTHRCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTHRCTL to value 0x0802_0020"]
impl crate::Resettable for DTHRCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0802_0020;
}
