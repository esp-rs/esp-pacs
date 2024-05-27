///Register `MAC_DATE` reader
pub type R = crate::R<MAC_DATE_SPEC>;
///Register `MAC_DATE` writer
pub type W = crate::W<MAC_DATE_SPEC>;
///Field `MAC_DATE` reader -
pub type MAC_DATE_R = crate::FieldReader<u32>;
///Field `MAC_DATE` writer -
pub type MAC_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn mac_date(&self) -> MAC_DATE_R {
        MAC_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAC_DATE")
            .field("mac_date", &self.mac_date())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    #[must_use]
    pub fn mac_date(&mut self) -> MAC_DATE_W<MAC_DATE_SPEC> {
        MAC_DATE_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`mac_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MAC_DATE_SPEC;
impl crate::RegisterSpec for MAC_DATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mac_date::R`](R) reader structure
impl crate::Readable for MAC_DATE_SPEC {}
///`write(|w| ..)` method takes [`mac_date::W`](W) writer structure
impl crate::Writable for MAC_DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MAC_DATE to value 0
impl crate::Resettable for MAC_DATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
