#[doc = "Register `LP_INT_RAW` reader"]
pub type R = crate::R<LP_INT_RAW_SPEC>;
#[doc = "Register `LP_INT_RAW` writer"]
pub type W = crate::W<LP_INT_RAW_SPEC>;
#[doc = "Field `BOD_MODE0_LP_INT_RAW` reader - brownout mode0 lp interrupt raw register"]
pub type BOD_MODE0_LP_INT_RAW_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_LP_INT_RAW` writer - brownout mode0 lp interrupt raw register"]
pub type BOD_MODE0_LP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - brownout mode0 lp interrupt raw register"]
    #[inline(always)]
    pub fn bod_mode0_lp_int_raw(&self) -> BOD_MODE0_LP_INT_RAW_R {
        BOD_MODE0_LP_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_RAW")
            .field("bod_mode0_lp_int_raw", &self.bod_mode0_lp_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - brownout mode0 lp interrupt raw register"]
    #[inline(always)]
    pub fn bod_mode0_lp_int_raw(&mut self) -> BOD_MODE0_LP_INT_RAW_W<LP_INT_RAW_SPEC> {
        BOD_MODE0_LP_INT_RAW_W::new(self, 31)
    }
}
#[doc = "lp interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_RAW_SPEC;
impl crate::RegisterSpec for LP_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_raw::R`](R) reader structure"]
impl crate::Readable for LP_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_int_raw::W`](W) writer structure"]
impl crate::Writable for LP_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_INT_RAW to value 0"]
impl crate::Resettable for LP_INT_RAW_SPEC {}
