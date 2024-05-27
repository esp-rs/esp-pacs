///Register `SW_INT_ENA` reader
pub type R = crate::R<SW_INT_ENA_SPEC>;
///Register `SW_INT_ENA` writer
pub type W = crate::W<SW_INT_ENA_SPEC>;
///Field `LP_SW_INT_ENA` reader - need_des
pub type LP_SW_INT_ENA_R = crate::BitReader;
///Field `LP_SW_INT_ENA` writer - need_des
pub type LP_SW_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn lp_sw_int_ena(&self) -> LP_SW_INT_ENA_R {
        LP_SW_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SW_INT_ENA")
            .field("lp_sw_int_ena", &self.lp_sw_int_ena())
            .finish()
    }
}
impl W {
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_sw_int_ena(&mut self) -> LP_SW_INT_ENA_W<SW_INT_ENA_SPEC> {
        LP_SW_INT_ENA_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`sw_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SW_INT_ENA_SPEC;
impl crate::RegisterSpec for SW_INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sw_int_ena::R`](R) reader structure
impl crate::Readable for SW_INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`sw_int_ena::W`](W) writer structure
impl crate::Writable for SW_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SW_INT_ENA to value 0
impl crate::Resettable for SW_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
