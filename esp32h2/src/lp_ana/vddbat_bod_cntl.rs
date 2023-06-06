#[doc = "Register `VDDBAT_BOD_CNTL` reader"]
pub struct R(crate::R<VDDBAT_BOD_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDDBAT_BOD_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDDBAT_BOD_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDDBAT_BOD_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VDDBAT_BOD_CNTL` writer"]
pub struct W(crate::W<VDDBAT_BOD_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDDBAT_BOD_CNTL_SPEC>;
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
impl From<crate::W<VDDBAT_BOD_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDDBAT_BOD_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDDBAT_UNDERVOLTAGE_FLAG` reader - need_des"]
pub type VDDBAT_UNDERVOLTAGE_FLAG_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGER` reader - need_des"]
pub type VDDBAT_CHARGER_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGER` writer - need_des"]
pub type VDDBAT_CHARGER_W<'a, const O: u8> = crate::BitWriter<'a, VDDBAT_BOD_CNTL_SPEC, O>;
#[doc = "Field `VDDBAT_CNT_CLR` reader - need_des"]
pub type VDDBAT_CNT_CLR_R = crate::BitReader;
#[doc = "Field `VDDBAT_CNT_CLR` writer - need_des"]
pub type VDDBAT_CNT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, VDDBAT_BOD_CNTL_SPEC, O>;
#[doc = "Field `VDDBAT_UPVOLTAGE_TARGET` reader - need_des"]
pub type VDDBAT_UPVOLTAGE_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `VDDBAT_UPVOLTAGE_TARGET` writer - need_des"]
pub type VDDBAT_UPVOLTAGE_TARGET_W<'a, const O: u8> =
    crate::FieldWriter<'a, VDDBAT_BOD_CNTL_SPEC, 10, O, u16>;
#[doc = "Field `VDDBAT_UNDERVOLTAGE_TARGET` reader - need_des"]
pub type VDDBAT_UNDERVOLTAGE_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `VDDBAT_UNDERVOLTAGE_TARGET` writer - need_des"]
pub type VDDBAT_UNDERVOLTAGE_TARGET_W<'a, const O: u8> =
    crate::FieldWriter<'a, VDDBAT_BOD_CNTL_SPEC, 10, O, u16>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn vddbat_undervoltage_flag(&self) -> VDDBAT_UNDERVOLTAGE_FLAG_R {
        VDDBAT_UNDERVOLTAGE_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn vddbat_charger(&self) -> VDDBAT_CHARGER_R {
        VDDBAT_CHARGER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn vddbat_cnt_clr(&self) -> VDDBAT_CNT_CLR_R {
        VDDBAT_CNT_CLR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:21 - need_des"]
    #[inline(always)]
    pub fn vddbat_upvoltage_target(&self) -> VDDBAT_UPVOLTAGE_TARGET_R {
        VDDBAT_UPVOLTAGE_TARGET_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn vddbat_undervoltage_target(&self) -> VDDBAT_UNDERVOLTAGE_TARGET_R {
        VDDBAT_UNDERVOLTAGE_TARGET_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDDBAT_BOD_CNTL")
            .field(
                "vddbat_undervoltage_flag",
                &format_args!("{}", self.vddbat_undervoltage_flag().bit()),
            )
            .field(
                "vddbat_charger",
                &format_args!("{}", self.vddbat_charger().bit()),
            )
            .field(
                "vddbat_cnt_clr",
                &format_args!("{}", self.vddbat_cnt_clr().bit()),
            )
            .field(
                "vddbat_upvoltage_target",
                &format_args!("{}", self.vddbat_upvoltage_target().bits()),
            )
            .field(
                "vddbat_undervoltage_target",
                &format_args!("{}", self.vddbat_undervoltage_target().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VDDBAT_BOD_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charger(&mut self) -> VDDBAT_CHARGER_W<10> {
        VDDBAT_CHARGER_W::new(self)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_cnt_clr(&mut self) -> VDDBAT_CNT_CLR_W<11> {
        VDDBAT_CNT_CLR_W::new(self)
    }
    #[doc = "Bits 12:21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_upvoltage_target(&mut self) -> VDDBAT_UPVOLTAGE_TARGET_W<12> {
        VDDBAT_UPVOLTAGE_TARGET_W::new(self)
    }
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_undervoltage_target(&mut self) -> VDDBAT_UNDERVOLTAGE_TARGET_W<22> {
        VDDBAT_UNDERVOLTAGE_TARGET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vddbat_bod_cntl](index.html) module"]
pub struct VDDBAT_BOD_CNTL_SPEC;
impl crate::RegisterSpec for VDDBAT_BOD_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vddbat_bod_cntl::R](R) reader structure"]
impl crate::Readable for VDDBAT_BOD_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vddbat_bod_cntl::W](W) writer structure"]
impl crate::Writable for VDDBAT_BOD_CNTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VDDBAT_BOD_CNTL to value 0xffc0_0000"]
impl crate::Resettable for VDDBAT_BOD_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffc0_0000;
}
