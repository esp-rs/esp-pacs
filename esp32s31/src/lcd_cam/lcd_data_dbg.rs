#[doc = "Register `LCD_DATA_DBG` reader"]
pub type R = crate::R<LCD_DATA_DBG_SPEC>;
#[doc = "Register `LCD_DATA_DBG` writer"]
pub type W = crate::W<LCD_DATA_DBG_SPEC>;
#[doc = "Field `LCD_DBG_DATA_RELPACE_ENA` reader - 1:replace the data read from dma.0:not replace the data read from dma"]
pub type LCD_DBG_DATA_RELPACE_ENA_R = crate::BitReader;
#[doc = "Field `LCD_DBG_DATA_RELPACE_ENA` writer - 1:replace the data read from dma.0:not replace the data read from dma"]
pub type LCD_DBG_DATA_RELPACE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_DBG_DATA` reader - the debug data that used to replace data read from dma"]
pub type LCD_DBG_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `LCD_DBG_DATA` writer - the debug data that used to replace data read from dma"]
pub type LCD_DBG_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 1:replace the data read from dma.0:not replace the data read from dma"]
    #[inline(always)]
    pub fn lcd_dbg_data_relpace_ena(&self) -> LCD_DBG_DATA_RELPACE_ENA_R {
        LCD_DBG_DATA_RELPACE_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:24 - the debug data that used to replace data read from dma"]
    #[inline(always)]
    pub fn lcd_dbg_data(&self) -> LCD_DBG_DATA_R {
        LCD_DBG_DATA_R::new((self.bits >> 1) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_DATA_DBG")
            .field("lcd_dbg_data_relpace_ena", &self.lcd_dbg_data_relpace_ena())
            .field("lcd_dbg_data", &self.lcd_dbg_data())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1:replace the data read from dma.0:not replace the data read from dma"]
    #[inline(always)]
    pub fn lcd_dbg_data_relpace_ena(
        &mut self,
    ) -> LCD_DBG_DATA_RELPACE_ENA_W<'_, LCD_DATA_DBG_SPEC> {
        LCD_DBG_DATA_RELPACE_ENA_W::new(self, 0)
    }
    #[doc = "Bits 1:24 - the debug data that used to replace data read from dma"]
    #[inline(always)]
    pub fn lcd_dbg_data(&mut self) -> LCD_DBG_DATA_W<'_, LCD_DATA_DBG_SPEC> {
        LCD_DBG_DATA_W::new(self, 1)
    }
}
#[doc = "LCDCAM LCD debug data register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_data_dbg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_data_dbg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_DATA_DBG_SPEC;
impl crate::RegisterSpec for LCD_DATA_DBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_data_dbg::R`](R) reader structure"]
impl crate::Readable for LCD_DATA_DBG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_data_dbg::W`](W) writer structure"]
impl crate::Writable for LCD_DATA_DBG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_DATA_DBG to value 0"]
impl crate::Resettable for LCD_DATA_DBG_SPEC {}
