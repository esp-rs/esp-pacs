#[doc = "Register `REGION7_PMS_ATTR` reader"]
pub type R = crate::R<REGION7_PMS_ATTR_SPEC>;
#[doc = "Register `REGION7_PMS_ATTR` writer"]
pub type W = crate::W<REGION7_PMS_ATTR_SPEC>;
#[doc = "Field `REGION7_R0_PMS_X` reader - Region execute authority in REE_MODE0"]
pub type REGION7_R0_PMS_X_R = crate::BitReader;
#[doc = "Field `REGION7_R0_PMS_X` writer - Region execute authority in REE_MODE0"]
pub type REGION7_R0_PMS_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION7_R0_PMS_W` reader - Region write authority in REE_MODE0"]
pub type REGION7_R0_PMS_W_R = crate::BitReader;
#[doc = "Field `REGION7_R0_PMS_W` writer - Region write authority in REE_MODE0"]
pub type REGION7_R0_PMS_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION7_R0_PMS_R` reader - Region read authority in REE_MODE0"]
pub type REGION7_R0_PMS_R_R = crate::BitReader;
#[doc = "Field `REGION7_R0_PMS_R` writer - Region read authority in REE_MODE0"]
pub type REGION7_R0_PMS_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION7_R1_PMS_X` reader - Region execute authority in REE_MODE1"]
pub type REGION7_R1_PMS_X_R = crate::BitReader;
#[doc = "Field `REGION7_R1_PMS_X` writer - Region execute authority in REE_MODE1"]
pub type REGION7_R1_PMS_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION7_R1_PMS_W` reader - Region write authority in REE_MODE1"]
pub type REGION7_R1_PMS_W_R = crate::BitReader;
#[doc = "Field `REGION7_R1_PMS_W` writer - Region write authority in REE_MODE1"]
pub type REGION7_R1_PMS_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION7_R1_PMS_R` reader - Region read authority in REE_MODE1"]
pub type REGION7_R1_PMS_R_R = crate::BitReader;
#[doc = "Field `REGION7_R1_PMS_R` writer - Region read authority in REE_MODE1"]
pub type REGION7_R1_PMS_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION7_R2_PMS_X` reader - Region execute authority in REE_MODE2"]
pub type REGION7_R2_PMS_X_R = crate::BitReader;
#[doc = "Field `REGION7_R2_PMS_X` writer - Region execute authority in REE_MODE2"]
pub type REGION7_R2_PMS_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION7_R2_PMS_W` reader - Region write authority in REE_MODE2"]
pub type REGION7_R2_PMS_W_R = crate::BitReader;
#[doc = "Field `REGION7_R2_PMS_W` writer - Region write authority in REE_MODE2"]
pub type REGION7_R2_PMS_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION7_R2_PMS_R` reader - Region read authority in REE_MODE2"]
pub type REGION7_R2_PMS_R_R = crate::BitReader;
#[doc = "Field `REGION7_R2_PMS_R` writer - Region read authority in REE_MODE2"]
pub type REGION7_R2_PMS_R_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region execute authority in REE_MODE0"]
    #[inline(always)]
    pub fn region7_r0_pms_x(&self) -> REGION7_R0_PMS_X_R {
        REGION7_R0_PMS_X_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Region write authority in REE_MODE0"]
    #[inline(always)]
    pub fn region7_r0_pms_w(&self) -> REGION7_R0_PMS_W_R {
        REGION7_R0_PMS_W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Region read authority in REE_MODE0"]
    #[inline(always)]
    pub fn region7_r0_pms_r(&self) -> REGION7_R0_PMS_R_R {
        REGION7_R0_PMS_R_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Region execute authority in REE_MODE1"]
    #[inline(always)]
    pub fn region7_r1_pms_x(&self) -> REGION7_R1_PMS_X_R {
        REGION7_R1_PMS_X_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Region write authority in REE_MODE1"]
    #[inline(always)]
    pub fn region7_r1_pms_w(&self) -> REGION7_R1_PMS_W_R {
        REGION7_R1_PMS_W_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Region read authority in REE_MODE1"]
    #[inline(always)]
    pub fn region7_r1_pms_r(&self) -> REGION7_R1_PMS_R_R {
        REGION7_R1_PMS_R_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Region execute authority in REE_MODE2"]
    #[inline(always)]
    pub fn region7_r2_pms_x(&self) -> REGION7_R2_PMS_X_R {
        REGION7_R2_PMS_X_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Region write authority in REE_MODE2"]
    #[inline(always)]
    pub fn region7_r2_pms_w(&self) -> REGION7_R2_PMS_W_R {
        REGION7_R2_PMS_W_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Region read authority in REE_MODE2"]
    #[inline(always)]
    pub fn region7_r2_pms_r(&self) -> REGION7_R2_PMS_R_R {
        REGION7_R2_PMS_R_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION7_PMS_ATTR")
            .field(
                "region7_r0_pms_x",
                &format_args!("{}", self.region7_r0_pms_x().bit()),
            )
            .field(
                "region7_r0_pms_w",
                &format_args!("{}", self.region7_r0_pms_w().bit()),
            )
            .field(
                "region7_r0_pms_r",
                &format_args!("{}", self.region7_r0_pms_r().bit()),
            )
            .field(
                "region7_r1_pms_x",
                &format_args!("{}", self.region7_r1_pms_x().bit()),
            )
            .field(
                "region7_r1_pms_w",
                &format_args!("{}", self.region7_r1_pms_w().bit()),
            )
            .field(
                "region7_r1_pms_r",
                &format_args!("{}", self.region7_r1_pms_r().bit()),
            )
            .field(
                "region7_r2_pms_x",
                &format_args!("{}", self.region7_r2_pms_x().bit()),
            )
            .field(
                "region7_r2_pms_w",
                &format_args!("{}", self.region7_r2_pms_w().bit()),
            )
            .field(
                "region7_r2_pms_r",
                &format_args!("{}", self.region7_r2_pms_r().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REGION7_PMS_ATTR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Region execute authority in REE_MODE0"]
    #[inline(always)]
    #[must_use]
    pub fn region7_r0_pms_x(&mut self) -> REGION7_R0_PMS_X_W<REGION7_PMS_ATTR_SPEC> {
        REGION7_R0_PMS_X_W::new(self, 0)
    }
    #[doc = "Bit 1 - Region write authority in REE_MODE0"]
    #[inline(always)]
    #[must_use]
    pub fn region7_r0_pms_w(&mut self) -> REGION7_R0_PMS_W_W<REGION7_PMS_ATTR_SPEC> {
        REGION7_R0_PMS_W_W::new(self, 1)
    }
    #[doc = "Bit 2 - Region read authority in REE_MODE0"]
    #[inline(always)]
    #[must_use]
    pub fn region7_r0_pms_r(&mut self) -> REGION7_R0_PMS_R_W<REGION7_PMS_ATTR_SPEC> {
        REGION7_R0_PMS_R_W::new(self, 2)
    }
    #[doc = "Bit 4 - Region execute authority in REE_MODE1"]
    #[inline(always)]
    #[must_use]
    pub fn region7_r1_pms_x(&mut self) -> REGION7_R1_PMS_X_W<REGION7_PMS_ATTR_SPEC> {
        REGION7_R1_PMS_X_W::new(self, 4)
    }
    #[doc = "Bit 5 - Region write authority in REE_MODE1"]
    #[inline(always)]
    #[must_use]
    pub fn region7_r1_pms_w(&mut self) -> REGION7_R1_PMS_W_W<REGION7_PMS_ATTR_SPEC> {
        REGION7_R1_PMS_W_W::new(self, 5)
    }
    #[doc = "Bit 6 - Region read authority in REE_MODE1"]
    #[inline(always)]
    #[must_use]
    pub fn region7_r1_pms_r(&mut self) -> REGION7_R1_PMS_R_W<REGION7_PMS_ATTR_SPEC> {
        REGION7_R1_PMS_R_W::new(self, 6)
    }
    #[doc = "Bit 8 - Region execute authority in REE_MODE2"]
    #[inline(always)]
    #[must_use]
    pub fn region7_r2_pms_x(&mut self) -> REGION7_R2_PMS_X_W<REGION7_PMS_ATTR_SPEC> {
        REGION7_R2_PMS_X_W::new(self, 8)
    }
    #[doc = "Bit 9 - Region write authority in REE_MODE2"]
    #[inline(always)]
    #[must_use]
    pub fn region7_r2_pms_w(&mut self) -> REGION7_R2_PMS_W_W<REGION7_PMS_ATTR_SPEC> {
        REGION7_R2_PMS_W_W::new(self, 9)
    }
    #[doc = "Bit 10 - Region read authority in REE_MODE2"]
    #[inline(always)]
    #[must_use]
    pub fn region7_r2_pms_r(&mut self) -> REGION7_R2_PMS_R_W<REGION7_PMS_ATTR_SPEC> {
        REGION7_R2_PMS_R_W::new(self, 10)
    }
}
#[doc = "Region access authority attribute register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region7_pms_attr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region7_pms_attr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGION7_PMS_ATTR_SPEC;
impl crate::RegisterSpec for REGION7_PMS_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region7_pms_attr::R`](R) reader structure"]
impl crate::Readable for REGION7_PMS_ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`region7_pms_attr::W`](W) writer structure"]
impl crate::Writable for REGION7_PMS_ATTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGION7_PMS_ATTR to value 0"]
impl crate::Resettable for REGION7_PMS_ATTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
