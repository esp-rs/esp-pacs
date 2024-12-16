#[doc = "Register `MISC` reader"]
pub type R = crate::R<MISC_SPEC>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MISC_SPEC>;
#[doc = "Field `CS_DIS(0-1)` reader - Set this bit to raise high SPI_CS%s pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS%s is in low level when SPI1 transfer starts"]
pub type CS_DIS_R = crate::BitReader;
#[doc = "Field `CS_DIS(0-1)` writer - Set this bit to raise high SPI_CS%s pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS%s is in low level when SPI1 transfer starts"]
pub type CS_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSUB_PIN` reader - Flash is connected to SPI SUBPIN bus."]
pub type FSUB_PIN_R = crate::BitReader;
#[doc = "Field `FSUB_PIN` writer - Flash is connected to SPI SUBPIN bus."]
pub type FSUB_PIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSUB_PIN` reader - Ext_RAM is connected to SPI SUBPIN bus."]
pub type SSUB_PIN_R = crate::BitReader;
#[doc = "Field `SSUB_PIN` writer - Ext_RAM is connected to SPI SUBPIN bus."]
pub type SSUB_PIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_IDLE_EDGE` reader - 1: SPI_CLK line is high when idle. 0: SPI_CLK line is low when idle"]
pub type CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` writer - 1: SPI_CLK line is high when idle. 0: SPI_CLK line is low when idle"]
pub type CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - SPI_CS line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `CS_KEEP_ACTIVE` writer - SPI_CS line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Set this bit to raise high SPI_CS(0-1) pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS(0-1) is in low level when SPI1 transfer starts"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CS0_DIS` field.</div>"]
    #[inline(always)]
    pub fn cs_dis(&self, n: u8) -> CS_DIS_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CS_DIS_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Set this bit to raise high SPI_CS(0-1) pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS(0-1) is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs_dis_iter(&self) -> impl Iterator<Item = CS_DIS_R> + '_ {
        (0..2).map(move |n| CS_DIS_R::new(((self.bits >> n) & 1) != 0))
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
    #[doc = "Bit 7 - Flash is connected to SPI SUBPIN bus."]
    #[inline(always)]
    pub fn fsub_pin(&self) -> FSUB_PIN_R {
        FSUB_PIN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Ext_RAM is connected to SPI SUBPIN bus."]
    #[inline(always)]
    pub fn ssub_pin(&self) -> SSUB_PIN_R {
        SSUB_PIN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: SPI_CLK line is high when idle. 0: SPI_CLK line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC")
            .field("fsub_pin", &self.fsub_pin())
            .field("ssub_pin", &self.ssub_pin())
            .field("ck_idle_edge", &self.ck_idle_edge())
            .field("cs_keep_active", &self.cs_keep_active())
            .field("cs0_dis", &self.cs0_dis())
            .field("cs1_dis", &self.cs1_dis())
            .finish()
    }
}
impl W {
    #[doc = "Set this bit to raise high SPI_CS(0-1) pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS(0-1) is in low level when SPI1 transfer starts"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CS0_DIS` field.</div>"]
    #[inline(always)]
    pub fn cs_dis(&mut self, n: u8) -> CS_DIS_W<MISC_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CS_DIS_W::new(self, n)
    }
    #[doc = "Bit 0 - Set this bit to raise high SPI_CS0 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS0 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs0_dis(&mut self) -> CS_DIS_W<MISC_SPEC> {
        CS_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to raise high SPI_CS1 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS1 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs1_dis(&mut self) -> CS_DIS_W<MISC_SPEC> {
        CS_DIS_W::new(self, 1)
    }
    #[doc = "Bit 7 - Flash is connected to SPI SUBPIN bus."]
    #[inline(always)]
    pub fn fsub_pin(&mut self) -> FSUB_PIN_W<MISC_SPEC> {
        FSUB_PIN_W::new(self, 7)
    }
    #[doc = "Bit 8 - Ext_RAM is connected to SPI SUBPIN bus."]
    #[inline(always)]
    pub fn ssub_pin(&mut self) -> SSUB_PIN_W<MISC_SPEC> {
        SSUB_PIN_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1: SPI_CLK line is high when idle. 0: SPI_CLK line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<MISC_SPEC> {
        CK_IDLE_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 10 - SPI_CS line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<MISC_SPEC> {
        CS_KEEP_ACTIVE_W::new(self, 10)
    }
}
#[doc = "SPI0 misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MISC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC to value 0"]
impl crate::Resettable for MISC_SPEC {
    const RESET_VALUE: u32 = 0;
}
