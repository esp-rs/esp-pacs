#[doc = "Register `DCFG` reader"]
pub struct R(crate::R<DCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCFG` writer"]
pub struct W(crate::W<DCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCFG_SPEC>;
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
impl From<crate::W<DCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NZSTSOUTHSHK` reader - "]
pub type NZSTSOUTHSHK_R = crate::BitReader;
#[doc = "Field `NZSTSOUTHSHK` writer - "]
pub type NZSTSOUTHSHK_W<'a, const O: u8> = crate::BitWriter<'a, DCFG_SPEC, O>;
#[doc = "Field `DEVADDR` reader - "]
pub type DEVADDR_R = crate::FieldReader;
#[doc = "Field `DEVADDR` writer - "]
pub type DEVADDR_W<'a, const O: u8> = crate::FieldWriter<'a, DCFG_SPEC, 7, O>;
#[doc = "Field `PERFRLINT` reader - "]
pub type PERFRLINT_R = crate::FieldReader;
#[doc = "Field `PERFRLINT` writer - "]
pub type PERFRLINT_W<'a, const O: u8> = crate::FieldWriter<'a, DCFG_SPEC, 2, O>;
#[doc = "Field `ENDEVOUTNAK` reader - "]
pub type ENDEVOUTNAK_R = crate::BitReader;
#[doc = "Field `ENDEVOUTNAK` writer - "]
pub type ENDEVOUTNAK_W<'a, const O: u8> = crate::BitWriter<'a, DCFG_SPEC, O>;
#[doc = "Field `XCVRDLY` reader - "]
pub type XCVRDLY_R = crate::BitReader;
#[doc = "Field `XCVRDLY` writer - "]
pub type XCVRDLY_W<'a, const O: u8> = crate::BitWriter<'a, DCFG_SPEC, O>;
#[doc = "Field `ERRATICINTMSK` reader - "]
pub type ERRATICINTMSK_R = crate::BitReader;
#[doc = "Field `ERRATICINTMSK` writer - "]
pub type ERRATICINTMSK_W<'a, const O: u8> = crate::BitWriter<'a, DCFG_SPEC, O>;
#[doc = "Field `EPMISCNT` reader - "]
pub type EPMISCNT_R = crate::FieldReader;
#[doc = "Field `EPMISCNT` writer - "]
pub type EPMISCNT_W<'a, const O: u8> = crate::FieldWriter<'a, DCFG_SPEC, 5, O>;
#[doc = "Field `DESCDMA` reader - "]
pub type DESCDMA_R = crate::BitReader;
#[doc = "Field `DESCDMA` writer - "]
pub type DESCDMA_W<'a, const O: u8> = crate::BitWriter<'a, DCFG_SPEC, O>;
#[doc = "Field `PERSCHINTVL` reader - "]
pub type PERSCHINTVL_R = crate::FieldReader;
#[doc = "Field `PERSCHINTVL` writer - "]
pub type PERSCHINTVL_W<'a, const O: u8> = crate::FieldWriter<'a, DCFG_SPEC, 2, O>;
#[doc = "Field `RESVALID` reader - "]
pub type RESVALID_R = crate::FieldReader;
#[doc = "Field `RESVALID` writer - "]
pub type RESVALID_W<'a, const O: u8> = crate::FieldWriter<'a, DCFG_SPEC, 6, O>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn nzstsouthshk(&self) -> NZSTSOUTHSHK_R {
        NZSTSOUTHSHK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:10"]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn perfrlint(&self) -> PERFRLINT_R {
        PERFRLINT_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn endevoutnak(&self) -> ENDEVOUTNAK_R {
        ENDEVOUTNAK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn xcvrdly(&self) -> XCVRDLY_R {
        XCVRDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn erraticintmsk(&self) -> ERRATICINTMSK_R {
        ERRATICINTMSK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:22"]
    #[inline(always)]
    pub fn epmiscnt(&self) -> EPMISCNT_R {
        EPMISCNT_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn descdma(&self) -> DESCDMA_R {
        DESCDMA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn perschintvl(&self) -> PERSCHINTVL_R {
        PERSCHINTVL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn resvalid(&self) -> RESVALID_R {
        RESVALID_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCFG")
            .field(
                "nzstsouthshk",
                &format_args!("{}", self.nzstsouthshk().bit()),
            )
            .field("devaddr", &format_args!("{}", self.devaddr().bits()))
            .field("perfrlint", &format_args!("{}", self.perfrlint().bits()))
            .field("endevoutnak", &format_args!("{}", self.endevoutnak().bit()))
            .field("xcvrdly", &format_args!("{}", self.xcvrdly().bit()))
            .field(
                "erraticintmsk",
                &format_args!("{}", self.erraticintmsk().bit()),
            )
            .field("epmiscnt", &format_args!("{}", self.epmiscnt().bits()))
            .field("descdma", &format_args!("{}", self.descdma().bit()))
            .field(
                "perschintvl",
                &format_args!("{}", self.perschintvl().bits()),
            )
            .field("resvalid", &format_args!("{}", self.resvalid().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn nzstsouthshk(&mut self) -> NZSTSOUTHSHK_W<2> {
        NZSTSOUTHSHK_W::new(self)
    }
    #[doc = "Bits 4:10"]
    #[inline(always)]
    #[must_use]
    pub fn devaddr(&mut self) -> DEVADDR_W<4> {
        DEVADDR_W::new(self)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    #[must_use]
    pub fn perfrlint(&mut self) -> PERFRLINT_W<11> {
        PERFRLINT_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn endevoutnak(&mut self) -> ENDEVOUTNAK_W<13> {
        ENDEVOUTNAK_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn xcvrdly(&mut self) -> XCVRDLY_W<14> {
        XCVRDLY_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn erraticintmsk(&mut self) -> ERRATICINTMSK_W<15> {
        ERRATICINTMSK_W::new(self)
    }
    #[doc = "Bits 18:22"]
    #[inline(always)]
    #[must_use]
    pub fn epmiscnt(&mut self) -> EPMISCNT_W<18> {
        EPMISCNT_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn descdma(&mut self) -> DESCDMA_W<23> {
        DESCDMA_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn perschintvl(&mut self) -> PERSCHINTVL_W<24> {
        PERSCHINTVL_W::new(self)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    #[must_use]
    pub fn resvalid(&mut self) -> RESVALID_W<26> {
        RESVALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfg](index.html) module"]
pub struct DCFG_SPEC;
impl crate::RegisterSpec for DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcfg::R](R) reader structure"]
impl crate::Readable for DCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcfg::W](W) writer structure"]
impl crate::Writable for DCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCFG to value 0x0810_0000"]
impl crate::Resettable for DCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0810_0000;
}
