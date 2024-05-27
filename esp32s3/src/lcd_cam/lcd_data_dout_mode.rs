///Register `LCD_DATA_DOUT_MODE` reader
pub type R = crate::R<LCD_DATA_DOUT_MODE_SPEC>;
///Register `LCD_DATA_DOUT_MODE` writer
pub type W = crate::W<LCD_DATA_DOUT_MODE_SPEC>;
/**The output data bit %s is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DELAY_MODE {
    ///0: Output without delay
    None = 0,
    ///1: Delayed by the rising edge of LCD_CLK
    RaisingEdge = 1,
    ///2: Delayed by the falling edge of LCD_CLK
    FallingEdge = 2,
}
impl From<DELAY_MODE> for u8 {
    #[inline(always)]
    fn from(variant: DELAY_MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DELAY_MODE {
    type Ux = u8;
}
impl crate::IsEnum for DELAY_MODE {}
///Field `DOUT_MODE(0-15)` reader - The output data bit %s is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
pub type DOUT_MODE_R = crate::FieldReader<DELAY_MODE>;
impl DOUT_MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DELAY_MODE> {
        match self.bits {
            0 => Some(DELAY_MODE::None),
            1 => Some(DELAY_MODE::RaisingEdge),
            2 => Some(DELAY_MODE::FallingEdge),
            _ => None,
        }
    }
    ///Output without delay
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DELAY_MODE::None
    }
    ///Delayed by the rising edge of LCD_CLK
    #[inline(always)]
    pub fn is_raising_edge(&self) -> bool {
        *self == DELAY_MODE::RaisingEdge
    }
    ///Delayed by the falling edge of LCD_CLK
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == DELAY_MODE::FallingEdge
    }
}
///Field `DOUT_MODE(0-15)` writer - The output data bit %s is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
pub type DOUT_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DELAY_MODE>;
impl<'a, REG> DOUT_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Output without delay
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DELAY_MODE::None)
    }
    ///Delayed by the rising edge of LCD_CLK
    #[inline(always)]
    pub fn raising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(DELAY_MODE::RaisingEdge)
    }
    ///Delayed by the falling edge of LCD_CLK
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(DELAY_MODE::FallingEdge)
    }
}
impl R {
    ///The output data bit (0-15) is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `DOUT0_MODE` field
    #[inline(always)]
    pub fn dout_mode(&self, n: u8) -> DOUT_MODE_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        DOUT_MODE_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    ///Iterator for array of:
    ///The output data bit (0-15) is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout_mode_iter(&self) -> impl Iterator<Item = DOUT_MODE_R> + '_ {
        (0..16).map(move |n| DOUT_MODE_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    ///Bits 0:1 - The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout0_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - The output data bit 1 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout1_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout2_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - The output data bit 3 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout3_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout4_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - The output data bit 5 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout5_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout6_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - The output data bit 7 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout7_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout8_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - The output data bit 9 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout9_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout10_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - The output data bit 11 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout11_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout12_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - The output data bit 13 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout13_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout14_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - The output data bit 15 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    pub fn dout15_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD_DATA_DOUT_MODE")
            .field("dout0_mode", &self.dout0_mode())
            .field("dout1_mode", &self.dout1_mode())
            .field("dout2_mode", &self.dout2_mode())
            .field("dout3_mode", &self.dout3_mode())
            .field("dout4_mode", &self.dout4_mode())
            .field("dout5_mode", &self.dout5_mode())
            .field("dout6_mode", &self.dout6_mode())
            .field("dout7_mode", &self.dout7_mode())
            .field("dout8_mode", &self.dout8_mode())
            .field("dout9_mode", &self.dout9_mode())
            .field("dout10_mode", &self.dout10_mode())
            .field("dout11_mode", &self.dout11_mode())
            .field("dout12_mode", &self.dout12_mode())
            .field("dout13_mode", &self.dout13_mode())
            .field("dout14_mode", &self.dout14_mode())
            .field("dout15_mode", &self.dout15_mode())
            .finish()
    }
}
impl W {
    ///The output data bit (0-15) is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `DOUT0_MODE` field
    #[inline(always)]
    #[must_use]
    pub fn dout_mode(&mut self, n: u8) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        DOUT_MODE_W::new(self, n * 2)
    }
    ///Bits 0:1 - The output data bit 0 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dout0_mode(&mut self) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 0)
    }
    ///Bits 2:3 - The output data bit 1 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dout1_mode(&mut self) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 2)
    }
    ///Bits 4:5 - The output data bit 2 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dout2_mode(&mut self) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 4)
    }
    ///Bits 6:7 - The output data bit 3 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dout3_mode(&mut self) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 6)
    }
    ///Bits 8:9 - The output data bit 4 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dout4_mode(&mut self) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 8)
    }
    ///Bits 10:11 - The output data bit 5 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dout5_mode(&mut self) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 10)
    }
    ///Bits 12:13 - The output data bit 6 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dout6_mode(&mut self) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 12)
    }
    ///Bits 14:15 - The output data bit 7 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dout7_mode(&mut self) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 14)
    }
    ///Bits 16:17 - The output data bit 8 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dout8_mode(&mut self) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 16)
    }
    ///Bits 18:19 - The output data bit 9 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dout9_mode(&mut self) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 18)
    }
    ///Bits 20:21 - The output data bit 10 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dout10_mode(&mut self) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 20)
    }
    ///Bits 22:23 - The output data bit 11 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dout11_mode(&mut self) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 22)
    }
    ///Bits 24:25 - The output data bit 12 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dout12_mode(&mut self) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 24)
    }
    ///Bits 26:27 - The output data bit 13 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dout13_mode(&mut self) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 26)
    }
    ///Bits 28:29 - The output data bit 14 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dout14_mode(&mut self) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 28)
    }
    ///Bits 30:31 - The output data bit 15 is delayed by module clock LCD_CLK. 0: output without delay. 1: delayed by the rising edge of LCD_CLK. 2: delayed by the falling edge of LCD_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dout15_mode(&mut self) -> DOUT_MODE_W<LCD_DATA_DOUT_MODE_SPEC> {
        DOUT_MODE_W::new(self, 30)
    }
}
/**LCD data delay configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`lcd_data_dout_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcd_data_dout_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LCD_DATA_DOUT_MODE_SPEC;
impl crate::RegisterSpec for LCD_DATA_DOUT_MODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lcd_data_dout_mode::R`](R) reader structure
impl crate::Readable for LCD_DATA_DOUT_MODE_SPEC {}
///`write(|w| ..)` method takes [`lcd_data_dout_mode::W`](W) writer structure
impl crate::Writable for LCD_DATA_DOUT_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LCD_DATA_DOUT_MODE to value 0
impl crate::Resettable for LCD_DATA_DOUT_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
