#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2_SPEC>;
#[doc = "Field `SSI` reader - "]
pub type SSI_R = crate::BitReader;
#[doc = "Field `SSI` writer - "]
pub type SSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` reader - "]
pub type SSM_R = crate::BitReader;
#[doc = "Field `SSM` writer - "]
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXONLY` reader - "]
pub type RXONLY_R = crate::BitReader;
#[doc = "Field `RXONLY` writer - "]
pub type RXONLY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCNEXT` reader - "]
pub type CRCNEXT_R = crate::BitReader;
#[doc = "Field `CRCNEXT` writer - "]
pub type CRCNEXT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CECEN` reader - "]
pub type CECEN_R = crate::BitReader;
#[doc = "Field `CECEN` writer - "]
pub type CECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDOE` reader - "]
pub type BDOE_R = crate::BitReader;
#[doc = "Field `BDOE` writer - "]
pub type BDOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDM` reader - "]
pub type BDM_R = crate::BitReader;
#[doc = "Field `BDM` writer - "]
pub type BDM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxonly(&self) -> RXONLY_R {
        RXONLY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn crcnext(&self) -> CRCNEXT_R {
        CRCNEXT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bdoe(&self) -> BDOE_R {
        BDOE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bdm(&self) -> BDM_R {
        BDM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ssi(&mut self) -> SSI_W<CR2_SPEC> {
        SSI_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<CR2_SPEC> {
        SSM_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rxonly(&mut self) -> RXONLY_W<CR2_SPEC> {
        RXONLY_W::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn crcnext(&mut self) -> CRCNEXT_W<CR2_SPEC> {
        CRCNEXT_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cecen(&mut self) -> CECEN_W<CR2_SPEC> {
        CECEN_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn bdoe(&mut self) -> BDOE_W<CR2_SPEC> {
        BDOE_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn bdm(&mut self) -> BDM_W<CR2_SPEC> {
        BDM_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
