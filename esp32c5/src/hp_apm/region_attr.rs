#[doc = "Register `REGION%s_ATTR` reader"]
pub type R = crate::R<REGION_ATTR_SPEC>;
#[doc = "Register `REGION%s_ATTR` writer"]
pub type W = crate::W<REGION_ATTR_SPEC>;
#[doc = "Field `REGION_R0_X` reader - Configures the execution authority of REE_MODE 0 in region %s."]
pub type REGION_R0_X_R = crate::BitReader;
#[doc = "Field `REGION_R0_X` writer - Configures the execution authority of REE_MODE 0 in region %s."]
pub type REGION_R0_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION_R0_W` reader - Configures the write authority of REE_MODE 0 in region %s."]
pub type REGION_R0_W_R = crate::BitReader;
#[doc = "Field `REGION_R0_W` writer - Configures the write authority of REE_MODE 0 in region %s."]
pub type REGION_R0_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION_R0_R` reader - Configures the read authority of REE_MODE 0 in region %s."]
pub type REGION_R0_R_R = crate::BitReader;
#[doc = "Field `REGION_R0_R` writer - Configures the read authority of REE_MODE 0 in region %s."]
pub type REGION_R0_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION_R1_X` reader - Configures the execution authority of REE_MODE 1 in region %s."]
pub type REGION_R1_X_R = crate::BitReader;
#[doc = "Field `REGION_R1_X` writer - Configures the execution authority of REE_MODE 1 in region %s."]
pub type REGION_R1_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION_R1_W` reader - Configures the write authority of REE_MODE 1 in region %s."]
pub type REGION_R1_W_R = crate::BitReader;
#[doc = "Field `REGION_R1_W` writer - Configures the write authority of REE_MODE 1 in region %s."]
pub type REGION_R1_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION_R1_R` reader - Configures the read authority of REE_MODE 1 in region %s."]
pub type REGION_R1_R_R = crate::BitReader;
#[doc = "Field `REGION_R1_R` writer - Configures the read authority of REE_MODE 1 in region %s."]
pub type REGION_R1_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION_R2_X` reader - Configures the execution authority of REE_MODE 2 in region %s."]
pub type REGION_R2_X_R = crate::BitReader;
#[doc = "Field `REGION_R2_X` writer - Configures the execution authority of REE_MODE 2 in region %s."]
pub type REGION_R2_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION_R2_W` reader - Configures the write authority of REE_MODE 2 in region %s."]
pub type REGION_R2_W_R = crate::BitReader;
#[doc = "Field `REGION_R2_W` writer - Configures the write authority of REE_MODE 2 in region %s."]
pub type REGION_R2_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION_R2_R` reader - Configures the read authority of REE_MODE 2 in region %s."]
pub type REGION_R2_R_R = crate::BitReader;
#[doc = "Field `REGION_R2_R` writer - Configures the read authority of REE_MODE 2 in region %s."]
pub type REGION_R2_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGION_LOCK` reader - Set 1 to lock region0 configuration"]
pub type REGION_LOCK_R = crate::BitReader;
#[doc = "Field `REGION_LOCK` writer - Set 1 to lock region0 configuration"]
pub type REGION_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the execution authority of REE_MODE 0 in region %s."]
    #[inline(always)]
    pub fn region_r0_x(&self) -> REGION_R0_X_R {
        REGION_R0_X_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures the write authority of REE_MODE 0 in region %s."]
    #[inline(always)]
    pub fn region_r0_w(&self) -> REGION_R0_W_R {
        REGION_R0_W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures the read authority of REE_MODE 0 in region %s."]
    #[inline(always)]
    pub fn region_r0_r(&self) -> REGION_R0_R_R {
        REGION_R0_R_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures the execution authority of REE_MODE 1 in region %s."]
    #[inline(always)]
    pub fn region_r1_x(&self) -> REGION_R1_X_R {
        REGION_R1_X_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures the write authority of REE_MODE 1 in region %s."]
    #[inline(always)]
    pub fn region_r1_w(&self) -> REGION_R1_W_R {
        REGION_R1_W_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures the read authority of REE_MODE 1 in region %s."]
    #[inline(always)]
    pub fn region_r1_r(&self) -> REGION_R1_R_R {
        REGION_R1_R_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures the execution authority of REE_MODE 2 in region %s."]
    #[inline(always)]
    pub fn region_r2_x(&self) -> REGION_R2_X_R {
        REGION_R2_X_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures the write authority of REE_MODE 2 in region %s."]
    #[inline(always)]
    pub fn region_r2_w(&self) -> REGION_R2_W_R {
        REGION_R2_W_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures the read authority of REE_MODE 2 in region %s."]
    #[inline(always)]
    pub fn region_r2_r(&self) -> REGION_R2_R_R {
        REGION_R2_R_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set 1 to lock region0 configuration"]
    #[inline(always)]
    pub fn region_lock(&self) -> REGION_LOCK_R {
        REGION_LOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION_ATTR")
            .field("region_r0_x", &self.region_r0_x())
            .field("region_r0_w", &self.region_r0_w())
            .field("region_r0_r", &self.region_r0_r())
            .field("region_r1_x", &self.region_r1_x())
            .field("region_r1_w", &self.region_r1_w())
            .field("region_r1_r", &self.region_r1_r())
            .field("region_r2_x", &self.region_r2_x())
            .field("region_r2_w", &self.region_r2_w())
            .field("region_r2_r", &self.region_r2_r())
            .field("region_lock", &self.region_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures the execution authority of REE_MODE 0 in region %s."]
    #[inline(always)]
    pub fn region_r0_x(&mut self) -> REGION_R0_X_W<REGION_ATTR_SPEC> {
        REGION_R0_X_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures the write authority of REE_MODE 0 in region %s."]
    #[inline(always)]
    pub fn region_r0_w(&mut self) -> REGION_R0_W_W<REGION_ATTR_SPEC> {
        REGION_R0_W_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures the read authority of REE_MODE 0 in region %s."]
    #[inline(always)]
    pub fn region_r0_r(&mut self) -> REGION_R0_R_W<REGION_ATTR_SPEC> {
        REGION_R0_R_W::new(self, 2)
    }
    #[doc = "Bit 4 - Configures the execution authority of REE_MODE 1 in region %s."]
    #[inline(always)]
    pub fn region_r1_x(&mut self) -> REGION_R1_X_W<REGION_ATTR_SPEC> {
        REGION_R1_X_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures the write authority of REE_MODE 1 in region %s."]
    #[inline(always)]
    pub fn region_r1_w(&mut self) -> REGION_R1_W_W<REGION_ATTR_SPEC> {
        REGION_R1_W_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures the read authority of REE_MODE 1 in region %s."]
    #[inline(always)]
    pub fn region_r1_r(&mut self) -> REGION_R1_R_W<REGION_ATTR_SPEC> {
        REGION_R1_R_W::new(self, 6)
    }
    #[doc = "Bit 8 - Configures the execution authority of REE_MODE 2 in region %s."]
    #[inline(always)]
    pub fn region_r2_x(&mut self) -> REGION_R2_X_W<REGION_ATTR_SPEC> {
        REGION_R2_X_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures the write authority of REE_MODE 2 in region %s."]
    #[inline(always)]
    pub fn region_r2_w(&mut self) -> REGION_R2_W_W<REGION_ATTR_SPEC> {
        REGION_R2_W_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures the read authority of REE_MODE 2 in region %s."]
    #[inline(always)]
    pub fn region_r2_r(&mut self) -> REGION_R2_R_W<REGION_ATTR_SPEC> {
        REGION_R2_R_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set 1 to lock region0 configuration"]
    #[inline(always)]
    pub fn region_lock(&mut self) -> REGION_LOCK_W<REGION_ATTR_SPEC> {
        REGION_LOCK_W::new(self, 11)
    }
}
#[doc = "Region access authority attribute register\n\nYou can [`read`](crate::Reg::read) this register and get [`region_attr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region_attr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGION_ATTR_SPEC;
impl crate::RegisterSpec for REGION_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region_attr::R`](R) reader structure"]
impl crate::Readable for REGION_ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`region_attr::W`](W) writer structure"]
impl crate::Writable for REGION_ATTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGION%s_ATTR to value 0"]
impl crate::Resettable for REGION_ATTR_SPEC {}
