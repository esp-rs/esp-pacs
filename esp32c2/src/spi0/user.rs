///Register `USER` reader
pub type R = crate::R<USER_SPEC>;
///Register `USER` writer
pub type W = crate::W<USER_SPEC>;
///Field `CS_HOLD` reader - spi cs keep low when spi is in done phase. 1: enable 0: disable.
pub type CS_HOLD_R = crate::BitReader;
///Field `CS_HOLD` writer - spi cs keep low when spi is in done phase. 1: enable 0: disable.
pub type CS_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CS_SETUP` reader - spi cs is enable when spi is in prepare phase. 1: enable 0: disable.
pub type CS_SETUP_R = crate::BitReader;
///Field `CS_SETUP` writer - spi cs is enable when spi is in prepare phase. 1: enable 0: disable.
pub type CS_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CK_OUT_EDGE` reader - the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode.
pub type CK_OUT_EDGE_R = crate::BitReader;
///Field `CK_OUT_EDGE` writer - the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode.
pub type CK_OUT_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USR_DUMMY_IDLE` reader - spi clock is disable in dummy phase when the bit is enable.
pub type USR_DUMMY_IDLE_R = crate::BitReader;
///Field `USR_DUMMY_IDLE` writer - spi clock is disable in dummy phase when the bit is enable.
pub type USR_DUMMY_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USR_DUMMY` reader - This bit enable the dummy phase of an operation.
pub type USR_DUMMY_R = crate::BitReader;
///Field `USR_DUMMY` writer - This bit enable the dummy phase of an operation.
pub type USR_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 6 - spi cs keep low when spi is in done phase. 1: enable 0: disable.
    #[inline(always)]
    pub fn cs_hold(&self) -> CS_HOLD_R {
        CS_HOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable.
    #[inline(always)]
    pub fn cs_setup(&self) -> CS_SETUP_R {
        CS_SETUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode.
    #[inline(always)]
    pub fn ck_out_edge(&self) -> CK_OUT_EDGE_R {
        CK_OUT_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 26 - spi clock is disable in dummy phase when the bit is enable.
    #[inline(always)]
    pub fn usr_dummy_idle(&self) -> USR_DUMMY_IDLE_R {
        USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 29 - This bit enable the dummy phase of an operation.
    #[inline(always)]
    pub fn usr_dummy(&self) -> USR_DUMMY_R {
        USR_DUMMY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER")
            .field("cs_hold", &self.cs_hold())
            .field("cs_setup", &self.cs_setup())
            .field("ck_out_edge", &self.ck_out_edge())
            .field("usr_dummy_idle", &self.usr_dummy_idle())
            .field("usr_dummy", &self.usr_dummy())
            .finish()
    }
}
impl W {
    ///Bit 6 - spi cs keep low when spi is in done phase. 1: enable 0: disable.
    #[inline(always)]
    #[must_use]
    pub fn cs_hold(&mut self) -> CS_HOLD_W<USER_SPEC> {
        CS_HOLD_W::new(self, 6)
    }
    ///Bit 7 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable.
    #[inline(always)]
    #[must_use]
    pub fn cs_setup(&mut self) -> CS_SETUP_W<USER_SPEC> {
        CS_SETUP_W::new(self, 7)
    }
    ///Bit 9 - the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode.
    #[inline(always)]
    #[must_use]
    pub fn ck_out_edge(&mut self) -> CK_OUT_EDGE_W<USER_SPEC> {
        CK_OUT_EDGE_W::new(self, 9)
    }
    ///Bit 26 - spi clock is disable in dummy phase when the bit is enable.
    #[inline(always)]
    #[must_use]
    pub fn usr_dummy_idle(&mut self) -> USR_DUMMY_IDLE_W<USER_SPEC> {
        USR_DUMMY_IDLE_W::new(self, 26)
    }
    ///Bit 29 - This bit enable the dummy phase of an operation.
    #[inline(always)]
    #[must_use]
    pub fn usr_dummy(&mut self) -> USR_DUMMY_W<USER_SPEC> {
        USR_DUMMY_W::new(self, 29)
    }
}
/**SPI0 user register.

You can [`read`](crate::generic::Reg::read) this register and get [`user::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct USER_SPEC;
impl crate::RegisterSpec for USER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`user::R`](R) reader structure
impl crate::Readable for USER_SPEC {}
///`write(|w| ..)` method takes [`user::W`](W) writer structure
impl crate::Writable for USER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets USER to value 0
impl crate::Resettable for USER_SPEC {
    const RESET_VALUE: u32 = 0;
}
