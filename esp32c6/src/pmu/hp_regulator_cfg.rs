#[doc = "Register `HP_REGULATOR_CFG` reader"]
pub struct R(crate::R<HP_REGULATOR_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_REGULATOR_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_REGULATOR_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_REGULATOR_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_REGULATOR_CFG` writer"]
pub struct W(crate::W<HP_REGULATOR_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_REGULATOR_CFG_SPEC>;
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
impl From<crate::W<HP_REGULATOR_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_REGULATOR_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIG_REGULATOR_EN_CAL` reader - need_des"]
pub type DIG_REGULATOR_EN_CAL_R = crate::BitReader;
#[doc = "Field `DIG_REGULATOR_EN_CAL` writer - need_des"]
pub type DIG_REGULATOR_EN_CAL_W<'a, const O: u8> = crate::BitWriter<'a, HP_REGULATOR_CFG_SPEC, O>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn dig_regulator_en_cal(&self) -> DIG_REGULATOR_EN_CAL_R {
        DIG_REGULATOR_EN_CAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_REGULATOR_CFG")
            .field(
                "dig_regulator_en_cal",
                &format_args!("{}", self.dig_regulator_en_cal().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_REGULATOR_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dig_regulator_en_cal(&mut self) -> DIG_REGULATOR_EN_CAL_W<31> {
        DIG_REGULATOR_EN_CAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_regulator_cfg](index.html) module"]
pub struct HP_REGULATOR_CFG_SPEC;
impl crate::RegisterSpec for HP_REGULATOR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_regulator_cfg::R](R) reader structure"]
impl crate::Readable for HP_REGULATOR_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_regulator_cfg::W](W) writer structure"]
impl crate::Writable for HP_REGULATOR_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_REGULATOR_CFG to value 0"]
impl crate::Resettable for HP_REGULATOR_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
