///Register `PIN` reader
pub type R = crate::R<PIN_SPEC>;
///Register `PIN` writer
pub type W = crate::W<PIN_SPEC>;
///Field `CS0_DIS` reader - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin
pub type CS0_DIS_R = crate::BitReader;
///Field `CS0_DIS` writer - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin
pub type CS0_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS1_DIS` reader - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin
pub type CS1_DIS_R = crate::BitReader;
///Field `CS1_DIS` writer - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin
pub type CS1_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS2_DIS` reader - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin
pub type CS2_DIS_R = crate::BitReader;
///Field `CS2_DIS` writer - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin
pub type CS2_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_DIS` reader - 1: spi clk out disable 0: spi clk out enable
pub type CK_DIS_R = crate::BitReader;
///Field `CK_DIS` writer - 1: spi clk out disable 0: spi clk out enable
pub type CK_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MASTER_CS_POL` reader - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol.
pub type MASTER_CS_POL_R = crate::FieldReader;
///Field `MASTER_CS_POL` writer - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol.
pub type MASTER_CS_POL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MASTER_CK_SEL` reader - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis.
pub type MASTER_CK_SEL_R = crate::FieldReader;
///Field `MASTER_CK_SEL` writer - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis.
pub type MASTER_CK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle
pub type CK_IDLE_EDGE_R = crate::BitReader;
///Field `CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle
pub type CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set.
pub type CS_KEEP_ACTIVE_R = crate::BitReader;
///Field `CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set.
pub type CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin
    #[inline(always)]
    pub fn cs0_dis(&self) -> CS0_DIS_R {
        CS0_DIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin
    #[inline(always)]
    pub fn cs1_dis(&self) -> CS1_DIS_R {
        CS1_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin
    #[inline(always)]
    pub fn cs2_dis(&self) -> CS2_DIS_R {
        CS2_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - 1: spi clk out disable 0: spi clk out enable
    #[inline(always)]
    pub fn ck_dis(&self) -> CK_DIS_R {
        CK_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:8 - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol.
    #[inline(always)]
    pub fn master_cs_pol(&self) -> MASTER_CS_POL_R {
        MASTER_CS_POL_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 11:13 - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis.
    #[inline(always)]
    pub fn master_ck_sel(&self) -> MASTER_CK_SEL_R {
        MASTER_CK_SEL_R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - spi cs line keep low when the bit is set.
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN")
            .field("cs0_dis", &self.cs0_dis())
            .field("cs1_dis", &self.cs1_dis())
            .field("cs2_dis", &self.cs2_dis())
            .field("ck_dis", &self.ck_dis())
            .field("master_cs_pol", &self.master_cs_pol())
            .field("master_ck_sel", &self.master_ck_sel())
            .field("ck_idle_edge", &self.ck_idle_edge())
            .field("cs_keep_active", &self.cs_keep_active())
            .finish()
    }
}
impl W {
    ///Bit 0 - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin
    #[inline(always)]
    #[must_use]
    pub fn cs0_dis(&mut self) -> CS0_DIS_W<PIN_SPEC> {
        CS0_DIS_W::new(self, 0)
    }
    ///Bit 1 - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin
    #[inline(always)]
    #[must_use]
    pub fn cs1_dis(&mut self) -> CS1_DIS_W<PIN_SPEC> {
        CS1_DIS_W::new(self, 1)
    }
    ///Bit 2 - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin
    #[inline(always)]
    #[must_use]
    pub fn cs2_dis(&mut self) -> CS2_DIS_W<PIN_SPEC> {
        CS2_DIS_W::new(self, 2)
    }
    ///Bit 5 - 1: spi clk out disable 0: spi clk out enable
    #[inline(always)]
    #[must_use]
    pub fn ck_dis(&mut self) -> CK_DIS_W<PIN_SPEC> {
        CK_DIS_W::new(self, 5)
    }
    ///Bits 6:8 - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol.
    #[inline(always)]
    #[must_use]
    pub fn master_cs_pol(&mut self) -> MASTER_CS_POL_W<PIN_SPEC> {
        MASTER_CS_POL_W::new(self, 6)
    }
    ///Bits 11:13 - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis.
    #[inline(always)]
    #[must_use]
    pub fn master_ck_sel(&mut self) -> MASTER_CK_SEL_W<PIN_SPEC> {
        MASTER_CK_SEL_W::new(self, 11)
    }
    ///Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle
    #[inline(always)]
    #[must_use]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<PIN_SPEC> {
        CK_IDLE_EDGE_W::new(self, 29)
    }
    ///Bit 30 - spi cs line keep low when the bit is set.
    #[inline(always)]
    #[must_use]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<PIN_SPEC> {
        CS_KEEP_ACTIVE_W::new(self, 30)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`pin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pin::R`](R) reader structure
impl crate::Readable for PIN_SPEC {}
///`write(|w| ..)` method takes [`pin::W`](W) writer structure
impl crate::Writable for PIN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PIN to value 0x06
impl crate::Resettable for PIN_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
