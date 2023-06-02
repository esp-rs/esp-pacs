#[doc = "Register `CORE_1_REGION_PMS_CONSTRAIN_1` reader"]
pub struct R(crate::R<CORE_1_REGION_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_REGION_PMS_CONSTRAIN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_REGION_PMS_CONSTRAIN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_REGION_PMS_CONSTRAIN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_1_REGION_PMS_CONSTRAIN_1` writer"]
pub struct W(crate::W<CORE_1_REGION_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_REGION_PMS_CONSTRAIN_1_SPEC>;
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
impl From<crate::W<CORE_1_REGION_PMS_CONSTRAIN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_REGION_PMS_CONSTRAIN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_0` reader - Region 0 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_0_R = crate::FieldReader;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_0` writer - Region 0 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_REGION_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_1` reader - Region 1 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_1_R = crate::FieldReader;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_1` writer - Region 1 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_REGION_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_2` reader - Region 2 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_2_R = crate::FieldReader;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_2` writer - Region 2 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_REGION_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_3` reader - Region 3 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_3_R = crate::FieldReader;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_3` writer - Region 3 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_REGION_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_4` reader - Region 4 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_4_R = crate::FieldReader;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_4` writer - Region 4 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_4_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_REGION_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_5` reader - Region 5 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_5_R = crate::FieldReader;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_5` writer - Region 5 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_5_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_REGION_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_6` reader - Region 6 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_6_R = crate::FieldReader;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_6` writer - Region 6 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_6_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_REGION_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_7` reader - Region 7 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_7_R = crate::FieldReader;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_7` writer - Region 7 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_7_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_REGION_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_8` reader - Region 8 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_8_R = crate::FieldReader;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_8` writer - Region 8 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_8_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_REGION_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_9` reader - Region 9 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_9_R = crate::FieldReader;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_9` writer - Region 9 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_9_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_REGION_PMS_CONSTRAIN_1_SPEC, 2, O>;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_10` reader - Region 10 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_10_R = crate::FieldReader;
#[doc = "Field `CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_10` writer - Region 10 permission in world 0 for core1."]
pub type CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_10_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_REGION_PMS_CONSTRAIN_1_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Region 0 permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_world_0_area_0(
        &self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_0_R {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Region 1 permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_world_0_area_1(
        &self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_1_R {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Region 2 permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_world_0_area_2(
        &self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_2_R {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Region 3 permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_world_0_area_3(
        &self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_3_R {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Region 4 permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_world_0_area_4(
        &self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_4_R {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Region 5 permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_world_0_area_5(
        &self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_5_R {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Region 6 permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_world_0_area_6(
        &self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_6_R {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Region 7 permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_world_0_area_7(
        &self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_7_R {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Region 8 permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_world_0_area_8(
        &self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_8_R {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Region 9 permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_world_0_area_9(
        &self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_9_R {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Region 10 permission in world 0 for core1."]
    #[inline(always)]
    pub fn core_1_region_pms_constrain_world_0_area_10(
        &self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_10_R {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_10_R::new(((self.bits >> 20) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_REGION_PMS_CONSTRAIN_1")
            .field(
                "core_1_region_pms_constrain_world_0_area_0",
                &format_args!(
                    "{}",
                    self.core_1_region_pms_constrain_world_0_area_0().bits()
                ),
            )
            .field(
                "core_1_region_pms_constrain_world_0_area_1",
                &format_args!(
                    "{}",
                    self.core_1_region_pms_constrain_world_0_area_1().bits()
                ),
            )
            .field(
                "core_1_region_pms_constrain_world_0_area_2",
                &format_args!(
                    "{}",
                    self.core_1_region_pms_constrain_world_0_area_2().bits()
                ),
            )
            .field(
                "core_1_region_pms_constrain_world_0_area_3",
                &format_args!(
                    "{}",
                    self.core_1_region_pms_constrain_world_0_area_3().bits()
                ),
            )
            .field(
                "core_1_region_pms_constrain_world_0_area_4",
                &format_args!(
                    "{}",
                    self.core_1_region_pms_constrain_world_0_area_4().bits()
                ),
            )
            .field(
                "core_1_region_pms_constrain_world_0_area_5",
                &format_args!(
                    "{}",
                    self.core_1_region_pms_constrain_world_0_area_5().bits()
                ),
            )
            .field(
                "core_1_region_pms_constrain_world_0_area_6",
                &format_args!(
                    "{}",
                    self.core_1_region_pms_constrain_world_0_area_6().bits()
                ),
            )
            .field(
                "core_1_region_pms_constrain_world_0_area_7",
                &format_args!(
                    "{}",
                    self.core_1_region_pms_constrain_world_0_area_7().bits()
                ),
            )
            .field(
                "core_1_region_pms_constrain_world_0_area_8",
                &format_args!(
                    "{}",
                    self.core_1_region_pms_constrain_world_0_area_8().bits()
                ),
            )
            .field(
                "core_1_region_pms_constrain_world_0_area_9",
                &format_args!(
                    "{}",
                    self.core_1_region_pms_constrain_world_0_area_9().bits()
                ),
            )
            .field(
                "core_1_region_pms_constrain_world_0_area_10",
                &format_args!(
                    "{}",
                    self.core_1_region_pms_constrain_world_0_area_10().bits()
                ),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_REGION_PMS_CONSTRAIN_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Region 0 permission in world 0 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_region_pms_constrain_world_0_area_0(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_0_W<0> {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_0_W::new(self)
    }
    #[doc = "Bits 2:3 - Region 1 permission in world 0 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_region_pms_constrain_world_0_area_1(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_1_W<2> {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_1_W::new(self)
    }
    #[doc = "Bits 4:5 - Region 2 permission in world 0 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_region_pms_constrain_world_0_area_2(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_2_W<4> {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_2_W::new(self)
    }
    #[doc = "Bits 6:7 - Region 3 permission in world 0 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_region_pms_constrain_world_0_area_3(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_3_W<6> {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_3_W::new(self)
    }
    #[doc = "Bits 8:9 - Region 4 permission in world 0 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_region_pms_constrain_world_0_area_4(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_4_W<8> {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_4_W::new(self)
    }
    #[doc = "Bits 10:11 - Region 5 permission in world 0 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_region_pms_constrain_world_0_area_5(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_5_W<10> {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_5_W::new(self)
    }
    #[doc = "Bits 12:13 - Region 6 permission in world 0 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_region_pms_constrain_world_0_area_6(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_6_W<12> {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_6_W::new(self)
    }
    #[doc = "Bits 14:15 - Region 7 permission in world 0 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_region_pms_constrain_world_0_area_7(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_7_W<14> {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_7_W::new(self)
    }
    #[doc = "Bits 16:17 - Region 8 permission in world 0 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_region_pms_constrain_world_0_area_8(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_8_W<16> {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_8_W::new(self)
    }
    #[doc = "Bits 18:19 - Region 9 permission in world 0 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_region_pms_constrain_world_0_area_9(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_9_W<18> {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_9_W::new(self)
    }
    #[doc = "Bits 20:21 - Region 10 permission in world 0 for core1."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_region_pms_constrain_world_0_area_10(
        &mut self,
    ) -> CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_10_W<20> {
        CORE_1_REGION_PMS_CONSTRAIN_WORLD_0_AREA_10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core1 region permission register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_region_pms_constrain_1](index.html) module"]
pub struct CORE_1_REGION_PMS_CONSTRAIN_1_SPEC;
impl crate::RegisterSpec for CORE_1_REGION_PMS_CONSTRAIN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_region_pms_constrain_1::R](R) reader structure"]
impl crate::Readable for CORE_1_REGION_PMS_CONSTRAIN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_region_pms_constrain_1::W](W) writer structure"]
impl crate::Writable for CORE_1_REGION_PMS_CONSTRAIN_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_1_REGION_PMS_CONSTRAIN_1 to value 0x003f_ffff"]
impl crate::Resettable for CORE_1_REGION_PMS_CONSTRAIN_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x003f_ffff;
}
