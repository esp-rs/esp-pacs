///Register `L2_CACHE_PWR_CTRL` reader
pub type R = crate::R<L2_CACHE_PWR_CTRL_SPEC>;
///Register `L2_CACHE_PWR_CTRL` writer
pub type W = crate::W<L2_CACHE_PWR_CTRL_SPEC>;
///Field `REG_L2_CACHE_MEM_FO` reader - need_des
pub type REG_L2_CACHE_MEM_FO_R = crate::FieldReader;
///Field `REG_L2_CACHE_MEM_FO` writer - need_des
pub type REG_L2_CACHE_MEM_FO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - need_des
    #[inline(always)]
    pub fn reg_l2_cache_mem_fo(&self) -> REG_L2_CACHE_MEM_FO_R {
        REG_L2_CACHE_MEM_FO_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_PWR_CTRL")
            .field("reg_l2_cache_mem_fo", &self.reg_l2_cache_mem_fo())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - need_des
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_cache_mem_fo(&mut self) -> REG_L2_CACHE_MEM_FO_W<L2_CACHE_PWR_CTRL_SPEC> {
        REG_L2_CACHE_MEM_FO_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_pwr_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_pwr_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_CACHE_PWR_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_PWR_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_cache_pwr_ctrl::R`](R) reader structure
impl crate::Readable for L2_CACHE_PWR_CTRL_SPEC {}
///`write(|w| ..)` method takes [`l2_cache_pwr_ctrl::W`](W) writer structure
impl crate::Writable for L2_CACHE_PWR_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_CACHE_PWR_CTRL to value 0
impl crate::Resettable for L2_CACHE_PWR_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
