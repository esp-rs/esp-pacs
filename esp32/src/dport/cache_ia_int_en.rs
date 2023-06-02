#[doc = "Register `CACHE_IA_INT_EN` reader"]
pub struct R(crate::R<CACHE_IA_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_IA_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_IA_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_IA_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_IA_INT_EN` writer"]
pub struct W(crate::W<CACHE_IA_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_IA_INT_EN_SPEC>;
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
impl From<crate::W<CACHE_IA_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_IA_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_IA_INT_EN` reader - Interrupt enable bits for various invalid cache access reasons"]
pub type CACHE_IA_INT_EN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CACHE_IA_INT_EN` writer - Interrupt enable bits for various invalid cache access reasons"]
pub type CACHE_IA_INT_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, CACHE_IA_INT_EN_SPEC, 28, O, u32, u32>;
#[doc = "Field `CACHE_IA_INT_APP_DROM0` reader - APP CPU invalid access to DROM0 when cache is disabled"]
pub type CACHE_IA_INT_APP_DROM0_R = crate::BitReader;
#[doc = "Field `CACHE_IA_INT_APP_DROM0` writer - APP CPU invalid access to DROM0 when cache is disabled"]
pub type CACHE_IA_INT_APP_DROM0_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_IA_INT_EN_SPEC, O>;
#[doc = "Field `CACHE_IA_INT_APP_IRAM0` reader - APP CPU invalid access to IRAM0 when cache is disabled"]
pub type CACHE_IA_INT_APP_IRAM0_R = crate::BitReader;
#[doc = "Field `CACHE_IA_INT_APP_IRAM0` writer - APP CPU invalid access to IRAM0 when cache is disabled"]
pub type CACHE_IA_INT_APP_IRAM0_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_IA_INT_EN_SPEC, O>;
#[doc = "Field `CACHE_IA_INT_APP_IRAM1` reader - APP CPU invalid access to IRAM1 when cache is disabled"]
pub type CACHE_IA_INT_APP_IRAM1_R = crate::BitReader;
#[doc = "Field `CACHE_IA_INT_APP_IRAM1` writer - APP CPU invalid access to IRAM1 when cache is disabled"]
pub type CACHE_IA_INT_APP_IRAM1_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_IA_INT_EN_SPEC, O>;
#[doc = "Field `CACHE_IA_INT_APP_IROM0` reader - APP CPU invalid access to IROM0 when cache is disabled"]
pub type CACHE_IA_INT_APP_IROM0_R = crate::BitReader;
#[doc = "Field `CACHE_IA_INT_APP_IROM0` writer - APP CPU invalid access to IROM0 when cache is disabled"]
pub type CACHE_IA_INT_APP_IROM0_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_IA_INT_EN_SPEC, O>;
#[doc = "Field `CACHE_IA_INT_APP_DRAM1` reader - APP CPU invalid access to DRAM1 when cache is disabled"]
pub type CACHE_IA_INT_APP_DRAM1_R = crate::BitReader;
#[doc = "Field `CACHE_IA_INT_APP_DRAM1` writer - APP CPU invalid access to DRAM1 when cache is disabled"]
pub type CACHE_IA_INT_APP_DRAM1_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_IA_INT_EN_SPEC, O>;
#[doc = "Field `CACHE_IA_INT_APP_OPPOSITE` reader - APP CPU invalid access to APP CPU cache when cache disabled"]
pub type CACHE_IA_INT_APP_OPPOSITE_R = crate::BitReader;
#[doc = "Field `CACHE_IA_INT_APP_OPPOSITE` writer - APP CPU invalid access to APP CPU cache when cache disabled"]
pub type CACHE_IA_INT_APP_OPPOSITE_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_IA_INT_EN_SPEC, O>;
#[doc = "Field `CACHE_IA_INT_PRO_DROM0` reader - PRO CPU invalid access to DROM0 when cache is disabled"]
pub type CACHE_IA_INT_PRO_DROM0_R = crate::BitReader;
#[doc = "Field `CACHE_IA_INT_PRO_DROM0` writer - PRO CPU invalid access to DROM0 when cache is disabled"]
pub type CACHE_IA_INT_PRO_DROM0_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_IA_INT_EN_SPEC, O>;
#[doc = "Field `CACHE_IA_INT_PRO_IRAM0` reader - PRO CPU invalid access to IRAM0 when cache is disabled"]
pub type CACHE_IA_INT_PRO_IRAM0_R = crate::BitReader;
#[doc = "Field `CACHE_IA_INT_PRO_IRAM0` writer - PRO CPU invalid access to IRAM0 when cache is disabled"]
pub type CACHE_IA_INT_PRO_IRAM0_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_IA_INT_EN_SPEC, O>;
#[doc = "Field `CACHE_IA_INT_PRO_IRAM1` reader - PRO CPU invalid access to IRAM1 when cache is disabled"]
pub type CACHE_IA_INT_PRO_IRAM1_R = crate::BitReader;
#[doc = "Field `CACHE_IA_INT_PRO_IRAM1` writer - PRO CPU invalid access to IRAM1 when cache is disabled"]
pub type CACHE_IA_INT_PRO_IRAM1_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_IA_INT_EN_SPEC, O>;
#[doc = "Field `CACHE_IA_INT_PRO_IROM0` reader - PRO CPU invalid access to IROM0 when cache is disabled"]
pub type CACHE_IA_INT_PRO_IROM0_R = crate::BitReader;
#[doc = "Field `CACHE_IA_INT_PRO_IROM0` writer - PRO CPU invalid access to IROM0 when cache is disabled"]
pub type CACHE_IA_INT_PRO_IROM0_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_IA_INT_EN_SPEC, O>;
#[doc = "Field `CACHE_IA_INT_PRO_DRAM1` reader - PRO CPU invalid access to DRAM1 when cache is disabled"]
pub type CACHE_IA_INT_PRO_DRAM1_R = crate::BitReader;
#[doc = "Field `CACHE_IA_INT_PRO_DRAM1` writer - PRO CPU invalid access to DRAM1 when cache is disabled"]
pub type CACHE_IA_INT_PRO_DRAM1_W<'a, const O: u8> = crate::BitWriter<'a, CACHE_IA_INT_EN_SPEC, O>;
#[doc = "Field `CACHE_IA_INT_PRO_OPPOSITE` reader - PRO CPU invalid access to APP CPU cache when cache disabled"]
pub type CACHE_IA_INT_PRO_OPPOSITE_R = crate::BitReader;
#[doc = "Field `CACHE_IA_INT_PRO_OPPOSITE` writer - PRO CPU invalid access to APP CPU cache when cache disabled"]
pub type CACHE_IA_INT_PRO_OPPOSITE_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_IA_INT_EN_SPEC, O>;
impl R {
    #[doc = "Bits 0:27 - Interrupt enable bits for various invalid cache access reasons"]
    #[inline(always)]
    pub fn cache_ia_int_en(&self) -> CACHE_IA_INT_EN_R {
        CACHE_IA_INT_EN_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 0 - APP CPU invalid access to DROM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_drom0(&self) -> CACHE_IA_INT_APP_DROM0_R {
        CACHE_IA_INT_APP_DROM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - APP CPU invalid access to IRAM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_iram0(&self) -> CACHE_IA_INT_APP_IRAM0_R {
        CACHE_IA_INT_APP_IRAM0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - APP CPU invalid access to IRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_iram1(&self) -> CACHE_IA_INT_APP_IRAM1_R {
        CACHE_IA_INT_APP_IRAM1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - APP CPU invalid access to IROM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_irom0(&self) -> CACHE_IA_INT_APP_IROM0_R {
        CACHE_IA_INT_APP_IROM0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - APP CPU invalid access to DRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_dram1(&self) -> CACHE_IA_INT_APP_DRAM1_R {
        CACHE_IA_INT_APP_DRAM1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - APP CPU invalid access to APP CPU cache when cache disabled"]
    #[inline(always)]
    pub fn cache_ia_int_app_opposite(&self) -> CACHE_IA_INT_APP_OPPOSITE_R {
        CACHE_IA_INT_APP_OPPOSITE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 14 - PRO CPU invalid access to DROM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_drom0(&self) -> CACHE_IA_INT_PRO_DROM0_R {
        CACHE_IA_INT_PRO_DROM0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PRO CPU invalid access to IRAM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_iram0(&self) -> CACHE_IA_INT_PRO_IRAM0_R {
        CACHE_IA_INT_PRO_IRAM0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PRO CPU invalid access to IRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_iram1(&self) -> CACHE_IA_INT_PRO_IRAM1_R {
        CACHE_IA_INT_PRO_IRAM1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PRO CPU invalid access to IROM0 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_irom0(&self) -> CACHE_IA_INT_PRO_IROM0_R {
        CACHE_IA_INT_PRO_IROM0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PRO CPU invalid access to DRAM1 when cache is disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_dram1(&self) -> CACHE_IA_INT_PRO_DRAM1_R {
        CACHE_IA_INT_PRO_DRAM1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PRO CPU invalid access to APP CPU cache when cache disabled"]
    #[inline(always)]
    pub fn cache_ia_int_pro_opposite(&self) -> CACHE_IA_INT_PRO_OPPOSITE_R {
        CACHE_IA_INT_PRO_OPPOSITE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_IA_INT_EN")
            .field(
                "cache_ia_int_en",
                &format_args!("{}", self.cache_ia_int_en().bits()),
            )
            .field(
                "cache_ia_int_app_drom0",
                &format_args!("{}", self.cache_ia_int_app_drom0().bit()),
            )
            .field(
                "cache_ia_int_app_iram0",
                &format_args!("{}", self.cache_ia_int_app_iram0().bit()),
            )
            .field(
                "cache_ia_int_app_iram1",
                &format_args!("{}", self.cache_ia_int_app_iram1().bit()),
            )
            .field(
                "cache_ia_int_app_irom0",
                &format_args!("{}", self.cache_ia_int_app_irom0().bit()),
            )
            .field(
                "cache_ia_int_app_dram1",
                &format_args!("{}", self.cache_ia_int_app_dram1().bit()),
            )
            .field(
                "cache_ia_int_app_opposite",
                &format_args!("{}", self.cache_ia_int_app_opposite().bit()),
            )
            .field(
                "cache_ia_int_pro_drom0",
                &format_args!("{}", self.cache_ia_int_pro_drom0().bit()),
            )
            .field(
                "cache_ia_int_pro_iram0",
                &format_args!("{}", self.cache_ia_int_pro_iram0().bit()),
            )
            .field(
                "cache_ia_int_pro_iram1",
                &format_args!("{}", self.cache_ia_int_pro_iram1().bit()),
            )
            .field(
                "cache_ia_int_pro_irom0",
                &format_args!("{}", self.cache_ia_int_pro_irom0().bit()),
            )
            .field(
                "cache_ia_int_pro_dram1",
                &format_args!("{}", self.cache_ia_int_pro_dram1().bit()),
            )
            .field(
                "cache_ia_int_pro_opposite",
                &format_args!("{}", self.cache_ia_int_pro_opposite().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_IA_INT_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:27 - Interrupt enable bits for various invalid cache access reasons"]
    #[inline(always)]
    #[must_use]
    pub fn cache_ia_int_en(&mut self) -> CACHE_IA_INT_EN_W<0> {
        CACHE_IA_INT_EN_W::new(self)
    }
    #[doc = "Bit 0 - APP CPU invalid access to DROM0 when cache is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cache_ia_int_app_drom0(&mut self) -> CACHE_IA_INT_APP_DROM0_W<0> {
        CACHE_IA_INT_APP_DROM0_W::new(self)
    }
    #[doc = "Bit 1 - APP CPU invalid access to IRAM0 when cache is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cache_ia_int_app_iram0(&mut self) -> CACHE_IA_INT_APP_IRAM0_W<1> {
        CACHE_IA_INT_APP_IRAM0_W::new(self)
    }
    #[doc = "Bit 2 - APP CPU invalid access to IRAM1 when cache is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cache_ia_int_app_iram1(&mut self) -> CACHE_IA_INT_APP_IRAM1_W<2> {
        CACHE_IA_INT_APP_IRAM1_W::new(self)
    }
    #[doc = "Bit 3 - APP CPU invalid access to IROM0 when cache is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cache_ia_int_app_irom0(&mut self) -> CACHE_IA_INT_APP_IROM0_W<3> {
        CACHE_IA_INT_APP_IROM0_W::new(self)
    }
    #[doc = "Bit 4 - APP CPU invalid access to DRAM1 when cache is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cache_ia_int_app_dram1(&mut self) -> CACHE_IA_INT_APP_DRAM1_W<4> {
        CACHE_IA_INT_APP_DRAM1_W::new(self)
    }
    #[doc = "Bit 5 - APP CPU invalid access to APP CPU cache when cache disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cache_ia_int_app_opposite(&mut self) -> CACHE_IA_INT_APP_OPPOSITE_W<5> {
        CACHE_IA_INT_APP_OPPOSITE_W::new(self)
    }
    #[doc = "Bit 14 - PRO CPU invalid access to DROM0 when cache is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cache_ia_int_pro_drom0(&mut self) -> CACHE_IA_INT_PRO_DROM0_W<14> {
        CACHE_IA_INT_PRO_DROM0_W::new(self)
    }
    #[doc = "Bit 15 - PRO CPU invalid access to IRAM0 when cache is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cache_ia_int_pro_iram0(&mut self) -> CACHE_IA_INT_PRO_IRAM0_W<15> {
        CACHE_IA_INT_PRO_IRAM0_W::new(self)
    }
    #[doc = "Bit 16 - PRO CPU invalid access to IRAM1 when cache is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cache_ia_int_pro_iram1(&mut self) -> CACHE_IA_INT_PRO_IRAM1_W<16> {
        CACHE_IA_INT_PRO_IRAM1_W::new(self)
    }
    #[doc = "Bit 17 - PRO CPU invalid access to IROM0 when cache is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cache_ia_int_pro_irom0(&mut self) -> CACHE_IA_INT_PRO_IROM0_W<17> {
        CACHE_IA_INT_PRO_IROM0_W::new(self)
    }
    #[doc = "Bit 18 - PRO CPU invalid access to DRAM1 when cache is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cache_ia_int_pro_dram1(&mut self) -> CACHE_IA_INT_PRO_DRAM1_W<18> {
        CACHE_IA_INT_PRO_DRAM1_W::new(self)
    }
    #[doc = "Bit 19 - PRO CPU invalid access to APP CPU cache when cache disabled"]
    #[inline(always)]
    #[must_use]
    pub fn cache_ia_int_pro_opposite(&mut self) -> CACHE_IA_INT_PRO_OPPOSITE_W<19> {
        CACHE_IA_INT_PRO_OPPOSITE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_ia_int_en](index.html) module"]
pub struct CACHE_IA_INT_EN_SPEC;
impl crate::RegisterSpec for CACHE_IA_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_ia_int_en::R](R) reader structure"]
impl crate::Readable for CACHE_IA_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_ia_int_en::W](W) writer structure"]
impl crate::Writable for CACHE_IA_INT_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_IA_INT_EN to value 0"]
impl crate::Resettable for CACHE_IA_INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
