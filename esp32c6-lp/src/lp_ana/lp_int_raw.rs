///Register `LP_INT_RAW` reader
pub type R = crate::R<LP_INT_RAW_SPEC>;
///Register `LP_INT_RAW` writer
pub type W = crate::W<LP_INT_RAW_SPEC>;
///Field `BOD_MODE0` reader - need_des
pub type BOD_MODE0_R = crate::BitReader;
///Field `BOD_MODE0` writer - need_des
pub type BOD_MODE0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn bod_mode0(&self) -> BOD_MODE0_R {
        BOD_MODE0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_RAW")
            .field("bod_mode0", &self.bod_mode0())
            .finish()
    }
}
impl W {
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn bod_mode0(&mut self) -> BOD_MODE0_W<LP_INT_RAW_SPEC> {
        BOD_MODE0_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_INT_RAW_SPEC;
impl crate::RegisterSpec for LP_INT_RAW_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_int_raw::R`](R) reader structure
impl crate::Readable for LP_INT_RAW_SPEC {}
///`write(|w| ..)` method takes [`lp_int_raw::W`](W) writer structure
impl crate::Writable for LP_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_INT_RAW to value 0
impl crate::Resettable for LP_INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
