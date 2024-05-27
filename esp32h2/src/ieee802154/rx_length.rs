///Register `RX_LENGTH` reader
pub type R = crate::R<RX_LENGTH_SPEC>;
///Register `RX_LENGTH` writer
pub type W = crate::W<RX_LENGTH_SPEC>;
///Field `RX_LENGTH` reader -
pub type RX_LENGTH_R = crate::FieldReader;
///Field `RX_LENGTH` writer -
pub type RX_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6
    #[inline(always)]
    pub fn rx_length(&self) -> RX_LENGTH_R {
        RX_LENGTH_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_LENGTH")
            .field("rx_length", &self.rx_length())
            .finish()
    }
}
impl W {
    ///Bits 0:6
    #[inline(always)]
    #[must_use]
    pub fn rx_length(&mut self) -> RX_LENGTH_W<RX_LENGTH_SPEC> {
        RX_LENGTH_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`rx_length::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_length::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RX_LENGTH_SPEC;
impl crate::RegisterSpec for RX_LENGTH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rx_length::R`](R) reader structure
impl crate::Readable for RX_LENGTH_SPEC {}
///`write(|w| ..)` method takes [`rx_length::W`](W) writer structure
impl crate::Writable for RX_LENGTH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RX_LENGTH to value 0
impl crate::Resettable for RX_LENGTH_SPEC {
    const RESET_VALUE: u32 = 0;
}
