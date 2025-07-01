#[doc = "Register `MISC` reader"]
pub type R = crate::R<MISC_SPEC>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MISC_SPEC>;
#[doc = "Field `CS0_DIS` reader - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state."]
pub type CS0_DIS_R = crate::BitReader;
#[doc = "Field `CS0_DIS` writer - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state."]
pub type CS0_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS1_DIS` reader - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state."]
pub type CS1_DIS_R = crate::BitReader;
#[doc = "Field `CS1_DIS` writer - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state."]
pub type CS1_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS2_DIS` reader - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state."]
pub type CS2_DIS_R = crate::BitReader;
#[doc = "Field `CS2_DIS` writer - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state."]
pub type CS2_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_DIS` reader - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
pub type CK_DIS_R = crate::BitReader;
#[doc = "Field `CK_DIS` writer - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
pub type CK_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER_CS_POL` reader - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
pub type MASTER_CS_POL_R = crate::FieldReader;
#[doc = "Field `MASTER_CS_POL` writer - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
pub type MASTER_CS_POL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SLAVE_CS_POL` reader - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
pub type SLAVE_CS_POL_R = crate::BitReader;
#[doc = "Field `SLAVE_CS_POL` writer - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
pub type SLAVE_CS_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
pub type CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
pub type CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set. Can be configured in CONF state."]
pub type CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set. Can be configured in CONF state."]
pub type CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUAD_DIN_PIN_SWAP` reader - 1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
pub type QUAD_DIN_PIN_SWAP_R = crate::BitReader;
#[doc = "Field `QUAD_DIN_PIN_SWAP` writer - 1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
pub type QUAD_DIN_PIN_SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs0_dis(&self) -> CS0_DIS_R {
        CS0_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs1_dis(&self) -> CS1_DIS_R {
        CS1_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs2_dis(&self) -> CS2_DIS_R {
        CS2_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ck_dis(&self) -> CK_DIS_R {
        CK_DIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
    #[inline(always)]
    pub fn master_cs_pol(&self) -> MASTER_CS_POL_R {
        MASTER_CS_POL_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 23 - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
    #[inline(always)]
    pub fn slave_cs_pol(&self) -> SLAVE_CS_POL_R {
        SLAVE_CS_POL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn quad_din_pin_swap(&self) -> QUAD_DIN_PIN_SWAP_R {
        QUAD_DIN_PIN_SWAP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC")
            .field("cs0_dis", &self.cs0_dis())
            .field("cs1_dis", &self.cs1_dis())
            .field("cs2_dis", &self.cs2_dis())
            .field("ck_dis", &self.ck_dis())
            .field("master_cs_pol", &self.master_cs_pol())
            .field("slave_cs_pol", &self.slave_cs_pol())
            .field("ck_idle_edge", &self.ck_idle_edge())
            .field("cs_keep_active", &self.cs_keep_active())
            .field("quad_din_pin_swap", &self.quad_din_pin_swap())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs0_dis(&mut self) -> CS0_DIS_W<MISC_SPEC> {
        CS0_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs1_dis(&mut self) -> CS1_DIS_W<MISC_SPEC> {
        CS1_DIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs2_dis(&mut self) -> CS2_DIS_W<MISC_SPEC> {
        CS2_DIS_W::new(self, 2)
    }
    #[doc = "Bit 6 - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ck_dis(&mut self) -> CK_DIS_W<MISC_SPEC> {
        CK_DIS_W::new(self, 6)
    }
    #[doc = "Bits 7:9 - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
    #[inline(always)]
    pub fn master_cs_pol(&mut self) -> MASTER_CS_POL_W<MISC_SPEC> {
        MASTER_CS_POL_W::new(self, 7)
    }
    #[doc = "Bit 23 - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
    #[inline(always)]
    pub fn slave_cs_pol(&mut self) -> SLAVE_CS_POL_W<MISC_SPEC> {
        SLAVE_CS_POL_W::new(self, 23)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<MISC_SPEC> {
        CK_IDLE_EDGE_W::new(self, 29)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<MISC_SPEC> {
        CS_KEEP_ACTIVE_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn quad_din_pin_swap(&mut self) -> QUAD_DIN_PIN_SWAP_W<MISC_SPEC> {
        QUAD_DIN_PIN_SWAP_W::new(self, 31)
    }
}
#[doc = "SPI misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets MISC to value 0x06"]
impl crate::Resettable for MISC_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
