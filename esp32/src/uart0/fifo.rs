///Register `FIFO` reader
pub type R = crate::R<FIFO_SPEC>;
///Register `FIFO` writer
pub type W = crate::W<FIFO_SPEC>;
///Field `RXFIFO_RD_BYTE` reader - This register stores one byte data read by rx fifo.
pub type RXFIFO_RD_BYTE_R = crate::FieldReader;
///Field `RXFIFO_RD_BYTE` writer - This register stores one byte data read by rx fifo.
pub type RXFIFO_RD_BYTE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - This register stores one byte data read by rx fifo.
    #[inline(always)]
    pub fn rxfifo_rd_byte(&self) -> RXFIFO_RD_BYTE_R {
        RXFIFO_RD_BYTE_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO")
            .field("rxfifo_rd_byte", &self.rxfifo_rd_byte())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - This register stores one byte data read by rx fifo.
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_rd_byte(&mut self) -> RXFIFO_RD_BYTE_W<FIFO_SPEC> {
        RXFIFO_RD_BYTE_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FIFO_SPEC;
impl crate::RegisterSpec for FIFO_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fifo::R`](R) reader structure
impl crate::Readable for FIFO_SPEC {}
///`write(|w| ..)` method takes [`fifo::W`](W) writer structure
impl crate::Writable for FIFO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FIFO to value 0
impl crate::Resettable for FIFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
