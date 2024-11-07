#[doc = "Register `LCD_D_NUM` reader"]
pub type R = crate::R<LCD_D_NUM_SPEC>;
#[doc = "Register `LCD_D_NUM` writer"]
pub type W = crate::W<LCD_D_NUM_SPEC>;
#[doc = "Field `D_DQS_NUM` reader - the output spi_dqs is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_DQS_NUM_R = crate::FieldReader;
#[doc = "Field `D_DQS_NUM` writer - the output spi_dqs is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_DQS_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `D_CD_NUM` reader - the output spi_cd is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_CD_NUM_R = crate::FieldReader;
#[doc = "Field `D_CD_NUM` writer - the output spi_cd is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_CD_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `D_DE_NUM` reader - the output spi_de is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_DE_NUM_R = crate::FieldReader;
#[doc = "Field `D_DE_NUM` writer - the output spi_de is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_DE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `D_HSYNC_NUM` reader - the output spi_hsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_HSYNC_NUM_R = crate::FieldReader;
#[doc = "Field `D_HSYNC_NUM` writer - the output spi_hsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_HSYNC_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `D_VSYNC_NUM` reader - the output spi_vsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_VSYNC_NUM_R = crate::FieldReader;
#[doc = "Field `D_VSYNC_NUM` writer - the output spi_vsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
pub type D_VSYNC_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - the output spi_dqs is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_dqs_num(&self) -> D_DQS_NUM_R {
        D_DQS_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the output spi_cd is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_cd_num(&self) -> D_CD_NUM_R {
        D_CD_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the output spi_de is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_de_num(&self) -> D_DE_NUM_R {
        D_DE_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the output spi_hsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_hsync_num(&self) -> D_HSYNC_NUM_R {
        D_HSYNC_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the output spi_vsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_vsync_num(&self) -> D_VSYNC_NUM_R {
        D_VSYNC_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_D_NUM")
            .field("d_dqs_num", &self.d_dqs_num())
            .field("d_cd_num", &self.d_cd_num())
            .field("d_de_num", &self.d_de_num())
            .field("d_hsync_num", &self.d_hsync_num())
            .field("d_vsync_num", &self.d_vsync_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - the output spi_dqs is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_dqs_num(&mut self) -> D_DQS_NUM_W<LCD_D_NUM_SPEC> {
        D_DQS_NUM_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - the output spi_cd is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_cd_num(&mut self) -> D_CD_NUM_W<LCD_D_NUM_SPEC> {
        D_CD_NUM_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - the output spi_de is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_de_num(&mut self) -> D_DE_NUM_W<LCD_D_NUM_SPEC> {
        D_DE_NUM_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - the output spi_hsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_hsync_num(&mut self) -> D_HSYNC_NUM_W<LCD_D_NUM_SPEC> {
        D_HSYNC_NUM_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - the output spi_vsync is delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,... Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_vsync_num(&mut self) -> D_VSYNC_NUM_W<LCD_D_NUM_SPEC> {
        D_VSYNC_NUM_W::new(self, 8)
    }
}
#[doc = "LCD delay mode\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_d_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_d_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCD_D_NUM_SPEC;
impl crate::RegisterSpec for LCD_D_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_d_num::R`](R) reader structure"]
impl crate::Readable for LCD_D_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcd_d_num::W`](W) writer structure"]
impl crate::Writable for LCD_D_NUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCD_D_NUM to value 0"]
impl crate::Resettable for LCD_D_NUM_SPEC {
    const RESET_VALUE: u32 = 0;
}
