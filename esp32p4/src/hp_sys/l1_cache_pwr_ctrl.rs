#[doc = "Register `L1_CACHE_PWR_CTRL` reader"]
pub type R = crate::R<L1_CACHE_PWR_CTRL_SPEC>;
#[doc = "Register `L1_CACHE_PWR_CTRL` writer"]
pub type W = crate::W<L1_CACHE_PWR_CTRL_SPEC>;
#[doc = "Field `REG_L1_CACHE_MEM_FO` reader - need_des"]
pub type REG_L1_CACHE_MEM_FO_R = crate::FieldReader;
#[doc = "Field `REG_L1_CACHE_MEM_FO` writer - need_des"]
pub type REG_L1_CACHE_MEM_FO_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - need_des"]
    #[inline(always)]
    pub fn reg_l1_cache_mem_fo(&self) -> REG_L1_CACHE_MEM_FO_R {
        REG_L1_CACHE_MEM_FO_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_PWR_CTRL")
            .field("reg_l1_cache_mem_fo", &self.reg_l1_cache_mem_fo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - need_des"]
    #[inline(always)]
    pub fn reg_l1_cache_mem_fo(&mut self) -> REG_L1_CACHE_MEM_FO_W<'_, L1_CACHE_PWR_CTRL_SPEC> {
        REG_L1_CACHE_MEM_FO_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_pwr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_pwr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_PWR_CTRL_SPEC;
impl crate::RegisterSpec for L1_CACHE_PWR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_pwr_ctrl::R`](R) reader structure"]
impl crate::Readable for L1_CACHE_PWR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_pwr_ctrl::W`](W) writer structure"]
impl crate::Writable for L1_CACHE_PWR_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_PWR_CTRL to value 0"]
impl crate::Resettable for L1_CACHE_PWR_CTRL_SPEC {}
