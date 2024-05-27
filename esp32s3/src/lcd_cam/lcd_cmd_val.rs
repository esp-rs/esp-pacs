#[doc = "Register `LCD_CMD_VAL` reader"]
pub type R = crate::R<LCD_CMD_VAL_SPEC>;
#[doc = "Register `LCD_CMD_VAL` writer"]
pub type W = crate::W<LCD_CMD_VAL_SPEC>;
#[doc = "Field `LCD_CMD_VALUE` reader - The LCD write command value."]
pub type LCD_CMD_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `LCD_CMD_VALUE` writer - The LCD write command value."]
pub type LCD_CMD_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The LCD write command value."]
    #[inline(always)]
    pub fn lcd_cmd_value(&self) -> LCD_CMD_VALUE_R {
        LCD_CMD_VALUE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_CMD_VAL")
            .field("lcd_cmd_value", &self.lcd_cmd_value())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The LCD write command value."]
    #[inline(always)]
    #[must_use]
    pub fn lcd_cmd_value(&mut self) -> LCD_CMD_VALUE_W<LCD_CMD_VAL_SPEC> {
        LCD_CMD_VALUE_W::new(self, 0)
    }
}
#[doc = "LCD command value configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcd_cmd_val::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_cmd_val::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_CMD_VAL_SPEC;
impl crate::RegisterSpec for LCD_CMD_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_cmd_val::R`](R) reader structure"]
impl crate::Readable for LCD_CMD_VAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_cmd_val::W`](W) writer structure"]
impl crate::Writable for LCD_CMD_VAL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCD_CMD_VAL to value 0"]
impl crate::Resettable for LCD_CMD_VAL_SPEC {
    const RESET_VALUE: u32 = 0;
}
