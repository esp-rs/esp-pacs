#[doc = "Register `LCD_BW_MONITOR_DBG` reader"]
pub type R = crate::R<LCD_BW_MONITOR_DBG_SPEC>;
#[doc = "Register `LCD_BW_MONITOR_DBG` writer"]
pub type W = crate::W<LCD_BW_MONITOR_DBG_SPEC>;
#[doc = "Field `LCD_BW_MONITORE_ENA` reader - 1:enable lcd monitor bandwidth.0: not enable lcd monitor bandwidth"]
pub type LCD_BW_MONITORE_ENA_R = crate::BitReader;
#[doc = "Field `LCD_BW_MONITORE_ENA` writer - 1:enable lcd monitor bandwidth.0: not enable lcd monitor bandwidth"]
pub type LCD_BW_MONITORE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_INVALID_RD_NUMBER` reader - the statistical data that lcd rd invalid data"]
pub type LCD_INVALID_RD_NUMBER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 1:enable lcd monitor bandwidth.0: not enable lcd monitor bandwidth"]
    #[inline(always)]
    pub fn lcd_bw_monitore_ena(&self) -> LCD_BW_MONITORE_ENA_R {
        LCD_BW_MONITORE_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - the statistical data that lcd rd invalid data"]
    #[inline(always)]
    pub fn lcd_invalid_rd_number(&self) -> LCD_INVALID_RD_NUMBER_R {
        LCD_INVALID_RD_NUMBER_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_BW_MONITOR_DBG")
            .field("lcd_bw_monitore_ena", &self.lcd_bw_monitore_ena())
            .field("lcd_invalid_rd_number", &self.lcd_invalid_rd_number())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1:enable lcd monitor bandwidth.0: not enable lcd monitor bandwidth"]
    #[inline(always)]
    pub fn lcd_bw_monitore_ena(&mut self) -> LCD_BW_MONITORE_ENA_W<'_, LCD_BW_MONITOR_DBG_SPEC> {
        LCD_BW_MONITORE_ENA_W::new(self, 0)
    }
}
#[doc = "LCDCAM LCD monitor bandwidth debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_bw_monitor_dbg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_bw_monitor_dbg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_BW_MONITOR_DBG_SPEC;
impl crate::RegisterSpec for LCD_BW_MONITOR_DBG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_bw_monitor_dbg::R`](R) reader structure"]
impl crate::Readable for LCD_BW_MONITOR_DBG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_bw_monitor_dbg::W`](W) writer structure"]
impl crate::Writable for LCD_BW_MONITOR_DBG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_BW_MONITOR_DBG to value 0"]
impl crate::Resettable for LCD_BW_MONITOR_DBG_SPEC {}
