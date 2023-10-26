#[doc = "Register `RDCLR1` reader"]
pub type R = crate::R<RDCLR1_SPEC>;
#[doc = "Register `RDCLR1` writer"]
pub type W = crate::W<RDCLR1_SPEC>;
#[doc = "Field `SLCHOST_SLC1_BIT7_CLRADDR` reader - *******Description***********"]
pub type SLCHOST_SLC1_BIT7_CLRADDR_R = crate::FieldReader<u16>;
#[doc = "Field `SLCHOST_SLC1_BIT7_CLRADDR` writer - *******Description***********"]
pub type SLCHOST_SLC1_BIT7_CLRADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `SLCHOST_SLC1_BIT6_CLRADDR` reader - *******Description***********"]
pub type SLCHOST_SLC1_BIT6_CLRADDR_R = crate::FieldReader<u16>;
#[doc = "Field `SLCHOST_SLC1_BIT6_CLRADDR` writer - *******Description***********"]
pub type SLCHOST_SLC1_BIT6_CLRADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_slc1_bit7_clraddr(&self) -> SLCHOST_SLC1_BIT7_CLRADDR_R {
        SLCHOST_SLC1_BIT7_CLRADDR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_slc1_bit6_clraddr(&self) -> SLCHOST_SLC1_BIT6_CLRADDR_R {
        SLCHOST_SLC1_BIT6_CLRADDR_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDCLR1")
            .field(
                "slchost_slc1_bit7_clraddr",
                &format_args!("{}", self.slchost_slc1_bit7_clraddr().bits()),
            )
            .field(
                "slchost_slc1_bit6_clraddr",
                &format_args!("{}", self.slchost_slc1_bit6_clraddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RDCLR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_slc1_bit7_clraddr(&mut self) -> SLCHOST_SLC1_BIT7_CLRADDR_W<RDCLR1_SPEC, 0> {
        SLCHOST_SLC1_BIT7_CLRADDR_W::new(self)
    }
    #[doc = "Bits 9:17 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_slc1_bit6_clraddr(&mut self) -> SLCHOST_SLC1_BIT6_CLRADDR_W<RDCLR1_SPEC, 9> {
        SLCHOST_SLC1_BIT6_CLRADDR_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdclr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdclr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDCLR1_SPEC;
impl crate::RegisterSpec for RDCLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdclr1::R`](R) reader structure"]
impl crate::Readable for RDCLR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rdclr1::W`](W) writer structure"]
impl crate::Writable for RDCLR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RDCLR1 to value 0x0003_c1e0"]
impl crate::Resettable for RDCLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_c1e0;
}
