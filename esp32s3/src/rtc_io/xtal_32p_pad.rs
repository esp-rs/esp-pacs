///Register `XTAL_32P_PAD` reader
pub type R = crate::R<XTAL_32P_PAD_SPEC>;
///Register `XTAL_32P_PAD` writer
pub type W = crate::W<XTAL_32P_PAD_SPEC>;
///Field `X32P_FUN_IE` reader - input enable in work mode
pub type X32P_FUN_IE_R = crate::BitReader;
///Field `X32P_FUN_IE` writer - input enable in work mode
pub type X32P_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `X32P_SLP_OE` reader - output enable in sleep mode
pub type X32P_SLP_OE_R = crate::BitReader;
///Field `X32P_SLP_OE` writer - output enable in sleep mode
pub type X32P_SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `X32P_SLP_IE` reader - input enable in sleep mode
pub type X32P_SLP_IE_R = crate::BitReader;
///Field `X32P_SLP_IE` writer - input enable in sleep mode
pub type X32P_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `X32P_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode
pub type X32P_SLP_SEL_R = crate::BitReader;
///Field `X32P_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode
pub type X32P_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `X32P_FUN_SEL` reader - function sel
pub type X32P_FUN_SEL_R = crate::FieldReader;
///Field `X32P_FUN_SEL` writer - function sel
pub type X32P_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `X32P_MUX_SEL` reader - 1: use RTC GPIO,0: use digital GPIO
pub type X32P_MUX_SEL_R = crate::BitReader;
///Field `X32P_MUX_SEL` writer - 1: use RTC GPIO,0: use digital GPIO
pub type X32P_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `X32P_RUE` reader - RUE
pub type X32P_RUE_R = crate::BitReader;
///Field `X32P_RUE` writer - RUE
pub type X32P_RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `X32P_RDE` reader - RDE
pub type X32P_RDE_R = crate::BitReader;
///Field `X32P_RDE` writer - RDE
pub type X32P_RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `X32P_DRV` reader - DRV
pub type X32P_DRV_R = crate::FieldReader;
///Field `X32P_DRV` writer - DRV
pub type X32P_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 13 - input enable in work mode
    #[inline(always)]
    pub fn x32p_fun_ie(&self) -> X32P_FUN_IE_R {
        X32P_FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - output enable in sleep mode
    #[inline(always)]
    pub fn x32p_slp_oe(&self) -> X32P_SLP_OE_R {
        X32P_SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - input enable in sleep mode
    #[inline(always)]
    pub fn x32p_slp_ie(&self) -> X32P_SLP_IE_R {
        X32P_SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn x32p_slp_sel(&self) -> X32P_SLP_SEL_R {
        X32P_SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - function sel
    #[inline(always)]
    pub fn x32p_fun_sel(&self) -> X32P_FUN_SEL_R {
        X32P_FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bit 19 - 1: use RTC GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn x32p_mux_sel(&self) -> X32P_MUX_SEL_R {
        X32P_MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 27 - RUE
    #[inline(always)]
    pub fn x32p_rue(&self) -> X32P_RUE_R {
        X32P_RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - RDE
    #[inline(always)]
    pub fn x32p_rde(&self) -> X32P_RDE_R {
        X32P_RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:30 - DRV
    #[inline(always)]
    pub fn x32p_drv(&self) -> X32P_DRV_R {
        X32P_DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTAL_32P_PAD")
            .field("x32p_fun_ie", &self.x32p_fun_ie())
            .field("x32p_slp_oe", &self.x32p_slp_oe())
            .field("x32p_slp_ie", &self.x32p_slp_ie())
            .field("x32p_slp_sel", &self.x32p_slp_sel())
            .field("x32p_fun_sel", &self.x32p_fun_sel())
            .field("x32p_mux_sel", &self.x32p_mux_sel())
            .field("x32p_rue", &self.x32p_rue())
            .field("x32p_rde", &self.x32p_rde())
            .field("x32p_drv", &self.x32p_drv())
            .finish()
    }
}
impl W {
    ///Bit 13 - input enable in work mode
    #[inline(always)]
    #[must_use]
    pub fn x32p_fun_ie(&mut self) -> X32P_FUN_IE_W<XTAL_32P_PAD_SPEC> {
        X32P_FUN_IE_W::new(self, 13)
    }
    ///Bit 14 - output enable in sleep mode
    #[inline(always)]
    #[must_use]
    pub fn x32p_slp_oe(&mut self) -> X32P_SLP_OE_W<XTAL_32P_PAD_SPEC> {
        X32P_SLP_OE_W::new(self, 14)
    }
    ///Bit 15 - input enable in sleep mode
    #[inline(always)]
    #[must_use]
    pub fn x32p_slp_ie(&mut self) -> X32P_SLP_IE_W<XTAL_32P_PAD_SPEC> {
        X32P_SLP_IE_W::new(self, 15)
    }
    ///Bit 16 - 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    #[must_use]
    pub fn x32p_slp_sel(&mut self) -> X32P_SLP_SEL_W<XTAL_32P_PAD_SPEC> {
        X32P_SLP_SEL_W::new(self, 16)
    }
    ///Bits 17:18 - function sel
    #[inline(always)]
    #[must_use]
    pub fn x32p_fun_sel(&mut self) -> X32P_FUN_SEL_W<XTAL_32P_PAD_SPEC> {
        X32P_FUN_SEL_W::new(self, 17)
    }
    ///Bit 19 - 1: use RTC GPIO,0: use digital GPIO
    #[inline(always)]
    #[must_use]
    pub fn x32p_mux_sel(&mut self) -> X32P_MUX_SEL_W<XTAL_32P_PAD_SPEC> {
        X32P_MUX_SEL_W::new(self, 19)
    }
    ///Bit 27 - RUE
    #[inline(always)]
    #[must_use]
    pub fn x32p_rue(&mut self) -> X32P_RUE_W<XTAL_32P_PAD_SPEC> {
        X32P_RUE_W::new(self, 27)
    }
    ///Bit 28 - RDE
    #[inline(always)]
    #[must_use]
    pub fn x32p_rde(&mut self) -> X32P_RDE_W<XTAL_32P_PAD_SPEC> {
        X32P_RDE_W::new(self, 28)
    }
    ///Bits 29:30 - DRV
    #[inline(always)]
    #[must_use]
    pub fn x32p_drv(&mut self) -> X32P_DRV_W<XTAL_32P_PAD_SPEC> {
        X32P_DRV_W::new(self, 29)
    }
}
/**configure RTC PAD15

You can [`read`](crate::generic::Reg::read) this register and get [`xtal_32p_pad::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal_32p_pad::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct XTAL_32P_PAD_SPEC;
impl crate::RegisterSpec for XTAL_32P_PAD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`xtal_32p_pad::R`](R) reader structure
impl crate::Readable for XTAL_32P_PAD_SPEC {}
///`write(|w| ..)` method takes [`xtal_32p_pad::W`](W) writer structure
impl crate::Writable for XTAL_32P_PAD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets XTAL_32P_PAD to value 0x4000_0000
impl crate::Resettable for XTAL_32P_PAD_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
