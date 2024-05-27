///Register `INT_RAW` reader
pub type R = crate::R<INT_RAW_SPEC>;
///Register `INT_RAW` writer
pub type W = crate::W<INT_RAW_SPEC>;
///Field `OVERFLOW` reader - need_des
pub type OVERFLOW_R = crate::BitReader;
///Field `OVERFLOW` writer - need_des
pub type OVERFLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOC_WAKEUP` reader - need_des
pub type SOC_WAKEUP_R = crate::BitReader;
///Field `SOC_WAKEUP` writer - need_des
pub type SOC_WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn soc_wakeup(&self) -> SOC_WAKEUP_R {
        SOC_WAKEUP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("overflow", &self.overflow())
            .field("soc_wakeup", &self.soc_wakeup())
            .finish()
    }
}
impl W {
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OVERFLOW_W<INT_RAW_SPEC> {
        OVERFLOW_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn soc_wakeup(&mut self) -> SOC_WAKEUP_W<INT_RAW_SPEC> {
        SOC_WAKEUP_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_raw::R`](R) reader structure
impl crate::Readable for INT_RAW_SPEC {}
///`write(|w| ..)` method takes [`int_raw::W`](W) writer structure
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_RAW to value 0
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
