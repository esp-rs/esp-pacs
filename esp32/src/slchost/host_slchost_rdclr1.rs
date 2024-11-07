#[doc = "Register `HOST_SLCHOST_RDCLR1` reader"]
pub type R = crate::R<HOST_SLCHOST_RDCLR1_SPEC>;
#[doc = "Register `HOST_SLCHOST_RDCLR1` writer"]
pub type W = crate::W<HOST_SLCHOST_RDCLR1_SPEC>;
#[doc = "Field `HOST_SLCHOST_SLC1_BIT7_CLRADDR` reader - "]
pub type HOST_SLCHOST_SLC1_BIT7_CLRADDR_R = crate::FieldReader<u16>;
#[doc = "Field `HOST_SLCHOST_SLC1_BIT7_CLRADDR` writer - "]
pub type HOST_SLCHOST_SLC1_BIT7_CLRADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `HOST_SLCHOST_SLC1_BIT6_CLRADDR` reader - "]
pub type HOST_SLCHOST_SLC1_BIT6_CLRADDR_R = crate::FieldReader<u16>;
#[doc = "Field `HOST_SLCHOST_SLC1_BIT6_CLRADDR` writer - "]
pub type HOST_SLCHOST_SLC1_BIT6_CLRADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn host_slchost_slc1_bit7_clraddr(&self) -> HOST_SLCHOST_SLC1_BIT7_CLRADDR_R {
        HOST_SLCHOST_SLC1_BIT7_CLRADDR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn host_slchost_slc1_bit6_clraddr(&self) -> HOST_SLCHOST_SLC1_BIT6_CLRADDR_R {
        HOST_SLCHOST_SLC1_BIT6_CLRADDR_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_RDCLR1")
            .field(
                "host_slchost_slc1_bit7_clraddr",
                &self.host_slchost_slc1_bit7_clraddr(),
            )
            .field(
                "host_slchost_slc1_bit6_clraddr",
                &self.host_slchost_slc1_bit6_clraddr(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn host_slchost_slc1_bit7_clraddr(
        &mut self,
    ) -> HOST_SLCHOST_SLC1_BIT7_CLRADDR_W<HOST_SLCHOST_RDCLR1_SPEC> {
        HOST_SLCHOST_SLC1_BIT7_CLRADDR_W::new(self, 0)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn host_slchost_slc1_bit6_clraddr(
        &mut self,
    ) -> HOST_SLCHOST_SLC1_BIT6_CLRADDR_W<HOST_SLCHOST_RDCLR1_SPEC> {
        HOST_SLCHOST_SLC1_BIT6_CLRADDR_W::new(self, 9)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`host_slchost_rdclr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_slchost_rdclr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_RDCLR1_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_RDCLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_rdclr1::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_RDCLR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_rdclr1::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_RDCLR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_RDCLR1 to value 0x0003_c1e0"]
impl crate::Resettable for HOST_SLCHOST_RDCLR1_SPEC {
    const RESET_VALUE: u32 = 0x0003_c1e0;
}
