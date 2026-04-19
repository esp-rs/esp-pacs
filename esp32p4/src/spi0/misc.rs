#[doc = "Register `MISC` reader"]
pub type R = crate::R<MISC_SPEC>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MISC_SPEC>;
#[doc = "Field `DQ_OE_CTRL` reader - For SPI BUS IO, APB ctrl IO DQ OE func.1: enable 0: disable."]
pub type DQ_OE_CTRL_R = crate::BitReader;
#[doc = "Field `DQ_OE_CTRL` writer - For SPI BUS IO, APB ctrl IO DQ OE func.1: enable 0: disable."]
pub type DQ_OE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_OE_CTRL` reader - For SPI BUS IO, APB ctrl IO CK OE func.1: enable 0: disable."]
pub type CK_OE_CTRL_R = crate::BitReader;
#[doc = "Field `CK_OE_CTRL` writer - For SPI BUS IO, APB ctrl IO CK OE func.1: enable 0: disable."]
pub type CK_OE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_OE_CTRL` reader - For SPI BUS IO, APB ctrl IO CS OE func.1: enable 0: disable."]
pub type CS_OE_CTRL_R = crate::BitReader;
#[doc = "Field `CS_OE_CTRL` writer - For SPI BUS IO, APB ctrl IO CS OE func.1: enable 0: disable."]
pub type CS_OE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSUB_PIN` reader - For SPI0, flash is connected to SUBPINs."]
pub type FSUB_PIN_R = crate::BitReader;
#[doc = "Field `SSUB_PIN` reader - For SPI0, sram is connected to SUBPINs."]
pub type SSUB_PIN_R = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` reader - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
pub type CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` writer - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
pub type CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - SPI_CS line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `CS_KEEP_ACTIVE` writer - SPI_CS line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - For SPI BUS IO, APB ctrl IO DQ OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn dq_oe_ctrl(&self) -> DQ_OE_CTRL_R {
        DQ_OE_CTRL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For SPI BUS IO, APB ctrl IO CK OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn ck_oe_ctrl(&self) -> CK_OE_CTRL_R {
        CK_OE_CTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - For SPI BUS IO, APB ctrl IO CS OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn cs_oe_ctrl(&self) -> CS_OE_CTRL_R {
        CS_OE_CTRL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - For SPI0, flash is connected to SUBPINs."]
    #[inline(always)]
    pub fn fsub_pin(&self) -> FSUB_PIN_R {
        FSUB_PIN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - For SPI0, sram is connected to SUBPINs."]
    #[inline(always)]
    pub fn ssub_pin(&self) -> SSUB_PIN_R {
        SSUB_PIN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC")
            .field("dq_oe_ctrl", &self.dq_oe_ctrl())
            .field("ck_oe_ctrl", &self.ck_oe_ctrl())
            .field("cs_oe_ctrl", &self.cs_oe_ctrl())
            .field("fsub_pin", &self.fsub_pin())
            .field("ssub_pin", &self.ssub_pin())
            .field("ck_idle_edge", &self.ck_idle_edge())
            .field("cs_keep_active", &self.cs_keep_active())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - For SPI BUS IO, APB ctrl IO DQ OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn dq_oe_ctrl(&mut self) -> DQ_OE_CTRL_W<'_, MISC_SPEC> {
        DQ_OE_CTRL_W::new(self, 4)
    }
    #[doc = "Bit 5 - For SPI BUS IO, APB ctrl IO CK OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn ck_oe_ctrl(&mut self) -> CK_OE_CTRL_W<'_, MISC_SPEC> {
        CK_OE_CTRL_W::new(self, 5)
    }
    #[doc = "Bit 6 - For SPI BUS IO, APB ctrl IO CS OE func.1: enable 0: disable."]
    #[inline(always)]
    pub fn cs_oe_ctrl(&mut self) -> CS_OE_CTRL_W<'_, MISC_SPEC> {
        CS_OE_CTRL_W::new(self, 6)
    }
    #[doc = "Bit 9 - 1: SPI_CLK line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<'_, MISC_SPEC> {
        CK_IDLE_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<'_, MISC_SPEC> {
        CS_KEEP_ACTIVE_W::new(self, 10)
    }
}
#[doc = "SPI0 misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MISC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC to value 0x70"]
impl crate::Resettable for MISC_SPEC {
    const RESET_VALUE: u32 = 0x70;
}
