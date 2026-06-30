#[doc = "Register `LP_ANA_INT_RAW` reader"]
pub type R = crate::R<LP_ANA_INT_RAW_SPEC>;
#[doc = "Register `LP_ANA_INT_RAW` writer"]
pub type W = crate::W<LP_ANA_INT_RAW_SPEC>;
#[doc = "Field `LP_ANA_BOD_MODE0_INT_RAW` reader - need_des"]
pub type LP_ANA_BOD_MODE0_INT_RAW_R = crate::BitReader;
#[doc = "Field `LP_ANA_BOD_MODE0_INT_RAW` writer - need_des"]
pub type LP_ANA_BOD_MODE0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_ana_bod_mode0_int_raw(&self) -> LP_ANA_BOD_MODE0_INT_RAW_R {
        LP_ANA_BOD_MODE0_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_INT_RAW")
            .field("lp_ana_bod_mode0_int_raw", &self.lp_ana_bod_mode0_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_ana_bod_mode0_int_raw(
        &mut self,
    ) -> LP_ANA_BOD_MODE0_INT_RAW_W<'_, LP_ANA_INT_RAW_SPEC> {
        LP_ANA_BOD_MODE0_INT_RAW_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ana_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ana_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_INT_RAW_SPEC;
impl crate::RegisterSpec for LP_ANA_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_int_raw::R`](R) reader structure"]
impl crate::Readable for LP_ANA_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_int_raw::W`](W) writer structure"]
impl crate::Writable for LP_ANA_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_ANA_INT_RAW to value 0"]
impl crate::Resettable for LP_ANA_INT_RAW_SPEC {}
