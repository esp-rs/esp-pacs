///Register `INF1_SHORT_ADDR` reader
pub type R = crate::R<INF1_SHORT_ADDR_SPEC>;
///Register `INF1_SHORT_ADDR` writer
pub type W = crate::W<INF1_SHORT_ADDR_SPEC>;
///Field `MAC_INF1_SHORT_ADDR` reader -
pub type MAC_INF1_SHORT_ADDR_R = crate::FieldReader<u16>;
///Field `MAC_INF1_SHORT_ADDR` writer -
pub type MAC_INF1_SHORT_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15
    #[inline(always)]
    pub fn mac_inf1_short_addr(&self) -> MAC_INF1_SHORT_ADDR_R {
        MAC_INF1_SHORT_ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INF1_SHORT_ADDR")
            .field("mac_inf1_short_addr", &self.mac_inf1_short_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:15
    #[inline(always)]
    #[must_use]
    pub fn mac_inf1_short_addr(&mut self) -> MAC_INF1_SHORT_ADDR_W<INF1_SHORT_ADDR_SPEC> {
        MAC_INF1_SHORT_ADDR_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`inf1_short_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf1_short_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INF1_SHORT_ADDR_SPEC;
impl crate::RegisterSpec for INF1_SHORT_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`inf1_short_addr::R`](R) reader structure
impl crate::Readable for INF1_SHORT_ADDR_SPEC {}
///`write(|w| ..)` method takes [`inf1_short_addr::W`](W) writer structure
impl crate::Writable for INF1_SHORT_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INF1_SHORT_ADDR to value 0
impl crate::Resettable for INF1_SHORT_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
