///Register `PIN0` reader
pub type R = crate::R<PIN0_SPEC>;
///Register `PIN0` writer
pub type W = crate::W<PIN0_SPEC>;
///Field `SYNC_BYPASS` reader - need des
pub type SYNC_BYPASS_R = crate::FieldReader;
///Field `SYNC_BYPASS` writer - need des
pub type SYNC_BYPASS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PAD_DRIVER` reader - need des
pub type PAD_DRIVER_R = crate::BitReader;
///Field `PAD_DRIVER` writer - need des
pub type PAD_DRIVER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EDGE_WAKEUP_CLR` writer - need des
pub type EDGE_WAKEUP_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INT_TYPE` reader - need des
pub type INT_TYPE_R = crate::FieldReader;
///Field `INT_TYPE` writer - need des
pub type INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `WAKEUP_ENABLE` reader - need des
pub type WAKEUP_ENABLE_R = crate::BitReader;
///Field `WAKEUP_ENABLE` writer - need des
pub type WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FILTER_EN` reader - need des
pub type FILTER_EN_R = crate::BitReader;
///Field `FILTER_EN` writer - need des
pub type FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - need des
    #[inline(always)]
    pub fn sync_bypass(&self) -> SYNC_BYPASS_R {
        SYNC_BYPASS_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - need des
    #[inline(always)]
    pub fn pad_driver(&self) -> PAD_DRIVER_R {
        PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 7:9 - need des
    #[inline(always)]
    pub fn int_type(&self) -> INT_TYPE_R {
        INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    ///Bit 10 - need des
    #[inline(always)]
    pub fn wakeup_enable(&self) -> WAKEUP_ENABLE_R {
        WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - need des
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN0")
            .field("sync_bypass", &self.sync_bypass())
            .field("pad_driver", &self.pad_driver())
            .field("int_type", &self.int_type())
            .field("wakeup_enable", &self.wakeup_enable())
            .field("filter_en", &self.filter_en())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - need des
    #[inline(always)]
    #[must_use]
    pub fn sync_bypass(&mut self) -> SYNC_BYPASS_W<PIN0_SPEC> {
        SYNC_BYPASS_W::new(self, 0)
    }
    ///Bit 2 - need des
    #[inline(always)]
    #[must_use]
    pub fn pad_driver(&mut self) -> PAD_DRIVER_W<PIN0_SPEC> {
        PAD_DRIVER_W::new(self, 2)
    }
    ///Bit 3 - need des
    #[inline(always)]
    #[must_use]
    pub fn edge_wakeup_clr(&mut self) -> EDGE_WAKEUP_CLR_W<PIN0_SPEC> {
        EDGE_WAKEUP_CLR_W::new(self, 3)
    }
    ///Bits 7:9 - need des
    #[inline(always)]
    #[must_use]
    pub fn int_type(&mut self) -> INT_TYPE_W<PIN0_SPEC> {
        INT_TYPE_W::new(self, 7)
    }
    ///Bit 10 - need des
    #[inline(always)]
    #[must_use]
    pub fn wakeup_enable(&mut self) -> WAKEUP_ENABLE_W<PIN0_SPEC> {
        WAKEUP_ENABLE_W::new(self, 10)
    }
    ///Bit 11 - need des
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<PIN0_SPEC> {
        FILTER_EN_W::new(self, 11)
    }
}
/**need des

You can [`read`](crate::generic::Reg::read) this register and get [`pin0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PIN0_SPEC;
impl crate::RegisterSpec for PIN0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pin0::R`](R) reader structure
impl crate::Readable for PIN0_SPEC {}
///`write(|w| ..)` method takes [`pin0::W`](W) writer structure
impl crate::Writable for PIN0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PIN0 to value 0
impl crate::Resettable for PIN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
