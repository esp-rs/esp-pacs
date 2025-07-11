#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `BOD_MODE0_INT_RAW` reader - brownout mode0 interrupt raw register"]
pub type BOD_MODE0_INT_RAW_R = crate::BitReader;
#[doc = "Field `BOD_MODE0_INT_RAW` writer - brownout mode0 interrupt raw register"]
pub type BOD_MODE0_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - brownout mode0 interrupt raw register"]
    #[inline(always)]
    pub fn bod_mode0_int_raw(&self) -> BOD_MODE0_INT_RAW_R {
        BOD_MODE0_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("bod_mode0_int_raw", &self.bod_mode0_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - brownout mode0 interrupt raw register"]
    #[inline(always)]
    pub fn bod_mode0_int_raw(&mut self) -> BOD_MODE0_INT_RAW_W<INT_RAW_SPEC> {
        BOD_MODE0_INT_RAW_W::new(self, 31)
    }
}
#[doc = "interrpt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
