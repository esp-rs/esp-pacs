///Register `DPI_LCD_CTL` reader
pub type R = crate::R<DPI_LCD_CTL_SPEC>;
///Register `DPI_LCD_CTL` writer
pub type W = crate::W<DPI_LCD_CTL_SPEC>;
///Field `DPISHUTDN` reader - this bit configures dpishutdn signal in dpi interface
pub type DPISHUTDN_R = crate::BitReader;
///Field `DPISHUTDN` writer - this bit configures dpishutdn signal in dpi interface
pub type DPISHUTDN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPICOLORM` reader - this bit configures dpicolorm signal in dpi interface
pub type DPICOLORM_R = crate::BitReader;
///Field `DPICOLORM` writer - this bit configures dpicolorm signal in dpi interface
pub type DPICOLORM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DPIUPDATECFG` reader - this bit configures dpiupdatecfg signal in dpi interface
pub type DPIUPDATECFG_R = crate::BitReader;
///Field `DPIUPDATECFG` writer - this bit configures dpiupdatecfg signal in dpi interface
pub type DPIUPDATECFG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - this bit configures dpishutdn signal in dpi interface
    #[inline(always)]
    pub fn dpishutdn(&self) -> DPISHUTDN_R {
        DPISHUTDN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - this bit configures dpicolorm signal in dpi interface
    #[inline(always)]
    pub fn dpicolorm(&self) -> DPICOLORM_R {
        DPICOLORM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - this bit configures dpiupdatecfg signal in dpi interface
    #[inline(always)]
    pub fn dpiupdatecfg(&self) -> DPIUPDATECFG_R {
        DPIUPDATECFG_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_LCD_CTL")
            .field("dpishutdn", &self.dpishutdn())
            .field("dpicolorm", &self.dpicolorm())
            .field("dpiupdatecfg", &self.dpiupdatecfg())
            .finish()
    }
}
impl W {
    ///Bit 0 - this bit configures dpishutdn signal in dpi interface
    #[inline(always)]
    #[must_use]
    pub fn dpishutdn(&mut self) -> DPISHUTDN_W<DPI_LCD_CTL_SPEC> {
        DPISHUTDN_W::new(self, 0)
    }
    ///Bit 1 - this bit configures dpicolorm signal in dpi interface
    #[inline(always)]
    #[must_use]
    pub fn dpicolorm(&mut self) -> DPICOLORM_W<DPI_LCD_CTL_SPEC> {
        DPICOLORM_W::new(self, 1)
    }
    ///Bit 2 - this bit configures dpiupdatecfg signal in dpi interface
    #[inline(always)]
    #[must_use]
    pub fn dpiupdatecfg(&mut self) -> DPIUPDATECFG_W<DPI_LCD_CTL_SPEC> {
        DPIUPDATECFG_W::new(self, 2)
    }
}
/**dsi bridge dpi signal control register

You can [`read`](crate::generic::Reg::read) this register and get [`dpi_lcd_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_lcd_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DPI_LCD_CTL_SPEC;
impl crate::RegisterSpec for DPI_LCD_CTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dpi_lcd_ctl::R`](R) reader structure
impl crate::Readable for DPI_LCD_CTL_SPEC {}
///`write(|w| ..)` method takes [`dpi_lcd_ctl::W`](W) writer structure
impl crate::Writable for DPI_LCD_CTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DPI_LCD_CTL to value 0
impl crate::Resettable for DPI_LCD_CTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
