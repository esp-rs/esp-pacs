///Register `DCDC_CTRL` reader
pub type R = crate::R<DCDC_CTRL_SPEC>;
///Register `DCDC_CTRL` writer
pub type W = crate::W<DCDC_CTRL_SPEC>;
///Field `DCDC_PRE_UP_DELAY` reader -
pub type DCDC_PRE_UP_DELAY_R = crate::FieldReader;
///Field `DCDC_PRE_UP_DELAY` writer -
pub type DCDC_PRE_UP_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DCDC_DOWN_DELAY` reader -
pub type DCDC_DOWN_DELAY_R = crate::FieldReader;
///Field `DCDC_DOWN_DELAY` writer -
pub type DCDC_DOWN_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EN` reader -
pub type EN_R = crate::BitReader;
///Field `EN` writer -
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_DCDC_UP` reader -
pub type TX_DCDC_UP_R = crate::BitReader;
///Field `TX_DCDC_UP` writer -
pub type TX_DCDC_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7
    #[inline(always)]
    pub fn dcdc_pre_up_delay(&self) -> DCDC_PRE_UP_DELAY_R {
        DCDC_PRE_UP_DELAY_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15
    #[inline(always)]
    pub fn dcdc_down_delay(&self) -> DCDC_DOWN_DELAY_R {
        DCDC_DOWN_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 31
    #[inline(always)]
    pub fn tx_dcdc_up(&self) -> TX_DCDC_UP_R {
        TX_DCDC_UP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_CTRL")
            .field("dcdc_pre_up_delay", &self.dcdc_pre_up_delay())
            .field("dcdc_down_delay", &self.dcdc_down_delay())
            .field("en", &self.en())
            .field("tx_dcdc_up", &self.tx_dcdc_up())
            .finish()
    }
}
impl W {
    ///Bits 0:7
    #[inline(always)]
    #[must_use]
    pub fn dcdc_pre_up_delay(&mut self) -> DCDC_PRE_UP_DELAY_W<DCDC_CTRL_SPEC> {
        DCDC_PRE_UP_DELAY_W::new(self, 0)
    }
    ///Bits 8:15
    #[inline(always)]
    #[must_use]
    pub fn dcdc_down_delay(&mut self) -> DCDC_DOWN_DELAY_W<DCDC_CTRL_SPEC> {
        DCDC_DOWN_DELAY_W::new(self, 8)
    }
    ///Bit 16
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<DCDC_CTRL_SPEC> {
        EN_W::new(self, 16)
    }
    ///Bit 31
    #[inline(always)]
    #[must_use]
    pub fn tx_dcdc_up(&mut self) -> TX_DCDC_UP_W<DCDC_CTRL_SPEC> {
        TX_DCDC_UP_W::new(self, 31)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`dcdc_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DCDC_CTRL_SPEC;
impl crate::RegisterSpec for DCDC_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dcdc_ctrl::R`](R) reader structure
impl crate::Readable for DCDC_CTRL_SPEC {}
///`write(|w| ..)` method takes [`dcdc_ctrl::W`](W) writer structure
impl crate::Writable for DCDC_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCDC_CTRL to value 0
impl crate::Resettable for DCDC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
