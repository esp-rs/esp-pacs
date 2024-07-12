#[doc = "Register `CACHE_RESET_CONFIG` reader"]
pub type R = crate::R<CACHE_RESET_CONFIG_SPEC>;
#[doc = "Register `CACHE_RESET_CONFIG` writer"]
pub type W = crate::W<CACHE_RESET_CONFIG_SPEC>;
#[doc = "Field `REG_L1_D_CACHE_RESET` reader - set 1 to reset l1 dcahce"]
pub type REG_L1_D_CACHE_RESET_R = crate::BitReader;
#[doc = "Field `REG_L1_D_CACHE_RESET` writer - set 1 to reset l1 dcahce"]
pub type REG_L1_D_CACHE_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L1_I1_CACHE_RESET` reader - set 1 to reset l1 icahce1"]
pub type REG_L1_I1_CACHE_RESET_R = crate::BitReader;
#[doc = "Field `REG_L1_I1_CACHE_RESET` writer - set 1 to reset l1 icahce1"]
pub type REG_L1_I1_CACHE_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_L1_I0_CACHE_RESET` reader - set 1 to reset l1 icahce0"]
pub type REG_L1_I0_CACHE_RESET_R = crate::BitReader;
#[doc = "Field `REG_L1_I0_CACHE_RESET` writer - set 1 to reset l1 icahce0"]
pub type REG_L1_I0_CACHE_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - set 1 to reset l1 dcahce"]
    #[inline(always)]
    pub fn reg_l1_d_cache_reset(&self) -> REG_L1_D_CACHE_RESET_R {
        REG_L1_D_CACHE_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - set 1 to reset l1 icahce1"]
    #[inline(always)]
    pub fn reg_l1_i1_cache_reset(&self) -> REG_L1_I1_CACHE_RESET_R {
        REG_L1_I1_CACHE_RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - set 1 to reset l1 icahce0"]
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
    #[doc = "Bit 1 - set 1 to reset l1 dcahce"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l1_d_cache_reset(&mut self) -> REG_L1_D_CACHE_RESET_W<CACHE_RESET_CONFIG_SPEC> {
        REG_L1_D_CACHE_RESET_W::new(self, 1)
    }
    #[doc = "Bit 4 - set 1 to reset l1 icahce1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l1_i1_cache_reset(&mut self) -> REG_L1_I1_CACHE_RESET_W<CACHE_RESET_CONFIG_SPEC> {
        REG_L1_I1_CACHE_RESET_W::new(self, 4)
    }
    #[doc = "Bit 5 - set 1 to reset l1 icahce0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l1_i0_cache_reset(&mut self) -> REG_L1_I0_CACHE_RESET_W<CACHE_RESET_CONFIG_SPEC> {
        REG_L1_I0_CACHE_RESET_W::new(self, 5)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_reset_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_reset_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_RESET_CONFIG_SPEC;
impl crate::RegisterSpec for CACHE_RESET_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_reset_config::R`](R) reader structure"]
impl crate::Readable for CACHE_RESET_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_reset_config::W`](W) writer structure"]
impl crate::Writable for CACHE_RESET_CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CACHE_RESET_CONFIG to value 0"]
impl crate::Resettable for CACHE_RESET_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
