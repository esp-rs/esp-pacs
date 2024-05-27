///Register `DIEPMSK` reader
pub type R = crate::R<DIEPMSK_SPEC>;
///Register `DIEPMSK` writer
pub type W = crate::W<DIEPMSK_SPEC>;
///Field `DI_XFERCOMPLMSK` reader -
pub type DI_XFERCOMPLMSK_R = crate::BitReader;
///Field `DI_XFERCOMPLMSK` writer -
pub type DI_XFERCOMPLMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DI_EPDISBLDMSK` reader -
pub type DI_EPDISBLDMSK_R = crate::BitReader;
///Field `DI_EPDISBLDMSK` writer -
pub type DI_EPDISBLDMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DI_AHBERMSK` reader -
pub type DI_AHBERMSK_R = crate::BitReader;
///Field `DI_AHBERMSK` writer -
pub type DI_AHBERMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMEOUTMSK` reader -
pub type TIMEOUTMSK_R = crate::BitReader;
///Field `TIMEOUTMSK` writer -
pub type TIMEOUTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTKNTXFEMPMSK` reader -
pub type INTKNTXFEMPMSK_R = crate::BitReader;
///Field `INTKNTXFEMPMSK` writer -
pub type INTKNTXFEMPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INTKNEPMISMSK` reader -
pub type INTKNEPMISMSK_R = crate::BitReader;
///Field `INTKNEPMISMSK` writer -
pub type INTKNEPMISMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INEPNAKEFFMSK` reader -
pub type INEPNAKEFFMSK_R = crate::BitReader;
///Field `INEPNAKEFFMSK` writer -
pub type INEPNAKEFFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFIFOUNDRNMSK` reader -
pub type TXFIFOUNDRNMSK_R = crate::BitReader;
///Field `TXFIFOUNDRNMSK` writer -
pub type TXFIFOUNDRNMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BNAININTRMSK` reader -
pub type BNAININTRMSK_R = crate::BitReader;
///Field `BNAININTRMSK` writer -
pub type BNAININTRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DI_NAKMSK` reader -
pub type DI_NAKMSK_R = crate::BitReader;
///Field `DI_NAKMSK` writer -
pub type DI_NAKMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn di_xfercomplmsk(&self) -> DI_XFERCOMPLMSK_R {
        DI_XFERCOMPLMSK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn di_epdisbldmsk(&self) -> DI_EPDISBLDMSK_R {
        DI_EPDISBLDMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn di_ahbermsk(&self) -> DI_AHBERMSK_R {
        DI_AHBERMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn timeoutmsk(&self) -> TIMEOUTMSK_R {
        TIMEOUTMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn intkntxfempmsk(&self) -> INTKNTXFEMPMSK_R {
        INTKNTXFEMPMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn intknepmismsk(&self) -> INTKNEPMISMSK_R {
        INTKNEPMISMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn inepnakeffmsk(&self) -> INEPNAKEFFMSK_R {
        INEPNAKEFFMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8
    #[inline(always)]
    pub fn txfifoundrnmsk(&self) -> TXFIFOUNDRNMSK_R {
        TXFIFOUNDRNMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9
    #[inline(always)]
    pub fn bnainintrmsk(&self) -> BNAININTRMSK_R {
        BNAININTRMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13
    #[inline(always)]
    pub fn di_nakmsk(&self) -> DI_NAKMSK_R {
        DI_NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPMSK")
            .field("di_xfercomplmsk", &self.di_xfercomplmsk())
            .field("di_epdisbldmsk", &self.di_epdisbldmsk())
            .field("di_ahbermsk", &self.di_ahbermsk())
            .field("timeoutmsk", &self.timeoutmsk())
            .field("intkntxfempmsk", &self.intkntxfempmsk())
            .field("intknepmismsk", &self.intknepmismsk())
            .field("inepnakeffmsk", &self.inepnakeffmsk())
            .field("txfifoundrnmsk", &self.txfifoundrnmsk())
            .field("bnainintrmsk", &self.bnainintrmsk())
            .field("di_nakmsk", &self.di_nakmsk())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn di_xfercomplmsk(&mut self) -> DI_XFERCOMPLMSK_W<DIEPMSK_SPEC> {
        DI_XFERCOMPLMSK_W::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    #[must_use]
    pub fn di_epdisbldmsk(&mut self) -> DI_EPDISBLDMSK_W<DIEPMSK_SPEC> {
        DI_EPDISBLDMSK_W::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    #[must_use]
    pub fn di_ahbermsk(&mut self) -> DI_AHBERMSK_W<DIEPMSK_SPEC> {
        DI_AHBERMSK_W::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    #[must_use]
    pub fn timeoutmsk(&mut self) -> TIMEOUTMSK_W<DIEPMSK_SPEC> {
        TIMEOUTMSK_W::new(self, 3)
    }
    ///Bit 4
    #[inline(always)]
    #[must_use]
    pub fn intkntxfempmsk(&mut self) -> INTKNTXFEMPMSK_W<DIEPMSK_SPEC> {
        INTKNTXFEMPMSK_W::new(self, 4)
    }
    ///Bit 5
    #[inline(always)]
    #[must_use]
    pub fn intknepmismsk(&mut self) -> INTKNEPMISMSK_W<DIEPMSK_SPEC> {
        INTKNEPMISMSK_W::new(self, 5)
    }
    ///Bit 6
    #[inline(always)]
    #[must_use]
    pub fn inepnakeffmsk(&mut self) -> INEPNAKEFFMSK_W<DIEPMSK_SPEC> {
        INEPNAKEFFMSK_W::new(self, 6)
    }
    ///Bit 8
    #[inline(always)]
    #[must_use]
    pub fn txfifoundrnmsk(&mut self) -> TXFIFOUNDRNMSK_W<DIEPMSK_SPEC> {
        TXFIFOUNDRNMSK_W::new(self, 8)
    }
    ///Bit 9
    #[inline(always)]
    #[must_use]
    pub fn bnainintrmsk(&mut self) -> BNAININTRMSK_W<DIEPMSK_SPEC> {
        BNAININTRMSK_W::new(self, 9)
    }
    ///Bit 13
    #[inline(always)]
    #[must_use]
    pub fn di_nakmsk(&mut self) -> DI_NAKMSK_W<DIEPMSK_SPEC> {
        DI_NAKMSK_W::new(self, 13)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DIEPMSK_SPEC;
impl crate::RegisterSpec for DIEPMSK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`diepmsk::R`](R) reader structure
impl crate::Readable for DIEPMSK_SPEC {}
///`write(|w| ..)` method takes [`diepmsk::W`](W) writer structure
impl crate::Writable for DIEPMSK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DIEPMSK to value 0
impl crate::Resettable for DIEPMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
