#[doc = "Register `ATTR` reader"]
pub type R = crate::R<ATTR_SPEC>;
#[doc = "Register `ATTR` writer"]
pub type W = crate::W<ATTR_SPEC>;
#[doc = "Field `R_X(0-2)` reader - Configures the execution permission in region X"]
pub type R_X_R = crate::BitReader;
#[doc = "Field `R_X(0-2)` writer - Configures the execution permission in region X"]
pub type R_X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R_W(0-2)` reader - Configures the write permission in region X"]
pub type R_W_R = crate::BitReader;
#[doc = "Field `R_W(0-2)` writer - Configures the write permission in region X"]
pub type R_W_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R_R(0-2)` reader - Configures the write permission in region X"]
pub type R_R_R = crate::BitReader;
#[doc = "Field `R_R(0-2)` writer - Configures the write permission in region X"]
pub type R_R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - Configures to lock the value of region 0 configuration registers (LP_APM_REGION0_ADDR_START_REG, LP_APM_REGION0_ADDR_END_REG and LP_APM_REGION0_ATTR_REG). 0: Do not lock 1: Lock"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Configures to lock the value of region 0 configuration registers (LP_APM_REGION0_ADDR_START_REG, LP_APM_REGION0_ADDR_END_REG and LP_APM_REGION0_ATTR_REG). 0: Do not lock 1: Lock"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Configures the execution permission in region X"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `R0_X` field.</div>"]
    #[inline(always)]
    pub fn r_x(&self, n: u8) -> R_X_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        R_X_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configures the execution permission in region X"]
    #[inline(always)]
    pub fn r_x_iter(&self) -> impl Iterator<Item = R_X_R> + '_ {
        (0..3).map(move |n| R_X_R::new(((self.bits >> (n * 4)) & 1) != 0))
    }
    #[doc = "Bit 0 - Configures the execution permission in region X"]
    #[inline(always)]
    pub fn r0_x(&self) -> R_X_R {
        R_X_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Configures the execution permission in region X"]
    #[inline(always)]
    pub fn r1_x(&self) -> R_X_R {
        R_X_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures the execution permission in region X"]
    #[inline(always)]
    pub fn r2_x(&self) -> R_X_R {
        R_X_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Configures the write permission in region X"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `R0_W` field.</div>"]
    #[inline(always)]
    pub fn r_w(&self, n: u8) -> R_W_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        R_W_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configures the write permission in region X"]
    #[inline(always)]
    pub fn r_w_iter(&self) -> impl Iterator<Item = R_W_R> + '_ {
        (0..3).map(move |n| R_W_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Configures the write permission in region X"]
    #[inline(always)]
    pub fn r0_w(&self) -> R_W_R {
        R_W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures the write permission in region X"]
    #[inline(always)]
    pub fn r1_w(&self) -> R_W_R {
        R_W_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures the write permission in region X"]
    #[inline(always)]
    pub fn r2_w(&self) -> R_W_R {
        R_W_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Configures the write permission in region X"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `R0_R` field.</div>"]
    #[inline(always)]
    pub fn r_r(&self, n: u8) -> R_R_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        R_R_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configures the write permission in region X"]
    #[inline(always)]
    pub fn r_r_iter(&self) -> impl Iterator<Item = R_R_R> + '_ {
        (0..3).map(move |n| R_R_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - Configures the write permission in region X"]
    #[inline(always)]
    pub fn r0_r(&self) -> R_R_R {
        R_R_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures the write permission in region X"]
    #[inline(always)]
    pub fn r1_r(&self) -> R_R_R {
        R_R_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures the write permission in region X"]
    #[inline(always)]
    pub fn r2_r(&self) -> R_R_R {
        R_R_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures to lock the value of region 0 configuration registers (LP_APM_REGION0_ADDR_START_REG, LP_APM_REGION0_ADDR_END_REG and LP_APM_REGION0_ATTR_REG). 0: Do not lock 1: Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ATTR")
            .field("r0_x", &self.r0_x())
            .field("r1_x", &self.r1_x())
            .field("r2_x", &self.r2_x())
            .field("r0_w", &self.r0_w())
            .field("r1_w", &self.r1_w())
            .field("r2_w", &self.r2_w())
            .field("r0_r", &self.r0_r())
            .field("r1_r", &self.r1_r())
            .field("r2_r", &self.r2_r())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    #[doc = "Configures the execution permission in region X"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `R0_X` field.</div>"]
    #[inline(always)]
    pub fn r_x(&mut self, n: u8) -> R_X_W<'_, ATTR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        R_X_W::new(self, n * 4)
    }
    #[doc = "Bit 0 - Configures the execution permission in region X"]
    #[inline(always)]
    pub fn r0_x(&mut self) -> R_X_W<'_, ATTR_SPEC> {
        R_X_W::new(self, 0)
    }
    #[doc = "Bit 4 - Configures the execution permission in region X"]
    #[inline(always)]
    pub fn r1_x(&mut self) -> R_X_W<'_, ATTR_SPEC> {
        R_X_W::new(self, 4)
    }
    #[doc = "Bit 8 - Configures the execution permission in region X"]
    #[inline(always)]
    pub fn r2_x(&mut self) -> R_X_W<'_, ATTR_SPEC> {
        R_X_W::new(self, 8)
    }
    #[doc = "Configures the write permission in region X"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `R0_W` field.</div>"]
    #[inline(always)]
    pub fn r_w(&mut self, n: u8) -> R_W_W<'_, ATTR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        R_W_W::new(self, n * 4 + 1)
    }
    #[doc = "Bit 1 - Configures the write permission in region X"]
    #[inline(always)]
    pub fn r0_w(&mut self) -> R_W_W<'_, ATTR_SPEC> {
        R_W_W::new(self, 1)
    }
    #[doc = "Bit 5 - Configures the write permission in region X"]
    #[inline(always)]
    pub fn r1_w(&mut self) -> R_W_W<'_, ATTR_SPEC> {
        R_W_W::new(self, 5)
    }
    #[doc = "Bit 9 - Configures the write permission in region X"]
    #[inline(always)]
    pub fn r2_w(&mut self) -> R_W_W<'_, ATTR_SPEC> {
        R_W_W::new(self, 9)
    }
    #[doc = "Configures the write permission in region X"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `R0_R` field.</div>"]
    #[inline(always)]
    pub fn r_r(&mut self, n: u8) -> R_R_W<'_, ATTR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        R_R_W::new(self, n * 4 + 2)
    }
    #[doc = "Bit 2 - Configures the write permission in region X"]
    #[inline(always)]
    pub fn r0_r(&mut self) -> R_R_W<'_, ATTR_SPEC> {
        R_R_W::new(self, 2)
    }
    #[doc = "Bit 6 - Configures the write permission in region X"]
    #[inline(always)]
    pub fn r1_r(&mut self) -> R_R_W<'_, ATTR_SPEC> {
        R_R_W::new(self, 6)
    }
    #[doc = "Bit 10 - Configures the write permission in region X"]
    #[inline(always)]
    pub fn r2_r(&mut self) -> R_R_W<'_, ATTR_SPEC> {
        R_R_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures to lock the value of region 0 configuration registers (LP_APM_REGION0_ADDR_START_REG, LP_APM_REGION0_ADDR_END_REG and LP_APM_REGION0_ATTR_REG). 0: Do not lock 1: Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, ATTR_SPEC> {
        LOCK_W::new(self, 11)
    }
}
#[doc = "Region access permissions configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`attr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`attr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATTR_SPEC;
impl crate::RegisterSpec for ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`attr::R`](R) reader structure"]
impl crate::Readable for ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`attr::W`](W) writer structure"]
impl crate::Writable for ATTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ATTR to value 0"]
impl crate::Resettable for ATTR_SPEC {}
