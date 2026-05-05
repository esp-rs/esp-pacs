#[doc = "Register `USER` reader"]
pub type R = crate::R<USER_SPEC>;
#[doc = "Register `USER` writer"]
pub type W = crate::W<USER_SPEC>;
#[doc = "Field `LP_REG_DOUTDIN` reader - Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
pub type LP_REG_DOUTDIN_R = crate::BitReader;
#[doc = "Field `LP_REG_DOUTDIN` writer - Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
pub type LP_REG_DOUTDIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_TSCK_I_EDGE` reader - In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
pub type LP_REG_TSCK_I_EDGE_R = crate::BitReader;
#[doc = "Field `LP_REG_TSCK_I_EDGE` writer - In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
pub type LP_REG_TSCK_I_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_CS_HOLD` reader - spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
pub type LP_REG_CS_HOLD_R = crate::BitReader;
#[doc = "Field `LP_REG_CS_HOLD` writer - spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
pub type LP_REG_CS_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_CS_SETUP` reader - spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
pub type LP_REG_CS_SETUP_R = crate::BitReader;
#[doc = "Field `LP_REG_CS_SETUP` writer - spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
pub type LP_REG_CS_SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_RSCK_I_EDGE` reader - In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
pub type LP_REG_RSCK_I_EDGE_R = crate::BitReader;
#[doc = "Field `LP_REG_RSCK_I_EDGE` writer - In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
pub type LP_REG_RSCK_I_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_CK_OUT_EDGE` reader - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state."]
pub type LP_REG_CK_OUT_EDGE_R = crate::BitReader;
#[doc = "Field `LP_REG_CK_OUT_EDGE` writer - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state."]
pub type LP_REG_CK_OUT_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SIO` reader - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
pub type LP_REG_SIO_R = crate::BitReader;
#[doc = "Field `LP_REG_SIO` writer - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
pub type LP_REG_SIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_USR_MISO_HIGHPART` reader - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
pub type LP_REG_USR_MISO_HIGHPART_R = crate::BitReader;
#[doc = "Field `LP_REG_USR_MISO_HIGHPART` writer - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
pub type LP_REG_USR_MISO_HIGHPART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_USR_MOSI_HIGHPART` reader - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
pub type LP_REG_USR_MOSI_HIGHPART_R = crate::BitReader;
#[doc = "Field `LP_REG_USR_MOSI_HIGHPART` writer - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
pub type LP_REG_USR_MOSI_HIGHPART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_USR_DUMMY_IDLE` reader - spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
pub type LP_REG_USR_DUMMY_IDLE_R = crate::BitReader;
#[doc = "Field `LP_REG_USR_DUMMY_IDLE` writer - spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
pub type LP_REG_USR_DUMMY_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_USR_MOSI` reader - This bit enable the write-data phase of an operation. Can be configured in CONF state."]
pub type LP_REG_USR_MOSI_R = crate::BitReader;
#[doc = "Field `LP_REG_USR_MOSI` writer - This bit enable the write-data phase of an operation. Can be configured in CONF state."]
pub type LP_REG_USR_MOSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_USR_MISO` reader - This bit enable the read-data phase of an operation. Can be configured in CONF state."]
pub type LP_REG_USR_MISO_R = crate::BitReader;
#[doc = "Field `LP_REG_USR_MISO` writer - This bit enable the read-data phase of an operation. Can be configured in CONF state."]
pub type LP_REG_USR_MISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_USR_DUMMY` reader - This bit enable the dummy phase of an operation. Can be configured in CONF state."]
pub type LP_REG_USR_DUMMY_R = crate::BitReader;
#[doc = "Field `LP_REG_USR_DUMMY` writer - This bit enable the dummy phase of an operation. Can be configured in CONF state."]
pub type LP_REG_USR_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_USR_ADDR` reader - This bit enable the address phase of an operation. Can be configured in CONF state."]
pub type LP_REG_USR_ADDR_R = crate::BitReader;
#[doc = "Field `LP_REG_USR_ADDR` writer - This bit enable the address phase of an operation. Can be configured in CONF state."]
pub type LP_REG_USR_ADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_USR_COMMAND` reader - This bit enable the command phase of an operation. Can be configured in CONF state."]
pub type LP_REG_USR_COMMAND_R = crate::BitReader;
#[doc = "Field `LP_REG_USR_COMMAND` writer - This bit enable the command phase of an operation. Can be configured in CONF state."]
pub type LP_REG_USR_COMMAND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_doutdin(&self) -> LP_REG_DOUTDIN_R {
        LP_REG_DOUTDIN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
    #[inline(always)]
    pub fn lp_reg_tsck_i_edge(&self) -> LP_REG_TSCK_I_EDGE_R {
        LP_REG_TSCK_I_EDGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_cs_hold(&self) -> LP_REG_CS_HOLD_R {
        LP_REG_CS_HOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_cs_setup(&self) -> LP_REG_CS_SETUP_R {
        LP_REG_CS_SETUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
    #[inline(always)]
    pub fn lp_reg_rsck_i_edge(&self) -> LP_REG_RSCK_I_EDGE_R {
        LP_REG_RSCK_I_EDGE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_ck_out_edge(&self) -> LP_REG_CK_OUT_EDGE_R {
        LP_REG_CK_OUT_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_sio(&self) -> LP_REG_SIO_R {
        LP_REG_SIO_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_miso_highpart(&self) -> LP_REG_USR_MISO_HIGHPART_R {
        LP_REG_USR_MISO_HIGHPART_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_mosi_highpart(&self) -> LP_REG_USR_MOSI_HIGHPART_R {
        LP_REG_USR_MOSI_HIGHPART_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_dummy_idle(&self) -> LP_REG_USR_DUMMY_IDLE_R {
        LP_REG_USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_mosi(&self) -> LP_REG_USR_MOSI_R {
        LP_REG_USR_MOSI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_miso(&self) -> LP_REG_USR_MISO_R {
        LP_REG_USR_MISO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_dummy(&self) -> LP_REG_USR_DUMMY_R {
        LP_REG_USR_DUMMY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_addr(&self) -> LP_REG_USR_ADDR_R {
        LP_REG_USR_ADDR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit enable the command phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_command(&self) -> LP_REG_USR_COMMAND_R {
        LP_REG_USR_COMMAND_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER")
            .field("lp_reg_doutdin", &self.lp_reg_doutdin())
            .field("lp_reg_tsck_i_edge", &self.lp_reg_tsck_i_edge())
            .field("lp_reg_cs_hold", &self.lp_reg_cs_hold())
            .field("lp_reg_cs_setup", &self.lp_reg_cs_setup())
            .field("lp_reg_rsck_i_edge", &self.lp_reg_rsck_i_edge())
            .field("lp_reg_ck_out_edge", &self.lp_reg_ck_out_edge())
            .field("lp_reg_sio", &self.lp_reg_sio())
            .field("lp_reg_usr_miso_highpart", &self.lp_reg_usr_miso_highpart())
            .field("lp_reg_usr_mosi_highpart", &self.lp_reg_usr_mosi_highpart())
            .field("lp_reg_usr_dummy_idle", &self.lp_reg_usr_dummy_idle())
            .field("lp_reg_usr_mosi", &self.lp_reg_usr_mosi())
            .field("lp_reg_usr_miso", &self.lp_reg_usr_miso())
            .field("lp_reg_usr_dummy", &self.lp_reg_usr_dummy())
            .field("lp_reg_usr_addr", &self.lp_reg_usr_addr())
            .field("lp_reg_usr_command", &self.lp_reg_usr_command())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_doutdin(&mut self) -> LP_REG_DOUTDIN_W<'_, USER_SPEC> {
        LP_REG_DOUTDIN_W::new(self, 0)
    }
    #[doc = "Bit 5 - In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
    #[inline(always)]
    pub fn lp_reg_tsck_i_edge(&mut self) -> LP_REG_TSCK_I_EDGE_W<'_, USER_SPEC> {
        LP_REG_TSCK_I_EDGE_W::new(self, 5)
    }
    #[doc = "Bit 6 - spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_cs_hold(&mut self) -> LP_REG_CS_HOLD_W<'_, USER_SPEC> {
        LP_REG_CS_HOLD_W::new(self, 6)
    }
    #[doc = "Bit 7 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_cs_setup(&mut self) -> LP_REG_CS_SETUP_W<'_, USER_SPEC> {
        LP_REG_CS_SETUP_W::new(self, 7)
    }
    #[doc = "Bit 8 - In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
    #[inline(always)]
    pub fn lp_reg_rsck_i_edge(&mut self) -> LP_REG_RSCK_I_EDGE_W<'_, USER_SPEC> {
        LP_REG_RSCK_I_EDGE_W::new(self, 8)
    }
    #[doc = "Bit 9 - the bit combined with spi_mosi_delay_mode bits to set mosi signal delay mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_ck_out_edge(&mut self) -> LP_REG_CK_OUT_EDGE_W<'_, USER_SPEC> {
        LP_REG_CK_OUT_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 17 - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_sio(&mut self) -> LP_REG_SIO_W<'_, USER_SPEC> {
        LP_REG_SIO_W::new(self, 17)
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_miso_highpart(&mut self) -> LP_REG_USR_MISO_HIGHPART_W<'_, USER_SPEC> {
        LP_REG_USR_MISO_HIGHPART_W::new(self, 24)
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer spi_w8~spi_w15. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_mosi_highpart(&mut self) -> LP_REG_USR_MOSI_HIGHPART_W<'_, USER_SPEC> {
        LP_REG_USR_MOSI_HIGHPART_W::new(self, 25)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_dummy_idle(&mut self) -> LP_REG_USR_DUMMY_IDLE_W<'_, USER_SPEC> {
        LP_REG_USR_DUMMY_IDLE_W::new(self, 26)
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_mosi(&mut self) -> LP_REG_USR_MOSI_W<'_, USER_SPEC> {
        LP_REG_USR_MOSI_W::new(self, 27)
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_miso(&mut self) -> LP_REG_USR_MISO_W<'_, USER_SPEC> {
        LP_REG_USR_MISO_W::new(self, 28)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_dummy(&mut self) -> LP_REG_USR_DUMMY_W<'_, USER_SPEC> {
        LP_REG_USR_DUMMY_W::new(self, 29)
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_addr(&mut self) -> LP_REG_USR_ADDR_W<'_, USER_SPEC> {
        LP_REG_USR_ADDR_W::new(self, 30)
    }
    #[doc = "Bit 31 - This bit enable the command phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_command(&mut self) -> LP_REG_USR_COMMAND_W<'_, USER_SPEC> {
        LP_REG_USR_COMMAND_W::new(self, 31)
    }
}
#[doc = "SPI USER control register\n\nYou can [`read`](crate::Reg::read) this register and get [`user::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
