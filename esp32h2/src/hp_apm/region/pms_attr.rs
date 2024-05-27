///Register `PMS_ATTR` reader
pub type R = crate::R<PMS_ATTR_SPEC>;
///Register `PMS_ATTR` writer
pub type W = crate::W<PMS_ATTR_SPEC>;
///Field `R_PMS_X(0-2)` reader - Region execute authority in REE_MODE%s
pub type R_PMS_X_R = crate::BitReader;
///Field `R_PMS_X(0-2)` writer - Region execute authority in REE_MODE%s
pub type R_PMS_X_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R_PMS_W(0-2)` reader - Region write authority in REE_MODE%s
pub type R_PMS_W_R = crate::BitReader;
///Field `R_PMS_W(0-2)` writer - Region write authority in REE_MODE%s
pub type R_PMS_W_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `R_PMS_R(0-2)` reader - Region read authority in REE_MODE%s
pub type R_PMS_R_R = crate::BitReader;
///Field `R_PMS_R(0-2)` writer - Region read authority in REE_MODE%s
pub type R_PMS_R_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Region execute authority in REE_MODE(0-2)
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `R0_PMS_X` field
    #[inline(always)]
    pub fn r_pms_x(&self, n: u8) -> R_PMS_X_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        R_PMS_X_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Region execute authority in REE_MODE(0-2)
    #[inline(always)]
    pub fn r_pms_x_iter(&self) -> impl Iterator<Item = R_PMS_X_R> + '_ {
        (0..3).map(move |n| R_PMS_X_R::new(((self.bits >> (n * 4)) & 1) != 0))
    }
    ///Bit 0 - Region execute authority in REE_MODE0
    #[inline(always)]
    pub fn r0_pms_x(&self) -> R_PMS_X_R {
        R_PMS_X_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Region execute authority in REE_MODE1
    #[inline(always)]
    pub fn r1_pms_x(&self) -> R_PMS_X_R {
        R_PMS_X_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Region execute authority in REE_MODE2
    #[inline(always)]
    pub fn r2_pms_x(&self) -> R_PMS_X_R {
        R_PMS_X_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Region write authority in REE_MODE(0-2)
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `R0_PMS_W` field
    #[inline(always)]
    pub fn r_pms_w(&self, n: u8) -> R_PMS_W_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        R_PMS_W_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Region write authority in REE_MODE(0-2)
    #[inline(always)]
    pub fn r_pms_w_iter(&self) -> impl Iterator<Item = R_PMS_W_R> + '_ {
        (0..3).map(move |n| R_PMS_W_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0))
    }
    ///Bit 1 - Region write authority in REE_MODE0
    #[inline(always)]
    pub fn r0_pms_w(&self) -> R_PMS_W_R {
        R_PMS_W_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - Region write authority in REE_MODE1
    #[inline(always)]
    pub fn r1_pms_w(&self) -> R_PMS_W_R {
        R_PMS_W_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - Region write authority in REE_MODE2
    #[inline(always)]
    pub fn r2_pms_w(&self) -> R_PMS_W_R {
        R_PMS_W_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Region read authority in REE_MODE(0-2)
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `R0_PMS_R` field
    #[inline(always)]
    pub fn r_pms_r(&self, n: u8) -> R_PMS_R_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        R_PMS_R_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Region read authority in REE_MODE(0-2)
    #[inline(always)]
    pub fn r_pms_r_iter(&self) -> impl Iterator<Item = R_PMS_R_R> + '_ {
        (0..3).map(move |n| R_PMS_R_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0))
    }
    ///Bit 2 - Region read authority in REE_MODE0
    #[inline(always)]
    pub fn r0_pms_r(&self) -> R_PMS_R_R {
        R_PMS_R_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - Region read authority in REE_MODE1
    #[inline(always)]
    pub fn r1_pms_r(&self) -> R_PMS_R_R {
        R_PMS_R_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 10 - Region read authority in REE_MODE2
    #[inline(always)]
    pub fn r2_pms_r(&self) -> R_PMS_R_R {
        R_PMS_R_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMS_ATTR")
            .field("r0_pms_x", &self.r0_pms_x())
            .field("r1_pms_x", &self.r1_pms_x())
            .field("r2_pms_x", &self.r2_pms_x())
            .field("r0_pms_w", &self.r0_pms_w())
            .field("r1_pms_w", &self.r1_pms_w())
            .field("r2_pms_w", &self.r2_pms_w())
            .field("r0_pms_r", &self.r0_pms_r())
            .field("r1_pms_r", &self.r1_pms_r())
            .field("r2_pms_r", &self.r2_pms_r())
            .finish()
    }
}
impl W {
    ///Region execute authority in REE_MODE(0-2)
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `R0_PMS_X` field
    #[inline(always)]
    #[must_use]
    pub fn r_pms_x(&mut self, n: u8) -> R_PMS_X_W<PMS_ATTR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        R_PMS_X_W::new(self, n * 4)
    }
    ///Bit 0 - Region execute authority in REE_MODE0
    #[inline(always)]
    #[must_use]
    pub fn r0_pms_x(&mut self) -> R_PMS_X_W<PMS_ATTR_SPEC> {
        R_PMS_X_W::new(self, 0)
    }
    ///Bit 4 - Region execute authority in REE_MODE1
    #[inline(always)]
    #[must_use]
    pub fn r1_pms_x(&mut self) -> R_PMS_X_W<PMS_ATTR_SPEC> {
        R_PMS_X_W::new(self, 4)
    }
    ///Bit 8 - Region execute authority in REE_MODE2
    #[inline(always)]
    #[must_use]
    pub fn r2_pms_x(&mut self) -> R_PMS_X_W<PMS_ATTR_SPEC> {
        R_PMS_X_W::new(self, 8)
    }
    ///Region write authority in REE_MODE(0-2)
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `R0_PMS_W` field
    #[inline(always)]
    #[must_use]
    pub fn r_pms_w(&mut self, n: u8) -> R_PMS_W_W<PMS_ATTR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        R_PMS_W_W::new(self, n * 4 + 1)
    }
    ///Bit 1 - Region write authority in REE_MODE0
    #[inline(always)]
    #[must_use]
    pub fn r0_pms_w(&mut self) -> R_PMS_W_W<PMS_ATTR_SPEC> {
        R_PMS_W_W::new(self, 1)
    }
    ///Bit 5 - Region write authority in REE_MODE1
    #[inline(always)]
    #[must_use]
    pub fn r1_pms_w(&mut self) -> R_PMS_W_W<PMS_ATTR_SPEC> {
        R_PMS_W_W::new(self, 5)
    }
    ///Bit 9 - Region write authority in REE_MODE2
    #[inline(always)]
    #[must_use]
    pub fn r2_pms_w(&mut self) -> R_PMS_W_W<PMS_ATTR_SPEC> {
        R_PMS_W_W::new(self, 9)
    }
    ///Region read authority in REE_MODE(0-2)
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `R0_PMS_R` field
    #[inline(always)]
    #[must_use]
    pub fn r_pms_r(&mut self, n: u8) -> R_PMS_R_W<PMS_ATTR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        R_PMS_R_W::new(self, n * 4 + 2)
    }
    ///Bit 2 - Region read authority in REE_MODE0
    #[inline(always)]
    #[must_use]
    pub fn r0_pms_r(&mut self) -> R_PMS_R_W<PMS_ATTR_SPEC> {
        R_PMS_R_W::new(self, 2)
    }
    ///Bit 6 - Region read authority in REE_MODE1
    #[inline(always)]
    #[must_use]
    pub fn r1_pms_r(&mut self) -> R_PMS_R_W<PMS_ATTR_SPEC> {
        R_PMS_R_W::new(self, 6)
    }
    ///Bit 10 - Region read authority in REE_MODE2
    #[inline(always)]
    #[must_use]
    pub fn r2_pms_r(&mut self) -> R_PMS_R_W<PMS_ATTR_SPEC> {
        R_PMS_R_W::new(self, 10)
    }
}
/**Region access authority attribute register

You can [`read`](crate::generic::Reg::read) this register and get [`pms_attr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pms_attr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PMS_ATTR_SPEC;
impl crate::RegisterSpec for PMS_ATTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pms_attr::R`](R) reader structure
impl crate::Readable for PMS_ATTR_SPEC {}
///`write(|w| ..)` method takes [`pms_attr::W`](W) writer structure
impl crate::Writable for PMS_ATTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PMS_ATTR to value 0
impl crate::Resettable for PMS_ATTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
