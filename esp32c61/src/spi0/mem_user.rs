#[doc = "Register `MEM_USER` reader"]
pub type R = crate::R<MEM_USER_SPEC>;
#[doc = "Register `MEM_USER` writer"]
pub type W = crate::W<MEM_USER_SPEC>;
#[doc = "Field `MEM_CS_HOLD` reader - spi cs keep low when spi is in done phase. 1: enable 0: disable."]
pub type MEM_CS_HOLD_R = crate::BitReader;
#[doc = "Field `MEM_CS_HOLD` writer - spi cs keep low when spi is in done phase. 1: enable 0: disable."]
pub type MEM_CS_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CS_SETUP` reader - spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
pub type MEM_CS_SETUP_R = crate::BitReader;
#[doc = "Field `MEM_CS_SETUP` writer - spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
pub type MEM_CS_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_CK_OUT_EDGE` reader - The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3."]
pub type MEM_CK_OUT_EDGE_R = crate::BitReader;
#[doc = "Field `MEM_CK_OUT_EDGE` writer - The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3."]
pub type MEM_CK_OUT_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_USR_DUMMY_IDLE` reader - spi clock is disable in dummy phase when the bit is enable."]
pub type MEM_USR_DUMMY_IDLE_R = crate::BitReader;
#[doc = "Field `MEM_USR_DUMMY_IDLE` writer - spi clock is disable in dummy phase when the bit is enable."]
pub type MEM_USR_DUMMY_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_USR_DUMMY` reader - This bit enable the dummy phase of an operation."]
pub type MEM_USR_DUMMY_R = crate::BitReader;
#[doc = "Field `MEM_USR_DUMMY` writer - This bit enable the dummy phase of an operation."]
pub type MEM_USR_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - spi cs keep low when spi is in done phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_cs_hold(&self) -> MEM_CS_HOLD_R {
        MEM_CS_HOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_cs_setup(&self) -> MEM_CS_SETUP_R {
        MEM_CS_SETUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3."]
    #[inline(always)]
    pub fn mem_ck_out_edge(&self) -> MEM_CK_OUT_EDGE_R {
        MEM_CK_OUT_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable."]
    #[inline(always)]
    pub fn mem_usr_dummy_idle(&self) -> MEM_USR_DUMMY_IDLE_R {
        MEM_USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation."]
    #[inline(always)]
    pub fn mem_usr_dummy(&self) -> MEM_USR_DUMMY_R {
        MEM_USR_DUMMY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_USER")
            .field("mem_cs_hold", &self.mem_cs_hold())
            .field("mem_cs_setup", &self.mem_cs_setup())
            .field("mem_ck_out_edge", &self.mem_ck_out_edge())
            .field("mem_usr_dummy_idle", &self.mem_usr_dummy_idle())
            .field("mem_usr_dummy", &self.mem_usr_dummy())
            .finish()
    }
}
impl W {
    #[doc = "Bit 6 - spi cs keep low when spi is in done phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_cs_hold(&mut self) -> MEM_CS_HOLD_W<MEM_USER_SPEC> {
        MEM_CS_HOLD_W::new(self, 6)
    }
    #[doc = "Bit 7 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn mem_cs_setup(&mut self) -> MEM_CS_SETUP_W<MEM_USER_SPEC> {
        MEM_CS_SETUP_W::new(self, 7)
    }
    #[doc = "Bit 9 - The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3."]
    #[inline(always)]
    pub fn mem_ck_out_edge(&mut self) -> MEM_CK_OUT_EDGE_W<MEM_USER_SPEC> {
        MEM_CK_OUT_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable."]
    #[inline(always)]
    pub fn mem_usr_dummy_idle(&mut self) -> MEM_USR_DUMMY_IDLE_W<MEM_USER_SPEC> {
        MEM_USR_DUMMY_IDLE_W::new(self, 26)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation."]
    #[inline(always)]
    pub fn mem_usr_dummy(&mut self) -> MEM_USR_DUMMY_W<MEM_USER_SPEC> {
        MEM_USR_DUMMY_W::new(self, 29)
    }
}
#[doc = "SPI0 user register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_user::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_user::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_USER_SPEC;
impl crate::RegisterSpec for MEM_USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_user::R`](R) reader structure"]
impl crate::Readable for MEM_USER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_user::W`](W) writer structure"]
impl crate::Writable for MEM_USER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_USER to value 0"]
impl crate::Resettable for MEM_USER_SPEC {}
