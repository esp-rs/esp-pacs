#[doc = "Register `DIG_ISO` reader"]
pub struct R(crate::R<DIG_ISO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIG_ISO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIG_ISO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIG_ISO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIG_ISO` writer"]
pub struct W(crate::W<DIG_ISO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIG_ISO_SPEC>;
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
impl From<crate::W<DIG_ISO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIG_ISO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_OFF` reader - Need add desc"]
pub type FORCE_OFF_R = crate::BitReader;
#[doc = "Field `FORCE_OFF` writer - Need add desc"]
pub type FORCE_OFF_W<'a, const O: u8> = crate::BitWriter<'a, DIG_ISO_SPEC, O>;
#[doc = "Field `FORCE_ON` reader - Need add desc"]
pub type FORCE_ON_R = crate::BitReader;
#[doc = "Field `FORCE_ON` writer - Need add desc"]
pub type FORCE_ON_W<'a, const O: u8> = crate::BitWriter<'a, DIG_ISO_SPEC, O>;
#[doc = "Field `DG_PAD_AUTOHOLD` reader - read only register to indicate digital pad auto-hold status"]
pub type DG_PAD_AUTOHOLD_R = crate::BitReader;
#[doc = "Field `DG_PAD_AUTOHOLD` writer - read only register to indicate digital pad auto-hold status"]
pub type DG_PAD_AUTOHOLD_W<'a, const O: u8> = crate::BitWriter<'a, DIG_ISO_SPEC, O>;
#[doc = "Field `CLR_DG_PAD_AUTOHOLD` reader - wtite only register to clear digital pad auto-hold"]
pub type CLR_DG_PAD_AUTOHOLD_R = crate::BitReader;
#[doc = "Field `CLR_DG_PAD_AUTOHOLD` writer - wtite only register to clear digital pad auto-hold"]
pub type CLR_DG_PAD_AUTOHOLD_W<'a, const O: u8> = crate::BitWriter<'a, DIG_ISO_SPEC, O>;
#[doc = "Field `DG_PAD_AUTOHOLD_EN` reader - digital pad enable auto-hold"]
pub type DG_PAD_AUTOHOLD_EN_R = crate::BitReader;
#[doc = "Field `DG_PAD_AUTOHOLD_EN` writer - digital pad enable auto-hold"]
pub type DG_PAD_AUTOHOLD_EN_W<'a, const O: u8> = crate::BitWriter<'a, DIG_ISO_SPEC, O>;
#[doc = "Field `DG_PAD_FORCE_NOISO` reader - digital pad force no ISO"]
pub type DG_PAD_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `DG_PAD_FORCE_NOISO` writer - digital pad force no ISO"]
pub type DG_PAD_FORCE_NOISO_W<'a, const O: u8> = crate::BitWriter<'a, DIG_ISO_SPEC, O>;
#[doc = "Field `DG_PAD_FORCE_ISO` reader - digital pad force ISO"]
pub type DG_PAD_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `DG_PAD_FORCE_ISO` writer - digital pad force ISO"]
pub type DG_PAD_FORCE_ISO_W<'a, const O: u8> = crate::BitWriter<'a, DIG_ISO_SPEC, O>;
#[doc = "Field `DG_PAD_FORCE_UNHOLD` reader - digital pad force un-hold"]
pub type DG_PAD_FORCE_UNHOLD_R = crate::BitReader;
#[doc = "Field `DG_PAD_FORCE_UNHOLD` writer - digital pad force un-hold"]
pub type DG_PAD_FORCE_UNHOLD_W<'a, const O: u8> = crate::BitWriter<'a, DIG_ISO_SPEC, O>;
#[doc = "Field `DG_PAD_FORCE_HOLD` reader - digital pad force hold"]
pub type DG_PAD_FORCE_HOLD_R = crate::BitReader;
#[doc = "Field `DG_PAD_FORCE_HOLD` writer - digital pad force hold"]
pub type DG_PAD_FORCE_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, DIG_ISO_SPEC, O>;
#[doc = "Field `DG_WRAP_FORCE_ISO` reader - digital core force ISO"]
pub type DG_WRAP_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `DG_WRAP_FORCE_ISO` writer - digital core force ISO"]
pub type DG_WRAP_FORCE_ISO_W<'a, const O: u8> = crate::BitWriter<'a, DIG_ISO_SPEC, O>;
#[doc = "Field `DG_WRAP_FORCE_NOISO` reader - Need add desc"]
pub type DG_WRAP_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `DG_WRAP_FORCE_NOISO` writer - Need add desc"]
pub type DG_WRAP_FORCE_NOISO_W<'a, const O: u8> = crate::BitWriter<'a, DIG_ISO_SPEC, O>;
impl R {
    #[doc = "Bit 7 - Need add desc"]
    #[inline(always)]
    pub fn force_off(&self) -> FORCE_OFF_R {
        FORCE_OFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Need add desc"]
    #[inline(always)]
    pub fn force_on(&self) -> FORCE_ON_R {
        FORCE_ON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - read only register to indicate digital pad auto-hold status"]
    #[inline(always)]
    pub fn dg_pad_autohold(&self) -> DG_PAD_AUTOHOLD_R {
        DG_PAD_AUTOHOLD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - wtite only register to clear digital pad auto-hold"]
    #[inline(always)]
    pub fn clr_dg_pad_autohold(&self) -> CLR_DG_PAD_AUTOHOLD_R {
        CLR_DG_PAD_AUTOHOLD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - digital pad enable auto-hold"]
    #[inline(always)]
    pub fn dg_pad_autohold_en(&self) -> DG_PAD_AUTOHOLD_EN_R {
        DG_PAD_AUTOHOLD_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - digital pad force no ISO"]
    #[inline(always)]
    pub fn dg_pad_force_noiso(&self) -> DG_PAD_FORCE_NOISO_R {
        DG_PAD_FORCE_NOISO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - digital pad force ISO"]
    #[inline(always)]
    pub fn dg_pad_force_iso(&self) -> DG_PAD_FORCE_ISO_R {
        DG_PAD_FORCE_ISO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - digital pad force un-hold"]
    #[inline(always)]
    pub fn dg_pad_force_unhold(&self) -> DG_PAD_FORCE_UNHOLD_R {
        DG_PAD_FORCE_UNHOLD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - digital pad force hold"]
    #[inline(always)]
    pub fn dg_pad_force_hold(&self) -> DG_PAD_FORCE_HOLD_R {
        DG_PAD_FORCE_HOLD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 30 - digital core force ISO"]
    #[inline(always)]
    pub fn dg_wrap_force_iso(&self) -> DG_WRAP_FORCE_ISO_R {
        DG_WRAP_FORCE_ISO_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn dg_wrap_force_noiso(&self) -> DG_WRAP_FORCE_NOISO_R {
        DG_WRAP_FORCE_NOISO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIG_ISO")
            .field("force_off", &format_args!("{}", self.force_off().bit()))
            .field("force_on", &format_args!("{}", self.force_on().bit()))
            .field(
                "dg_pad_autohold",
                &format_args!("{}", self.dg_pad_autohold().bit()),
            )
            .field(
                "clr_dg_pad_autohold",
                &format_args!("{}", self.clr_dg_pad_autohold().bit()),
            )
            .field(
                "dg_pad_autohold_en",
                &format_args!("{}", self.dg_pad_autohold_en().bit()),
            )
            .field(
                "dg_pad_force_noiso",
                &format_args!("{}", self.dg_pad_force_noiso().bit()),
            )
            .field(
                "dg_pad_force_iso",
                &format_args!("{}", self.dg_pad_force_iso().bit()),
            )
            .field(
                "dg_pad_force_unhold",
                &format_args!("{}", self.dg_pad_force_unhold().bit()),
            )
            .field(
                "dg_pad_force_hold",
                &format_args!("{}", self.dg_pad_force_hold().bit()),
            )
            .field(
                "dg_wrap_force_iso",
                &format_args!("{}", self.dg_wrap_force_iso().bit()),
            )
            .field(
                "dg_wrap_force_noiso",
                &format_args!("{}", self.dg_wrap_force_noiso().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIG_ISO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 7 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn force_off(&mut self) -> FORCE_OFF_W<7> {
        FORCE_OFF_W::new(self)
    }
    #[doc = "Bit 8 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn force_on(&mut self) -> FORCE_ON_W<8> {
        FORCE_ON_W::new(self)
    }
    #[doc = "Bit 9 - read only register to indicate digital pad auto-hold status"]
    #[inline(always)]
    #[must_use]
    pub fn dg_pad_autohold(&mut self) -> DG_PAD_AUTOHOLD_W<9> {
        DG_PAD_AUTOHOLD_W::new(self)
    }
    #[doc = "Bit 10 - wtite only register to clear digital pad auto-hold"]
    #[inline(always)]
    #[must_use]
    pub fn clr_dg_pad_autohold(&mut self) -> CLR_DG_PAD_AUTOHOLD_W<10> {
        CLR_DG_PAD_AUTOHOLD_W::new(self)
    }
    #[doc = "Bit 11 - digital pad enable auto-hold"]
    #[inline(always)]
    #[must_use]
    pub fn dg_pad_autohold_en(&mut self) -> DG_PAD_AUTOHOLD_EN_W<11> {
        DG_PAD_AUTOHOLD_EN_W::new(self)
    }
    #[doc = "Bit 12 - digital pad force no ISO"]
    #[inline(always)]
    #[must_use]
    pub fn dg_pad_force_noiso(&mut self) -> DG_PAD_FORCE_NOISO_W<12> {
        DG_PAD_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 13 - digital pad force ISO"]
    #[inline(always)]
    #[must_use]
    pub fn dg_pad_force_iso(&mut self) -> DG_PAD_FORCE_ISO_W<13> {
        DG_PAD_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 14 - digital pad force un-hold"]
    #[inline(always)]
    #[must_use]
    pub fn dg_pad_force_unhold(&mut self) -> DG_PAD_FORCE_UNHOLD_W<14> {
        DG_PAD_FORCE_UNHOLD_W::new(self)
    }
    #[doc = "Bit 15 - digital pad force hold"]
    #[inline(always)]
    #[must_use]
    pub fn dg_pad_force_hold(&mut self) -> DG_PAD_FORCE_HOLD_W<15> {
        DG_PAD_FORCE_HOLD_W::new(self)
    }
    #[doc = "Bit 30 - digital core force ISO"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_force_iso(&mut self) -> DG_WRAP_FORCE_ISO_W<30> {
        DG_WRAP_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_force_noiso(&mut self) -> DG_WRAP_FORCE_NOISO_W<31> {
        DG_WRAP_FORCE_NOISO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_iso](index.html) module"]
pub struct DIG_ISO_SPEC;
impl crate::RegisterSpec for DIG_ISO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dig_iso::R](R) reader structure"]
impl crate::Readable for DIG_ISO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dig_iso::W](W) writer structure"]
impl crate::Writable for DIG_ISO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIG_ISO to value 0x8000_5080"]
impl crate::Resettable for DIG_ISO_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_5080;
}
