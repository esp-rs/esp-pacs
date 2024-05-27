///Register `STORE2` reader
pub type R = crate::R<STORE2_SPEC>;
///Register `STORE2` writer
pub type W = crate::W<STORE2_SPEC>;
///Field `LP_AON_STORE2` reader - need_des
pub type LP_AON_STORE2_R = crate::FieldReader<u32>;
///Field `LP_AON_STORE2` writer - need_des
pub type LP_AON_STORE2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - need_des
    #[inline(always)]
    pub fn lp_aon_store2(&self) -> LP_AON_STORE2_R {
        LP_AON_STORE2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STORE2")
            .field("lp_aon_store2", &self.lp_aon_store2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_aon_store2(&mut self) -> LP_AON_STORE2_W<STORE2_SPEC> {
        LP_AON_STORE2_W::new(self, 0)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`store2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`store2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STORE2_SPEC;
impl crate::RegisterSpec for STORE2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`store2::R`](R) reader structure
impl crate::Readable for STORE2_SPEC {}
///`write(|w| ..)` method takes [`store2::W`](W) writer structure
impl crate::Writable for STORE2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STORE2 to value 0
impl crate::Resettable for STORE2_SPEC {
    const RESET_VALUE: u32 = 0;
}
