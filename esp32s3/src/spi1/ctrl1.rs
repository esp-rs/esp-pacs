#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `CLK_MODE` reader - SPI Bus clock (SPI_CLK) mode bits. 0: SPI Bus clock (SPI_CLK) is off when CS inactive 1: SPI_CLK is delayed one cycle after SPI_CS inactive 2: SPI_CLK is delayed two cycles after SPI_CS inactive 3: SPI_CLK is always on."]
pub type CLK_MODE_R = crate::FieldReader;
#[doc = "Field `CLK_MODE` writer - SPI Bus clock (SPI_CLK) mode bits. 0: SPI Bus clock (SPI_CLK) is off when CS inactive 1: SPI_CLK is delayed one cycle after SPI_CS inactive 2: SPI_CLK is delayed two cycles after SPI_CS inactive 3: SPI_CLK is always on."]
pub type CLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CS_HOLD_DLY_RES` reader - After RES/DP/HPM/PES/PER command is sent, SPI1 may waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or * 256) SPI_CLK cycles."]
pub type CS_HOLD_DLY_RES_R = crate::FieldReader<u16>;
#[doc = "Field `CS_HOLD_DLY_RES` writer - After RES/DP/HPM/PES/PER command is sent, SPI1 may waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or * 256) SPI_CLK cycles."]
pub type CS_HOLD_DLY_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:1 - SPI Bus clock (SPI_CLK) mode bits. 0: SPI Bus clock (SPI_CLK) is off when CS inactive 1: SPI_CLK is delayed one cycle after SPI_CS inactive 2: SPI_CLK is delayed two cycles after SPI_CS inactive 3: SPI_CLK is always on."]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:11 - After RES/DP/HPM/PES/PER command is sent, SPI1 may waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or * 256) SPI_CLK cycles."]
    #[inline(always)]
    pub fn cs_hold_dly_res(&self) -> CS_HOLD_DLY_RES_R {
        CS_HOLD_DLY_RES_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("clk_mode", &self.clk_mode())
            .field("cs_hold_dly_res", &self.cs_hold_dly_res())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI Bus clock (SPI_CLK) mode bits. 0: SPI Bus clock (SPI_CLK) is off when CS inactive 1: SPI_CLK is delayed one cycle after SPI_CS inactive 2: SPI_CLK is delayed two cycles after SPI_CS inactive 3: SPI_CLK is always on."]
    #[inline(always)]
    pub fn clk_mode(&mut self) -> CLK_MODE_W<CTRL1_SPEC> {
        CLK_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:11 - After RES/DP/HPM/PES/PER command is sent, SPI1 may waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or * 256) SPI_CLK cycles."]
    #[inline(always)]
    pub fn cs_hold_dly_res(&mut self) -> CS_HOLD_DLY_RES_W<CTRL1_SPEC> {
        CS_HOLD_DLY_RES_W::new(self, 2)
    }
}
#[doc = "SPI1 control1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0x0ffc"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x0ffc;
}
