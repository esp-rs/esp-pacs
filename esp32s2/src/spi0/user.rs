#[doc = "Register `USER` reader"]
pub type R = crate::R<USER_SPEC>;
#[doc = "Register `USER` writer"]
pub type W = crate::W<USER_SPEC>;
#[doc = "Field `CS_HOLD` reader - "]
pub type CS_HOLD_R = crate::BitReader;
#[doc = "Field `CS_HOLD` writer - "]
pub type CS_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_SETUP` reader - "]
pub type CS_SETUP_R = crate::BitReader;
#[doc = "Field `CS_SETUP` writer - "]
pub type CS_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_OUT_EDGE` reader - "]
pub type CK_OUT_EDGE_R = crate::BitReader;
#[doc = "Field `CK_OUT_EDGE` writer - "]
pub type CK_OUT_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWRITE_DUAL` reader - "]
pub type FWRITE_DUAL_R = crate::BitReader;
#[doc = "Field `FWRITE_DUAL` writer - "]
pub type FWRITE_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWRITE_QUAD` reader - "]
pub type FWRITE_QUAD_R = crate::BitReader;
#[doc = "Field `FWRITE_QUAD` writer - "]
pub type FWRITE_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWRITE_DIO` reader - "]
pub type FWRITE_DIO_R = crate::BitReader;
#[doc = "Field `FWRITE_DIO` writer - "]
pub type FWRITE_DIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FWRITE_QIO` reader - "]
pub type FWRITE_QIO_R = crate::BitReader;
#[doc = "Field `FWRITE_QIO` writer - "]
pub type FWRITE_QIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_MISO_HIGHPART` reader - "]
pub type USR_MISO_HIGHPART_R = crate::BitReader;
#[doc = "Field `USR_MISO_HIGHPART` writer - "]
pub type USR_MISO_HIGHPART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_MOSI_HIGHPART` reader - "]
pub type USR_MOSI_HIGHPART_R = crate::BitReader;
#[doc = "Field `USR_MOSI_HIGHPART` writer - "]
pub type USR_MOSI_HIGHPART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DUMMY_IDLE` reader - "]
pub type USR_DUMMY_IDLE_R = crate::BitReader;
#[doc = "Field `USR_DUMMY_IDLE` writer - "]
pub type USR_DUMMY_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_MOSI` reader - "]
pub type USR_MOSI_R = crate::BitReader;
#[doc = "Field `USR_MOSI` writer - "]
pub type USR_MOSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_MISO` reader - "]
pub type USR_MISO_R = crate::BitReader;
#[doc = "Field `USR_MISO` writer - "]
pub type USR_MISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DUMMY` reader - "]
pub type USR_DUMMY_R = crate::BitReader;
#[doc = "Field `USR_DUMMY` writer - "]
pub type USR_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_ADDR` reader - "]
pub type USR_ADDR_R = crate::BitReader;
#[doc = "Field `USR_ADDR` writer - "]
pub type USR_ADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_COMMAND` reader - "]
pub type USR_COMMAND_R = crate::BitReader;
#[doc = "Field `USR_COMMAND` writer - "]
pub type USR_COMMAND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cs_hold(&self) -> CS_HOLD_R {
        CS_HOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cs_setup(&self) -> CS_SETUP_R {
        CS_SETUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ck_out_edge(&self) -> CK_OUT_EDGE_R {
        CK_OUT_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn fwrite_dual(&self) -> FWRITE_DUAL_R {
        FWRITE_DUAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fwrite_quad(&self) -> FWRITE_QUAD_R {
        FWRITE_QUAD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fwrite_dio(&self) -> FWRITE_DIO_R {
        FWRITE_DIO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn fwrite_qio(&self) -> FWRITE_QIO_R {
        FWRITE_QIO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usr_miso_highpart(&self) -> USR_MISO_HIGHPART_R {
        USR_MISO_HIGHPART_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn usr_mosi_highpart(&self) -> USR_MOSI_HIGHPART_R {
        USR_MOSI_HIGHPART_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn usr_dummy_idle(&self) -> USR_DUMMY_IDLE_R {
        USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn usr_mosi(&self) -> USR_MOSI_R {
        USR_MOSI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn usr_miso(&self) -> USR_MISO_R {
        USR_MISO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn usr_dummy(&self) -> USR_DUMMY_R {
        USR_DUMMY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn usr_addr(&self) -> USR_ADDR_R {
        USR_ADDR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn usr_command(&self) -> USR_COMMAND_R {
        USR_COMMAND_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER")
            .field("usr_command", &self.usr_command())
            .field("usr_addr", &self.usr_addr())
            .field("usr_dummy", &self.usr_dummy())
            .field("usr_miso", &self.usr_miso())
            .field("usr_mosi", &self.usr_mosi())
            .field("usr_dummy_idle", &self.usr_dummy_idle())
            .field("usr_mosi_highpart", &self.usr_mosi_highpart())
            .field("usr_miso_highpart", &self.usr_miso_highpart())
            .field("fwrite_qio", &self.fwrite_qio())
            .field("fwrite_dio", &self.fwrite_dio())
            .field("fwrite_quad", &self.fwrite_quad())
            .field("fwrite_dual", &self.fwrite_dual())
            .field("ck_out_edge", &self.ck_out_edge())
            .field("cs_setup", &self.cs_setup())
            .field("cs_hold", &self.cs_hold())
            .finish()
    }
}
impl W {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cs_hold(&mut self) -> CS_HOLD_W<USER_SPEC> {
        CS_HOLD_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cs_setup(&mut self) -> CS_SETUP_W<USER_SPEC> {
        CS_SETUP_W::new(self, 7)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ck_out_edge(&mut self) -> CK_OUT_EDGE_W<USER_SPEC> {
        CK_OUT_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn fwrite_dual(&mut self) -> FWRITE_DUAL_W<USER_SPEC> {
        FWRITE_DUAL_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fwrite_quad(&mut self) -> FWRITE_QUAD_W<USER_SPEC> {
        FWRITE_QUAD_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fwrite_dio(&mut self) -> FWRITE_DIO_W<USER_SPEC> {
        FWRITE_DIO_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn fwrite_qio(&mut self) -> FWRITE_QIO_W<USER_SPEC> {
        FWRITE_QIO_W::new(self, 15)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usr_miso_highpart(&mut self) -> USR_MISO_HIGHPART_W<USER_SPEC> {
        USR_MISO_HIGHPART_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn usr_mosi_highpart(&mut self) -> USR_MOSI_HIGHPART_W<USER_SPEC> {
        USR_MOSI_HIGHPART_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn usr_dummy_idle(&mut self) -> USR_DUMMY_IDLE_W<USER_SPEC> {
        USR_DUMMY_IDLE_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn usr_mosi(&mut self) -> USR_MOSI_W<USER_SPEC> {
        USR_MOSI_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn usr_miso(&mut self) -> USR_MISO_W<USER_SPEC> {
        USR_MISO_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn usr_dummy(&mut self) -> USR_DUMMY_W<USER_SPEC> {
        USR_DUMMY_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn usr_addr(&mut self) -> USR_ADDR_W<USER_SPEC> {
        USR_ADDR_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn usr_command(&mut self) -> USR_COMMAND_W<USER_SPEC> {
        USR_COMMAND_W::new(self, 31)
    }
}
#[doc = "SPI Memory User Register\n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USER_SPEC;
impl crate::RegisterSpec for USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user::R`](R) reader structure"]
impl crate::Readable for USER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`user::W`](W) writer structure"]
impl crate::Writable for USER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USER to value 0"]
impl crate::Resettable for USER_SPEC {}
