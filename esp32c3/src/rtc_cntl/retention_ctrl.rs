///Register `RETENTION_CTRL` reader
pub type R = crate::R<RETENTION_CTRL_SPEC>;
///Register `RETENTION_CTRL` writer
pub type W = crate::W<RETENTION_CTRL_SPEC>;
///Field `RETENTION_CLK_SEL` reader - Retention clk sel
pub type RETENTION_CLK_SEL_R = crate::BitReader;
///Field `RETENTION_CLK_SEL` writer - Retention clk sel
pub type RETENTION_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RETENTION_DONE_WAIT` reader - Retention done wait time
pub type RETENTION_DONE_WAIT_R = crate::FieldReader;
///Field `RETENTION_DONE_WAIT` writer - Retention done wait time
pub type RETENTION_DONE_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RETENTION_CLKOFF_WAIT` reader - Retention clkoff wait time
pub type RETENTION_CLKOFF_WAIT_R = crate::FieldReader;
///Field `RETENTION_CLKOFF_WAIT` writer - Retention clkoff wait time
pub type RETENTION_CLKOFF_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RETENTION_EN` reader - enable cpu retention when light sleep
pub type RETENTION_EN_R = crate::BitReader;
///Field `RETENTION_EN` writer - enable cpu retention when light sleep
pub type RETENTION_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RETENTION_WAIT` reader - wait cycles for rention operation
pub type RETENTION_WAIT_R = crate::FieldReader;
///Field `RETENTION_WAIT` writer - wait cycles for rention operation
pub type RETENTION_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bit 18 - Retention clk sel
    #[inline(always)]
    pub fn retention_clk_sel(&self) -> RETENTION_CLK_SEL_R {
        RETENTION_CLK_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:21 - Retention done wait time
    #[inline(always)]
    pub fn retention_done_wait(&self) -> RETENTION_DONE_WAIT_R {
        RETENTION_DONE_WAIT_R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bits 22:25 - Retention clkoff wait time
    #[inline(always)]
    pub fn retention_clkoff_wait(&self) -> RETENTION_CLKOFF_WAIT_R {
        RETENTION_CLKOFF_WAIT_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    ///Bit 26 - enable cpu retention when light sleep
    #[inline(always)]
    pub fn retention_en(&self) -> RETENTION_EN_R {
        RETENTION_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:31 - wait cycles for rention operation
    #[inline(always)]
    pub fn retention_wait(&self) -> RETENTION_WAIT_R {
        RETENTION_WAIT_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_CTRL")
            .field("retention_clk_sel", &self.retention_clk_sel())
            .field("retention_done_wait", &self.retention_done_wait())
            .field("retention_clkoff_wait", &self.retention_clkoff_wait())
            .field("retention_en", &self.retention_en())
            .field("retention_wait", &self.retention_wait())
            .finish()
    }
}
impl W {
    ///Bit 18 - Retention clk sel
    #[inline(always)]
    #[must_use]
    pub fn retention_clk_sel(&mut self) -> RETENTION_CLK_SEL_W<RETENTION_CTRL_SPEC> {
        RETENTION_CLK_SEL_W::new(self, 18)
    }
    ///Bits 19:21 - Retention done wait time
    #[inline(always)]
    #[must_use]
    pub fn retention_done_wait(&mut self) -> RETENTION_DONE_WAIT_W<RETENTION_CTRL_SPEC> {
        RETENTION_DONE_WAIT_W::new(self, 19)
    }
    ///Bits 22:25 - Retention clkoff wait time
    #[inline(always)]
    #[must_use]
    pub fn retention_clkoff_wait(&mut self) -> RETENTION_CLKOFF_WAIT_W<RETENTION_CTRL_SPEC> {
        RETENTION_CLKOFF_WAIT_W::new(self, 22)
    }
    ///Bit 26 - enable cpu retention when light sleep
    #[inline(always)]
    #[must_use]
    pub fn retention_en(&mut self) -> RETENTION_EN_W<RETENTION_CTRL_SPEC> {
        RETENTION_EN_W::new(self, 26)
    }
    ///Bits 27:31 - wait cycles for rention operation
    #[inline(always)]
    #[must_use]
    pub fn retention_wait(&mut self) -> RETENTION_WAIT_W<RETENTION_CTRL_SPEC> {
        RETENTION_WAIT_W::new(self, 27)
    }
}
/**rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`retention_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RETENTION_CTRL_SPEC;
impl crate::RegisterSpec for RETENTION_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`retention_ctrl::R`](R) reader structure
impl crate::Readable for RETENTION_CTRL_SPEC {}
///`write(|w| ..)` method takes [`retention_ctrl::W`](W) writer structure
impl crate::Writable for RETENTION_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RETENTION_CTRL to value 0xa0d0_0000
impl crate::Resettable for RETENTION_CTRL_SPEC {
    const RESET_VALUE: u32 = 0xa0d0_0000;
}
