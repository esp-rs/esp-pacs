#[doc = "Register `DCTL` reader"]
pub struct R(crate::R<DCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCTL` writer"]
pub struct W(crate::W<DCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCTL_SPEC>;
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
impl From<crate::W<DCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMTWKUPSIG` reader - "]
pub type RMTWKUPSIG_R = crate::BitReader;
#[doc = "Field `RMTWKUPSIG` writer - "]
pub type RMTWKUPSIG_W<'a, const O: u8> = crate::BitWriter<'a, DCTL_SPEC, O>;
#[doc = "Field `SFTDISCON` reader - "]
pub type SFTDISCON_R = crate::BitReader;
#[doc = "Field `SFTDISCON` writer - "]
pub type SFTDISCON_W<'a, const O: u8> = crate::BitWriter<'a, DCTL_SPEC, O>;
#[doc = "Field `GNPINNAKSTS` reader - "]
pub type GNPINNAKSTS_R = crate::BitReader;
#[doc = "Field `GOUTNAKSTS` reader - "]
pub type GOUTNAKSTS_R = crate::BitReader;
#[doc = "Field `TSTCTL` reader - "]
pub type TSTCTL_R = crate::FieldReader;
#[doc = "Field `TSTCTL` writer - "]
pub type TSTCTL_W<'a, const O: u8> = crate::FieldWriter<'a, DCTL_SPEC, 3, O>;
#[doc = "Field `SGNPINNAK` writer - "]
pub type SGNPINNAK_W<'a, const O: u8> = crate::BitWriter<'a, DCTL_SPEC, O>;
#[doc = "Field `CGNPINNAK` writer - "]
pub type CGNPINNAK_W<'a, const O: u8> = crate::BitWriter<'a, DCTL_SPEC, O>;
#[doc = "Field `SGOUTNAK` writer - "]
pub type SGOUTNAK_W<'a, const O: u8> = crate::BitWriter<'a, DCTL_SPEC, O>;
#[doc = "Field `CGOUTNAK` writer - "]
pub type CGOUTNAK_W<'a, const O: u8> = crate::BitWriter<'a, DCTL_SPEC, O>;
#[doc = "Field `PWRONPRGDONE` reader - "]
pub type PWRONPRGDONE_R = crate::BitReader;
#[doc = "Field `PWRONPRGDONE` writer - "]
pub type PWRONPRGDONE_W<'a, const O: u8> = crate::BitWriter<'a, DCTL_SPEC, O>;
#[doc = "Field `GMC` reader - "]
pub type GMC_R = crate::FieldReader;
#[doc = "Field `GMC` writer - "]
pub type GMC_W<'a, const O: u8> = crate::FieldWriter<'a, DCTL_SPEC, 2, O>;
#[doc = "Field `IGNRFRMNUM` reader - "]
pub type IGNRFRMNUM_R = crate::BitReader;
#[doc = "Field `IGNRFRMNUM` writer - "]
pub type IGNRFRMNUM_W<'a, const O: u8> = crate::BitWriter<'a, DCTL_SPEC, O>;
#[doc = "Field `NAKONBBLE` reader - "]
pub type NAKONBBLE_R = crate::BitReader;
#[doc = "Field `NAKONBBLE` writer - "]
pub type NAKONBBLE_W<'a, const O: u8> = crate::BitWriter<'a, DCTL_SPEC, O>;
#[doc = "Field `ENCOUNTONBNA` reader - "]
pub type ENCOUNTONBNA_R = crate::BitReader;
#[doc = "Field `ENCOUNTONBNA` writer - "]
pub type ENCOUNTONBNA_W<'a, const O: u8> = crate::BitWriter<'a, DCTL_SPEC, O>;
#[doc = "Field `DEEPSLEEPBESLREJECT` reader - "]
pub type DEEPSLEEPBESLREJECT_R = crate::BitReader;
#[doc = "Field `DEEPSLEEPBESLREJECT` writer - "]
pub type DEEPSLEEPBESLREJECT_W<'a, const O: u8> = crate::BitWriter<'a, DCTL_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rmtwkupsig(&self) -> RMTWKUPSIG_R {
        RMTWKUPSIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sftdiscon(&self) -> SFTDISCON_R {
        SFTDISCON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gnpinnaksts(&self) -> GNPINNAKSTS_R {
        GNPINNAKSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn goutnaksts(&self) -> GOUTNAKSTS_R {
        GOUTNAKSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tstctl(&self) -> TSTCTL_R {
        TSTCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pwronprgdone(&self) -> PWRONPRGDONE_R {
        PWRONPRGDONE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn gmc(&self) -> GMC_R {
        GMC_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ignrfrmnum(&self) -> IGNRFRMNUM_R {
        IGNRFRMNUM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn nakonbble(&self) -> NAKONBBLE_R {
        NAKONBBLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn encountonbna(&self) -> ENCOUNTONBNA_R {
        ENCOUNTONBNA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn deepsleepbeslreject(&self) -> DEEPSLEEPBESLREJECT_R {
        DEEPSLEEPBESLREJECT_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCTL")
            .field("rmtwkupsig", &format_args!("{}", self.rmtwkupsig().bit()))
            .field("sftdiscon", &format_args!("{}", self.sftdiscon().bit()))
            .field("gnpinnaksts", &format_args!("{}", self.gnpinnaksts().bit()))
            .field("goutnaksts", &format_args!("{}", self.goutnaksts().bit()))
            .field("tstctl", &format_args!("{}", self.tstctl().bits()))
            .field(
                "pwronprgdone",
                &format_args!("{}", self.pwronprgdone().bit()),
            )
            .field("gmc", &format_args!("{}", self.gmc().bits()))
            .field("ignrfrmnum", &format_args!("{}", self.ignrfrmnum().bit()))
            .field("nakonbble", &format_args!("{}", self.nakonbble().bit()))
            .field(
                "encountonbna",
                &format_args!("{}", self.encountonbna().bit()),
            )
            .field(
                "deepsleepbeslreject",
                &format_args!("{}", self.deepsleepbeslreject().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rmtwkupsig(&mut self) -> RMTWKUPSIG_W<0> {
        RMTWKUPSIG_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sftdiscon(&mut self) -> SFTDISCON_W<1> {
        SFTDISCON_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn tstctl(&mut self) -> TSTCTL_W<4> {
        TSTCTL_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn sgnpinnak(&mut self) -> SGNPINNAK_W<7> {
        SGNPINNAK_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cgnpinnak(&mut self) -> CGNPINNAK_W<8> {
        CGNPINNAK_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn sgoutnak(&mut self) -> SGOUTNAK_W<9> {
        SGOUTNAK_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cgoutnak(&mut self) -> CGOUTNAK_W<10> {
        CGOUTNAK_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pwronprgdone(&mut self) -> PWRONPRGDONE_W<11> {
        PWRONPRGDONE_W::new(self)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    #[must_use]
    pub fn gmc(&mut self) -> GMC_W<13> {
        GMC_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ignrfrmnum(&mut self) -> IGNRFRMNUM_W<15> {
        IGNRFRMNUM_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn nakonbble(&mut self) -> NAKONBBLE_W<16> {
        NAKONBBLE_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn encountonbna(&mut self) -> ENCOUNTONBNA_W<17> {
        ENCOUNTONBNA_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn deepsleepbeslreject(&mut self) -> DEEPSLEEPBESLREJECT_W<18> {
        DEEPSLEEPBESLREJECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctl](index.html) module"]
pub struct DCTL_SPEC;
impl crate::RegisterSpec for DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dctl::R](R) reader structure"]
impl crate::Readable for DCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dctl::W](W) writer structure"]
impl crate::Writable for DCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCTL to value 0x2000"]
impl crate::Resettable for DCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
