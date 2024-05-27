///Register `TX_POWER` reader
pub type R = crate::R<TX_POWER_SPEC>;
///Register `TX_POWER` writer
pub type W = crate::W<TX_POWER_SPEC>;
///Field `TX_POWER` reader -
pub type TX_POWER_R = crate::FieldReader;
///Field `TX_POWER` writer -
pub type TX_POWER_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4
    #[inline(always)]
    pub fn tx_power(&self) -> TX_POWER_R {
        TX_POWER_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_POWER")
            .field("tx_power", &self.tx_power())
            .finish()
    }
}
impl W {
    ///Bits 0:4
    #[inline(always)]
    #[must_use]
    pub fn tx_power(&mut self) -> TX_POWER_W<TX_POWER_SPEC> {
        TX_POWER_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`tx_power::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_power::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TX_POWER_SPEC;
impl crate::RegisterSpec for TX_POWER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tx_power::R`](R) reader structure
impl crate::Readable for TX_POWER_SPEC {}
///`write(|w| ..)` method takes [`tx_power::W`](W) writer structure
impl crate::Writable for TX_POWER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_POWER to value 0
impl crate::Resettable for TX_POWER_SPEC {
    const RESET_VALUE: u32 = 0;
}
