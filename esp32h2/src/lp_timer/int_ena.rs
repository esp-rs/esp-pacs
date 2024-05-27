///Register `INT_ENA` reader
pub type R = crate::R<INT_ENA_SPEC>;
///Register `INT_ENA` writer
pub type W = crate::W<INT_ENA_SPEC>;
///Field `OVERFLOW_ENA` reader - need_des
pub type OVERFLOW_ENA_R = crate::BitReader;
///Field `OVERFLOW_ENA` writer - need_des
pub type OVERFLOW_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOC_WAKEUP_INT_ENA` reader - need_des
pub type SOC_WAKEUP_INT_ENA_R = crate::BitReader;
///Field `SOC_WAKEUP_INT_ENA` writer - need_des
pub type SOC_WAKEUP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn overflow_ena(&self) -> OVERFLOW_ENA_R {
        OVERFLOW_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn soc_wakeup_int_ena(&self) -> SOC_WAKEUP_INT_ENA_R {
        SOC_WAKEUP_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("overflow_ena", &self.overflow_ena())
            .field("soc_wakeup_int_ena", &self.soc_wakeup_int_ena())
            .finish()
    }
}
impl W {
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn overflow_ena(&mut self) -> OVERFLOW_ENA_W<INT_ENA_SPEC> {
        OVERFLOW_ENA_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn soc_wakeup_int_ena(&mut self) -> SOC_WAKEUP_INT_ENA_W<INT_ENA_SPEC> {
        SOC_WAKEUP_INT_ENA_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_ena::R`](R) reader structure
impl crate::Readable for INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`int_ena::W`](W) writer structure
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ENA to value 0
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
