#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `OVERFLOW_RAW` reader - need_des"]
pub type OVERFLOW_RAW_R = crate::BitReader;
#[doc = "Field `OVERFLOW_RAW` writer - need_des"]
pub type OVERFLOW_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC_WAKEUP_INT_RAW` reader - need_des"]
pub type SOC_WAKEUP_INT_RAW_R = crate::BitReader;
#[doc = "Field `SOC_WAKEUP_INT_RAW` writer - need_des"]
pub type SOC_WAKEUP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn overflow_raw(&self) -> OVERFLOW_RAW_R {
        OVERFLOW_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup_int_raw(&self) -> SOC_WAKEUP_INT_RAW_R {
        SOC_WAKEUP_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("overflow_raw", &self.overflow_raw())
            .field("soc_wakeup_int_raw", &self.soc_wakeup_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn overflow_raw(&mut self) -> OVERFLOW_RAW_W<INT_RAW_SPEC> {
        OVERFLOW_RAW_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup_int_raw(&mut self) -> SOC_WAKEUP_INT_RAW_W<INT_RAW_SPEC> {
        SOC_WAKEUP_INT_RAW_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
