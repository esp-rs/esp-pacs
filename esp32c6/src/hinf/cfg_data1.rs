///Register `CFG_DATA1` reader
pub type R = crate::R<CFG_DATA1_SPEC>;
///Register `CFG_DATA1` writer
pub type W = crate::W<CFG_DATA1_SPEC>;
///Field `SDIO_ENABLE` reader - Sdio clock enable
pub type SDIO_ENABLE_R = crate::BitReader;
///Field `SDIO_ENABLE` writer - Sdio clock enable
pub type SDIO_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIO_IOREADY1` reader - sdio function1 io ready signal in cis
pub type SDIO_IOREADY1_R = crate::BitReader;
///Field `SDIO_IOREADY1` writer - sdio function1 io ready signal in cis
pub type SDIO_IOREADY1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HIGHSPEED_ENABLE` reader - Highspeed enable in cccr
pub type HIGHSPEED_ENABLE_R = crate::BitReader;
///Field `HIGHSPEED_ENABLE` writer - Highspeed enable in cccr
pub type HIGHSPEED_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HIGHSPEED_MODE` reader - highspeed mode status in cccr
pub type HIGHSPEED_MODE_R = crate::BitReader;
///Field `SDIO_CD_ENABLE` reader - sdio card detect enable
pub type SDIO_CD_ENABLE_R = crate::BitReader;
///Field `SDIO_CD_ENABLE` writer - sdio card detect enable
pub type SDIO_CD_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIO_IOREADY2` reader - sdio function1 io ready signal in cis
pub type SDIO_IOREADY2_R = crate::BitReader;
///Field `SDIO_IOREADY2` writer - sdio function1 io ready signal in cis
pub type SDIO_IOREADY2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIO_INT_MASK` reader - mask sdio interrupt in cccr, high active
pub type SDIO_INT_MASK_R = crate::BitReader;
///Field `SDIO_INT_MASK` writer - mask sdio interrupt in cccr, high active
pub type SDIO_INT_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IOENABLE2` reader - ioe2 status in cccr
pub type IOENABLE2_R = crate::BitReader;
///Field `CD_DISABLE` reader - card disable status in cccr
pub type CD_DISABLE_R = crate::BitReader;
///Field `FUNC1_EPS` reader - function1 eps status in fbr
pub type FUNC1_EPS_R = crate::BitReader;
///Field `EMP` reader - empc status in cccr
pub type EMP_R = crate::BitReader;
///Field `IOENABLE1` reader - ioe1 status in cccr
pub type IOENABLE1_R = crate::BitReader;
///Field `SDIO_VER` reader - sdio version in cccr
pub type SDIO_VER_R = crate::FieldReader<u16>;
///Field `SDIO_VER` writer - sdio version in cccr
pub type SDIO_VER_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `FUNC2_EPS` reader - function2 eps status in fbr
pub type FUNC2_EPS_R = crate::BitReader;
///Field `SDIO20_CONF` reader - 29\],sdio negedge sample enablel.\[30\],sdio posedge sample enable.\[31\],sdio cmd/dat in delayed cycles control,0:no delay, 1:delay 1 cycle. \[25\]: sdio1.1 dat/cmd sending out edge control,1:negedge,0:posedge when highseed mode. \[26\]: sdio2.0 dat/cmd sending out edge control,1:negedge when \[12\]=0,0:negedge when \[12\]=0,posedge when highspeed mode enable. \[27\]: sdio interrupt sending out delay control,1:delay one cycle, 0: no delay. \[28\]: sdio data pad pull up enable
pub type SDIO20_CONF_R = crate::FieldReader;
///Field `SDIO20_CONF` writer - 29\],sdio negedge sample enablel.\[30\],sdio posedge sample enable.\[31\],sdio cmd/dat in delayed cycles control,0:no delay, 1:delay 1 cycle. \[25\]: sdio1.1 dat/cmd sending out edge control,1:negedge,0:posedge when highseed mode. \[26\]: sdio2.0 dat/cmd sending out edge control,1:negedge when \[12\]=0,0:negedge when \[12\]=0,posedge when highspeed mode enable. \[27\]: sdio interrupt sending out delay control,1:delay one cycle, 0: no delay. \[28\]: sdio data pad pull up enable
pub type SDIO20_CONF_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bit 0 - Sdio clock enable
    #[inline(always)]
    pub fn sdio_enable(&self) -> SDIO_ENABLE_R {
        SDIO_ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - sdio function1 io ready signal in cis
    #[inline(always)]
    pub fn sdio_ioready1(&self) -> SDIO_IOREADY1_R {
        SDIO_IOREADY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Highspeed enable in cccr
    #[inline(always)]
    pub fn highspeed_enable(&self) -> HIGHSPEED_ENABLE_R {
        HIGHSPEED_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - highspeed mode status in cccr
    #[inline(always)]
    pub fn highspeed_mode(&self) -> HIGHSPEED_MODE_R {
        HIGHSPEED_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - sdio card detect enable
    #[inline(always)]
    pub fn sdio_cd_enable(&self) -> SDIO_CD_ENABLE_R {
        SDIO_CD_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - sdio function1 io ready signal in cis
    #[inline(always)]
    pub fn sdio_ioready2(&self) -> SDIO_IOREADY2_R {
        SDIO_IOREADY2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - mask sdio interrupt in cccr, high active
    #[inline(always)]
    pub fn sdio_int_mask(&self) -> SDIO_INT_MASK_R {
        SDIO_INT_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ioe2 status in cccr
    #[inline(always)]
    pub fn ioenable2(&self) -> IOENABLE2_R {
        IOENABLE2_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - card disable status in cccr
    #[inline(always)]
    pub fn cd_disable(&self) -> CD_DISABLE_R {
        CD_DISABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - function1 eps status in fbr
    #[inline(always)]
    pub fn func1_eps(&self) -> FUNC1_EPS_R {
        FUNC1_EPS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - empc status in cccr
    #[inline(always)]
    pub fn emp(&self) -> EMP_R {
        EMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ioe1 status in cccr
    #[inline(always)]
    pub fn ioenable1(&self) -> IOENABLE1_R {
        IOENABLE1_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:23 - sdio version in cccr
    #[inline(always)]
    pub fn sdio_ver(&self) -> SDIO_VER_R {
        SDIO_VER_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    ///Bit 24 - function2 eps status in fbr
    #[inline(always)]
    pub fn func2_eps(&self) -> FUNC2_EPS_R {
        FUNC2_EPS_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:31 - 29\],sdio negedge sample enablel.\[30\],sdio posedge sample enable.\[31\],sdio cmd/dat in delayed cycles control,0:no delay, 1:delay 1 cycle. \[25\]: sdio1.1 dat/cmd sending out edge control,1:negedge,0:posedge when highseed mode. \[26\]: sdio2.0 dat/cmd sending out edge control,1:negedge when \[12\]=0,0:negedge when \[12\]=0,posedge when highspeed mode enable. \[27\]: sdio interrupt sending out delay control,1:delay one cycle, 0: no delay. \[28\]: sdio data pad pull up enable
    #[inline(always)]
    pub fn sdio20_conf(&self) -> SDIO20_CONF_R {
        SDIO20_CONF_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG_DATA1")
            .field("sdio_enable", &self.sdio_enable())
            .field("sdio_ioready1", &self.sdio_ioready1())
            .field("highspeed_enable", &self.highspeed_enable())
            .field("highspeed_mode", &self.highspeed_mode())
            .field("sdio_cd_enable", &self.sdio_cd_enable())
            .field("sdio_ioready2", &self.sdio_ioready2())
            .field("sdio_int_mask", &self.sdio_int_mask())
            .field("ioenable2", &self.ioenable2())
            .field("cd_disable", &self.cd_disable())
            .field("func1_eps", &self.func1_eps())
            .field("emp", &self.emp())
            .field("ioenable1", &self.ioenable1())
            .field("sdio_ver", &self.sdio_ver())
            .field("func2_eps", &self.func2_eps())
            .field("sdio20_conf", &self.sdio20_conf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Sdio clock enable
    #[inline(always)]
    #[must_use]
    pub fn sdio_enable(&mut self) -> SDIO_ENABLE_W<CFG_DATA1_SPEC> {
        SDIO_ENABLE_W::new(self, 0)
    }
    ///Bit 1 - sdio function1 io ready signal in cis
    #[inline(always)]
    #[must_use]
    pub fn sdio_ioready1(&mut self) -> SDIO_IOREADY1_W<CFG_DATA1_SPEC> {
        SDIO_IOREADY1_W::new(self, 1)
    }
    ///Bit 2 - Highspeed enable in cccr
    #[inline(always)]
    #[must_use]
    pub fn highspeed_enable(&mut self) -> HIGHSPEED_ENABLE_W<CFG_DATA1_SPEC> {
        HIGHSPEED_ENABLE_W::new(self, 2)
    }
    ///Bit 4 - sdio card detect enable
    #[inline(always)]
    #[must_use]
    pub fn sdio_cd_enable(&mut self) -> SDIO_CD_ENABLE_W<CFG_DATA1_SPEC> {
        SDIO_CD_ENABLE_W::new(self, 4)
    }
    ///Bit 5 - sdio function1 io ready signal in cis
    #[inline(always)]
    #[must_use]
    pub fn sdio_ioready2(&mut self) -> SDIO_IOREADY2_W<CFG_DATA1_SPEC> {
        SDIO_IOREADY2_W::new(self, 5)
    }
    ///Bit 6 - mask sdio interrupt in cccr, high active
    #[inline(always)]
    #[must_use]
    pub fn sdio_int_mask(&mut self) -> SDIO_INT_MASK_W<CFG_DATA1_SPEC> {
        SDIO_INT_MASK_W::new(self, 6)
    }
    ///Bits 12:23 - sdio version in cccr
    #[inline(always)]
    #[must_use]
    pub fn sdio_ver(&mut self) -> SDIO_VER_W<CFG_DATA1_SPEC> {
        SDIO_VER_W::new(self, 12)
    }
    ///Bits 25:31 - 29\],sdio negedge sample enablel.\[30\],sdio posedge sample enable.\[31\],sdio cmd/dat in delayed cycles control,0:no delay, 1:delay 1 cycle. \[25\]: sdio1.1 dat/cmd sending out edge control,1:negedge,0:posedge when highseed mode. \[26\]: sdio2.0 dat/cmd sending out edge control,1:negedge when \[12\]=0,0:negedge when \[12\]=0,posedge when highspeed mode enable. \[27\]: sdio interrupt sending out delay control,1:delay one cycle, 0: no delay. \[28\]: sdio data pad pull up enable
    #[inline(always)]
    #[must_use]
    pub fn sdio20_conf(&mut self) -> SDIO20_CONF_W<CFG_DATA1_SPEC> {
        SDIO20_CONF_W::new(self, 25)
    }
}
/**SDIO configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cfg_data1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_data1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CFG_DATA1_SPEC;
impl crate::RegisterSpec for CFG_DATA1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cfg_data1::R`](R) reader structure
impl crate::Readable for CFG_DATA1_SPEC {}
///`write(|w| ..)` method takes [`cfg_data1::W`](W) writer structure
impl crate::Writable for CFG_DATA1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFG_DATA1 to value 0x0023_2011
impl crate::Resettable for CFG_DATA1_SPEC {
    const RESET_VALUE: u32 = 0x0023_2011;
}
