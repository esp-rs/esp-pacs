#[doc = "Register `SPI_MEM_USER` reader"]
pub type R = crate::R<SPI_MEM_USER_SPEC>;
#[doc = "Register `SPI_MEM_USER` writer"]
pub type W = crate::W<SPI_MEM_USER_SPEC>;
#[doc = "Field `SPI_MEM_CS_HOLD` reader - spi cs keep low when spi is in done phase. 1: enable 0: disable."]
pub type SPI_MEM_CS_HOLD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CS_HOLD` writer - spi cs keep low when spi is in done phase. 1: enable 0: disable."]
pub type SPI_MEM_CS_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_CS_SETUP` reader - spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
pub type SPI_MEM_CS_SETUP_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CS_SETUP` writer - spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
pub type SPI_MEM_CS_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_CK_OUT_EDGE` reader - The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3."]
pub type SPI_MEM_CK_OUT_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CK_OUT_EDGE` writer - The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3."]
pub type SPI_MEM_CK_OUT_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_USR_DUMMY_IDLE` reader - spi clock is disable in dummy phase when the bit is enable."]
pub type SPI_MEM_USR_DUMMY_IDLE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_USR_DUMMY_IDLE` writer - spi clock is disable in dummy phase when the bit is enable."]
pub type SPI_MEM_USR_DUMMY_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_USR_DUMMY` reader - This bit enable the dummy phase of an operation."]
pub type SPI_MEM_USR_DUMMY_R = crate::BitReader;
#[doc = "Field `SPI_MEM_USR_DUMMY` writer - This bit enable the dummy phase of an operation."]
pub type SPI_MEM_USR_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - spi cs keep low when spi is in done phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_cs_hold(&self) -> SPI_MEM_CS_HOLD_R {
        SPI_MEM_CS_HOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_cs_setup(&self) -> SPI_MEM_CS_SETUP_R {
        SPI_MEM_CS_SETUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3."]
    #[inline(always)]
    pub fn spi_mem_ck_out_edge(&self) -> SPI_MEM_CK_OUT_EDGE_R {
        SPI_MEM_CK_OUT_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable."]
    #[inline(always)]
    pub fn spi_mem_usr_dummy_idle(&self) -> SPI_MEM_USR_DUMMY_IDLE_R {
        SPI_MEM_USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation."]
    #[inline(always)]
    pub fn spi_mem_usr_dummy(&self) -> SPI_MEM_USR_DUMMY_R {
        SPI_MEM_USR_DUMMY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_USER")
            .field(
                "spi_mem_cs_hold",
                &format_args!("{}", self.spi_mem_cs_hold().bit()),
            )
            .field(
                "spi_mem_cs_setup",
                &format_args!("{}", self.spi_mem_cs_setup().bit()),
            )
            .field(
                "spi_mem_ck_out_edge",
                &format_args!("{}", self.spi_mem_ck_out_edge().bit()),
            )
            .field(
                "spi_mem_usr_dummy_idle",
                &format_args!("{}", self.spi_mem_usr_dummy_idle().bit()),
            )
            .field(
                "spi_mem_usr_dummy",
                &format_args!("{}", self.spi_mem_usr_dummy().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_USER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 6 - spi cs keep low when spi is in done phase. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_cs_hold(&mut self) -> SPI_MEM_CS_HOLD_W<SPI_MEM_USER_SPEC> {
        SPI_MEM_CS_HOLD_W::new(self, 6)
    }
    #[doc = "Bit 7 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_cs_setup(&mut self) -> SPI_MEM_CS_SETUP_W<SPI_MEM_USER_SPEC> {
        SPI_MEM_CS_SETUP_W::new(self, 7)
    }
    #[doc = "Bit 9 - The bit combined with SPI_MEM_CK_IDLE_EDGE bit to control SPI clock mode 0~3."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_ck_out_edge(&mut self) -> SPI_MEM_CK_OUT_EDGE_W<SPI_MEM_USER_SPEC> {
        SPI_MEM_CK_OUT_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_usr_dummy_idle(&mut self) -> SPI_MEM_USR_DUMMY_IDLE_W<SPI_MEM_USER_SPEC> {
        SPI_MEM_USR_DUMMY_IDLE_W::new(self, 26)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_usr_dummy(&mut self) -> SPI_MEM_USR_DUMMY_W<SPI_MEM_USER_SPEC> {
        SPI_MEM_USR_DUMMY_W::new(self, 29)
    }
}
#[doc = "SPI0 user register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_user::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_user::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_USER_SPEC;
impl crate::RegisterSpec for SPI_MEM_USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_user::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_USER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_user::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_USER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_USER to value 0"]
impl crate::Resettable for SPI_MEM_USER_SPEC {
    const RESET_VALUE: u32 = 0;
}
