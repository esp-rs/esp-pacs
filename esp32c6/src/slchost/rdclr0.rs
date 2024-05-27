///Register `RDCLR0` reader
pub type R = crate::R<RDCLR0_SPEC>;
///Register `RDCLR0` writer
pub type W = crate::W<RDCLR0_SPEC>;
///Field `SLCHOST_SLC0_BIT7_CLRADDR` reader - *******Description***********
pub type SLCHOST_SLC0_BIT7_CLRADDR_R = crate::FieldReader<u16>;
///Field `SLCHOST_SLC0_BIT7_CLRADDR` writer - *******Description***********
pub type SLCHOST_SLC0_BIT7_CLRADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `SLCHOST_SLC0_BIT6_CLRADDR` reader - *******Description***********
pub type SLCHOST_SLC0_BIT6_CLRADDR_R = crate::FieldReader<u16>;
///Field `SLCHOST_SLC0_BIT6_CLRADDR` writer - *******Description***********
pub type SLCHOST_SLC0_BIT6_CLRADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - *******Description***********
    #[inline(always)]
    pub fn slchost_slc0_bit7_clraddr(&self) -> SLCHOST_SLC0_BIT7_CLRADDR_R {
        SLCHOST_SLC0_BIT7_CLRADDR_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:17 - *******Description***********
    #[inline(always)]
    pub fn slchost_slc0_bit6_clraddr(&self) -> SLCHOST_SLC0_BIT6_CLRADDR_R {
        SLCHOST_SLC0_BIT6_CLRADDR_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDCLR0")
            .field(
                "slchost_slc0_bit7_clraddr",
                &self.slchost_slc0_bit7_clraddr(),
            )
            .field(
                "slchost_slc0_bit6_clraddr",
                &self.slchost_slc0_bit6_clraddr(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:8 - *******Description***********
    #[inline(always)]
    #[must_use]
    pub fn slchost_slc0_bit7_clraddr(&mut self) -> SLCHOST_SLC0_BIT7_CLRADDR_W<RDCLR0_SPEC> {
        SLCHOST_SLC0_BIT7_CLRADDR_W::new(self, 0)
    }
    ///Bits 9:17 - *******Description***********
    #[inline(always)]
    #[must_use]
    pub fn slchost_slc0_bit6_clraddr(&mut self) -> SLCHOST_SLC0_BIT6_CLRADDR_W<RDCLR0_SPEC> {
        SLCHOST_SLC0_BIT6_CLRADDR_W::new(self, 9)
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdclr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdclr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDCLR0_SPEC;
impl crate::RegisterSpec for RDCLR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rdclr0::R`](R) reader structure
impl crate::Readable for RDCLR0_SPEC {}
///`write(|w| ..)` method takes [`rdclr0::W`](W) writer structure
impl crate::Writable for RDCLR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RDCLR0 to value 0x0003_c044
impl crate::Resettable for RDCLR0_SPEC {
    const RESET_VALUE: u32 = 0x0003_c044;
}
