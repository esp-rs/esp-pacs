#[doc = "Register `HOST_SLCHOST_RDCLR0` reader"]
pub type R = crate::R<HOST_SLCHOST_RDCLR0_SPEC>;
#[doc = "Register `HOST_SLCHOST_RDCLR0` writer"]
pub type W = crate::W<HOST_SLCHOST_RDCLR0_SPEC>;
#[doc = "Field `HOST_SLCHOST_SLC0_BIT7_CLRADDR` reader - "]
pub type HOST_SLCHOST_SLC0_BIT7_CLRADDR_R = crate::FieldReader<u16>;
#[doc = "Field `HOST_SLCHOST_SLC0_BIT7_CLRADDR` writer - "]
pub type HOST_SLCHOST_SLC0_BIT7_CLRADDR_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `HOST_SLCHOST_SLC0_BIT6_CLRADDR` reader - "]
pub type HOST_SLCHOST_SLC0_BIT6_CLRADDR_R = crate::FieldReader<u16>;
#[doc = "Field `HOST_SLCHOST_SLC0_BIT6_CLRADDR` writer - "]
pub type HOST_SLCHOST_SLC0_BIT6_CLRADDR_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn host_slchost_slc0_bit7_clraddr(&self) -> HOST_SLCHOST_SLC0_BIT7_CLRADDR_R {
        HOST_SLCHOST_SLC0_BIT7_CLRADDR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn host_slchost_slc0_bit6_clraddr(&self) -> HOST_SLCHOST_SLC0_BIT6_CLRADDR_R {
        HOST_SLCHOST_SLC0_BIT6_CLRADDR_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_RDCLR0")
            .field(
                "host_slchost_slc0_bit7_clraddr",
                &format_args!("{}", self.host_slchost_slc0_bit7_clraddr().bits()),
            )
            .field(
                "host_slchost_slc0_bit6_clraddr",
                &format_args!("{}", self.host_slchost_slc0_bit6_clraddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_RDCLR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_slc0_bit7_clraddr(
        &mut self,
    ) -> HOST_SLCHOST_SLC0_BIT7_CLRADDR_W<HOST_SLCHOST_RDCLR0_SPEC, 0> {
        HOST_SLCHOST_SLC0_BIT7_CLRADDR_W::new(self)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_slc0_bit6_clraddr(
        &mut self,
    ) -> HOST_SLCHOST_SLC0_BIT6_CLRADDR_W<HOST_SLCHOST_RDCLR0_SPEC, 9> {
        HOST_SLCHOST_SLC0_BIT6_CLRADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_rdclr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_rdclr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_RDCLR0_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_RDCLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_rdclr0::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_RDCLR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_rdclr0::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_RDCLR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_RDCLR0 to value 0x0003_c044"]
impl crate::Resettable for HOST_SLCHOST_RDCLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_c044;
}
