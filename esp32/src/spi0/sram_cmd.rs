///Register `SRAM_CMD` reader
pub type R = crate::R<SRAM_CMD_SPEC>;
///Register `SRAM_CMD` writer
pub type W = crate::W<SRAM_CMD_SPEC>;
///Field `SRAM_DIO` reader - For SPI0 SRAM DIO mode enable . SRAM DIO enable command will be send when the bit is set. The bit will be cleared once the operation done.
pub type SRAM_DIO_R = crate::BitReader;
///Field `SRAM_DIO` writer - For SPI0 SRAM DIO mode enable . SRAM DIO enable command will be send when the bit is set. The bit will be cleared once the operation done.
pub type SRAM_DIO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM_QIO` reader - For SPI0 SRAM QIO mode enable . SRAM QIO enable command will be send when the bit is set. The bit will be cleared once the operation done.
pub type SRAM_QIO_R = crate::BitReader;
///Field `SRAM_QIO` writer - For SPI0 SRAM QIO mode enable . SRAM QIO enable command will be send when the bit is set. The bit will be cleared once the operation done.
pub type SRAM_QIO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM_RSTIO` reader - For SPI0 SRAM IO mode reset enable. SRAM IO mode reset operation will be triggered when the bit is set. The bit will be cleared once the operation done
pub type SRAM_RSTIO_R = crate::BitReader;
///Field `SRAM_RSTIO` writer - For SPI0 SRAM IO mode reset enable. SRAM IO mode reset operation will be triggered when the bit is set. The bit will be cleared once the operation done
pub type SRAM_RSTIO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - For SPI0 SRAM DIO mode enable . SRAM DIO enable command will be send when the bit is set. The bit will be cleared once the operation done.
    #[inline(always)]
    pub fn sram_dio(&self) -> SRAM_DIO_R {
        SRAM_DIO_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - For SPI0 SRAM QIO mode enable . SRAM QIO enable command will be send when the bit is set. The bit will be cleared once the operation done.
    #[inline(always)]
    pub fn sram_qio(&self) -> SRAM_QIO_R {
        SRAM_QIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - For SPI0 SRAM IO mode reset enable. SRAM IO mode reset operation will be triggered when the bit is set. The bit will be cleared once the operation done
    #[inline(always)]
    pub fn sram_rstio(&self) -> SRAM_RSTIO_R {
        SRAM_RSTIO_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_CMD")
            .field("sram_dio", &self.sram_dio())
            .field("sram_qio", &self.sram_qio())
            .field("sram_rstio", &self.sram_rstio())
            .finish()
    }
}
impl W {
    ///Bit 0 - For SPI0 SRAM DIO mode enable . SRAM DIO enable command will be send when the bit is set. The bit will be cleared once the operation done.
    #[inline(always)]
    #[must_use]
    pub fn sram_dio(&mut self) -> SRAM_DIO_W<SRAM_CMD_SPEC> {
        SRAM_DIO_W::new(self, 0)
    }
    ///Bit 1 - For SPI0 SRAM QIO mode enable . SRAM QIO enable command will be send when the bit is set. The bit will be cleared once the operation done.
    #[inline(always)]
    #[must_use]
    pub fn sram_qio(&mut self) -> SRAM_QIO_W<SRAM_CMD_SPEC> {
        SRAM_QIO_W::new(self, 1)
    }
    ///Bit 4 - For SPI0 SRAM IO mode reset enable. SRAM IO mode reset operation will be triggered when the bit is set. The bit will be cleared once the operation done
    #[inline(always)]
    #[must_use]
    pub fn sram_rstio(&mut self) -> SRAM_RSTIO_W<SRAM_CMD_SPEC> {
        SRAM_RSTIO_W::new(self, 4)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`sram_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SRAM_CMD_SPEC;
impl crate::RegisterSpec for SRAM_CMD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sram_cmd::R`](R) reader structure
impl crate::Readable for SRAM_CMD_SPEC {}
///`write(|w| ..)` method takes [`sram_cmd::W`](W) writer structure
impl crate::Writable for SRAM_CMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SRAM_CMD to value 0
impl crate::Resettable for SRAM_CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
