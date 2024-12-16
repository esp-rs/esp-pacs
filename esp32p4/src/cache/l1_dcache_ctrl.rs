#[doc = "Register `L1_DCACHE_CTRL` reader"]
pub type R = crate::R<L1_DCACHE_CTRL_SPEC>;
#[doc = "Register `L1_DCACHE_CTRL` writer"]
pub type W = crate::W<L1_DCACHE_CTRL_SPEC>;
#[doc = "Field `L1_DCACHE_SHUT_DBUS0` reader - The bit is used to disable core0 dbus access L1-DCache, 0: enable, 1: disable"]
pub type L1_DCACHE_SHUT_DBUS0_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_SHUT_DBUS0` writer - The bit is used to disable core0 dbus access L1-DCache, 0: enable, 1: disable"]
pub type L1_DCACHE_SHUT_DBUS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_SHUT_DBUS1` reader - The bit is used to disable core1 dbus access L1-DCache, 0: enable, 1: disable"]
pub type L1_DCACHE_SHUT_DBUS1_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_SHUT_DBUS1` writer - The bit is used to disable core1 dbus access L1-DCache, 0: enable, 1: disable"]
pub type L1_DCACHE_SHUT_DBUS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_SHUT_DBUS2` reader - Reserved"]
pub type L1_DCACHE_SHUT_DBUS2_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_SHUT_DBUS3` reader - Reserved"]
pub type L1_DCACHE_SHUT_DBUS3_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_SHUT_DMA` reader - The bit is used to disable DMA access L1-DCache, 0: enable, 1: disable"]
pub type L1_DCACHE_SHUT_DMA_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_SHUT_DMA` writer - The bit is used to disable DMA access L1-DCache, 0: enable, 1: disable"]
pub type L1_DCACHE_SHUT_DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_UNDEF_OP` reader - Reserved"]
pub type L1_DCACHE_UNDEF_OP_R = crate::FieldReader;
#[doc = "Field `L1_DCACHE_UNDEF_OP` writer - Reserved"]
pub type L1_DCACHE_UNDEF_OP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - The bit is used to disable core0 dbus access L1-DCache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_dcache_shut_dbus0(&self) -> L1_DCACHE_SHUT_DBUS0_R {
        L1_DCACHE_SHUT_DBUS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to disable core1 dbus access L1-DCache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_dcache_shut_dbus1(&self) -> L1_DCACHE_SHUT_DBUS1_R {
        L1_DCACHE_SHUT_DBUS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_dcache_shut_dbus2(&self) -> L1_DCACHE_SHUT_DBUS2_R {
        L1_DCACHE_SHUT_DBUS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_dcache_shut_dbus3(&self) -> L1_DCACHE_SHUT_DBUS3_R {
        L1_DCACHE_SHUT_DBUS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to disable DMA access L1-DCache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_dcache_shut_dma(&self) -> L1_DCACHE_SHUT_DMA_R {
        L1_DCACHE_SHUT_DMA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn l1_dcache_undef_op(&self) -> L1_DCACHE_UNDEF_OP_R {
        L1_DCACHE_UNDEF_OP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_DCACHE_CTRL")
            .field("l1_dcache_shut_dbus0", &self.l1_dcache_shut_dbus0())
            .field("l1_dcache_shut_dbus1", &self.l1_dcache_shut_dbus1())
            .field("l1_dcache_shut_dbus2", &self.l1_dcache_shut_dbus2())
            .field("l1_dcache_shut_dbus3", &self.l1_dcache_shut_dbus3())
            .field("l1_dcache_shut_dma", &self.l1_dcache_shut_dma())
            .field("l1_dcache_undef_op", &self.l1_dcache_undef_op())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to disable core0 dbus access L1-DCache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_dcache_shut_dbus0(&mut self) -> L1_DCACHE_SHUT_DBUS0_W<L1_DCACHE_CTRL_SPEC> {
        L1_DCACHE_SHUT_DBUS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to disable core1 dbus access L1-DCache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_dcache_shut_dbus1(&mut self) -> L1_DCACHE_SHUT_DBUS1_W<L1_DCACHE_CTRL_SPEC> {
        L1_DCACHE_SHUT_DBUS1_W::new(self, 1)
    }
    #[doc = "Bit 4 - The bit is used to disable DMA access L1-DCache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_dcache_shut_dma(&mut self) -> L1_DCACHE_SHUT_DMA_W<L1_DCACHE_CTRL_SPEC> {
        L1_DCACHE_SHUT_DMA_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn l1_dcache_undef_op(&mut self) -> L1_DCACHE_UNDEF_OP_W<L1_DCACHE_CTRL_SPEC> {
        L1_DCACHE_UNDEF_OP_W::new(self, 8)
    }
}
#[doc = "L1 data Cache(L1-DCache) control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_dcache_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_dcache_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_DCACHE_CTRL_SPEC;
impl crate::RegisterSpec for L1_DCACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dcache_ctrl::R`](R) reader structure"]
impl crate::Readable for L1_DCACHE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_dcache_ctrl::W`](W) writer structure"]
impl crate::Writable for L1_DCACHE_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1_DCACHE_CTRL to value 0"]
impl crate::Resettable for L1_DCACHE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
