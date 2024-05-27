///Register `PAD8` reader
pub type R = crate::R<PAD8_SPEC>;
///Register `PAD8` writer
pub type W = crate::W<PAD8_SPEC>;
///Field `REG_PAD8_DRV` reader - Reserved
pub type REG_PAD8_DRV_R = crate::FieldReader;
///Field `REG_PAD8_DRV` writer - Reserved
pub type REG_PAD8_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `REG_PAD8_RDE` reader - Reserved
pub type REG_PAD8_RDE_R = crate::BitReader;
///Field `REG_PAD8_RDE` writer - Reserved
pub type REG_PAD8_RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_PAD8_RUE` reader - Reserved
pub type REG_PAD8_RUE_R = crate::BitReader;
///Field `REG_PAD8_RUE` writer - Reserved
pub type REG_PAD8_RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_PAD8_MUX_SEL` reader - 1:use LP GPIO,0: use digital GPIO
pub type REG_PAD8_MUX_SEL_R = crate::BitReader;
///Field `REG_PAD8_MUX_SEL` writer - 1:use LP GPIO,0: use digital GPIO
pub type REG_PAD8_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_PAD8_FUN_SEL` reader - function sel
pub type REG_PAD8_FUN_SEL_R = crate::FieldReader;
///Field `REG_PAD8_FUN_SEL` writer - function sel
pub type REG_PAD8_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `REG_PAD8_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode
pub type REG_PAD8_SLP_SEL_R = crate::BitReader;
///Field `REG_PAD8_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode
pub type REG_PAD8_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_PAD8_SLP_IE` reader - input enable in sleep mode
pub type REG_PAD8_SLP_IE_R = crate::BitReader;
///Field `REG_PAD8_SLP_IE` writer - input enable in sleep mode
pub type REG_PAD8_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_PAD8_SLP_OE` reader - output enable in sleep mode
pub type REG_PAD8_SLP_OE_R = crate::BitReader;
///Field `REG_PAD8_SLP_OE` writer - output enable in sleep mode
pub type REG_PAD8_SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_PAD8_FUN_IE` reader - input enable in work mode
pub type REG_PAD8_FUN_IE_R = crate::BitReader;
///Field `REG_PAD8_FUN_IE` writer - input enable in work mode
pub type REG_PAD8_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_PAD8_FILTER_EN` reader - need des
pub type REG_PAD8_FILTER_EN_R = crate::BitReader;
///Field `REG_PAD8_FILTER_EN` writer - need des
pub type REG_PAD8_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Reserved
    #[inline(always)]
    pub fn reg_pad8_drv(&self) -> REG_PAD8_DRV_R {
        REG_PAD8_DRV_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Reserved
    #[inline(always)]
    pub fn reg_pad8_rde(&self) -> REG_PAD8_RDE_R {
        REG_PAD8_RDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Reserved
    #[inline(always)]
    pub fn reg_pad8_rue(&self) -> REG_PAD8_RUE_R {
        REG_PAD8_RUE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    pub fn reg_pad8_mux_sel(&self) -> REG_PAD8_MUX_SEL_R {
        REG_PAD8_MUX_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - function sel
    #[inline(always)]
    pub fn reg_pad8_fun_sel(&self) -> REG_PAD8_FUN_SEL_R {
        REG_PAD8_FUN_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    pub fn reg_pad8_slp_sel(&self) -> REG_PAD8_SLP_SEL_R {
        REG_PAD8_SLP_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - input enable in sleep mode
    #[inline(always)]
    pub fn reg_pad8_slp_ie(&self) -> REG_PAD8_SLP_IE_R {
        REG_PAD8_SLP_IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - output enable in sleep mode
    #[inline(always)]
    pub fn reg_pad8_slp_oe(&self) -> REG_PAD8_SLP_OE_R {
        REG_PAD8_SLP_OE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - input enable in work mode
    #[inline(always)]
    pub fn reg_pad8_fun_ie(&self) -> REG_PAD8_FUN_IE_R {
        REG_PAD8_FUN_IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - need des
    #[inline(always)]
    pub fn reg_pad8_filter_en(&self) -> REG_PAD8_FILTER_EN_R {
        REG_PAD8_FILTER_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD8")
            .field("reg_pad8_drv", &self.reg_pad8_drv())
            .field("reg_pad8_rde", &self.reg_pad8_rde())
            .field("reg_pad8_rue", &self.reg_pad8_rue())
            .field("reg_pad8_mux_sel", &self.reg_pad8_mux_sel())
            .field("reg_pad8_fun_sel", &self.reg_pad8_fun_sel())
            .field("reg_pad8_slp_sel", &self.reg_pad8_slp_sel())
            .field("reg_pad8_slp_ie", &self.reg_pad8_slp_ie())
            .field("reg_pad8_slp_oe", &self.reg_pad8_slp_oe())
            .field("reg_pad8_fun_ie", &self.reg_pad8_fun_ie())
            .field("reg_pad8_filter_en", &self.reg_pad8_filter_en())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn reg_pad8_drv(&mut self) -> REG_PAD8_DRV_W<PAD8_SPEC> {
        REG_PAD8_DRV_W::new(self, 0)
    }
    ///Bit 2 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn reg_pad8_rde(&mut self) -> REG_PAD8_RDE_W<PAD8_SPEC> {
        REG_PAD8_RDE_W::new(self, 2)
    }
    ///Bit 3 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn reg_pad8_rue(&mut self) -> REG_PAD8_RUE_W<PAD8_SPEC> {
        REG_PAD8_RUE_W::new(self, 3)
    }
    ///Bit 4 - 1:use LP GPIO,0: use digital GPIO
    #[inline(always)]
    #[must_use]
    pub fn reg_pad8_mux_sel(&mut self) -> REG_PAD8_MUX_SEL_W<PAD8_SPEC> {
        REG_PAD8_MUX_SEL_W::new(self, 4)
    }
    ///Bits 5:6 - function sel
    #[inline(always)]
    #[must_use]
    pub fn reg_pad8_fun_sel(&mut self) -> REG_PAD8_FUN_SEL_W<PAD8_SPEC> {
        REG_PAD8_FUN_SEL_W::new(self, 5)
    }
    ///Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode
    #[inline(always)]
    #[must_use]
    pub fn reg_pad8_slp_sel(&mut self) -> REG_PAD8_SLP_SEL_W<PAD8_SPEC> {
        REG_PAD8_SLP_SEL_W::new(self, 7)
    }
    ///Bit 8 - input enable in sleep mode
    #[inline(always)]
    #[must_use]
    pub fn reg_pad8_slp_ie(&mut self) -> REG_PAD8_SLP_IE_W<PAD8_SPEC> {
        REG_PAD8_SLP_IE_W::new(self, 8)
    }
    ///Bit 9 - output enable in sleep mode
    #[inline(always)]
    #[must_use]
    pub fn reg_pad8_slp_oe(&mut self) -> REG_PAD8_SLP_OE_W<PAD8_SPEC> {
        REG_PAD8_SLP_OE_W::new(self, 9)
    }
    ///Bit 10 - input enable in work mode
    #[inline(always)]
    #[must_use]
    pub fn reg_pad8_fun_ie(&mut self) -> REG_PAD8_FUN_IE_W<PAD8_SPEC> {
        REG_PAD8_FUN_IE_W::new(self, 10)
    }
    ///Bit 11 - need des
    #[inline(always)]
    #[must_use]
    pub fn reg_pad8_filter_en(&mut self) -> REG_PAD8_FILTER_EN_W<PAD8_SPEC> {
        REG_PAD8_FILTER_EN_W::new(self, 11)
    }
}
/**Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`pad8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PAD8_SPEC;
impl crate::RegisterSpec for PAD8_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pad8::R`](R) reader structure
impl crate::Readable for PAD8_SPEC {}
///`write(|w| ..)` method takes [`pad8::W`](W) writer structure
impl crate::Writable for PAD8_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PAD8 to value 0x02
impl crate::Resettable for PAD8_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
