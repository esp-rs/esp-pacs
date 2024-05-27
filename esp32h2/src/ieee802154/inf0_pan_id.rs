///Register `INF0_PAN_ID` reader
pub type R = crate::R<INF0_PAN_ID_SPEC>;
///Register `INF0_PAN_ID` writer
pub type W = crate::W<INF0_PAN_ID_SPEC>;
///Field `MAC_INF0_PAN_ID` reader -
pub type MAC_INF0_PAN_ID_R = crate::FieldReader<u16>;
///Field `MAC_INF0_PAN_ID` writer -
pub type MAC_INF0_PAN_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15
    #[inline(always)]
    pub fn mac_inf0_pan_id(&self) -> MAC_INF0_PAN_ID_R {
        MAC_INF0_PAN_ID_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INF0_PAN_ID")
            .field("mac_inf0_pan_id", &self.mac_inf0_pan_id())
            .finish()
    }
}
impl W {
    ///Bits 0:15
    #[inline(always)]
    #[must_use]
    pub fn mac_inf0_pan_id(&mut self) -> MAC_INF0_PAN_ID_W<INF0_PAN_ID_SPEC> {
        MAC_INF0_PAN_ID_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`inf0_pan_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf0_pan_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INF0_PAN_ID_SPEC;
impl crate::RegisterSpec for INF0_PAN_ID_SPEC {
    type Ux = u32;
}
///`read()` method returns [`inf0_pan_id::R`](R) reader structure
impl crate::Readable for INF0_PAN_ID_SPEC {}
///`write(|w| ..)` method takes [`inf0_pan_id::W`](W) writer structure
impl crate::Writable for INF0_PAN_ID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INF0_PAN_ID to value 0
impl crate::Resettable for INF0_PAN_ID_SPEC {
    const RESET_VALUE: u32 = 0;
}
