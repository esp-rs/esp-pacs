#[doc = "Register `SPI_MISC` reader"]
pub type R = crate::R<SPI_MISC_SPEC>;
#[doc = "Register `SPI_MISC` writer"]
pub type W = crate::W<SPI_MISC_SPEC>;
#[doc = "Field `SPI_CS0_DIS` reader - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state."]
pub type SPI_CS0_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CS0_DIS` writer - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state."]
pub type SPI_CS0_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS1_DIS` reader - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state."]
pub type SPI_CS1_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CS1_DIS` writer - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state."]
pub type SPI_CS1_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS2_DIS` reader - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state."]
pub type SPI_CS2_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CS2_DIS` writer - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state."]
pub type SPI_CS2_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CK_DIS` reader - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
pub type SPI_CK_DIS_R = crate::BitReader;
#[doc = "Field `SPI_CK_DIS` writer - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
pub type SPI_CK_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MASTER_CS_POL` reader - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
pub type SPI_MASTER_CS_POL_R = crate::FieldReader;
#[doc = "Field `SPI_MASTER_CS_POL` writer - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
pub type SPI_MASTER_CS_POL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SLAVE_CS_POL` reader - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
pub type SPI_SLAVE_CS_POL_R = crate::BitReader;
#[doc = "Field `SPI_SLAVE_CS_POL` writer - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
pub type SPI_SLAVE_CS_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
pub type SPI_CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
pub type SPI_CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set. Can be configured in CONF state."]
pub type SPI_CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `SPI_CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set. Can be configured in CONF state."]
pub type SPI_CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_QUAD_DIN_PIN_SWAP` reader - 1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
pub type SPI_QUAD_DIN_PIN_SWAP_R = crate::BitReader;
#[doc = "Field `SPI_QUAD_DIN_PIN_SWAP` writer - 1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
pub type SPI_QUAD_DIN_PIN_SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs0_dis(&self) -> SPI_CS0_DIS_R {
        SPI_CS0_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs1_dis(&self) -> SPI_CS1_DIS_R {
        SPI_CS1_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs2_dis(&self) -> SPI_CS2_DIS_R {
        SPI_CS2_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_ck_dis(&self) -> SPI_CK_DIS_R {
        SPI_CK_DIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_master_cs_pol(&self) -> SPI_MASTER_CS_POL_R {
        SPI_MASTER_CS_POL_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 23 - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_slave_cs_pol(&self) -> SPI_SLAVE_CS_POL_R {
        SPI_SLAVE_CS_POL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_ck_idle_edge(&self) -> SPI_CK_IDLE_EDGE_R {
        SPI_CK_IDLE_EDGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs_keep_active(&self) -> SPI_CS_KEEP_ACTIVE_R {
        SPI_CS_KEEP_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_quad_din_pin_swap(&self) -> SPI_QUAD_DIN_PIN_SWAP_R {
        SPI_QUAD_DIN_PIN_SWAP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MISC")
            .field("spi_cs0_dis", &self.spi_cs0_dis())
            .field("spi_cs1_dis", &self.spi_cs1_dis())
            .field("spi_cs2_dis", &self.spi_cs2_dis())
            .field("spi_ck_dis", &self.spi_ck_dis())
            .field("spi_master_cs_pol", &self.spi_master_cs_pol())
            .field("spi_slave_cs_pol", &self.spi_slave_cs_pol())
            .field("spi_ck_idle_edge", &self.spi_ck_idle_edge())
            .field("spi_cs_keep_active", &self.spi_cs_keep_active())
            .field("spi_quad_din_pin_swap", &self.spi_quad_din_pin_swap())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs0_dis(&mut self) -> SPI_CS0_DIS_W<'_, SPI_MISC_SPEC> {
        SPI_CS0_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs1_dis(&mut self) -> SPI_CS1_DIS_W<'_, SPI_MISC_SPEC> {
        SPI_CS1_DIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs2_dis(&mut self) -> SPI_CS2_DIS_W<'_, SPI_MISC_SPEC> {
        SPI_CS2_DIS_W::new(self, 2)
    }
    #[doc = "Bit 6 - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_ck_dis(&mut self) -> SPI_CK_DIS_W<'_, SPI_MISC_SPEC> {
        SPI_CK_DIS_W::new(self, 6)
    }
    #[doc = "Bits 7:9 - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_master_cs_pol(&mut self) -> SPI_MASTER_CS_POL_W<'_, SPI_MISC_SPEC> {
        SPI_MASTER_CS_POL_W::new(self, 7)
    }
    #[doc = "Bit 23 - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_slave_cs_pol(&mut self) -> SPI_SLAVE_CS_POL_W<'_, SPI_MISC_SPEC> {
        SPI_SLAVE_CS_POL_W::new(self, 23)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_ck_idle_edge(&mut self) -> SPI_CK_IDLE_EDGE_W<'_, SPI_MISC_SPEC> {
        SPI_CK_IDLE_EDGE_W::new(self, 29)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_cs_keep_active(&mut self) -> SPI_CS_KEEP_ACTIVE_W<'_, SPI_MISC_SPEC> {
        SPI_CS_KEEP_ACTIVE_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_quad_din_pin_swap(&mut self) -> SPI_QUAD_DIN_PIN_SWAP_W<'_, SPI_MISC_SPEC> {
        SPI_QUAD_DIN_PIN_SWAP_W::new(self, 31)
    }
}
#[doc = "SPI misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MISC_SPEC;
impl crate::RegisterSpec for SPI_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_misc::R`](R) reader structure"]
impl crate::Readable for SPI_MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_misc::W`](W) writer structure"]
impl crate::Writable for SPI_MISC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_MISC to value 0x06"]
impl crate::Resettable for SPI_MISC_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
