///Register `PERI_CLK_CTRL16` reader
pub type R = crate::R<PERI_CLK_CTRL16_SPEC>;
///Register `PERI_CLK_CTRL16` writer
pub type W = crate::W<PERI_CLK_CTRL16_SPEC>;
///Field `I2S1_TX_DIV_N` reader - Reserved
pub type I2S1_TX_DIV_N_R = crate::FieldReader;
///Field `I2S1_TX_DIV_N` writer - Reserved
pub type I2S1_TX_DIV_N_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `I2S1_TX_DIV_X` reader - Reserved
pub type I2S1_TX_DIV_X_R = crate::FieldReader<u16>;
///Field `I2S1_TX_DIV_X` writer - Reserved
pub type I2S1_TX_DIV_X_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `I2S1_TX_DIV_Y` reader - Reserved
pub type I2S1_TX_DIV_Y_R = crate::FieldReader<u16>;
///Field `I2S1_TX_DIV_Y` writer - Reserved
pub type I2S1_TX_DIV_Y_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:7 - Reserved
    #[inline(always)]
    pub fn i2s1_tx_div_n(&self) -> I2S1_TX_DIV_N_R {
        I2S1_TX_DIV_N_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:16 - Reserved
    #[inline(always)]
    pub fn i2s1_tx_div_x(&self) -> I2S1_TX_DIV_X_R {
        I2S1_TX_DIV_X_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    ///Bits 17:25 - Reserved
    #[inline(always)]
    pub fn i2s1_tx_div_y(&self) -> I2S1_TX_DIV_Y_R {
        I2S1_TX_DIV_Y_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL16")
            .field("i2s1_tx_div_n", &self.i2s1_tx_div_n())
            .field("i2s1_tx_div_x", &self.i2s1_tx_div_x())
            .field("i2s1_tx_div_y", &self.i2s1_tx_div_y())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn i2s1_tx_div_n(&mut self) -> I2S1_TX_DIV_N_W<PERI_CLK_CTRL16_SPEC> {
        I2S1_TX_DIV_N_W::new(self, 0)
    }
    ///Bits 8:16 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn i2s1_tx_div_x(&mut self) -> I2S1_TX_DIV_X_W<PERI_CLK_CTRL16_SPEC> {
        I2S1_TX_DIV_X_W::new(self, 8)
    }
    ///Bits 17:25 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn i2s1_tx_div_y(&mut self) -> I2S1_TX_DIV_Y_W<PERI_CLK_CTRL16_SPEC> {
        I2S1_TX_DIV_Y_W::new(self, 17)
    }
}
/**Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PERI_CLK_CTRL16_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL16_SPEC {
    type Ux = u32;
}
///`read()` method returns [`peri_clk_ctrl16::R`](R) reader structure
impl crate::Readable for PERI_CLK_CTRL16_SPEC {}
///`write(|w| ..)` method takes [`peri_clk_ctrl16::W`](W) writer structure
impl crate::Writable for PERI_CLK_CTRL16_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PERI_CLK_CTRL16 to value 0
impl crate::Resettable for PERI_CLK_CTRL16_SPEC {
    const RESET_VALUE: u32 = 0;
}
