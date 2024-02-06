#[doc = "Register `L1_ICACHE_CTRL` reader"]
pub type R = crate::R<L1_ICACHE_CTRL_SPEC>;
#[doc = "Register `L1_ICACHE_CTRL` writer"]
pub type W = crate::W<L1_ICACHE_CTRL_SPEC>;
#[doc = "Field `L1_ICACHE_SHUT_IBUS0` reader - The bit is used to disable core0 ibus access L1-ICache, 0: enable, 1: disable"]
pub type L1_ICACHE_SHUT_IBUS0_R = crate::BitReader;
#[doc = "Field `L1_ICACHE_SHUT_IBUS0` writer - The bit is used to disable core0 ibus access L1-ICache, 0: enable, 1: disable"]
pub type L1_ICACHE_SHUT_IBUS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE_SHUT_IBUS1` reader - The bit is used to disable core1 ibus access L1-ICache, 0: enable, 1: disable"]
pub type L1_ICACHE_SHUT_IBUS1_R = crate::BitReader;
#[doc = "Field `L1_ICACHE_SHUT_IBUS1` writer - The bit is used to disable core1 ibus access L1-ICache, 0: enable, 1: disable"]
pub type L1_ICACHE_SHUT_IBUS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE_SHUT_IBUS2` reader - Reserved"]
pub type L1_ICACHE_SHUT_IBUS2_R = crate::BitReader;
#[doc = "Field `L1_ICACHE_SHUT_IBUS3` reader - Reserved"]
pub type L1_ICACHE_SHUT_IBUS3_R = crate::BitReader;
#[doc = "Field `L1_ICACHE_UNDEF_OP` reader - Reserved"]
pub type L1_ICACHE_UNDEF_OP_R = crate::FieldReader;
#[doc = "Field `L1_ICACHE_UNDEF_OP` writer - Reserved"]
pub type L1_ICACHE_UNDEF_OP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - The bit is used to disable core0 ibus access L1-ICache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_icache_shut_ibus0(&self) -> L1_ICACHE_SHUT_IBUS0_R {
        L1_ICACHE_SHUT_IBUS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to disable core1 ibus access L1-ICache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_icache_shut_ibus1(&self) -> L1_ICACHE_SHUT_IBUS1_R {
        L1_ICACHE_SHUT_IBUS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache_shut_ibus2(&self) -> L1_ICACHE_SHUT_IBUS2_R {
        L1_ICACHE_SHUT_IBUS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache_shut_ibus3(&self) -> L1_ICACHE_SHUT_IBUS3_R {
        L1_ICACHE_SHUT_IBUS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn l1_icache_undef_op(&self) -> L1_ICACHE_UNDEF_OP_R {
        L1_ICACHE_UNDEF_OP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_ICACHE_CTRL")
            .field(
                "l1_icache_shut_ibus0",
                &format_args!("{}", self.l1_icache_shut_ibus0().bit()),
            )
            .field(
                "l1_icache_shut_ibus1",
                &format_args!("{}", self.l1_icache_shut_ibus1().bit()),
            )
            .field(
                "l1_icache_shut_ibus2",
                &format_args!("{}", self.l1_icache_shut_ibus2().bit()),
            )
            .field(
                "l1_icache_shut_ibus3",
                &format_args!("{}", self.l1_icache_shut_ibus3().bit()),
            )
            .field(
                "l1_icache_undef_op",
                &format_args!("{}", self.l1_icache_undef_op().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_ICACHE_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to disable core0 ibus access L1-ICache, 0: enable, 1: disable"]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache_shut_ibus0(&mut self) -> L1_ICACHE_SHUT_IBUS0_W<L1_ICACHE_CTRL_SPEC> {
        L1_ICACHE_SHUT_IBUS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to disable core1 ibus access L1-ICache, 0: enable, 1: disable"]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache_shut_ibus1(&mut self) -> L1_ICACHE_SHUT_IBUS1_W<L1_ICACHE_CTRL_SPEC> {
        L1_ICACHE_SHUT_IBUS1_W::new(self, 1)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache_undef_op(&mut self) -> L1_ICACHE_UNDEF_OP_W<L1_ICACHE_CTRL_SPEC> {
        L1_ICACHE_UNDEF_OP_W::new(self, 8)
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
#[doc = "L1 instruction Cache(L1-ICache) control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_icache_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_ICACHE_CTRL_SPEC;
impl crate::RegisterSpec for L1_ICACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache_ctrl::R`](R) reader structure"]
impl crate::Readable for L1_ICACHE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_icache_ctrl::W`](W) writer structure"]
impl crate::Writable for L1_ICACHE_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1_ICACHE_CTRL to value 0"]
impl crate::Resettable for L1_ICACHE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
