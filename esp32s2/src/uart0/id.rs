///Register `ID` reader
pub type R = crate::R<ID_SPEC>;
///Register `ID` writer
pub type W = crate::W<ID_SPEC>;
///Field `ID` reader - This register is used to configure the UART_ID.
pub type ID_R = crate::FieldReader<u32>;
///Field `ID` writer - This register is used to configure the UART_ID.
pub type ID_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - This register is used to configure the UART_ID.
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID").field("id", &self.id()).finish()
    }
}
impl W {
    ///Bits 0:31 - This register is used to configure the UART_ID.
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<ID_SPEC> {
        ID_W::new(self, 0)
    }
}
/**UART ID register

You can [`read`](crate::generic::Reg::read) this register and get [`id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
///`read()` method returns [`id::R`](R) reader structure
impl crate::Readable for ID_SPEC {}
///`write(|w| ..)` method takes [`id::W`](W) writer structure
impl crate::Writable for ID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ID to value 0x0500
impl crate::Resettable for ID_SPEC {
    const RESET_VALUE: u32 = 0x0500;
}
