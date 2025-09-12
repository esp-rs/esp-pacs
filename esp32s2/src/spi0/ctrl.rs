#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `FDUMMY_OUT` reader - "]
pub type FDUMMY_OUT_R = crate::BitReader;
#[doc = "Field `FDUMMY_OUT` writer - "]
pub type FDUMMY_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOUT_OCT` reader - "]
pub type FDOUT_OCT_R = crate::BitReader;
#[doc = "Field `FDOUT_OCT` writer - "]
pub type FDOUT_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDIN_OCT` reader - "]
pub type FDIN_OCT_R = crate::BitReader;
#[doc = "Field `FDIN_OCT` writer - "]
pub type FDIN_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FADDR_OCT` reader - "]
pub type FADDR_OCT_R = crate::BitReader;
#[doc = "Field `FADDR_OCT` writer - "]
pub type FADDR_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCMD_DUAL` reader - "]
pub type FCMD_DUAL_R = crate::BitReader;
#[doc = "Field `FCMD_DUAL` writer - "]
pub type FCMD_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCMD_QUAD` reader - "]
pub type FCMD_QUAD_R = crate::BitReader;
#[doc = "Field `FCMD_QUAD` writer - "]
pub type FCMD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCMD_OCT` reader - "]
pub type FCMD_OCT_R = crate::BitReader;
#[doc = "Field `FCMD_OCT` writer - "]
pub type FCMD_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCS_CRC_EN` reader - "]
pub type FCS_CRC_EN_R = crate::BitReader;
#[doc = "Field `FCS_CRC_EN` writer - "]
pub type FCS_CRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CRC_EN` reader - "]
pub type TX_CRC_EN_R = crate::BitReader;
#[doc = "Field `TX_CRC_EN` writer - "]
pub type TX_CRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTRD_MODE` reader - "]
pub type FASTRD_MODE_R = crate::BitReader;
#[doc = "Field `FASTRD_MODE` writer - "]
pub type FASTRD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_DUAL` reader - "]
pub type FREAD_DUAL_R = crate::BitReader;
#[doc = "Field `FREAD_DUAL` writer - "]
pub type FREAD_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESANDRES` reader - "]
pub type RESANDRES_R = crate::BitReader;
#[doc = "Field `RESANDRES` writer - "]
pub type RESANDRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Q_POL` reader - "]
pub type Q_POL_R = crate::BitReader;
#[doc = "Field `Q_POL` writer - "]
pub type Q_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_POL` reader - "]
pub type D_POL_R = crate::BitReader;
#[doc = "Field `D_POL` writer - "]
pub type D_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_QUAD` reader - "]
pub type FREAD_QUAD_R = crate::BitReader;
#[doc = "Field `FREAD_QUAD` writer - "]
pub type FREAD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP_REG` reader - "]
pub type WP_REG_R = crate::BitReader;
#[doc = "Field `WP_REG` writer - "]
pub type WP_REG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRSR_2B` reader - "]
pub type WRSR_2B_R = crate::BitReader;
#[doc = "Field `WRSR_2B` writer - "]
pub type WRSR_2B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_DIO` reader - "]
pub type FREAD_DIO_R = crate::BitReader;
#[doc = "Field `FREAD_DIO` writer - "]
pub type FREAD_DIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_QIO` reader - "]
pub type FREAD_QIO_R = crate::BitReader;
#[doc = "Field `FREAD_QIO` writer - "]
pub type FREAD_QIO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fdummy_out(&self) -> FDUMMY_OUT_R {
        FDUMMY_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fdout_oct(&self) -> FDOUT_OCT_R {
        FDOUT_OCT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fdin_oct(&self) -> FDIN_OCT_R {
        FDIN_OCT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn faddr_oct(&self) -> FADDR_OCT_R {
        FADDR_OCT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fcmd_dual(&self) -> FCMD_DUAL_R {
        FCMD_DUAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fcmd_quad(&self) -> FCMD_QUAD_R {
        FCMD_QUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fcmd_oct(&self) -> FCMD_OCT_R {
        FCMD_OCT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fcs_crc_en(&self) -> FCS_CRC_EN_R {
        FCS_CRC_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tx_crc_en(&self) -> TX_CRC_EN_R {
        TX_CRC_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fastrd_mode(&self) -> FASTRD_MODE_R {
        FASTRD_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fread_dual(&self) -> FREAD_DUAL_R {
        FREAD_DUAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn resandres(&self) -> RESANDRES_R {
        RESANDRES_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn q_pol(&self) -> Q_POL_R {
        Q_POL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn d_pol(&self) -> D_POL_R {
        D_POL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fread_quad(&self) -> FREAD_QUAD_R {
        FREAD_QUAD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wp_reg(&self) -> WP_REG_R {
        WP_REG_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wrsr_2b(&self) -> WRSR_2B_R {
        WRSR_2B_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn fread_dio(&self) -> FREAD_DIO_R {
        FREAD_DIO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn fread_qio(&self) -> FREAD_QIO_R {
        FREAD_QIO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("fread_qio", &self.fread_qio())
            .field("fread_dio", &self.fread_dio())
            .field("wrsr_2b", &self.wrsr_2b())
            .field("wp_reg", &self.wp_reg())
            .field("fread_quad", &self.fread_quad())
            .field("d_pol", &self.d_pol())
            .field("q_pol", &self.q_pol())
            .field("resandres", &self.resandres())
            .field("fread_dual", &self.fread_dual())
            .field("fastrd_mode", &self.fastrd_mode())
            .field("tx_crc_en", &self.tx_crc_en())
            .field("fcs_crc_en", &self.fcs_crc_en())
            .field("fcmd_oct", &self.fcmd_oct())
            .field("fcmd_quad", &self.fcmd_quad())
            .field("fcmd_dual", &self.fcmd_dual())
            .field("faddr_oct", &self.faddr_oct())
            .field("fdin_oct", &self.fdin_oct())
            .field("fdout_oct", &self.fdout_oct())
            .field("fdummy_out", &self.fdummy_out())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fdummy_out(&mut self) -> FDUMMY_OUT_W<'_, CTRL_SPEC> {
        FDUMMY_OUT_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fdout_oct(&mut self) -> FDOUT_OCT_W<'_, CTRL_SPEC> {
        FDOUT_OCT_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fdin_oct(&mut self) -> FDIN_OCT_W<'_, CTRL_SPEC> {
        FDIN_OCT_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn faddr_oct(&mut self) -> FADDR_OCT_W<'_, CTRL_SPEC> {
        FADDR_OCT_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fcmd_dual(&mut self) -> FCMD_DUAL_W<'_, CTRL_SPEC> {
        FCMD_DUAL_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fcmd_quad(&mut self) -> FCMD_QUAD_W<'_, CTRL_SPEC> {
        FCMD_QUAD_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fcmd_oct(&mut self) -> FCMD_OCT_W<'_, CTRL_SPEC> {
        FCMD_OCT_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fcs_crc_en(&mut self) -> FCS_CRC_EN_W<'_, CTRL_SPEC> {
        FCS_CRC_EN_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tx_crc_en(&mut self) -> TX_CRC_EN_W<'_, CTRL_SPEC> {
        TX_CRC_EN_W::new(self, 11)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fastrd_mode(&mut self) -> FASTRD_MODE_W<'_, CTRL_SPEC> {
        FASTRD_MODE_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fread_dual(&mut self) -> FREAD_DUAL_W<'_, CTRL_SPEC> {
        FREAD_DUAL_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn resandres(&mut self) -> RESANDRES_W<'_, CTRL_SPEC> {
        RESANDRES_W::new(self, 15)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn q_pol(&mut self) -> Q_POL_W<'_, CTRL_SPEC> {
        Q_POL_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn d_pol(&mut self) -> D_POL_W<'_, CTRL_SPEC> {
        D_POL_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fread_quad(&mut self) -> FREAD_QUAD_W<'_, CTRL_SPEC> {
        FREAD_QUAD_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wp_reg(&mut self) -> WP_REG_W<'_, CTRL_SPEC> {
        WP_REG_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wrsr_2b(&mut self) -> WRSR_2B_W<'_, CTRL_SPEC> {
        WRSR_2B_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn fread_dio(&mut self) -> FREAD_DIO_W<'_, CTRL_SPEC> {
        FREAD_DIO_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn fread_qio(&mut self) -> FREAD_QIO_W<'_, CTRL_SPEC> {
        FREAD_QIO_W::new(self, 24)
    }
}
#[doc = "SPI Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {}
