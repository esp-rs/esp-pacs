///Register `SAR_TSENS_CTRL2` reader
pub type R = crate::R<SAR_TSENS_CTRL2_SPEC>;
///Register `SAR_TSENS_CTRL2` writer
pub type W = crate::W<SAR_TSENS_CTRL2_SPEC>;
///Field `SAR_TSENS_XPD_WAIT` reader - no public
pub type SAR_TSENS_XPD_WAIT_R = crate::FieldReader<u16>;
///Field `SAR_TSENS_XPD_WAIT` writer - no public
pub type SAR_TSENS_XPD_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `SAR_TSENS_XPD_FORCE` reader - no public
pub type SAR_TSENS_XPD_FORCE_R = crate::FieldReader;
///Field `SAR_TSENS_XPD_FORCE` writer - no public
pub type SAR_TSENS_XPD_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SAR_TSENS_CLK_INV` reader - no public
pub type SAR_TSENS_CLK_INV_R = crate::BitReader;
///Field `SAR_TSENS_CLK_INV` writer - no public
pub type SAR_TSENS_CLK_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:11 - no public
    #[inline(always)]
    pub fn sar_tsens_xpd_wait(&self) -> SAR_TSENS_XPD_WAIT_R {
        SAR_TSENS_XPD_WAIT_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 12:13 - no public
    #[inline(always)]
    pub fn sar_tsens_xpd_force(&self) -> SAR_TSENS_XPD_FORCE_R {
        SAR_TSENS_XPD_FORCE_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - no public
    #[inline(always)]
    pub fn sar_tsens_clk_inv(&self) -> SAR_TSENS_CLK_INV_R {
        SAR_TSENS_CLK_INV_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TSENS_CTRL2")
            .field("sar_tsens_xpd_wait", &self.sar_tsens_xpd_wait())
            .field("sar_tsens_xpd_force", &self.sar_tsens_xpd_force())
            .field("sar_tsens_clk_inv", &self.sar_tsens_clk_inv())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - no public
    #[inline(always)]
    #[must_use]
    pub fn sar_tsens_xpd_wait(&mut self) -> SAR_TSENS_XPD_WAIT_W<SAR_TSENS_CTRL2_SPEC> {
        SAR_TSENS_XPD_WAIT_W::new(self, 0)
    }
    ///Bits 12:13 - no public
    #[inline(always)]
    #[must_use]
    pub fn sar_tsens_xpd_force(&mut self) -> SAR_TSENS_XPD_FORCE_W<SAR_TSENS_CTRL2_SPEC> {
        SAR_TSENS_XPD_FORCE_W::new(self, 12)
    }
    ///Bit 14 - no public
    #[inline(always)]
    #[must_use]
    pub fn sar_tsens_clk_inv(&mut self) -> SAR_TSENS_CLK_INV_W<SAR_TSENS_CTRL2_SPEC> {
        SAR_TSENS_CLK_INV_W::new(self, 14)
    }
}
/**configure tsens controller

You can [`read`](crate::generic::Reg::read) this register and get [`sar_tsens_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_tsens_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_TSENS_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_TSENS_CTRL2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_tsens_ctrl2::R`](R) reader structure
impl crate::Readable for SAR_TSENS_CTRL2_SPEC {}
///`write(|w| ..)` method takes [`sar_tsens_ctrl2::W`](W) writer structure
impl crate::Writable for SAR_TSENS_CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_TSENS_CTRL2 to value 0x4002
impl crate::Resettable for SAR_TSENS_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x4002;
}
