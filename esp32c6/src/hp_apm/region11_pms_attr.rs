#[doc = "Register `REGION11_PMS_ATTR` reader"]
pub struct R(crate::R<REGION11_PMS_ATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGION11_PMS_ATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGION11_PMS_ATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGION11_PMS_ATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGION11_PMS_ATTR` writer"]
pub struct W(crate::W<REGION11_PMS_ATTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGION11_PMS_ATTR_SPEC>;
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
impl From<crate::W<REGION11_PMS_ATTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGION11_PMS_ATTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION11_R0_PMS_X` reader - Region execute authority in REE_MODE0"]
pub type REGION11_R0_PMS_X_R = crate::BitReader;
#[doc = "Field `REGION11_R0_PMS_X` writer - Region execute authority in REE_MODE0"]
pub type REGION11_R0_PMS_X_W<'a, const O: u8> = crate::BitWriter<'a, REGION11_PMS_ATTR_SPEC, O>;
#[doc = "Field `REGION11_R0_PMS_W` reader - Region write authority in REE_MODE0"]
pub type REGION11_R0_PMS_W_R = crate::BitReader;
#[doc = "Field `REGION11_R0_PMS_W` writer - Region write authority in REE_MODE0"]
pub type REGION11_R0_PMS_W_W<'a, const O: u8> = crate::BitWriter<'a, REGION11_PMS_ATTR_SPEC, O>;
#[doc = "Field `REGION11_R0_PMS_R` reader - Region read authority in REE_MODE0"]
pub type REGION11_R0_PMS_R_R = crate::BitReader;
#[doc = "Field `REGION11_R0_PMS_R` writer - Region read authority in REE_MODE0"]
pub type REGION11_R0_PMS_R_W<'a, const O: u8> = crate::BitWriter<'a, REGION11_PMS_ATTR_SPEC, O>;
#[doc = "Field `REGION11_R1_PMS_X` reader - Region execute authority in REE_MODE1"]
pub type REGION11_R1_PMS_X_R = crate::BitReader;
#[doc = "Field `REGION11_R1_PMS_X` writer - Region execute authority in REE_MODE1"]
pub type REGION11_R1_PMS_X_W<'a, const O: u8> = crate::BitWriter<'a, REGION11_PMS_ATTR_SPEC, O>;
#[doc = "Field `REGION11_R1_PMS_W` reader - Region write authority in REE_MODE1"]
pub type REGION11_R1_PMS_W_R = crate::BitReader;
#[doc = "Field `REGION11_R1_PMS_W` writer - Region write authority in REE_MODE1"]
pub type REGION11_R1_PMS_W_W<'a, const O: u8> = crate::BitWriter<'a, REGION11_PMS_ATTR_SPEC, O>;
#[doc = "Field `REGION11_R1_PMS_R` reader - Region read authority in REE_MODE1"]
pub type REGION11_R1_PMS_R_R = crate::BitReader;
#[doc = "Field `REGION11_R1_PMS_R` writer - Region read authority in REE_MODE1"]
pub type REGION11_R1_PMS_R_W<'a, const O: u8> = crate::BitWriter<'a, REGION11_PMS_ATTR_SPEC, O>;
#[doc = "Field `REGION11_R2_PMS_X` reader - Region execute authority in REE_MODE2"]
pub type REGION11_R2_PMS_X_R = crate::BitReader;
#[doc = "Field `REGION11_R2_PMS_X` writer - Region execute authority in REE_MODE2"]
pub type REGION11_R2_PMS_X_W<'a, const O: u8> = crate::BitWriter<'a, REGION11_PMS_ATTR_SPEC, O>;
#[doc = "Field `REGION11_R2_PMS_W` reader - Region write authority in REE_MODE2"]
pub type REGION11_R2_PMS_W_R = crate::BitReader;
#[doc = "Field `REGION11_R2_PMS_W` writer - Region write authority in REE_MODE2"]
pub type REGION11_R2_PMS_W_W<'a, const O: u8> = crate::BitWriter<'a, REGION11_PMS_ATTR_SPEC, O>;
#[doc = "Field `REGION11_R2_PMS_R` reader - Region read authority in REE_MODE2"]
pub type REGION11_R2_PMS_R_R = crate::BitReader;
#[doc = "Field `REGION11_R2_PMS_R` writer - Region read authority in REE_MODE2"]
pub type REGION11_R2_PMS_R_W<'a, const O: u8> = crate::BitWriter<'a, REGION11_PMS_ATTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Region execute authority in REE_MODE0"]
    #[inline(always)]
    pub fn region11_r0_pms_x(&self) -> REGION11_R0_PMS_X_R {
        REGION11_R0_PMS_X_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Region write authority in REE_MODE0"]
    #[inline(always)]
    pub fn region11_r0_pms_w(&self) -> REGION11_R0_PMS_W_R {
        REGION11_R0_PMS_W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Region read authority in REE_MODE0"]
    #[inline(always)]
    pub fn region11_r0_pms_r(&self) -> REGION11_R0_PMS_R_R {
        REGION11_R0_PMS_R_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Region execute authority in REE_MODE1"]
    #[inline(always)]
    pub fn region11_r1_pms_x(&self) -> REGION11_R1_PMS_X_R {
        REGION11_R1_PMS_X_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Region write authority in REE_MODE1"]
    #[inline(always)]
    pub fn region11_r1_pms_w(&self) -> REGION11_R1_PMS_W_R {
        REGION11_R1_PMS_W_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Region read authority in REE_MODE1"]
    #[inline(always)]
    pub fn region11_r1_pms_r(&self) -> REGION11_R1_PMS_R_R {
        REGION11_R1_PMS_R_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Region execute authority in REE_MODE2"]
    #[inline(always)]
    pub fn region11_r2_pms_x(&self) -> REGION11_R2_PMS_X_R {
        REGION11_R2_PMS_X_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Region write authority in REE_MODE2"]
    #[inline(always)]
    pub fn region11_r2_pms_w(&self) -> REGION11_R2_PMS_W_R {
        REGION11_R2_PMS_W_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Region read authority in REE_MODE2"]
    #[inline(always)]
    pub fn region11_r2_pms_r(&self) -> REGION11_R2_PMS_R_R {
        REGION11_R2_PMS_R_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION11_PMS_ATTR")
            .field(
                "region11_r0_pms_x",
                &format_args!("{}", self.region11_r0_pms_x().bit()),
            )
            .field(
                "region11_r0_pms_w",
                &format_args!("{}", self.region11_r0_pms_w().bit()),
            )
            .field(
                "region11_r0_pms_r",
                &format_args!("{}", self.region11_r0_pms_r().bit()),
            )
            .field(
                "region11_r1_pms_x",
                &format_args!("{}", self.region11_r1_pms_x().bit()),
            )
            .field(
                "region11_r1_pms_w",
                &format_args!("{}", self.region11_r1_pms_w().bit()),
            )
            .field(
                "region11_r1_pms_r",
                &format_args!("{}", self.region11_r1_pms_r().bit()),
            )
            .field(
                "region11_r2_pms_x",
                &format_args!("{}", self.region11_r2_pms_x().bit()),
            )
            .field(
                "region11_r2_pms_w",
                &format_args!("{}", self.region11_r2_pms_w().bit()),
            )
            .field(
                "region11_r2_pms_r",
                &format_args!("{}", self.region11_r2_pms_r().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGION11_PMS_ATTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Region execute authority in REE_MODE0"]
    #[inline(always)]
    #[must_use]
    pub fn region11_r0_pms_x(&mut self) -> REGION11_R0_PMS_X_W<0> {
        REGION11_R0_PMS_X_W::new(self)
    }
    #[doc = "Bit 1 - Region write authority in REE_MODE0"]
    #[inline(always)]
    #[must_use]
    pub fn region11_r0_pms_w(&mut self) -> REGION11_R0_PMS_W_W<1> {
        REGION11_R0_PMS_W_W::new(self)
    }
    #[doc = "Bit 2 - Region read authority in REE_MODE0"]
    #[inline(always)]
    #[must_use]
    pub fn region11_r0_pms_r(&mut self) -> REGION11_R0_PMS_R_W<2> {
        REGION11_R0_PMS_R_W::new(self)
    }
    #[doc = "Bit 4 - Region execute authority in REE_MODE1"]
    #[inline(always)]
    #[must_use]
    pub fn region11_r1_pms_x(&mut self) -> REGION11_R1_PMS_X_W<4> {
        REGION11_R1_PMS_X_W::new(self)
    }
    #[doc = "Bit 5 - Region write authority in REE_MODE1"]
    #[inline(always)]
    #[must_use]
    pub fn region11_r1_pms_w(&mut self) -> REGION11_R1_PMS_W_W<5> {
        REGION11_R1_PMS_W_W::new(self)
    }
    #[doc = "Bit 6 - Region read authority in REE_MODE1"]
    #[inline(always)]
    #[must_use]
    pub fn region11_r1_pms_r(&mut self) -> REGION11_R1_PMS_R_W<6> {
        REGION11_R1_PMS_R_W::new(self)
    }
    #[doc = "Bit 8 - Region execute authority in REE_MODE2"]
    #[inline(always)]
    #[must_use]
    pub fn region11_r2_pms_x(&mut self) -> REGION11_R2_PMS_X_W<8> {
        REGION11_R2_PMS_X_W::new(self)
    }
    #[doc = "Bit 9 - Region write authority in REE_MODE2"]
    #[inline(always)]
    #[must_use]
    pub fn region11_r2_pms_w(&mut self) -> REGION11_R2_PMS_W_W<9> {
        REGION11_R2_PMS_W_W::new(self)
    }
    #[doc = "Bit 10 - Region read authority in REE_MODE2"]
    #[inline(always)]
    #[must_use]
    pub fn region11_r2_pms_r(&mut self) -> REGION11_R2_PMS_R_W<10> {
        REGION11_R2_PMS_R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region access authority attribute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region11_pms_attr](index.html) module"]
pub struct REGION11_PMS_ATTR_SPEC;
impl crate::RegisterSpec for REGION11_PMS_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [region11_pms_attr::R](R) reader structure"]
impl crate::Readable for REGION11_PMS_ATTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [region11_pms_attr::W](W) writer structure"]
impl crate::Writable for REGION11_PMS_ATTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGION11_PMS_ATTR to value 0"]
impl crate::Resettable for REGION11_PMS_ATTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
