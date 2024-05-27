///Register `DATA` reader
pub type R = crate::R<DATA_SPEC>;
///Register `DATA` writer
pub type W = crate::W<DATA_SPEC>;
///Field `FIFO_RDATA` reader - reg_fifo_rdata
pub type FIFO_RDATA_R = crate::FieldReader;
///Field `FIFO_RDATA` writer - reg_fifo_rdata
pub type FIFO_RDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - reg_fifo_rdata
    #[inline(always)]
    pub fn fifo_rdata(&self) -> FIFO_RDATA_R {
        FIFO_RDATA_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA")
            .field("fifo_rdata", &self.fifo_rdata())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - reg_fifo_rdata
    #[inline(always)]
    #[must_use]
    pub fn fifo_rdata(&mut self) -> FIFO_RDATA_W<DATA_SPEC> {
        FIFO_RDATA_W::new(self, 0)
    }
}
/**I2C_FIFO_DATA_REG

You can [`read`](crate::generic::Reg::read) this register and get [`data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`data::R`](R) reader structure
impl crate::Readable for DATA_SPEC {}
///`write(|w| ..)` method takes [`data::W`](W) writer structure
impl crate::Writable for DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATA to value 0
impl crate::Resettable for DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
