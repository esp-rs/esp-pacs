#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `FDUMMY_OUT` reader - In the DUMMY phase the signal level of SPI bus is output by the SPI0 controller."]
pub type FDUMMY_OUT_R = crate::BitReader;
#[doc = "Field `FDUMMY_OUT` writer - In the DUMMY phase the signal level of SPI bus is output by the SPI0 controller."]
pub type FDUMMY_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOUT_OCT` reader - Set this bit to enable 8-bit-mode(8-bm) in DOUT phase."]
pub type FDOUT_OCT_R = crate::BitReader;
#[doc = "Field `FDOUT_OCT` writer - Set this bit to enable 8-bit-mode(8-bm) in DOUT phase."]
pub type FDOUT_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDIN_OCT` reader - Set this bit to enable 8-bit-mode(8-bm) in DIN phase."]
pub type FDIN_OCT_R = crate::BitReader;
#[doc = "Field `FDIN_OCT` writer - Set this bit to enable 8-bit-mode(8-bm) in DIN phase."]
pub type FDIN_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FADDR_OCT` reader - Set this bit to enable 8-bit-mode(8-bm) in ADDR phase."]
pub type FADDR_OCT_R = crate::BitReader;
#[doc = "Field `FADDR_OCT` writer - Set this bit to enable 8-bit-mode(8-bm) in ADDR phase."]
pub type FADDR_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCMD_DUAL` reader - Set this bit to enable 2-bit-mode(2-bm) in CMD phase."]
pub type FCMD_DUAL_R = crate::BitReader;
#[doc = "Field `FCMD_DUAL` writer - Set this bit to enable 2-bit-mode(2-bm) in CMD phase."]
pub type FCMD_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCMD_QUAD` reader - Set this bit to enable 4-bit-mode(4-bm) in CMD phase."]
pub type FCMD_QUAD_R = crate::BitReader;
#[doc = "Field `FCMD_QUAD` writer - Set this bit to enable 4-bit-mode(4-bm) in CMD phase."]
pub type FCMD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCMD_OCT` reader - Set this bit to enable 8-bit-mode(8-bm) in CMD phase."]
pub type FCMD_OCT_R = crate::BitReader;
#[doc = "Field `FCMD_OCT` writer - Set this bit to enable 8-bit-mode(8-bm) in CMD phase."]
pub type FCMD_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTRD_MODE` reader - This bit should be set when SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QUAD or SPI_MEM_FREAD_DUAL is set."]
pub type FASTRD_MODE_R = crate::BitReader;
#[doc = "Field `FASTRD_MODE` writer - This bit should be set when SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QUAD or SPI_MEM_FREAD_DUAL is set."]
pub type FASTRD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_DUAL` reader - In hardware 0x3B read operation, DIN phase apply 2 signals. 1: enable 0: disable."]
pub type FREAD_DUAL_R = crate::BitReader;
#[doc = "Field `FREAD_DUAL` writer - In hardware 0x3B read operation, DIN phase apply 2 signals. 1: enable 0: disable."]
pub type FREAD_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Q_POL` reader - The bit is used to set MISO line polarity, 1: high 0, low"]
pub type Q_POL_R = crate::BitReader;
#[doc = "Field `Q_POL` writer - The bit is used to set MISO line polarity, 1: high 0, low"]
pub type Q_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_POL` reader - The bit is used to set MOSI line polarity, 1: high 0, low"]
pub type D_POL_R = crate::BitReader;
#[doc = "Field `D_POL` writer - The bit is used to set MOSI line polarity, 1: high 0, low"]
pub type D_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_QUAD` reader - In hardware 0x6B read operation, DIN phase apply 4 signals(4-bit-mode). 1: enable 0: disable."]
pub type FREAD_QUAD_R = crate::BitReader;
#[doc = "Field `FREAD_QUAD` writer - In hardware 0x6B read operation, DIN phase apply 4 signals(4-bit-mode). 1: enable 0: disable."]
pub type FREAD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP` reader - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
pub type WP_R = crate::BitReader;
#[doc = "Field `WP` writer - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
pub type WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_DIO` reader - In hardware 0xBB read operation, ADDR phase and DIN phase apply 2 signals(2-bit-mode). 1: enable 0: disable."]
pub type FREAD_DIO_R = crate::BitReader;
#[doc = "Field `FREAD_DIO` writer - In hardware 0xBB read operation, ADDR phase and DIN phase apply 2 signals(2-bit-mode). 1: enable 0: disable."]
pub type FREAD_DIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_QIO` reader - In hardware 0xEB read operation, ADDR phase and DIN phase apply 4 signals(4-bit-mode). 1: enable 0: disable."]
pub type FREAD_QIO_R = crate::BitReader;
#[doc = "Field `FREAD_QIO` writer - In hardware 0xEB read operation, ADDR phase and DIN phase apply 4 signals(4-bit-mode). 1: enable 0: disable."]
pub type FREAD_QIO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - In the DUMMY phase the signal level of SPI bus is output by the SPI0 controller."]
    #[inline(always)]
    pub fn fdummy_out(&self) -> FDUMMY_OUT_R {
        FDUMMY_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable 8-bit-mode(8-bm) in DOUT phase."]
    #[inline(always)]
    pub fn fdout_oct(&self) -> FDOUT_OCT_R {
        FDOUT_OCT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable 8-bit-mode(8-bm) in DIN phase."]
    #[inline(always)]
    pub fn fdin_oct(&self) -> FDIN_OCT_R {
        FDIN_OCT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable 8-bit-mode(8-bm) in ADDR phase."]
    #[inline(always)]
    pub fn faddr_oct(&self) -> FADDR_OCT_R {
        FADDR_OCT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable 2-bit-mode(2-bm) in CMD phase."]
    #[inline(always)]
    pub fn fcmd_dual(&self) -> FCMD_DUAL_R {
        FCMD_DUAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to enable 4-bit-mode(4-bm) in CMD phase."]
    #[inline(always)]
    pub fn fcmd_quad(&self) -> FCMD_QUAD_R {
        FCMD_QUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to enable 8-bit-mode(8-bm) in CMD phase."]
    #[inline(always)]
    pub fn fcmd_oct(&self) -> FCMD_OCT_R {
        FCMD_OCT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit should be set when SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QUAD or SPI_MEM_FREAD_DUAL is set."]
    #[inline(always)]
    pub fn fastrd_mode(&self) -> FASTRD_MODE_R {
        FASTRD_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In hardware 0x3B read operation, DIN phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_dual(&self) -> FREAD_DUAL_R {
        FREAD_DUAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn q_pol(&self) -> Q_POL_R {
        Q_POL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn d_pol(&self) -> D_POL_R {
        D_POL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - In hardware 0x6B read operation, DIN phase apply 4 signals(4-bit-mode). 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_quad(&self) -> FREAD_QUAD_R {
        FREAD_QUAD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - In hardware 0xBB read operation, ADDR phase and DIN phase apply 2 signals(2-bit-mode). 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_dio(&self) -> FREAD_DIO_R {
        FREAD_DIO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - In hardware 0xEB read operation, ADDR phase and DIN phase apply 4 signals(4-bit-mode). 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_qio(&self) -> FREAD_QIO_R {
        FREAD_QIO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("fdummy_out", &self.fdummy_out())
            .field("fdout_oct", &self.fdout_oct())
            .field("fdin_oct", &self.fdin_oct())
            .field("faddr_oct", &self.faddr_oct())
            .field("fcmd_dual", &self.fcmd_dual())
            .field("fcmd_quad", &self.fcmd_quad())
            .field("fcmd_oct", &self.fcmd_oct())
            .field("fastrd_mode", &self.fastrd_mode())
            .field("fread_dual", &self.fread_dual())
            .field("q_pol", &self.q_pol())
            .field("d_pol", &self.d_pol())
            .field("fread_quad", &self.fread_quad())
            .field("wp", &self.wp())
            .field("fread_dio", &self.fread_dio())
            .field("fread_qio", &self.fread_qio())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - In the DUMMY phase the signal level of SPI bus is output by the SPI0 controller."]
    #[inline(always)]
    pub fn fdummy_out(&mut self) -> FDUMMY_OUT_W<CTRL_SPEC> {
        FDUMMY_OUT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable 8-bit-mode(8-bm) in DOUT phase."]
    #[inline(always)]
    pub fn fdout_oct(&mut self) -> FDOUT_OCT_W<CTRL_SPEC> {
        FDOUT_OCT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to enable 8-bit-mode(8-bm) in DIN phase."]
    #[inline(always)]
    pub fn fdin_oct(&mut self) -> FDIN_OCT_W<CTRL_SPEC> {
        FDIN_OCT_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to enable 8-bit-mode(8-bm) in ADDR phase."]
    #[inline(always)]
    pub fn faddr_oct(&mut self) -> FADDR_OCT_W<CTRL_SPEC> {
        FADDR_OCT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to enable 2-bit-mode(2-bm) in CMD phase."]
    #[inline(always)]
    pub fn fcmd_dual(&mut self) -> FCMD_DUAL_W<CTRL_SPEC> {
        FCMD_DUAL_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to enable 4-bit-mode(4-bm) in CMD phase."]
    #[inline(always)]
    pub fn fcmd_quad(&mut self) -> FCMD_QUAD_W<CTRL_SPEC> {
        FCMD_QUAD_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to enable 8-bit-mode(8-bm) in CMD phase."]
    #[inline(always)]
    pub fn fcmd_oct(&mut self) -> FCMD_OCT_W<CTRL_SPEC> {
        FCMD_OCT_W::new(self, 9)
    }
    #[doc = "Bit 13 - This bit should be set when SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QUAD or SPI_MEM_FREAD_DUAL is set."]
    #[inline(always)]
    pub fn fastrd_mode(&mut self) -> FASTRD_MODE_W<CTRL_SPEC> {
        FASTRD_MODE_W::new(self, 13)
    }
    #[doc = "Bit 14 - In hardware 0x3B read operation, DIN phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_dual(&mut self) -> FREAD_DUAL_W<CTRL_SPEC> {
        FREAD_DUAL_W::new(self, 14)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn q_pol(&mut self) -> Q_POL_W<CTRL_SPEC> {
        Q_POL_W::new(self, 18)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn d_pol(&mut self) -> D_POL_W<CTRL_SPEC> {
        D_POL_W::new(self, 19)
    }
    #[doc = "Bit 20 - In hardware 0x6B read operation, DIN phase apply 4 signals(4-bit-mode). 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_quad(&mut self) -> FREAD_QUAD_W<CTRL_SPEC> {
        FREAD_QUAD_W::new(self, 20)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W<CTRL_SPEC> {
        WP_W::new(self, 21)
    }
    #[doc = "Bit 23 - In hardware 0xBB read operation, ADDR phase and DIN phase apply 2 signals(2-bit-mode). 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_dio(&mut self) -> FREAD_DIO_W<CTRL_SPEC> {
        FREAD_DIO_W::new(self, 23)
    }
    #[doc = "Bit 24 - In hardware 0xEB read operation, ADDR phase and DIN phase apply 4 signals(4-bit-mode). 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_qio(&mut self) -> FREAD_QIO_W<CTRL_SPEC> {
        FREAD_QIO_W::new(self, 24)
    }
}
#[doc = "SPI0 control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x002c_2000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x002c_2000;
}
