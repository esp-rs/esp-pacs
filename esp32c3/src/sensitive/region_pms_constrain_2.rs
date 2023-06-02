#[doc = "Register `REGION_PMS_CONSTRAIN_2` reader"]
pub struct R(crate::R<REGION_PMS_CONSTRAIN_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGION_PMS_CONSTRAIN_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGION_PMS_CONSTRAIN_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGION_PMS_CONSTRAIN_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGION_PMS_CONSTRAIN_2` writer"]
pub struct W(crate::W<REGION_PMS_CONSTRAIN_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGION_PMS_CONSTRAIN_2_SPEC>;
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
impl From<crate::W<REGION_PMS_CONSTRAIN_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGION_PMS_CONSTRAIN_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_0` reader - region_pms_constrain_world_1_area_0"]
pub type REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_R = crate::FieldReader;
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_0` writer - region_pms_constrain_world_1_area_0"]
pub type REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, REGION_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_1` reader - region_pms_constrain_world_1_area_1"]
pub type REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_R = crate::FieldReader;
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_1` writer - region_pms_constrain_world_1_area_1"]
pub type REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, REGION_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_2` reader - region_pms_constrain_world_1_area_2"]
pub type REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_R = crate::FieldReader;
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_2` writer - region_pms_constrain_world_1_area_2"]
pub type REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, REGION_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_3` reader - region_pms_constrain_world_1_area_3"]
pub type REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_R = crate::FieldReader;
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_3` writer - region_pms_constrain_world_1_area_3"]
pub type REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, REGION_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_4` reader - region_pms_constrain_world_1_area_4"]
pub type REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_R = crate::FieldReader;
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_4` writer - region_pms_constrain_world_1_area_4"]
pub type REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_W<'a, const O: u8> =
    crate::FieldWriter<'a, REGION_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_5` reader - region_pms_constrain_world_1_area_5"]
pub type REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_R = crate::FieldReader;
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_5` writer - region_pms_constrain_world_1_area_5"]
pub type REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_W<'a, const O: u8> =
    crate::FieldWriter<'a, REGION_PMS_CONSTRAIN_2_SPEC, 2, O>;
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_6` reader - region_pms_constrain_world_1_area_6"]
pub type REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_R = crate::FieldReader;
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_6` writer - region_pms_constrain_world_1_area_6"]
pub type REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_W<'a, const O: u8> =
    crate::FieldWriter<'a, REGION_PMS_CONSTRAIN_2_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - region_pms_constrain_world_1_area_0"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_0(&self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_R {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - region_pms_constrain_world_1_area_1"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_1(&self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_R {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - region_pms_constrain_world_1_area_2"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_2(&self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_R {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - region_pms_constrain_world_1_area_3"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_3(&self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_R {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - region_pms_constrain_world_1_area_4"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_4(&self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_R {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - region_pms_constrain_world_1_area_5"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_5(&self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_R {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - region_pms_constrain_world_1_area_6"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_6(&self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_R {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_R::new(((self.bits >> 12) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION_PMS_CONSTRAIN_2")
            .field(
                "region_pms_constrain_world_1_area_0",
                &format_args!("{}", self.region_pms_constrain_world_1_area_0().bits()),
            )
            .field(
                "region_pms_constrain_world_1_area_1",
                &format_args!("{}", self.region_pms_constrain_world_1_area_1().bits()),
            )
            .field(
                "region_pms_constrain_world_1_area_2",
                &format_args!("{}", self.region_pms_constrain_world_1_area_2().bits()),
            )
            .field(
                "region_pms_constrain_world_1_area_3",
                &format_args!("{}", self.region_pms_constrain_world_1_area_3().bits()),
            )
            .field(
                "region_pms_constrain_world_1_area_4",
                &format_args!("{}", self.region_pms_constrain_world_1_area_4().bits()),
            )
            .field(
                "region_pms_constrain_world_1_area_5",
                &format_args!("{}", self.region_pms_constrain_world_1_area_5().bits()),
            )
            .field(
                "region_pms_constrain_world_1_area_6",
                &format_args!("{}", self.region_pms_constrain_world_1_area_6().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGION_PMS_CONSTRAIN_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - region_pms_constrain_world_1_area_0"]
    #[inline(always)]
    #[must_use]
    pub fn region_pms_constrain_world_1_area_0(
        &mut self,
    ) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_W<0> {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_W::new(self)
    }
    #[doc = "Bits 2:3 - region_pms_constrain_world_1_area_1"]
    #[inline(always)]
    #[must_use]
    pub fn region_pms_constrain_world_1_area_1(
        &mut self,
    ) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_W<2> {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_W::new(self)
    }
    #[doc = "Bits 4:5 - region_pms_constrain_world_1_area_2"]
    #[inline(always)]
    #[must_use]
    pub fn region_pms_constrain_world_1_area_2(
        &mut self,
    ) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_W<4> {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_W::new(self)
    }
    #[doc = "Bits 6:7 - region_pms_constrain_world_1_area_3"]
    #[inline(always)]
    #[must_use]
    pub fn region_pms_constrain_world_1_area_3(
        &mut self,
    ) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_W<6> {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_W::new(self)
    }
    #[doc = "Bits 8:9 - region_pms_constrain_world_1_area_4"]
    #[inline(always)]
    #[must_use]
    pub fn region_pms_constrain_world_1_area_4(
        &mut self,
    ) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_W<8> {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_W::new(self)
    }
    #[doc = "Bits 10:11 - region_pms_constrain_world_1_area_5"]
    #[inline(always)]
    #[must_use]
    pub fn region_pms_constrain_world_1_area_5(
        &mut self,
    ) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_W<10> {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_W::new(self)
    }
    #[doc = "Bits 12:13 - region_pms_constrain_world_1_area_6"]
    #[inline(always)]
    #[must_use]
    pub fn region_pms_constrain_world_1_area_6(
        &mut self,
    ) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_W<12> {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_pms_constrain_2](index.html) module"]
pub struct REGION_PMS_CONSTRAIN_2_SPEC;
impl crate::RegisterSpec for REGION_PMS_CONSTRAIN_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [region_pms_constrain_2::R](R) reader structure"]
impl crate::Readable for REGION_PMS_CONSTRAIN_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [region_pms_constrain_2::W](W) writer structure"]
impl crate::Writable for REGION_PMS_CONSTRAIN_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGION_PMS_CONSTRAIN_2 to value 0x3fff"]
impl crate::Resettable for REGION_PMS_CONSTRAIN_2_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff;
}
