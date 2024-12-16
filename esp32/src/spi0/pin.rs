#[doc = "Register `PIN` reader"]
pub type R = crate::R<PIN_SPEC>;
#[doc = "Register `PIN` writer"]
pub type W = crate::W<PIN_SPEC>;
#[doc = "Field `CS_DIS(0-2)` reader - Set this bit to raise high SPI_CS%s pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS%s is in low level when SPI1 transfer starts"]
pub type CS_DIS_R = crate::BitReader;
#[doc = "Field `CS_DIS(0-2)` writer - Set this bit to raise high SPI_CS%s pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS%s is in low level when SPI1 transfer starts"]
pub type CS_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_DIS` reader - 1: spi clk out disable 0: spi clk out enable"]
pub type CK_DIS_R = crate::BitReader;
#[doc = "Field `CK_DIS` writer - 1: spi clk out disable 0: spi clk out enable"]
pub type CK_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER_CS_POL` reader - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol."]
pub type MASTER_CS_POL_R = crate::FieldReader;
#[doc = "Field `MASTER_CS_POL` writer - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol."]
pub type MASTER_CS_POL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MASTER_CK_SEL` reader - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis."]
pub type MASTER_CK_SEL_R = crate::FieldReader;
#[doc = "Field `MASTER_CK_SEL` writer - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis."]
pub type MASTER_CK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Set this bit to raise high SPI_CS(0-2) pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS(0-2) is in low level when SPI1 transfer starts"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CS0_DIS` field.</div>"]
    #[inline(always)]
    pub fn cs_dis(&self, n: u8) -> CS_DIS_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CS_DIS_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Set this bit to raise high SPI_CS(0-2) pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS(0-2) is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs_dis_iter(&self) -> impl Iterator<Item = CS_DIS_R> + '_ {
        (0..3).map(move |n| CS_DIS_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Set this bit to raise high SPI_CS0 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS0 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs0_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to raise high SPI_CS1 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS1 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs1_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to raise high SPI_CS2 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS2 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs2_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: spi clk out disable 0: spi clk out enable"]
    #[inline(always)]
    pub fn ck_dis(&self) -> CK_DIS_R {
        CK_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol."]
    #[inline(always)]
    pub fn master_cs_pol(&self) -> MASTER_CS_POL_R {
        MASTER_CS_POL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 11:13 - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis."]
    #[inline(always)]
    pub fn master_ck_sel(&self) -> MASTER_CK_SEL_R {
        MASTER_CK_SEL_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN")
            .field("ck_dis", &self.ck_dis())
            .field("master_cs_pol", &self.master_cs_pol())
            .field("master_ck_sel", &self.master_ck_sel())
            .field("ck_idle_edge", &self.ck_idle_edge())
            .field("cs_keep_active", &self.cs_keep_active())
            .field("cs0_dis", &self.cs0_dis())
            .field("cs1_dis", &self.cs1_dis())
            .field("cs2_dis", &self.cs2_dis())
            .finish()
    }
}
impl W {
    #[doc = "Set this bit to raise high SPI_CS(0-2) pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS(0-2) is in low level when SPI1 transfer starts"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CS0_DIS` field.</div>"]
    #[inline(always)]
    pub fn cs_dis(&mut self, n: u8) -> CS_DIS_W<PIN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CS_DIS_W::new(self, n)
    }
    #[doc = "Bit 0 - Set this bit to raise high SPI_CS0 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS0 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs0_dis(&mut self) -> CS_DIS_W<PIN_SPEC> {
        CS_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to raise high SPI_CS1 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS1 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs1_dis(&mut self) -> CS_DIS_W<PIN_SPEC> {
        CS_DIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to raise high SPI_CS2 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS2 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs2_dis(&mut self) -> CS_DIS_W<PIN_SPEC> {
        CS_DIS_W::new(self, 2)
    }
    #[doc = "Bit 5 - 1: spi clk out disable 0: spi clk out enable"]
    #[inline(always)]
    pub fn ck_dis(&mut self) -> CK_DIS_W<PIN_SPEC> {
        CK_DIS_W::new(self, 5)
    }
    #[doc = "Bits 6:8 - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol."]
    #[inline(always)]
    pub fn master_cs_pol(&mut self) -> MASTER_CS_POL_W<PIN_SPEC> {
        MASTER_CS_POL_W::new(self, 6)
    }
    #[doc = "Bits 11:13 - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis."]
    #[inline(always)]
    pub fn master_ck_sel(&mut self) -> MASTER_CK_SEL_W<PIN_SPEC> {
        MASTER_CK_SEL_W::new(self, 11)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<PIN_SPEC> {
        CK_IDLE_EDGE_W::new(self, 29)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<PIN_SPEC> {
        CS_KEEP_ACTIVE_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin::R`](R) reader structure"]
impl crate::Readable for PIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pin::W`](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIN to value 0x06"]
impl crate::Resettable for PIN_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
