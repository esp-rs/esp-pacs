///Register `INF0_EXTEND_ADDR0` reader
pub type R = crate::R<INF0_EXTEND_ADDR0_SPEC>;
///Register `INF0_EXTEND_ADDR0` writer
pub type W = crate::W<INF0_EXTEND_ADDR0_SPEC>;
///Field `MAC_INF0_EXTEND_ADDR0` reader -
pub type MAC_INF0_EXTEND_ADDR0_R = crate::FieldReader<u32>;
///Field `MAC_INF0_EXTEND_ADDR0` writer -
pub type MAC_INF0_EXTEND_ADDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn mac_inf0_extend_addr0(&self) -> MAC_INF0_EXTEND_ADDR0_R {
        MAC_INF0_EXTEND_ADDR0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INF0_EXTEND_ADDR0")
            .field("mac_inf0_extend_addr0", &self.mac_inf0_extend_addr0())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    #[must_use]
    pub fn mac_inf0_extend_addr0(&mut self) -> MAC_INF0_EXTEND_ADDR0_W<INF0_EXTEND_ADDR0_SPEC> {
        MAC_INF0_EXTEND_ADDR0_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`inf0_extend_addr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf0_extend_addr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INF0_EXTEND_ADDR0_SPEC;
impl crate::RegisterSpec for INF0_EXTEND_ADDR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`inf0_extend_addr0::R`](R) reader structure
impl crate::Readable for INF0_EXTEND_ADDR0_SPEC {}
///`write(|w| ..)` method takes [`inf0_extend_addr0::W`](W) writer structure
impl crate::Writable for INF0_EXTEND_ADDR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INF0_EXTEND_ADDR0 to value 0
impl crate::Resettable for INF0_EXTEND_ADDR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
