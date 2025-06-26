#[doc = "Register `REGION1_PMS_ATTR` reader"]
pub type R = crate::R<REGION1_PMS_ATTR_SPEC>;
#[doc = "Register `REGION1_PMS_ATTR` writer"]
pub type W = crate::W<REGION1_PMS_ATTR_SPEC>;
#[doc = "Field `REGION1_R0_PMS_X` reader - Region execute authority in REE_MODE0"]
pub type REGION1_R0_PMS_X_R = crate::BitReader;
#[doc = "Field `REGION1_R0_PMS_X` writer - Region execute authority in REE_MODE0"]
pub type REGION1_R0_PMS_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION1_R0_PMS_W` reader - Region write authority in REE_MODE0"]
pub type REGION1_R0_PMS_W_R = crate::BitReader;
#[doc = "Field `REGION1_R0_PMS_W` writer - Region write authority in REE_MODE0"]
pub type REGION1_R0_PMS_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION1_R0_PMS_R` reader - Region read authority in REE_MODE0"]
pub type REGION1_R0_PMS_R_R = crate::BitReader;
#[doc = "Field `REGION1_R0_PMS_R` writer - Region read authority in REE_MODE0"]
pub type REGION1_R0_PMS_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION1_R1_PMS_X` reader - Region execute authority in REE_MODE1"]
pub type REGION1_R1_PMS_X_R = crate::BitReader;
#[doc = "Field `REGION1_R1_PMS_X` writer - Region execute authority in REE_MODE1"]
pub type REGION1_R1_PMS_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION1_R1_PMS_W` reader - Region write authority in REE_MODE1"]
pub type REGION1_R1_PMS_W_R = crate::BitReader;
#[doc = "Field `REGION1_R1_PMS_W` writer - Region write authority in REE_MODE1"]
pub type REGION1_R1_PMS_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION1_R1_PMS_R` reader - Region read authority in REE_MODE1"]
pub type REGION1_R1_PMS_R_R = crate::BitReader;
#[doc = "Field `REGION1_R1_PMS_R` writer - Region read authority in REE_MODE1"]
pub type REGION1_R1_PMS_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION1_R2_PMS_X` reader - Region execute authority in REE_MODE2"]
pub type REGION1_R2_PMS_X_R = crate::BitReader;
#[doc = "Field `REGION1_R2_PMS_X` writer - Region execute authority in REE_MODE2"]
pub type REGION1_R2_PMS_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION1_R2_PMS_W` reader - Region write authority in REE_MODE2"]
pub type REGION1_R2_PMS_W_R = crate::BitReader;
#[doc = "Field `REGION1_R2_PMS_W` writer - Region write authority in REE_MODE2"]
pub type REGION1_R2_PMS_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION1_R2_PMS_R` reader - Region read authority in REE_MODE2"]
pub type REGION1_R2_PMS_R_R = crate::BitReader;
#[doc = "Field `REGION1_R2_PMS_R` writer - Region read authority in REE_MODE2"]
pub type REGION1_R2_PMS_R_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Region execute authority in REE_MODE0"]
    #[inline(always)]
    pub fn region1_r0_pms_x(&self) -> REGION1_R0_PMS_X_R {
        REGION1_R0_PMS_X_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Region write authority in REE_MODE0"]
    #[inline(always)]
    pub fn region1_r0_pms_w(&self) -> REGION1_R0_PMS_W_R {
        REGION1_R0_PMS_W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Region read authority in REE_MODE0"]
    #[inline(always)]
    pub fn region1_r0_pms_r(&self) -> REGION1_R0_PMS_R_R {
        REGION1_R0_PMS_R_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Region execute authority in REE_MODE1"]
    #[inline(always)]
    pub fn region1_r1_pms_x(&self) -> REGION1_R1_PMS_X_R {
        REGION1_R1_PMS_X_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Region write authority in REE_MODE1"]
    #[inline(always)]
    pub fn region1_r1_pms_w(&self) -> REGION1_R1_PMS_W_R {
        REGION1_R1_PMS_W_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Region read authority in REE_MODE1"]
    #[inline(always)]
    pub fn region1_r1_pms_r(&self) -> REGION1_R1_PMS_R_R {
        REGION1_R1_PMS_R_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Region execute authority in REE_MODE2"]
    #[inline(always)]
    pub fn region1_r2_pms_x(&self) -> REGION1_R2_PMS_X_R {
        REGION1_R2_PMS_X_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Region write authority in REE_MODE2"]
    #[inline(always)]
    pub fn region1_r2_pms_w(&self) -> REGION1_R2_PMS_W_R {
        REGION1_R2_PMS_W_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Region read authority in REE_MODE2"]
    #[inline(always)]
    pub fn region1_r2_pms_r(&self) -> REGION1_R2_PMS_R_R {
        REGION1_R2_PMS_R_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION1_PMS_ATTR")
            .field("region1_r0_pms_x", &self.region1_r0_pms_x())
            .field("region1_r0_pms_w", &self.region1_r0_pms_w())
            .field("region1_r0_pms_r", &self.region1_r0_pms_r())
            .field("region1_r1_pms_x", &self.region1_r1_pms_x())
            .field("region1_r1_pms_w", &self.region1_r1_pms_w())
            .field("region1_r1_pms_r", &self.region1_r1_pms_r())
            .field("region1_r2_pms_x", &self.region1_r2_pms_x())
            .field("region1_r2_pms_w", &self.region1_r2_pms_w())
            .field("region1_r2_pms_r", &self.region1_r2_pms_r())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Region execute authority in REE_MODE0"]
    #[inline(always)]
    pub fn region1_r0_pms_x(&mut self) -> REGION1_R0_PMS_X_W<REGION1_PMS_ATTR_SPEC> {
        REGION1_R0_PMS_X_W::new(self, 0)
    }
    #[doc = "Bit 1 - Region write authority in REE_MODE0"]
    #[inline(always)]
    pub fn region1_r0_pms_w(&mut self) -> REGION1_R0_PMS_W_W<REGION1_PMS_ATTR_SPEC> {
        REGION1_R0_PMS_W_W::new(self, 1)
    }
    #[doc = "Bit 2 - Region read authority in REE_MODE0"]
    #[inline(always)]
    pub fn region1_r0_pms_r(&mut self) -> REGION1_R0_PMS_R_W<REGION1_PMS_ATTR_SPEC> {
        REGION1_R0_PMS_R_W::new(self, 2)
    }
    #[doc = "Bit 4 - Region execute authority in REE_MODE1"]
    #[inline(always)]
    pub fn region1_r1_pms_x(&mut self) -> REGION1_R1_PMS_X_W<REGION1_PMS_ATTR_SPEC> {
        REGION1_R1_PMS_X_W::new(self, 4)
    }
    #[doc = "Bit 5 - Region write authority in REE_MODE1"]
    #[inline(always)]
    pub fn region1_r1_pms_w(&mut self) -> REGION1_R1_PMS_W_W<REGION1_PMS_ATTR_SPEC> {
        REGION1_R1_PMS_W_W::new(self, 5)
    }
    #[doc = "Bit 6 - Region read authority in REE_MODE1"]
    #[inline(always)]
    pub fn region1_r1_pms_r(&mut self) -> REGION1_R1_PMS_R_W<REGION1_PMS_ATTR_SPEC> {
        REGION1_R1_PMS_R_W::new(self, 6)
    }
    #[doc = "Bit 8 - Region execute authority in REE_MODE2"]
    #[inline(always)]
    pub fn region1_r2_pms_x(&mut self) -> REGION1_R2_PMS_X_W<REGION1_PMS_ATTR_SPEC> {
        REGION1_R2_PMS_X_W::new(self, 8)
    }
    #[doc = "Bit 9 - Region write authority in REE_MODE2"]
    #[inline(always)]
    pub fn region1_r2_pms_w(&mut self) -> REGION1_R2_PMS_W_W<REGION1_PMS_ATTR_SPEC> {
        REGION1_R2_PMS_W_W::new(self, 9)
    }
    #[doc = "Bit 10 - Region read authority in REE_MODE2"]
    #[inline(always)]
    pub fn region1_r2_pms_r(&mut self) -> REGION1_R2_PMS_R_W<REGION1_PMS_ATTR_SPEC> {
        REGION1_R2_PMS_R_W::new(self, 10)
    }
}
#[doc = "Region access authority attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`region1_pms_attr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region1_pms_attr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGION1_PMS_ATTR_SPEC;
impl crate::RegisterSpec for REGION1_PMS_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region1_pms_attr::R`](R) reader structure"]
impl crate::Readable for REGION1_PMS_ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`region1_pms_attr::W`](W) writer structure"]
impl crate::Writable for REGION1_PMS_ATTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGION1_PMS_ATTR to value 0"]
impl crate::Resettable for REGION1_PMS_ATTR_SPEC {}
