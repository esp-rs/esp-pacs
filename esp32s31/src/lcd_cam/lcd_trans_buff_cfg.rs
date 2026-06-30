#[doc = "Register `LCD_TRANS_BUFF_CFG` reader"]
pub type R = crate::R<LCD_TRANS_BUFF_CFG_SPEC>;
#[doc = "Register `LCD_TRANS_BUFF_CFG` writer"]
pub type W = crate::W<LCD_TRANS_BUFF_CFG_SPEC>;
#[doc = "Field `LCD_TRANS_BUFFER_ENA` reader - 1:enable lcd trans buffer.0:not enable lcd trans buffer"]
pub type LCD_TRANS_BUFFER_ENA_R = crate::BitReader;
#[doc = "Field `LCD_TRANS_BUFFER_ENA` writer - 1:enable lcd trans buffer.0:not enable lcd trans buffer"]
pub type LCD_TRANS_BUFFER_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1:enable lcd trans buffer.0:not enable lcd trans buffer"]
    #[inline(always)]
    pub fn lcd_trans_buffer_ena(&self) -> LCD_TRANS_BUFFER_ENA_R {
        LCD_TRANS_BUFFER_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_TRANS_BUFF_CFG")
            .field("lcd_trans_buffer_ena", &self.lcd_trans_buffer_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1:enable lcd trans buffer.0:not enable lcd trans buffer"]
    #[inline(always)]
    pub fn lcd_trans_buffer_ena(&mut self) -> LCD_TRANS_BUFFER_ENA_W<'_, LCD_TRANS_BUFF_CFG_SPEC> {
        LCD_TRANS_BUFFER_ENA_W::new(self, 0)
    }
}
#[doc = "LCD trans buffer register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_trans_buff_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_trans_buff_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_TRANS_BUFF_CFG_SPEC;
impl crate::RegisterSpec for LCD_TRANS_BUFF_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_trans_buff_cfg::R`](R) reader structure"]
impl crate::Readable for LCD_TRANS_BUFF_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_trans_buff_cfg::W`](W) writer structure"]
impl crate::Writable for LCD_TRANS_BUFF_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_TRANS_BUFF_CFG to value 0"]
impl crate::Resettable for LCD_TRANS_BUFF_CFG_SPEC {}
