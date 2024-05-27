///Register `CTRL_DATE` reader
pub type R = crate::R<CTRL_DATE_SPEC>;
///Register `CTRL_DATE` writer
pub type W = crate::W<CTRL_DATE_SPEC>;
///Field `CTRL_DATE` reader - need_des
pub type CTRL_DATE_R = crate::FieldReader<u32>;
///Field `CTRL_DATE` writer - need_des
pub type CTRL_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `CLK_EN` reader - need_des
pub type CLK_EN_R = crate::BitReader;
///Field `CLK_EN` writer - need_des
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:30 - need_des
    #[inline(always)]
    pub fn ctrl_date(&self) -> CTRL_DATE_R {
        CTRL_DATE_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_DATE")
            .field("ctrl_date", &self.ctrl_date())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    ///Bits 0:30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn ctrl_date(&mut self) -> CTRL_DATE_W<CTRL_DATE_SPEC> {
        CTRL_DATE_W::new(self, 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CTRL_DATE_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
/**Register

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTRL_DATE_SPEC;
impl crate::RegisterSpec for CTRL_DATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ctrl_date::R`](R) reader structure
impl crate::Readable for CTRL_DATE_SPEC {}
///`write(|w| ..)` method takes [`ctrl_date::W`](W) writer structure
impl crate::Writable for CTRL_DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTRL_DATE to value 0x0221_2260
impl crate::Resettable for CTRL_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0221_2260;
}
