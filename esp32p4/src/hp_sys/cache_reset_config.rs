///Register `CACHE_RESET_CONFIG` reader
pub type R = crate::R<CACHE_RESET_CONFIG_SPEC>;
///Register `CACHE_RESET_CONFIG` writer
pub type W = crate::W<CACHE_RESET_CONFIG_SPEC>;
///Field `REG_L1_D_CACHE_RESET` reader - set 1 to reset l1 dcahce
pub type REG_L1_D_CACHE_RESET_R = crate::BitReader;
///Field `REG_L1_D_CACHE_RESET` writer - set 1 to reset l1 dcahce
pub type REG_L1_D_CACHE_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_L1_I1_CACHE_RESET` reader - set 1 to reset l1 icahce1
pub type REG_L1_I1_CACHE_RESET_R = crate::BitReader;
///Field `REG_L1_I1_CACHE_RESET` writer - set 1 to reset l1 icahce1
pub type REG_L1_I1_CACHE_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_L1_I0_CACHE_RESET` reader - set 1 to reset l1 icahce0
pub type REG_L1_I0_CACHE_RESET_R = crate::BitReader;
///Field `REG_L1_I0_CACHE_RESET` writer - set 1 to reset l1 icahce0
pub type REG_L1_I0_CACHE_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - set 1 to reset l1 dcahce
    #[inline(always)]
    pub fn reg_l1_d_cache_reset(&self) -> REG_L1_D_CACHE_RESET_R {
        REG_L1_D_CACHE_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - set 1 to reset l1 icahce1
    #[inline(always)]
    pub fn reg_l1_i1_cache_reset(&self) -> REG_L1_I1_CACHE_RESET_R {
        REG_L1_I1_CACHE_RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - set 1 to reset l1 icahce0
    #[inline(always)]
    pub fn reg_l1_i0_cache_reset(&self) -> REG_L1_I0_CACHE_RESET_R {
        REG_L1_I0_CACHE_RESET_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_RESET_CONFIG")
            .field("reg_l1_d_cache_reset", &self.reg_l1_d_cache_reset())
            .field("reg_l1_i1_cache_reset", &self.reg_l1_i1_cache_reset())
            .field("reg_l1_i0_cache_reset", &self.reg_l1_i0_cache_reset())
            .finish()
    }
}
impl W {
    ///Bit 1 - set 1 to reset l1 dcahce
    #[inline(always)]
    #[must_use]
    pub fn reg_l1_d_cache_reset(&mut self) -> REG_L1_D_CACHE_RESET_W<CACHE_RESET_CONFIG_SPEC> {
        REG_L1_D_CACHE_RESET_W::new(self, 1)
    }
    ///Bit 4 - set 1 to reset l1 icahce1
    #[inline(always)]
    #[must_use]
    pub fn reg_l1_i1_cache_reset(&mut self) -> REG_L1_I1_CACHE_RESET_W<CACHE_RESET_CONFIG_SPEC> {
        REG_L1_I1_CACHE_RESET_W::new(self, 4)
    }
    ///Bit 5 - set 1 to reset l1 icahce0
    #[inline(always)]
    #[must_use]
    pub fn reg_l1_i0_cache_reset(&mut self) -> REG_L1_I0_CACHE_RESET_W<CACHE_RESET_CONFIG_SPEC> {
        REG_L1_I0_CACHE_RESET_W::new(self, 5)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`cache_reset_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_reset_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHE_RESET_CONFIG_SPEC;
impl crate::RegisterSpec for CACHE_RESET_CONFIG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cache_reset_config::R`](R) reader structure
impl crate::Readable for CACHE_RESET_CONFIG_SPEC {}
///`write(|w| ..)` method takes [`cache_reset_config::W`](W) writer structure
impl crate::Writable for CACHE_RESET_CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_RESET_CONFIG to value 0
impl crate::Resettable for CACHE_RESET_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
